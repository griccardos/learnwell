// Taxi based on OpenAi Gym
// Taxi must pick up from one of the locations R,Y,G,B and drop off at one of the others
// Dont drive into walls
// End if drop off at the correct location
//    R: | : :G
//     : : : :
//     : : : :
//     | : | :
//    Y| : |B:
use learnwell::{
    agent::qlearning::QLearning, environment::Environment, runner::Runner,
    strategy::decliningrandom::DecliningRandom,
};

fn main() {
    // We need a few things to run
    //1. State Struct = TaxiState
    //2. Action (normally enum) = TaxiAction
    //3. Environment Struct that implements the Environment trait = TaxiEnvironment

    //4. the Algorithm,
    //in this case:
    // - QLearning
    // - with exploration strategy where random selection decreases towards experience

    // now we train
    let epochs = 400;
    Runner::run(
        QLearning::new(0.1, 0.98, DecliningRandom::new(epochs, 0.01)),
        TaxiEnvironment::default(),
        epochs,
    );
}

/// Step 1. State - must impl Hash, Eq, PartialEq, Clone
///This is what gets saved to QTable, so make sure it is small set of states
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct TaxiState {
    taxi: Point,
    dropoff: Point,
    passenger: Point,
    in_taxi: bool,
}
//we implement a default so we can reset state using default
impl Default for TaxiState {
    fn default() -> Self {
        //starting points [r,c]
        let points = [
            Point { y: 0, x: 0 },
            Point { y: 0, x: 4 },
            Point { y: 4, x: 0 },
            Point { y: 4, x: 3 },
        ];
        let passenger = points[fastrand::usize(0..points.len())].clone();
        let dropoff;
        loop {
            let temp = points[fastrand::usize(0..points.len())].clone();
            if temp.x != passenger.x || temp.y != passenger.y {
                dropoff = temp;
                break;
            }
        }
        let taxi = Point {
            x: fastrand::usize(0..5),
            y: fastrand::usize(0..5),
        };
        TaxiState {
            taxi,
            dropoff,
            passenger,
            in_taxi: false,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

/// Step 2. Action - must impl Hash, Eq, PartialEq, Clone
/// These are actions that can be performed
#[derive(Clone, Hash, PartialEq, Eq)]
pub enum TaxiAction {
    Up,
    Down,
    Left,
    Right,
    Dropoff,
    Pickup,
}

/// Step 3. Environment
/// This is the critical part that acts and communicates with the agent
/// You just need to implement the required traits in Environment correctly
/// Optionally add some helper methods to make things easier
///
/// -`reset` is called at the BEGINNING of each epoch
/// - Agent will pick an action for us
/// - we then use the action in `take_action_get_reward` step and return the reward
/// - we then tell agent if we should stop or not

#[derive(Default)]
pub struct TaxiEnvironment {
    found: usize,
    state: TaxiState,
}
//3.1 implement the `Environment` trait for the struct
impl Environment<TaxiState, TaxiAction> for TaxiEnvironment {
    fn state(&self) -> TaxiState {
        self.state.clone()
    }
    fn reset(&mut self, epoch: usize) {
        self.state = TaxiState::default();

        let update = 20;
        if epoch % update == 0 {
            println!(
                "{epoch}: found {:.2}%",
                100. * self.found as f32 / update as f32
            );
            self.found = 0;
        }
    }

    fn actions(&self) -> Vec<TaxiAction> {
        vec![
            TaxiAction::Up,
            TaxiAction::Down,
            TaxiAction::Left,
            TaxiAction::Right,
            TaxiAction::Dropoff,
            TaxiAction::Pickup,
        ]
    }
    //    R: | : :G
    //     : : : :
    //     : : : :
    //     | : | :
    //    Y| : |B:

    fn take_action_get_reward(&mut self, action: &TaxiAction) -> f64 {
        let mut reward = -1.;
        let state = &mut self.state;
        match (state.taxi.y, state.taxi.x, action) {
            //invalid movements
            (0, _, TaxiAction::Up) => reward = -10.,
            (4, _, TaxiAction::Down) => reward = -10.,
            (_, 0, TaxiAction::Left) => reward = -10.,
            (_, 4, TaxiAction::Right) => reward = -10.,
            (3, 0, TaxiAction::Right) => reward = -10.,
            (4, 0, TaxiAction::Right) => reward = -10.,
            (0, 1, TaxiAction::Right) => reward = -10.,
            (3, 2, TaxiAction::Right) => reward = -10.,
            (4, 2, TaxiAction::Right) => reward = -10.,
            (3, 1, TaxiAction::Left) => reward = -10.,
            (4, 1, TaxiAction::Left) => reward = -10.,
            (0, 2, TaxiAction::Left) => reward = -10.,
            (3, 3, TaxiAction::Left) => reward = -10.,
            (4, 3, TaxiAction::Left) => reward = -10.,
            //correct so change state
            (r, c, TaxiAction::Dropoff)
                if r == state.dropoff.y && c == state.dropoff.x && state.in_taxi =>
            {
                state.in_taxi = false;
                reward = 20.
            }
            (r, c, TaxiAction::Pickup)
                if r == state.passenger.y && c == state.passenger.x && !state.in_taxi =>
            {
                state.in_taxi = true;
                reward = 10.
            }
            (_, _, TaxiAction::Up) => state.taxi.y -= 1,
            (_, _, TaxiAction::Down) => state.taxi.y += 1,
            (_, _, TaxiAction::Left) => state.taxi.x -= 1,
            (_, _, TaxiAction::Right) => state.taxi.x += 1,

            _ => reward = -10., //invalid pickup or dropoff
        }
        //passenger gets taxi coords if in taxi
        if state.in_taxi {
            state.passenger.x = state.taxi.x;
            state.passenger.y = state.taxi.y;
        }
        reward
    }

    fn should_stop(&mut self, step: usize) -> bool {
        if step > 100 {
            true
        } else if self.is_finished() {
            self.found += 1;

            true
        } else {
            false
        }
    }
}

//3.2 Some helper methods
impl TaxiEnvironment {
    fn is_finished(&self) -> bool {
        let state = &self.state;
        state.passenger.x == state.dropoff.x
            && state.passenger.y == state.dropoff.y
            && !state.in_taxi
    }
}
