use environments::mouse::MouseEnvironment;
use learnwell::{
    agent::qlearning::QLearning, runner::Runner, strategy::decliningrandom::DecliningRandom,
};
mod environments;

fn main() {
    let epochs = 500;

    let agent = QLearning::new(
        0.5,
        0.5,
        DecliningRandom::new((epochs as f64 * 0.9) as usize, 0.005),
    );

    Runner::run(agent, MouseEnvironment::default(), epochs);
}
