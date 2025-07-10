#![allow(unused_imports, redundant_imports, unused_import_braces, dead_code, unused_variables)]
use std::{cell::RefCell, rc::Rc};
use minifb::{Window, WindowOptions};
use nalgebra::Vector2;

use crate::{app, engine::{control::keyboard::KeyboardController, engine_3d::Engine3D, rendering::{palettes::PALETTE_DEFAULT, renderer::Renderer, renderer_3d::Renderer3D, texture_poll::TexturePool}}};

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

fn build_window() -> Window {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let idx = y * WIDTH + x;
            buffer[idx] = PALETTE_DEFAULT::BLACK.to_u32();
        }
    }
    Window::new(
        "Cube",
        WIDTH,
        HEIGHT,
        WindowOptions {
            borderless: false,
            resize: false,
            scale: minifb::Scale::X2,
            ..WindowOptions::default()
        },
    ).unwrap_or_else(|e| panic!("Failed to create window: {}", e))
}

fn render_triangle(app: &mut MyApp, dt: f32) {
    app.engine.renderer.draw_triangle(
        Vector2::new(0, 100),
        Vector2::new(100, 150),
        Vector2::new(200, 100),
        0xffff,
    );
    app.engine.renderer.render(dt);
}

fn render_filled_triangle(app: &mut MyApp, dt: f32) {
    app.engine.renderer.fill_triangle(
        Vector2::new(0, 100),
        Vector2::new(100, 150),
        Vector2::new(200, 100),
        0xffff,
    );
    app.engine.renderer.render(dt);
}


pub struct MyApp {
    pub window: Rc<RefCell<Window>>,
    pub engine: Engine3D,
}

#[test]
fn test_draw_triangle() {
    // Build and initialize the window
    let window = build_window();
    let window_rc = Rc::new(RefCell::new(window));

    // Initialize TexturePool with a mock or test texture path
    let texture_pool = TexturePool::new();

    // Initialize the app
    let mut app = MyApp {
        window: window_rc.clone(),
        engine: Engine3D {
            running: true,
            renderer: Renderer3D::new(vec![0; WIDTH * HEIGHT], WIDTH, HEIGHT, window_rc.clone()),
            texture_poll: texture_pool,
            kbcontroller: KeyboardController::new(window_rc.clone()),
        },
    };

    // Render a single frame for testing
    let delta_time = 0.016; // Approx 60 FPS
    render_triangle(&mut app, delta_time);

    // Verify the triangle was drawn
    let buffer = app.engine.renderer.renderer_2d.buffer(); // Assuming Renderer3D has a buffer() method
    let mut triangle_drawn = false;
    for y in 100..151 { // Check the y-range where the triangle should be (100 to 150)
        for x in 0..201 { // Check the x-range (0 to 200)
            let idx = y * WIDTH + x;
            if buffer[idx] == 0xffff { // Check for the color 0xffff (yellow)
                triangle_drawn = true;
                break;
            }
        }
        if triangle_drawn {
            break;
        }
    }
    assert!(triangle_drawn, "Triangle was not drawn correctly in the buffer");

}

#[test]
fn test_filled_triangle() {
    // Build and initialize the window
    let window = build_window();
    let window_rc = Rc::new(RefCell::new(window));

    // Initialize TexturePool with a mock or test texture path
    let texture_pool = TexturePool::new();

    // Initialize the app
    let mut app = MyApp {
        window: window_rc.clone(),
        engine: Engine3D {
            running: true,
            renderer: Renderer3D::new(vec![0; WIDTH * HEIGHT], WIDTH, HEIGHT, window_rc.clone()),
            texture_poll: texture_pool,
            kbcontroller: KeyboardController::new(window_rc.clone()),
        },
    };

    // Render a single frame for testing
    let delta_time = 0.016; // Approx 60 FPS
    render_filled_triangle(&mut app, delta_time);

    // Verify the triangle was drawn
    let buffer = app.engine.renderer.renderer_2d.buffer(); // Assuming Renderer3D has a buffer() method
    let mut triangle_drawn = false;
    for y in 100..151 { // Check the y-range where the triangle should be (100 to 150)
        for x in 0..201 { // Check the x-range (0 to 200)
            let idx = y * WIDTH + x;
            if buffer[idx] == 0xffff { // Check for the color 0xffff (yellow)
                triangle_drawn = true;
                break;
            }
        }
        if triangle_drawn {
            break;
        }
    }
    assert!(triangle_drawn, "Triangle was not drawn correctly in the buffer");

}
