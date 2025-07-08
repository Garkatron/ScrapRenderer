use crate::engine::{rendering::renderer::Renderer, types::vector::vector2i::Vector2i};

pub struct Renderer2D {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
    window: minifb::Window,
}

impl Renderer2D {
    pub fn new(buffer: Vec<u32>, width: usize, height: usize, window: minifb::Window) -> Self {
        Self {
            buffer,
            width,
            height,
            window
        }
    }
}

impl Renderer for Renderer2D {
    fn render(&mut self, delta_time: f32) {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }
    fn clear(&mut self, color: u32) {
        self.buffer.fill(color);
    }
    fn draw_pixel(&mut self, pos: Vector2i, color: u32) {
        if pos.x >= 0 && pos.y >= 0 && pos.x < self.width as i32 && pos.y < self.height as i32 {
            let index = pos.y as usize * self.width + pos.x as usize;
            if index < self.buffer.len() {
                self.buffer[index] = color;
            }
        }
    }
    

    fn draw_square(&mut self, a: Vector2i, b: Vector2i, color: u32, filled: bool, fill_color: u32) {
        let min_x = a.x.min(b.x);
        let max_x = a.x.max(b.x);
        let min_y = a.y.min(b.y);
        let max_y = a.y.max(b.y);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let pos = Vector2i { x, y };
                if filled {
                    self.draw_pixel(pos, fill_color);
                } else {
                    if x == min_x || x == max_x || y == min_y || y == max_y {
                        self.draw_pixel(pos, color);
                    }
                }
            }
        }
    }

    // ? https://es.wikipedia.org/wiki/Algoritmo_de_Bresenham
    fn draw_line(&mut self, a: Vector2i, b: Vector2i, color: u32) {
        let mut x = a.x;
        let mut y = a.y;
    
        let dx = (b.x - a.x).abs();
        let dy = (b.y - a.y).abs();
    
        let sx = if a.x < b.x { 1 } else { -1 };
        let sy = if a.y < b.y { 1 } else { -1 };
    
        let mut err = if dx > dy { dx } else { -dy } / 2;
        let mut e2;
    
        loop {
            self.draw_pixel(Vector2i::new(x, y), color);
    
            if x == b.x && y == b.y {
                break;
            }
    
            e2 = err;
            if e2 > -dx {
                err -= dy;
                x += sx;
            }
            if e2 < dy {
                err += dx;
                y += sy;
            }
        }
    }
    

    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }
    fn window(&self) -> &minifb::Window {
        &self.window
    }

    fn draw_triangle(&mut self, a: Vector2i, b: Vector2i, c: Vector2i, color: u32) {
       self.draw_line(a, b, color);
       self.draw_line(b, c, color);
       self.draw_line(c, a, color);
    }
}
