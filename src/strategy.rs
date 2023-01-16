pub mod decliningrandom;

pub trait ExploreStrategy<A> {
    //to move to strategy trait
    fn pick_action(
        &mut self,
        actions: &[A],
        best: Option<A>, //best based on qlearning
        step: usize,
        epoch: usize,
    ) -> A;
}
