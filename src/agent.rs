use crate::environment::Environment;
pub mod qlearning;

pub trait Agent<S, A> {
    fn train(&mut self, epoch: usize, env: &mut dyn Environment<S, A>) -> TrainResult;

    fn pick_action(
        &mut self,
        actions: &[A],
        best: Option<A>, //best based on qlearning
        step: usize,
        epoch: usize,
    ) -> A;
}

pub struct TrainResult {
    pub steps: usize,
    pub stats: String,
}
