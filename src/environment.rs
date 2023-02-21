use show_image::ImageView;

pub trait Environment<S, A> {
    /// Gets the state
    fn state(&self) -> S;
    /// Reset the state to starting state
    fn reset(&mut self, epoch: usize);
    /// ALL actions
    fn all_actions(&self) -> Vec<A>;
    /// Modify state with action, and return reward
    fn take_action_get_reward(&mut self, action: &A) -> f64;
    /// Should we stop based on state or step count
    fn should_stop(&mut self, step: usize) -> bool;
    ///if you wish to display environment, or use deep qlearning, we implement this. otherwise return default
    fn get_image(&mut self) -> ImageView;
}
