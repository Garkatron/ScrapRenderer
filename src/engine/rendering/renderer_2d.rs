#![allow(unused_variables)]
use crate::engine::{
    rendering::renderer::Renderer,
    types::{colour::COLOUR, vector::vector2i::Vector2i},
};

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
            window,
        }
    }
}

impl Renderer for Renderer2D {
    fn render(&mut self, _delta_time: f32) {
        if let Err(e) = self
            .window
            .update_with_buffer(&self.buffer, self.width, self.height)
        {
            eprintln!("Error updating window buffer: {:?}", e);
        }
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
        let mut x0 = a.x;
        let mut y0 = a.y;
        let mut x1 = b.x;
        let mut y1 = b.y;

        if x0 == x1 && y0 == y1 {
            self.draw_pixel(Vector2i::new(x0, y0), color);
            return; // Only draws a pixel
        }

        // Detect a higher triangle comparing the absolute differense with x and y
        let mut steep = false;
        if (x0 - x1).abs() < (y0 - y1).abs() {
            // Swap cus loop can inter the higher value
            std::mem::swap(&mut x0, &mut y0);
            std::mem::swap(&mut x1, &mut y1);
            steep = true;
        }

        // Swap f x0 is greather cus we want iterate it over left-to-righ
        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }

        // Calc differences
        let dx = x1 - x0;
        let dy = y1 - y0;
        let derror2 = dy.abs() * 2;
        let mut error2 = 0;
        let ystep = if y0 < y1 { 1 } else { -1 }; // Up/Down

        let mut y = y0;
        for x in x0..=x1 {
            if steep {
                self.draw_pixel(Vector2i::new(y, x), color);
            } else {
                self.draw_pixel(Vector2i::new(x, y), color);
            }
            // if error i'ts greater than dx, we need step over y
            error2 += derror2;
            if error2 > dx {
                y += ystep;
                error2 -= dx * 2;
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

    fn get_x_at_y(&self, p1: Vector2i, p2: Vector2i, y: i32) -> i32 {
        if p1.y == p2.y {
            return p1.x; // Horizontal line, return x1
        }
        if y < p1.y.min(p2.y) || y > p1.y.max(p2.y) {
            return i32::MAX; // Out of range
        }
        // Linear interpolation
        let t = (y - p1.y) as f32 / (p2.y - p1.y) as f32;
        let x = p1.x as f32 + t * (p2.x - p1.x) as f32;
        x.round() as i32 // Round to nearest integer
    }

    // ? https://github.com/ssloy/tinyrenderer/wiki/Lesson-2:-Triangle-rasterization-and-back-face-culling
    fn fill_triangle(&mut self, v1: Vector2i, v2: Vector2i, v3: Vector2i, color: u32) {
        // Make mutable copies to sort
        let mut a = v1;
        let mut b = v2;
        let mut c = v3;
    
        // Sort points by y-coordinate (top to bottom)
        if a.y > b.y {
            std::mem::swap(&mut a, &mut b);
        }
        if b.y > c.y {
            std::mem::swap(&mut b, &mut c);
        }
        if a.y > b.y {
            std::mem::swap(&mut a, &mut b);
        }
        // Now: a.y <= b.y <= c.y
    
        // Check for degenerate triangle
        if a.y == b.y && b.y == c.y {
            return;
        }
    
        let total_height = c.y - a.y;
        if total_height == 0 {
            return; // Avoid division by zero
        }
    
        for i in 0..total_height {
            let second_half = i > b.y - a.y || b.y == a.y;
            let segment_height = if second_half { c.y - b.y } else { b.y - a.y };
            let alpha = i as f32 / total_height as f32;
            let beta = if segment_height != 0 {
                (i - (if second_half { b.y - a.y } else { 0 })) as f32 / segment_height as f32
            } else {
                0.0
            };
            let va = a + (c - a) * alpha;
            let vb = if second_half { b + (c - b) * beta } else { a + (b - a) * beta };
            // Draw horizontal line from min(va.x, vb.x) to max(va.x, vb.x)
            let (start_x, end_x) = if va.x > vb.x { (vb.x, va.x) } else { (va.x, vb.x) };
            for x in start_x..=end_x {
                self.draw_pixel(Vector2i { x, y: a.y + i }, color);
            }
        }

    }
}
