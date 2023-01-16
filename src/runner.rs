use std::time::Instant;

use show_image::{create_window, run_context};

use crate::{
    agent::Agent,
    environment::{Environment, EnvironmentDisplay},
};

pub struct Runner;

impl Runner {
    pub fn run<S, A>(
        mut agent: impl Agent<S, A>,
        mut environment: impl Environment<S, A>,
        epochs: usize,
    ) {
        for epoch in 1..=epochs {
            environment.reset(epoch);
            let _ = agent.train(epoch, &mut environment);
        }
    }

    pub fn run_with_display<S: Send + 'static, A>(
        mut agent: impl Agent<S, A> + Send + 'static,
        mut environment: impl Environment<S, A> + EnvironmentDisplay + Send + 'static,
        epochs: usize,
        fps: usize,
    ) -> ! {
        let spf = 1.0 / fps as f32;
        let sta = Instant::now();
        let mut last_render = Instant::now();
        run_context(move || {
            let window = create_window("Learnwell", Default::default()).unwrap();
            for epoch in 1..=epochs {
                environment.reset(epoch);
                let _ = agent.train(epoch, &mut environment);

                let elapsed = last_render.elapsed().as_secs_f32();
                if elapsed > spf {
                    last_render = Instant::now();
                    let image = environment.get_image();
                    window.set_image("im1", image).unwrap();
                }
            }
            println!("Done in {:.2}s", sta.elapsed().as_secs_f64());
            window.wait_until_destroyed()
        });
    }
}
