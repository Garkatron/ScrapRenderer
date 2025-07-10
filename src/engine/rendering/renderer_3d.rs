use std::{cell::RefCell, rc::Rc, vec};

use minifb::Window;

use crate::engine::{
    rendering::{mesh::Mesh, palette::Palette, renderer::Renderer, renderer_2d::Renderer2D, texture::Texture},
    types::{
        triangle::Triangle,
        vector::{vector2::Vector2, vector3::Vector3, vector4::Vector4, vector_ops::VectorOps},
    },
};

pub struct Renderer3D {
    pub renderer_2d: Renderer2D,
    pub depth_buffer: Vec<f32>
}

impl Renderer3D {
    pub fn new(buffer: Vec<u32>, width: usize, height: usize, window: Rc<RefCell<Window>>) -> Self {
        let renderer_2d: Renderer2D = Renderer2D::new(buffer, width, height, window);
        Self { renderer_2d, depth_buffer: vec![0.0; width * height]}
    }

    pub fn get_shading_color(dp: f32, palette: &'static dyn Palette) -> u32 {
        palette.get_shading_color(dp)
    }

    // https://github.com/OneLoneCoder/Javidx9/blob/master/ConsoleGameEngine/BiggerProjects/Engine3D/OneLoneCoder_olcEngine3D_Part3.cpp
    pub fn intersect_plane(
        plane_p: Vector4<f32>,    // A known point on the plane
        plane_n: Vector4<f32>,    // The normal vector of the plane
        line_start: Vector4<f32>, // Start point of the line
        line_end: Vector4<f32>,   // End point of the line
    ) -> (Vector4<f32>, f32) {
        // Normalize the plane's normal vector
        let plane_n = plane_n.normalize();

        // Calculate the plane's constant term: D = -N · P
        let plane_d = -plane_n.dot(plane_p);

        // Get the dot product of the line start and end with the plane normal
        let ad = line_start.dot(plane_n);
        let bd = line_end.dot(plane_n);

        // Compute the parameter t to find the intersection point along the line
        let t = (-plane_d - ad) / (bd - ad);

        // Get the direction vector from start to end
        let line_start_to_end = line_end - line_start;

        // Scale the direction vector to reach the intersection point
        let line_to_intersect = line_start_to_end * t;

        // Return the final intersection point
        (line_start + line_to_intersect, t)
    }

