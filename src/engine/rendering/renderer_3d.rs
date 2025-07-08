use std::{cell::RefCell, rc::Rc};

use minifb::Window;

use crate::engine::{
    rendering::{mesh::Mesh, palette::Palette, palettes::PALETTE_DEFAULT, renderer::Renderer, renderer_2d::Renderer2D},
    types::{
        triangle::Triangle,
        vector::{vector2i::Vector2i, vector3::Vector3},
    },
};

pub struct Renderer3D {
    pub renderer_2d: Renderer2D,
}

impl Renderer3D {
    pub fn new(buffer: Vec<u32>, width: usize, height: usize, window: Rc<RefCell<Window>>) -> Self {
        let renderer_2d = Renderer2D::new(buffer, width, height, window);
        Self { renderer_2d }
    }

    pub fn get_shading_color(dp: f32, palette: &'static dyn Palette) -> u32 {
       palette.get_shading_color(dp)
    }

    // https://github.com/OneLoneCoder/Javidx9/blob/master/ConsoleGameEngine/BiggerProjects/Engine3D/OneLoneCoder_olcEngine3D_Part3.cpp
    pub fn intersect_plane(
        plane_p: Vector3,    // A known point on the plane
        plane_n: Vector3,    // The normal vector of the plane
        line_start: Vector3, // Start point of the line
        line_end: Vector3,   // End point of the line
    ) -> Vector3 {
        // Normalize the plane's normal vector
        let plane_n = plane_n.normalize();

        // Calculate the plane's constant term: D = -N · P
        let plane_d = -plane_n.dot(&plane_p);

        // Get the dot product of the line start and end with the plane normal
        let ad = line_start.dot(&plane_n);
        let bd = line_end.dot(&plane_n);

        // Compute the parameter t to find the intersection point along the line
        let t = (-plane_d - ad) / (bd - ad);

        // Get the direction vector from start to end
        let line_start_to_end = line_end - line_start;

        // Scale the direction vector to reach the intersection point
        let line_to_intersect = line_start_to_end * t;

        // Return the final intersection point
        line_start + line_to_intersect
    }

    // https://github.com/OneLoneCoder/Javidx9/blob/master/ConsoleGameEngine/BiggerProjects/Engine3D/OneLoneCoder_olcEngine3D_Part3.cpp
    /// Intersects a triangle with a plane, possibly clipping it into 0, 1 or 2 triangles.
    /// Returns a vector of resulting triangles.
    pub fn triangle_clip_against_plane(
        plane_p: Vector3,
        plane_n: Vector3,
        in_tri: &Triangle,
    ) -> Vec<Triangle> {
        let plane_n = plane_n.normalize();

        // Signed distance from point to plane
        let dist = |p: &Vector3| -> f32 { plane_n.dot(&p) - plane_n.dot(&plane_p) };

        // Classify each vertex as inside or outside
        let mut inside_points: Vec<Vector3> = Vec::new();
        let mut outside_points: Vec<Vector3> = Vec::new();

        // Get sgined distance of each point in the triangle to plane
        let d0 = dist(&in_tri.v1);
        let d1 = dist(&in_tri.v2);
        let d2 = dist(&in_tri.v3);

        // Check if inside or not
        if d0 >= 0.0 {
            inside_points.push(in_tri.v1);
        } else {
            outside_points.push(in_tri.v1);
        }
        if d1 >= 0.0 {
            inside_points.push(in_tri.v2);
        } else {
            outside_points.push(in_tri.v2);
        }
        if d2 >= 0.0 {
            inside_points.push(in_tri.v3);
        } else {
            outside_points.push(in_tri.v3);
        }

        // Classify triangle points
        match inside_points.len() {
            0 => {
                // All points are outside the plane, triangle is fully clipped
                vec![]
            }
            3 => {
                // All points are inside, return original triangle
                vec![in_tri.clone()]
            }
            1 => {
                // 1 point inside, 2 outside — clip into 1 triangle
                let p0 = inside_points[0];
                let p1 = Renderer3D::intersect_plane(plane_p, plane_n, p0, outside_points[0]);
                let p2 = Renderer3D::intersect_plane(plane_p, plane_n, p0, outside_points[1]);

                vec![Triangle {
                    v1: p0,
                    v2: p1,
                    v3: p2,
                    light_color: in_tri.light_color,//PALETTE_DEFAULT::RED.to_u32(),
                }]
            }
            2 => {
                // 2 points inside, 1 outside — clip into 2 triangles (a quad)
                let p0 = inside_points[0];
                let p1 = inside_points[1];
                let i0 = Renderer3D::intersect_plane(plane_p, plane_n, p0, outside_points[0]);
                let i1 = Renderer3D::intersect_plane(plane_p, plane_n, p1, outside_points[0]);

                vec![
                    Triangle {
                        v1: p0,
                        v2: p1,
                        v3: i0,
                        light_color: in_tri.light_color,//PALETTE_DEFAULT::GREEN.to_u32(),
                    },
                    Triangle {
                        v1: p1,
                        v2: i1,
                        v3: i0,
                        light_color: in_tri.light_color,//PALETTE_DEFAULT::YELLOW.to_u32(),
                    },
                ]
            }
            _ => vec![], // Should never happen
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
        self.renderer_2d
            .draw_square(a, b, color, filled, fill_color);
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
