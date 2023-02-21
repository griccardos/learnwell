use learnwell::agent::nnbackend::NNBackend;

pub struct RunntBackend {
    nn: runnt::nn::NN,
}

impl RunntBackend {
    pub fn new(network_shape: &Vec<usize>, learning_rate: f32) -> Self {
        Self {
            nn: runnt::nn::NN::new(network_shape)
                .with_hidden_type(runnt::activation::ActivationType::Sigmoid)
                .with_output_type(runnt::activation::ActivationType::Linear)
                .with_learning_rate(learning_rate),
        }
    }
}

impl NNBackend for RunntBackend {
    fn update_from(&mut self, other: &Self) {
        self.nn.set_weights(&other.nn.get_weights());
    }

    fn forward(&mut self, input: &[f32]) -> Vec<f32> {
        self.nn.forward(input)
    }

    fn fit(&mut self, inputs: &[Vec<f32>], outputs: &[Vec<f32>], batch_size: usize) {
        /*//stochastic gradient descent on each item seems to work better than batch descent
        for (inp, out) in inputs.iter().zip(outputs) {
            self.nn.fit_one(inp, &out);
        }*/

        let is = inputs.iter().map(|x| x.as_slice()).collect::<Vec<_>>();
        let os = outputs.iter().map(|x| x.as_slice()).collect::<Vec<_>>();
        self.nn.fit_batch_size(&is, &os, batch_size);
    }
}
