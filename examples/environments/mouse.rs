// Mouse
// mouse eats 2, 4 or 10 cheese.
// Finish on 10 cheese or -10 poison.
// Max is 11 points
//
// |M | 2 | 0|
// |4 |-10|10|

use learnwell::environment::Environment;
use show_image::{ImageInfo, ImageView};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    r: usize,
    c: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MouseState {
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
pub enum MouseAction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default)]
pub struct MouseEnvironment {
    cumulative_rewards: f64,
    epoch: usize,
    state: MouseState,
    image: [u8; 18],
}

impl MouseEnvironment {
    fn save_image(&mut self) {
        let mut pixels: [u8; 18] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        //times 3 because rgb
        if let Some(p) = &self.state.two {
            pixels[p.r * 3 * 3 + p.c * 3 + 1] = 90;
        }
        if let Some(p) = &self.state.four {
            pixels[p.r * 3 * 3 + p.c * 3 + 1] = 180;
        }
        let p = &self.state.ten;
        pixels[p.r * 3 * 3 + p.c * 3 + 1] = 255;

        let p = &self.state.poison;
        pixels[p.r * 3 * 3 + p.c * 3 + 0] = 255;

        let p = &self.state.current;
        pixels[p.r * 3 * 3 + p.c * 3 + 0] = 100;
        pixels[p.r * 3 * 3 + p.c * 3 + 1] = 100;
        pixels[p.r * 3 * 3 + p.c * 3 + 2] = 100;

        self.image = pixels
    }
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
    fn all_actions(&self) -> Vec<MouseAction> {
        vec![
            MouseAction::Up,
            MouseAction::Down,
            MouseAction::Left,
            MouseAction::Right,
        ]
    }

    fn take_action_get_reward(&mut self, action: &MouseAction) -> f64 {
        let state = &mut self.state;
        let mut reward: i32 = match (state.current.r, state.current.c, action) {
            //invalid actions
            (0, _, MouseAction::Up) => -5,
            (1, _, MouseAction::Down) => -5,
            (_, 0, MouseAction::Left) => -5,
            (_, 2, MouseAction::Right) => -5,
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

    fn get_image(&mut self) -> show_image::ImageView {
        self.save_image();

        let image = ImageView::new(ImageInfo::rgb8(3, 2), &self.image);
        image
    }
}
