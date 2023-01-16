// Mouse
// mouse eats 2, 4 or 10 cheese.
// Finish on 10 cheese or -10 poison.
// Max is 11 points
//
// |M | 2 | 0|
// |4 |-10|10|

use learnwell::{
    agent::qlearning::QLearning, environment::Environment, runner::Runner,
    strategy::decliningrandom::DecliningRandom,
};

fn main() {
    let epochs = 500;

    let agent = QLearning::new(
        0.5,
        0.5,
        DecliningRandom::new((epochs as f64 * 0.9) as usize, 0.005),
    );

    Runner::run(agent, MouseEnvironment::default(), epochs);
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    r: usize,
    c: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct MouseState {
    current: Point,
    two: Option<Point>,
    four: Option<Point>,
    poison: Point,
    ten: Point,
}

impl Default for MouseState {
    fn default() -> Self {
        MouseState {
            current: Point { r: 0, c: 0 },
            two: Some(Point { r: 0, c: 1 }),
            four: Some(Point { r: 1, c: 0 }),
            poison: Point { r: 1, c: 1 },
            ten: Point { r: 1, c: 2 },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum MouseAction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default)]
struct MouseEnvironment {
    cumulative_rewards: f64,
    epoch: usize,
    state: MouseState,
}

impl Environment<MouseState, MouseAction> for MouseEnvironment {
    fn state(&self) -> MouseState {
        self.state.clone()
    }

    fn reset(&mut self, epoch: usize) {
        self.state = MouseState::default();
        self.epoch = epoch;

        let update = 20;
        if epoch % update == 0 {
            println!(
                "{epoch}: cumulative:{:.2} avg rewards {:.2}",
                self.cumulative_rewards,
                self.cumulative_rewards as f32 / update as f32
            );
            self.cumulative_rewards = 0.;
        }
    }

    ///example of actions which are dependent on state
    fn actions(&self) -> Vec<MouseAction> {
        let mut vec = vec![];

        if self.state.current.r == 1 {
            vec.push(MouseAction::Up)
        }

        if self.state.current.r == 0 {
            vec.push(MouseAction::Down)
        }

        if self.state.current.c > 0 {
            vec.push(MouseAction::Left)
        }

        if self.state.current.c < 2 {
            vec.push(MouseAction::Right)
        }

        vec
    }

    fn take_action_get_reward(&mut self, action: &MouseAction) -> f64 {
        let state = &mut self.state;
        let mut reward: i32 = match (state.current.r, state.current.c, action) {
            //invalid actions are not allowed as they are filtered out by actions method

            //valid actions:
            (_, _, MouseAction::Down) => {
                state.current.r += 1;
                -1
            }
            (_, _, MouseAction::Up) => {
                state.current.r -= 1;
                -1
            }
            (_, _, MouseAction::Left) => {
                state.current.c -= 1;
                -1
            }
            (_, _, MouseAction::Right) => {
                state.current.c += 1;
                -1
            }
        };

        if let Some(two) = &state.two {
            if two.c == state.current.c && two.r == state.current.r {
                reward += 2;
                state.two = None;
            }
        }

        if let Some(four) = &state.four {
            if four.c == state.current.c && four.r == state.current.r {
                reward += 4;
                state.four = None;
            }
        }

        reward += match (state.current.r, state.current.c) {
            //poison
            (1, 1) => -10,
            //ten
            (1, 2) => 10,
            _ => 0,
        };

        self.cumulative_rewards += reward as f64;
        reward as f64
    }

    fn should_stop(&mut self, step: usize) -> bool {
        if step > 100 {
            true
        } else {
            matches!(
                (self.state.current.r, self.state.current.c),
                (1, 1) | (1, 2)
            )
        }
    }
}
