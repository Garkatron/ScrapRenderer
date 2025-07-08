use crate::engine::{rendering::{mesh::Mesh, renderer::Renderer, renderer_2d::Renderer2D}, types::{colour::COLOUR, vector::vector2i::Vector2i}};

pub struct Renderer3D {
    renderer_2d: Renderer2D,
}

impl Renderer3D {
    pub fn new(buffer: Vec<u32>, width: usize, height: usize, window: minifb::Window) -> Self {
        Self {
            renderer_2d: Renderer2D::new(buffer, width, height, window)
        }
    }

    pub fn get_shading_color(dp: f32) -> u32 {
        match (dp*3.0) as i32 {
            0 => COLOUR::ORANGE.to_u32(),
            1 => COLOUR::YELLOW.to_u32(),
            2 => COLOUR::WHITE.to_u32(),
            _default => COLOUR::BLACK.to_u32()
        }
    }

    pub fn draw_mesh(&mut self, _mesh: Mesh) {
        
        
        todo!()
    }
}

/*
 0 => PALETTE_INKPINK::PINK4.to_u32(),
            1 => PALETTE_INKPINK::PINK3.to_u32(),
            2 => PALETTE_INKPINK::PINK2.to_u32(),
            4 => PALETTE_INKPINK::PINK1.to_u32(),
            5 => PALETTE_INKPINK::PINK0.to_u32(),
            6 => PALETTE_INKPINK::WHITE.to_u32(),
            */

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
        self.renderer_2d.draw_triangle(a, b, c, color)
    }

    fn fill_triangle(&mut self, a: Vector2i, b: Vector2i, c: Vector2i, color: u32) {
        self.renderer_2d.fill_triangle(a, b, c, color)
    }
    fn get_x_at_y(&self, p1: Vector2i, p2: Vector2i, y: i32) -> i32 {
        self.renderer_2d.get_x_at_y(p1, p2, y)
    }
}
