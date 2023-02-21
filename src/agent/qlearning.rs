use fxhash::FxHasher;

use crate::{
    agent::Agent, environment::Environment, progress::Progress, strategy::ExploreStrategy,
};
use core::hash::Hash;
use std::{collections::HashMap, hash::BuildHasherDefault};
pub struct QLearning<S, A> {
    pub qtable:
        HashMap<S, HashMap<A, f64, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>,
    alpha: f64,
    gamma: f64,
    strategy: Box<(dyn ExploreStrategy<A> + Send)>,
}

impl<S, A> QLearning<S, A> {
    /// alpha is the learning rate e.g 0.1
    /// gamma is the discount ratio e.g. 0.99 keeps 99% of historical value
    pub fn new(alpha: f64, gamma: f64, strategy: impl ExploreStrategy<A> + Send + 'static) -> Self {
        Self {
            qtable: HashMap::default(),
            alpha,
            gamma,
            strategy: Box::new(strategy),
        }
    }
}

impl<S, A> Agent<S, A> for QLearning<S, A>
where
    S: Clone + Hash + Eq,
    A: Clone + Hash + Eq,
{
    fn pick_action(
        &mut self,
        actions: &[A],
        best: Option<A>, //best based on qlearning
        progress: Progress,
    ) -> A {
        self.strategy.pick_action(actions, best, progress)
    }

    /// Trains 1 epoch
    fn step(&mut self, progress: Progress, env: &mut dyn Environment<S, A>) -> bool {
        let current_state = env.state();
        let best = self.qtable.get(&current_state).and_then(|x| {
            x.iter()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .map(|x| x.0.to_owned())
        });
        let actions = env.all_actions();
        if actions.is_empty() {
            return true;
        }
        let action = self.strategy.pick_action(&actions, best, progress);

        let default_value = 0.;
        let oldq = self
            .qtable
            .get(&current_state)
            .and_then(|x| x.get(&action))
            .unwrap_or(&default_value);

        let reward = env.take_action_get_reward(&action);
        let new_state = env.state();
        let done = env.should_stop(progress.epoch_step);

        let maxq_at_new_state = self
            .qtable
            .get(&new_state)
            .and_then(|x| x.values().max_by(|a, b| a.partial_cmp(b).unwrap()))
            .unwrap_or(&default_value);

        let newq = oldq + self.alpha * (reward + self.gamma * maxq_at_new_state - oldq);

        self.qtable
            .entry(current_state.clone())
            .or_insert_with(HashMap::default)
            .insert(action.clone(), newq);

        done
    }
}
