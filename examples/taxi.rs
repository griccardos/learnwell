use environments::taxi::TaxiEnvironment;
use learnwell::{
    agent::qlearning::QLearning, runner::Runner, strategy::decliningrandom::DecliningRandom,
};
mod environments;

fn main() {
    // now we train
    let epochs = 400;
    Runner::run(
        QLearning::new(0.1, 0.98, DecliningRandom::new(epochs, 0.01)),
        TaxiEnvironment::default(),
        epochs,
    );
}
