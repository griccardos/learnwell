use show_image::ImageView;

pub trait Environment<S, A> {
    /// Gets the state
    fn state(&self) -> S;
    ///Reset the state to starting state
    fn reset(&mut self, epoch: usize);
    ///Get valid actions for this state
    fn actions(&self) -> Vec<A>;
    /// Modify state with action, and return reward
    fn take_action_get_reward(&mut self, action: &A) -> f64;
    /// Should we stop based on state or step count
    fn should_stop(&mut self, step: usize) -> bool;
}

pub trait EnvironmentDisplay {
    fn get_image(&self) -> ImageView;
}
