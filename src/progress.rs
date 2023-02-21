#[derive(Clone, Copy)]
pub struct Progress {
    pub epoch: usize,
    pub epoch_step: usize,
    pub cumulative_steps: usize,
}
