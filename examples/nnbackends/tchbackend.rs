// Requires libtorch to be installed
// Usually it uses system version, else will try download.
// check tch crate for full instructions to build
// This remains commented otherwise torch will be required to run examples.

// Uncomment below and cargo.toml to include torch
/*
use learnwell::agent::nnbackend::NNBackend;
use tch::{nn, nn::Module, nn::OptimizerConfig, Tensor};

pub struct TchBackend {
    net: nn::Sequential,
    opt: nn::Optimizer,
    vs: nn::VarStore,
}

impl TchBackend {
    pub fn new(shape: &[usize], lr: f32) -> Self {
        let vs = nn::VarStore::new(tch::Device::Cpu);
        let mut net = nn::seq();
        //add input
        net = net.add(nn::linear(
            vs.root() / "Input Layer",
            shape[0] as i64,
            shape[1] as i64,
            Default::default(),
        ));
        //add hidden
        for i in 1..shape.len() - 1 {
            net = net.add_fn(|xs| xs.sigmoid());
            if i < shape.len() - 2 {
                net = net.add(nn::linear(
                    vs.root() / &format!("layer {}", i + 1),
                    shape[i] as i64,
                    shape[i + 1] as i64,
                    Default::default(),
                ));
            }
        }
        //add output
        let lasti = shape.len() - 1;
        net = net.add(nn::linear(
            vs.root() / "Output",
            shape[lasti - 1] as i64,
            shape[lasti] as i64,
            Default::default(),
        ));

        let opt = nn::Adam::default().build(&vs, lr as f64).unwrap();

        Self { net, opt, vs }
    }
}

impl NNBackend for TchBackend {
    fn forward(&mut self, input: &[f32]) -> Vec<f32> {
        let t = self.net.forward(&Tensor::of_slice(input));
        t.into()
    }

    fn fit(&mut self, inputs: &[Vec<f32>], outputs: &[Vec<f32>], _batch_size: usize) {
        /*for (inp, out) in inputs.iter().zip(outputs) {
            let loss = self
                .net
                .forward(&Tensor::of_slice(inp))
                //.huber_loss(&Tensor::of_slice(outputs), tch::Reduction::Sum,1.);
                .mse_loss(&Tensor::of_slice(&out), tch::Reduction::Mean);
            self.opt.backward_step(&loss);
        }*/
        let loss = self
            .net
            .forward(&Tensor::of_slice2(&inputs))
            //.huber_loss(&Tensor::of_slice(outputs), tch::Reduction::Sum,1.);
            .mse_loss(&Tensor::of_slice2(&outputs), tch::Reduction::Mean);
        self.opt.backward_step(&loss);
    }

    fn update_from(&mut self, other: &Self) {
        let path = std::path::Path::new("c:/temp/tch.txt");
        other.vs.save(path).unwrap();
        self.vs.load(path).unwrap();
    }
}
*/
