use environments::hike::Hike;
use learnwell::{
    agent::qlearning::QLearning,
    runner::{DisplayConfig, Runner},
    strategy::decliningrandom::DecliningRandom,
};

mod environments;

fn main() {
    let epochs = 700_000;

    Runner::run_with_display(
        QLearning::new(0.2, 0.99, DecliningRandom::new(epochs, 0.005)),
        Hike::new(),
        epochs,
        DisplayConfig::default(),
    );
}
