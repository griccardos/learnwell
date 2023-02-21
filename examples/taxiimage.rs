use std::time::Duration;

use environments::taxi::TaxiEnvironment;
use learnwell::{
    agent::deepqlearning::{DeepQLearning, DeepQLearningConfig},
    runner::{DisplayConfig, Runner},
    strategy::decliningrandom::DecliningRandom,
};
mod environments;
mod nnbackends;
use nnbackends::runntbackend::RunntBackend;

fn main() {
    let epochs = 5000;
    fastrand::seed(0);
    let mut env = TaxiEnvironment::default();
    let config = DeepQLearningConfig {
        nn_learning_rate: 0.005,
        replay_size: 128,
        nn_batch_size: 2,
        discount: 0.9,
        train_steps: 1,
        copy_nn_steps: 40,
        q_learning_rate: 0.2,
        history_size: 50000,
    };

    //function to create Neural network
    let nnf = |shape: &Vec<usize>, lr: f32| RunntBackend::new(&shape, lr);
    //let nnf = |shape: &Vec<usize>, lr: f32| TchBackend::new(&shape, lr);

    let agent = DeepQLearning::new(
        nnf,
        &[64, 32],
        config,
        DecliningRandom::new((epochs as f64 * 0.9) as usize, 0.005),
        &mut env,
    );

    //Runner::run(agent, env, epochs);
    Runner::run_with_display(
        agent,
        env,
        epochs,
        DisplayConfig {
            step_time: Duration::from_millis(50),
            step_time_start: 4990,
            ..Default::default()
        },
    );
}