    // https://github.com/OneLoneCoder/Javidx9/blob/master/ConsoleGameEngine/BiggerProjects/Engine3D/OneLoneCoder_olcEngine3D_Part3.cpp
    /// Intersects a triangle with a plane, possibly clipping it into 0, 1 or 2 triangles.
    /// Returns a vector of resulting triangles.
    pub fn triangle_clip_against_plane(
        plane_p: Vector4<f32>,
        plane_n: Vector4<f32>,
        in_tri: &Triangle,
    ) -> Vec<Triangle> {
        let plane_n = plane_n.normalize();

        // Signed distance from point to plane
        let dist = |p: &Vector4<f32>| -> f32 { plane_n.dot(*p) - plane_n.dot(plane_p) };

        // Classify each vertex as inside or outside
        let mut inside_points: Vec<Vector4<f32>> = Vec::new();
        let mut outside_points: Vec<Vector4<f32>> = Vec::new();
        let mut inside_points_tex: Vec<Vector3<f32>> = Vec::new();
        let mut outside_points_tex: Vec<Vector3<f32>> = Vec::new();

        // Get signed distance of each point in the triangle to plane
        let d0 = dist(&in_tri.v1);
        let d1 = dist(&in_tri.v2);
        let d2 = dist(&in_tri.v3);

        // Check if inside or not
        if d0 >= 0.0 {
            inside_points.push(in_tri.v1);
            inside_points_tex.push(in_tri.uv[0]);
        } else {
            outside_points.push(in_tri.v1);
            outside_points_tex.push(in_tri.uv[0]);
        }
        if d1 >= 0.0 {
            inside_points.push(in_tri.v2);
            inside_points_tex.push(in_tri.uv[1]);
        } else {
            outside_points.push(in_tri.v2);
            outside_points_tex.push(in_tri.uv[1]);
        }
        if d2 >= 0.0 {
            inside_points.push(in_tri.v3);
            inside_points_tex.push(in_tri.uv[2]);
        } else {
            outside_points.push(in_tri.v3);
            outside_points_tex.push(in_tri.uv[2]);
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
                let (p1, t1) = Renderer3D::intersect_plane(plane_p, plane_n, p0, outside_points[0]);
                let (p2, t2) = Renderer3D::intersect_plane(plane_p, plane_n, p0, outside_points[1]);

                let p0_tex = inside_points_tex[0];
                let uv1 = Vector3::new(
                    p0_tex.x + t1 * (outside_points_tex[0].x - p0_tex.x),
                    p0_tex.y + t1 * (outside_points_tex[0].y - p0_tex.y),
                    1.0,
                );
                let uv2 = Vector3::new(
                    p0_tex.x + t2 * (outside_points_tex[1].x - p0_tex.x),
                    p0_tex.y + t2 * (outside_points_tex[1].y - p0_tex.y),
                    1.0,
                );

                vec![
                    Triangle::new(p0, p1, p2)
                        .set_light_color(in_tri.light_color)
                        .set_uv([p0_tex, uv1, uv2]),
                ]
            }
            2 => {
                // 2 points inside, 1 outside — clip into 2 triangles (a quad)
                let p0 = inside_points[0];
                let p1 = inside_points[1];
                let (i0, t0) = Renderer3D::intersect_plane(plane_p, plane_n, p0, outside_points[0]);
                let (i1, t1) = Renderer3D::intersect_plane(plane_p, plane_n, p1, outside_points[0]);

                let p0_tex = inside_points_tex[0];
                let p1_tex = inside_points_tex[1];
                let uv0 = Vector3::new(
                    p0_tex.x + t0 * (outside_points_tex[0].x - p0_tex.x),
                    p0_tex.y + t0 * (outside_points_tex[0].y - p0_tex.y),
                    1.0,
                );
                let uv1 = Vector3::new(
                    p1_tex.x + t1 * (outside_points_tex[0].x - p1_tex.x),
                    p1_tex.y + t1 * (outside_points_tex[0].y - p1_tex.y),
                    1.0,
                );

                vec![
                    Triangle::new(p0, p1, i0)
                        .set_light_color(in_tri.light_color)
                        .set_uv([p0_tex, p1_tex, uv0]),
                    Triangle::new(p1, i1, i0)
                        .set_light_color(in_tri.light_color)
                        .set_uv([p1_tex, uv1, uv0]),
                ]
            }
            _ => vec![], // Should never happen
        }
    }

    pub fn draw_mesh(&mut self, _mesh: Mesh) {
        todo!()
    }

    pub fn textured_triangle(
        &mut self,
        p1: Vector2<i32>,
        uv1: crate::engine::types::vector::vector3::Vector3<f32>,
        p2: Vector2<i32>,
        uv2: crate::engine::types::vector::vector3::Vector3<f32>,
        p3: Vector2<i32>,
        uv3: crate::engine::types::vector::vector3::Vector3<f32>,
        tex: &super::texture::Texture,
    ) {
        // Extraer componentes de los vectores
        let mut x1 = p1.x;
        let mut y1 = p1.y;
        let mut u1 = uv1.x;
        let mut v1 = uv1.y;
        let mut w1 = uv1.z;
        let mut x2 = p2.x;
        let mut y2 = p2.y;
        let mut u2 = uv2.x;
        let mut v2 = uv2.y;
        let mut w2 = uv2.z;
        let mut x3 = p3.x;
        let mut y3 = p3.y;
        let mut u3 = uv3.x;
        let mut v3 = uv3.y;
        let mut w3 = uv3.z;

        // Sort vertices by y-coordinate (y1 <= y2 <= y3)
        if y2 < y1 {
            std::mem::swap(&mut y1, &mut y2);
            std::mem::swap(&mut x1, &mut x2);
            std::mem::swap(&mut u1, &mut u2);
            std::mem::swap(&mut v1, &mut v2);
            std::mem::swap(&mut w1, &mut w2);
        }
        if y3 < y1 {
            std::mem::swap(&mut y1, &mut y3);
            std::mem::swap(&mut x1, &mut x3);
            std::mem::swap(&mut u1, &mut u3);
            std::mem::swap(&mut v1, &mut v3);
            std::mem::swap(&mut w1, &mut w3);
        }
        if y3 < y2 {
            std::mem::swap(&mut y2, &mut y3);
            std::mem::swap(&mut x2, &mut x3);
            std::mem::swap(&mut u2, &mut u3);
            std::mem::swap(&mut v2, &mut v3);
            std::mem::swap(&mut w2, &mut w3);
        }

        // Calculate deltas for first segment (y1 to y2)
        let dy1 = y2 - y1;
        let dx1 = x2 - x1;
        let dv1 = v2 - v1;
        let du1 = u2 - u1;
        let dw1 = w2 - w1;

        // Calculate deltas for second segment (y1 to y3)
        let dy2 = y3 - y1;
        let dx2 = x3 - x1;
        let dv2 = v3 - v1;
        let du2 = u3 - u1;
        let dw2 = w3 - w1;

        let mut dax_step = 0.0;
        let mut dbx_step = 0.0;
        let mut du1_step = 0.0;
        let mut dv1_step = 0.0;
        let mut dw1_step = 0.0;
        let mut du2_step = 0.0;
        let mut dv2_step = 0.0;
        let mut dw2_step = 0.0;

        if dy1 != 0 {
            dax_step = dx1 as f32 / dy1.abs() as f32;
            du1_step = du1 / dy1.abs() as f32;
            dv1_step = dv1 / dy1.abs() as f32;
            dw1_step = dw1 / dy1.abs() as f32;
        }
        if dy2 != 0 {
            dbx_step = dx2 as f32 / dy2.abs() as f32;
            du2_step = du2 / dy2.abs() as f32;
            dv2_step = dv2 / dy2.abs() as f32;
            dw2_step = dw2 / dy2.abs() as f32;
        }

        // First half of the triangle (y1 to y2)
        if dy1 != 0 {
            for i in y1..=y2 {
                let ax = (x1 as f32 + (i - y1) as f32 * dax_step) as i32;
                let bx = (x1 as f32 + (i - y1) as f32 * dbx_step) as i32;

                let mut tex_su = u1 + (i - y1) as f32 * du1_step;
                let mut tex_sv = v1 + (i - y1) as f32 * dv1_step;
                let mut tex_sw = w1 + (i - y1) as f32 * dw1_step;

                let mut tex_eu = u1 + (i - y1) as f32 * du2_step;
                let mut tex_ev = v1 + (i - y1) as f32 * dv2_step;
                let mut tex_ew = w1 + (i - y1) as f32 * dw2_step;

                let (mut ax, mut bx) = if ax > bx { (bx, ax) } else { (ax, bx) };
                if ax > bx {
                    std::mem::swap(&mut tex_su, &mut tex_eu);
                    std::mem::swap(&mut tex_sv, &mut tex_ev);
                    std::mem::swap(&mut tex_sw, &mut tex_ew);
                }

                let mut tex_u = tex_su;
                let mut tex_v = tex_sv;
                let mut tex_w = tex_sw;

                let tstep = if bx != ax {
                    1.0 / (bx - ax) as f32
                } else {
                    0.0
                };
                let mut t = 0.0;

                for j in ax..bx {
                    tex_u = (1.0 - t) * tex_su + t * tex_eu;
                    tex_v = (1.0 - t) * tex_sv + t * tex_ev;
                    tex_w = (1.0 - t) * tex_sw + t * tex_ew;

                    let idx = i as usize * self.renderer_2d.width() + j as usize;
                    
                    if idx < self.depth_buffer.len() && tex_w > self.depth_buffer[idx] {
                       
                        let color = tex.get_pixel_as_u32((tex_u / tex_w) as u32, (tex_v / tex_w) as u32).expect("Cant get color");
                        self.draw_pixel(Vector2::new(j, i), color);
                        self.depth_buffer[idx] = tex_w;
                    }
                    t += tstep;
                }
            }
        }

        // Second half of the triangle (y2 to y3)
        let dy1 = y3 - y2;
        let dx1 = x3 - x2;
        let dv1 = v3 - v2;
        let du1 = u3 - u2;
        let dw1 = w3 - w2;

        if dy1 != 0 {
            dax_step = dx1 as f32 / dy1.abs() as f32;
            du1_step = du1 / dy1.abs() as f32;
            dv1_step = dv1 / dy1.abs() as f32;
            dw1_step = dw1 / dy1.abs() as f32;
        }

        if dy1 != 0 {
            for i in y2..=y3 {
                let ax = (x2 as f32 + (i - y2) as f32 * dax_step) as i32;
                let bx = (x1 as f32 + (i - y1) as f32 * dbx_step) as i32;

                let mut tex_su = u2 + (i - y2) as f32 * du1_step;
                let mut tex_sv = v2 + (i - y2) as f32 * dv1_step;
                let mut tex_sw = w2 + (i - y2) as f32 * dw1_step;

                let mut tex_eu = u1 + (i - y1) as f32 * du2_step;
                let mut tex_ev = v1 + (i - y1) as f32 * dv2_step;
                let mut tex_ew = w1 + (i - y1) as f32 * dw2_step;

                let (mut ax, mut bx) = if ax > bx { (bx, ax) } else { (ax, bx) };
                if ax > bx {
                    std::mem::swap(&mut tex_su, &mut tex_eu);
                    std::mem::swap(&mut tex_sv, &mut tex_ev);
                    std::mem::swap(&mut tex_sw, &mut tex_ew);
                }

                let mut tex_u = tex_su;
                let mut tex_v = tex_sv;
                let mut tex_w = tex_sw;

                let tstep = if bx != ax {
                    1.0 / (bx - ax) as f32
                } else {
                    0.0
                };
                let mut t = 0.0;

                for j in ax..bx {
                    tex_u = (1.0 - t) * tex_su + t * tex_eu;
                    tex_v = (1.0 - t) * tex_sv + t * tex_ev;
                    tex_w = (1.0 - t) * tex_sw + t * tex_ew;

                    let color = tex.get_pixel_as_u32((tex_u / tex_w) as u32, (tex_v / tex_w) as u32).expect("Cant get color");
                    self.draw_pixel(Vector2::new(j, i), color);
                    self.depth_buffer[(i * self.renderer_2d.width() as i32 + j) as usize] = tex_w;
                }
                t += tstep;
            }
        }
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

    fn draw_pixel(&mut self, pos: Vector2<i32>, color: u32) {
        self.renderer_2d.draw_pixel(pos, color);
    }

    fn draw_square(
        &mut self,
        a: Vector2<i32>,
        b: Vector2<i32>,
        color: u32,
        filled: bool,
        fill_color: u32,
    ) {
        self.renderer_2d
            .draw_square(a, b, color, filled, fill_color);
    }

    fn draw_line(&mut self, a: Vector2<i32>, b: Vector2<i32>, color: u32) {
        self.renderer_2d.draw_line(a, b, color);
    }

    fn width(&self) -> usize {
        self.renderer_2d.width()
    }

    fn height(&self) -> usize {
        self.renderer_2d.height()
    }

    fn draw_triangle(&mut self, a: Vector2<i32>, b: Vector2<i32>, c: Vector2<i32>, color: u32) {
        self.renderer_2d.draw_triangle(a, b, c, color)
    }

    fn fill_triangle(&mut self, a: Vector2<i32>, b: Vector2<i32>, c: Vector2<i32>, color: u32) {
        self.renderer_2d.fill_triangle(a, b, c, color)
    }
    fn get_x_at_y(&self, p1: Vector2<i32>, p2: Vector2<i32>, y: i32) -> i32 {
        self.renderer_2d.get_x_at_y(p1, p2, y)
    }
}
