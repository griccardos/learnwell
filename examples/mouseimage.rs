mod environments;
mod nnbackends;

use environments::mouse::MouseEnvironment;
use learnwell::{
    agent::deepqlearning::{DeepQLearning, DeepQLearningConfig},
    runner::Runner,
    strategy::decliningrandom::DecliningRandom,
};

use nnbackends::runntbackend::RunntBackend;

fn main() {
    let epochs = 3000;
    fastrand::seed(0);
    let mut env = MouseEnvironment::default();
    let config = DeepQLearningConfig {
        nn_learning_rate: 0.1,
        replay_size: 32,
        nn_batch_size: 16,
        discount: 0.9,
        train_steps: 1,
        copy_nn_steps: 20,
        q_learning_rate: 0.1,
        history_size: 10000,
    };

    let nn = |shape: &Vec<usize>, lr: f32| RunntBackend::new(&shape, lr);

    //uncomment to use torch
    //use nnbackends::tchbackend::TchBackend;
    //let nn = |shape: &Vec<usize>, lr: f32| TchBackend::new(&shape, lr);

    let agent = DeepQLearning::new(
        nn,
        &[32],
        config,
        DecliningRandom::new((epochs as f64 * 0.9) as usize, 0.005),
        &mut env,
    );

    Runner::run(agent, env, epochs);
}
