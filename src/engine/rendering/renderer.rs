use crate::engine::types::vector::vector2i::Vector2i;

pub trait Renderer {
    fn render(&mut self, delta_time: f32);
    fn clear(&mut self, color: u32);
    fn draw_square(&mut self, a: Vector2i, b: Vector2i, color: u32, filled: bool, fill_color: u32); 
    fn draw_pixel(&mut self, pos: Vector2i, color: u32);
    fn draw_line(&mut self, a: Vector2i, b: Vector2i, color: u32);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn window(&self) -> &minifb::Window;
    fn draw_triangle(&mut self, a: Vector2i, b: Vector2i, c: Vector2i, color: u32);
    fn fill_triangle(&mut self, a: Vector2i, b: Vector2i, c: Vector2i, color: u32);
    fn get_x_at_y(&self, p1: Vector2i, p2: Vector2i, y: i32) -> i32;
}
