pub trait NNBackend {
    fn update_from(&mut self, other: &Self);
    fn forward(&mut self, input: &[f32]) -> Vec<f32>;
    fn fit(&mut self, inputs: &[Vec<f32>], outputs: &[Vec<f32>], batch_size: usize);
}
