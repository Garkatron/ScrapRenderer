use nalgebra::Vector2;


pub trait Renderer {
    fn render(&mut self, delta_time: f32);
    fn clear(&mut self, color: u32);
    fn draw_square(&mut self, a: Vector2<i32>, b: Vector2<i32>, color: u32, filled: bool, fill_color: u32); 
    fn draw_pixel(&mut self, pos: Vector2<i32>, color: u32);
    fn draw_line(&mut self, a: Vector2<i32>, b: Vector2<i32>, color: u32);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn draw_triangle(&mut self, a: Vector2<i32>, b: Vector2<i32>, c: Vector2<i32>, color: u32);
    fn fill_triangle(&mut self, a: Vector2<i32>, b: Vector2<i32>, c: Vector2<i32>, color: u32);
    fn get_x_at_y(&self, p1: Vector2<i32>, p2: Vector2<i32>, y: i32) -> i32;
}
