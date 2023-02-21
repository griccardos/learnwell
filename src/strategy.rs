use crate::progress::Progress;

pub mod decliningrandom;

pub trait ExploreStrategy<A> {
    ///The action we pick based on `best` action there is, and `Progress`
    fn pick_action(
        &mut self,
        actions: &[A],
        best: Option<A>, //best based on qlearning etc.
        progress: Progress,
    ) -> A;
}
