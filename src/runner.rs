use std::time::{Duration, Instant};

use show_image::{create_window, run_context};

use crate::{agent::Agent, environment::Environment, progress::Progress};

pub struct Runner;

impl Runner {
    pub fn run<S, A>(
        mut agent: impl Agent<S, A>,
        mut environment: impl Environment<S, A>,
        epochs: usize,
    ) {
        let mut progress: Progress = Progress {
            epoch: 0,
            epoch_step: 0,
            cumulative_steps: 0,
        };
        for epoch in 1..=epochs {
            environment.reset(epoch);
            progress.epoch_step = 0;
            let mut done = false;
            while !done {
                progress.epoch = epoch;
                progress.epoch_step += 1;
                progress.cumulative_steps += 1;
                done = agent.step(progress, &mut environment);
            }
        }
    }

    pub fn run_with_display<S: Send + 'static, A>(
        mut agent: impl Agent<S, A> + Send + 'static,
        mut environment: impl Environment<S, A> + Send + 'static,
        epochs: usize,
        config: DisplayConfig,
    ) -> ! {
        let spf = 1.0 / config.fps as f32;
        let sta = Instant::now();
        let mut last_render = Instant::now();
        run_context(move || {
            let window = create_window("Learnwell", Default::default()).unwrap();

            let mut progress: Progress = Progress {
                epoch: 0,
                epoch_step: 0,
                cumulative_steps: 0,
            };
            for epoch in 1..=epochs {
                environment.reset(epoch);
                progress.epoch_step = 0;
                let mut done = false;
                while !done {
                    progress.epoch = epoch;
                    progress.epoch_step += 1;
                    progress.cumulative_steps += 1;
                    done = agent.step(progress, &mut environment);

                    let elapsed = last_render.elapsed().as_secs_f32();
                    if elapsed > spf {
                        last_render = Instant::now();
                        let image = environment.get_image();
                        let _ = window.set_image("im1", image);
                    }

                    if progress.epoch > config.step_time_start && !config.step_time.is_zero() {
                        std::thread::sleep(config.step_time)
                    }
                }
            }
            println!("Done in {:.2}s", sta.elapsed().as_secs_f64());
            window.wait_until_destroyed()
        });
    }
}

pub struct DisplayConfig {
    /// how many fps to show
    pub fps: usize,
    /// slow down each step to take at least this duration.
    /// Use 0 for no slowdown
    /// Useful to show learning
    pub step_time: Duration,
    /// which epoch to slow down from
    pub step_time_start: usize,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            fps: 10,
            step_time: Duration::from_millis(0),
            step_time_start: 0,
        }
    }
}
