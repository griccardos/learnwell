use super::ExploreStrategy;

pub struct DecliningRandom {
    total: usize,
    current: usize,
    min_exploration: f64,
}

impl DecliningRandom {
    pub fn new(total: usize, min_exploration: f64) -> Self {
        Self {
            total,
            min_exploration,
            current: 0,
        }
    }
    pub fn exploration_rate(&self) -> f64 {
        let percent_done = self.current as f64 / self.total as f64;
        let exploration = 1. - percent_done;
        exploration.max(self.min_exploration)
    }
}

impl<A: Clone> ExploreStrategy<A> for DecliningRandom {
    fn pick_action(
        &mut self,
        actions: &[A],
        best: Option<A>, //best based on qlearning
        epoch: usize,
        _step: usize,
    ) -> A {
        self.current = epoch;
        //random if exploring or no best, else choose best
        let exploration = self.exploration_rate();
        let must_explore = fastrand::f64() < exploration;
        match (must_explore, best) {
            (true, _) | (false, None) => {
                let i = fastrand::usize(0..actions.len());
                actions[i].to_owned()
            }
            (false, Some(best)) => best,
        }
    }
}
