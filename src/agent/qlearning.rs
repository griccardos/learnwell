use fxhash::FxHasher;

use crate::{agent::Agent, environment::Environment, strategy::ExploreStrategy};
use core::hash::Hash;
use std::{collections::HashMap, hash::BuildHasherDefault};

use super::TrainResult;
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
        step: usize,
        epoch: usize,
    ) -> A {
        self.strategy.pick_action(actions, best, step, epoch)
    }

    /// Trains 1 epoch
    fn train(&mut self, epoch: usize, env: &mut dyn Environment<S, A>) -> super::TrainResult {
        let mut done = false;
        let mut step = 0;
        while !done {
            step += 1;

            let current_state = env.state();
            let best = self.qtable.get(&current_state).and_then(|x| {
                x.iter()
                    .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                    .map(|x| x.0.to_owned())
            });
            let actions = env.actions();
            if actions.is_empty() {
                break;
            }
            let action = self.strategy.pick_action(&actions, best, epoch, step);

            let default_value = 0.;
            let oldq = self
                .qtable
                .get(&current_state)
                .and_then(|x| x.get(&action))
                .unwrap_or(&default_value);

            let reward = env.take_action_get_reward(&action);
            let new_state = env.state();
            done = env.should_stop(step);

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
        }

        //return results
        let num = self.qtable.values().map(|a| a.iter().len()).sum::<usize>();
        let stats = format!(
            "Qcount:{} Qavg:{}",
            self.qtable.len(),
            self.qtable
                .values()
                .map(|a| a.iter().map(|b| b.1).sum::<f64>())
                .sum::<f64>()
                / num as f64,
        );

        TrainResult { steps: step, stats }
    }
}
