use crate::{environment::Environment, progress::Progress, strategy::ExploreStrategy};
use core::hash::Hash;
use std::collections::VecDeque;

use super::{nnbackend::NNBackend, Agent};

/// Here we use neural network to predict actions
/// We use display (image) as input
/// We save history of states and actions as generated by NN
/// We then train NN at end of epoch
/// Not every optimisation is made, but we do use 2 nn, using first to make predictions,
/// and the second to predict future Q, which we only update periodically, so as to stabilise (and speed up) the learning
pub struct DeepQLearning<A, NB> {
    nn: NB,        //make predictions
    nn_target: NB, //used to calculate target Q, updated periodically
    config: DeepQLearningConfig,
    strategy: Box<(dyn ExploreStrategy<A> + Send)>,
    history: VecDeque<Replay>,
}

pub struct Replay {
    state: Vec<f32>,
    action_index: usize,
    next_state: Vec<f32>, //result of action
    reward: f32,
    done: bool,
}

pub struct DeepQLearningConfig {
    /// learning rate for neural net
    pub nn_learning_rate: f32,
    /// size of batch (this would be <= replay size)
    /// Use 1 for stochastic, or larger for mini batch gradient descent
    pub nn_batch_size: usize,
    ///how often to train
    pub train_steps: usize,
    ///how often to copy main nn to target nn
    pub copy_nn_steps: usize,

    /// size of replay history to train on
    pub replay_size: usize,

    ///alpha
    pub q_learning_rate: f32,
    ///gamma
    pub discount: f32,
    /// replay history size
    pub history_size: usize,
}

impl Default for DeepQLearningConfig {
    fn default() -> Self {
        Self {
            replay_size: 64,
            nn_batch_size: 16,
            train_steps: 10,
            copy_nn_steps: 80,
            discount: 0.9,
            q_learning_rate: 0.1,
            nn_learning_rate: 0.01,
            history_size: 10000,
        }
    }
}

impl<A, NB: NNBackend> DeepQLearning<A, NB> {
    /// nnf is the function used to create the neural network which conforms to traint NNBackend
    /// It passes in the `shape` of all layers including input, hidden and output, and `learning_rate` as per config
    /// `hidden_sizes` is a list of sizes of hidden layers
    /// We pass in the environment so that we can auto calculated the input size (based on image) and output sizes (based on actions)
    pub fn new<S, F: Fn(&Vec<usize>, f32) -> NB>(
        nnf: F,
        hidden_sizes: &[usize],
        config: DeepQLearningConfig,
        strategy: impl ExploreStrategy<A> + Send + 'static,
        env: &mut dyn Environment<S, A>,
    ) -> Self {
        env.reset(1);
        let input_state = env.get_image().data().len();
        let action_count = env.all_actions().len();

        let shape: Vec<usize> = vec![vec![input_state], hidden_sizes.to_vec(), vec![action_count]]
            .into_iter()
            .flatten()
            .collect();

        let nn = nnf(&shape, config.nn_learning_rate);
        let mut nn_target = nnf(&shape, config.nn_learning_rate);

        nn_target.update_from(&nn);

        Self {
            nn,
            nn_target,
            strategy: Box::new(strategy),
            history: VecDeque::new(),
            config,
        }
    }

    pub fn train_nn(&mut self) {
        if self.history.len() < self.config.replay_size {
            return;
        }

        //run fit `batch_count` times
        let (batch_inputs, batch_outputs) = self.get_training();
        self.nn
            .fit(&batch_inputs, &batch_outputs, self.config.nn_batch_size);
    }

    fn get_training(&mut self) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
        let selected: Vec<usize> =
            std::iter::repeat_with(|| fastrand::usize(0..self.history.len()))
                .take(self.config.replay_size)
                .collect();

        let mut set: Vec<&Replay> = vec![];
        selected.iter().for_each(|&x| set.push(&self.history[x]));

        let mut batch_inputs = vec![];
        let mut batch_outputs = vec![];
        for item in set {
            let input = &item.state;

            //next reward
            let next_reward;
            if item.done {
                next_reward = 0.; //there is no next if we are done
            } else {
                //we use nn_target to predict
                let mut outputs: Vec<f32> = self.nn_target.forward(&item.next_state);
                //let mut outputs: Vec<f32> = self.nn.forward(&item.next_state);
                outputs.sort_by(|a, b| a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal));
                next_reward = outputs.into_iter().last().unwrap_or_default();
            }

            //target reward
            let target_reward = item.reward + self.config.discount * next_reward;

            //predicted next - here we use the same for the others, but change the target action to be the best next
            let mut predicted_next = self.nn.forward(&input);

            let old_reward = predicted_next[item.action_index];
            let diff = target_reward - old_reward;
            let change = diff * self.config.q_learning_rate;

            predicted_next[item.action_index] += change;
            let output = &predicted_next;

            batch_inputs.push(input.to_owned());
            batch_outputs.push(output.to_owned());
        }
        (batch_inputs, batch_outputs)
    }

    fn get_best_action<S>(&mut self, env: &mut dyn Environment<S, A>) -> A
    where
        A: Clone + Hash + Eq,
    {
        let current_state = self.get_state(env);
        let mut output: Vec<(usize, f32)> = self
            .nn
            .forward(&current_state)
            .into_iter()
            .enumerate()
            .collect();
        output.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        let best_index = output.last().unwrap().0;

        let best = env.all_actions()[best_index].clone();
        best
    }

    fn get_state<S>(&self, env: &mut dyn Environment<S, A>) -> Vec<f32> {
        env.get_image().data().iter().map(|x| *x as f32).collect()
    }
}

impl<S, A, NB> Agent<S, A> for DeepQLearning<A, NB>
where
    A: Clone + Hash + Eq,
    NB: NNBackend,
{
    fn pick_action(
        &mut self,
        actions: &[A],
        best: Option<A>, //best based on qlearning
        progress: Progress,
    ) -> A {
        self.strategy.pick_action(actions, best, progress)
    }

    /// Trains every N number of steps
    fn step(&mut self, progress: Progress, env: &mut dyn Environment<S, A>) -> bool {
        let best = self.get_best_action(env);
        let actions = env.all_actions();
        if actions.is_empty() {
            return true;
        }

        let action = self.strategy.pick_action(&actions, Some(best), progress);
        let action_index = env
            .all_actions()
            .iter()
            .position(|x| x == &action)
            .unwrap()
            .to_owned();
        let state = self.get_state(env);
        //take action
        let reward = env.take_action_get_reward(&action) as f32;
        let done = env.should_stop(progress.epoch_step);
        let next_state = self.get_state(env);
        //now save to history
        self.history.push_back(Replay {
            state,
            action_index,
            next_state,
            reward,
            done,
        });

        if self.history.len() > self.config.history_size {
            self.history.pop_front();
        }

        if progress.cumulative_steps % self.config.train_steps == 0 {
            self.train_nn();
        }

        if progress.cumulative_steps % self.config.copy_nn_steps == 0 {
            self.nn_target.update_from(&self.nn);
        }

        done
    }
}
