use std::f32::consts::PI;

use crate::engine::{rendering::{mesh::Mesh, renderer::Renderer, renderer_2d::Renderer2D}, types::vector::Vector2i};

pub struct Renderer3D {
    renderer_2d: Renderer2D,
}

impl Renderer3D {
    pub fn new(buffer: Vec<u32>, width: usize, height: usize, window: minifb::Window) -> Self {
        Self {
            renderer_2d: Renderer2D::new(buffer, width, height, window)
        }
    }
    pub fn draw_mesh(&mut self, _mesh: Mesh) {
        
        
        todo!()
    }
}

impl Renderer for Renderer3D {
    
    fn render(&mut self, delta_time: f32) {
        self.renderer_2d.render(delta_time);
    }

    fn clear(&mut self, color: u32) {
        self.renderer_2d.clear(color);
    }

    fn draw_pixel(&mut self, pos: Vector2i, color: u32) {
        self.renderer_2d.draw_pixel(pos, color);
    }

    fn draw_square(&mut self, a: Vector2i, b: Vector2i, color: u32, filled: bool, fill_color: u32) {
        self.renderer_2d.draw_square(a, b, color, filled, fill_color);
    }

    fn draw_line(&mut self, a: Vector2i, b: Vector2i, color: u32) {
        self.renderer_2d.draw_line(a, b, color);
    }

    fn width(&self) -> usize {
        self.renderer_2d.width()
    }

    fn height(&self) -> usize {
        self.renderer_2d.height()
    }
    fn window(&self) -> &minifb::Window {
        self.renderer_2d.window()
    }

    fn draw_triangle(&mut self, a: Vector2i, b: Vector2i, c: Vector2i, color: u32) {
        self.renderer_2d.draw_triangle(a, b, c, color);
    }
}
