use std::time::Instant;

use minifb::{Window, WindowOptions};

use crate::{app::MyApp, engine::rendering::palettes::PALETTE_DEFAULT};


const WIDTH: usize = 500;
const HEIGHT: usize = 500;

pub mod engine;
pub mod app;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let idx = y * WIDTH + x;
  
            buffer[idx] = PALETTE_DEFAULT::BLACK.to_u32(); 
        }
    }

    let window = Window::new(
        "Cube",
        WIDTH,
        HEIGHT,
        WindowOptions {
            borderless: false,
            resize: false,
            scale: minifb::Scale::X2, 
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });



    // ? -------------------------------------------------------------------------------------------------

    let mut app= MyApp::new(WIDTH, HEIGHT, window);


    // ? -------------------------------------------------------------------------------------------------
    let mut last_frame = Instant::now();

    while WIDTH > 0
        && HEIGHT > 0
        && app.engine.running
    {
        let now = Instant::now();
        let delta = now.duration_since(last_frame);
        let delta_time = delta.as_secs_f32(); 
        last_frame = now;

        app.update(delta_time);
        app.render(delta_time);
    }
}
