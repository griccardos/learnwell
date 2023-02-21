use crate::{environment::Environment, progress::Progress};

pub mod deepqlearning;
pub mod nnbackend;
pub mod qlearning;

///agent which deals with state
pub trait Agent<S, A> {
    fn step(&mut self, progress: Progress, env: &mut dyn Environment<S, A>) -> bool;

    fn pick_action(
        &mut self,
        actions: &[A],
        best: Option<A>, //best based on qlearning
        progress: Progress,
    ) -> A;
}
