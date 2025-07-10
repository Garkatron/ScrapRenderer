use std::{cell::RefCell, f32::consts::PI, rc::Rc};

use minifb::Window;
use nalgebra::{Matrix3, Matrix4, Vector2, Vector3, Vector4};

use crate::engine::{
    rendering::{mesh::Mesh, palette::Palette, renderer::Renderer, renderer_2d::Renderer2D},
    types::{
        triangle::Triangle,
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
        plane_p: Vector3<f32>,    // A known point on the plane
        plane_n: Vector3<f32>,    // The normal vector of the plane
        line_start: Vector3<f32>, // Start point of the line
        line_end: Vector3<f32>,   // End point of the line
    ) -> Vector3<f32> {
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
        plane_p: Vector3<f32>,
        plane_n: Vector3<f32>,
        in_tri: &Triangle,
    ) -> Vec<Triangle> {
        let plane_n = plane_n.normalize();
    
        // Helper: extract Vector3 from Vector4
        let to_v3 = |v: &Vector4<f32>| Vector3::new(v.x, v.y, v.z);
        // Helper: reconstruct Vector4 from Vector3, preserving original w
        let to_v4 = |v3: Vector3<f32>, w: f32| Vector4::new(v3.x, v3.y, v3.z, w);
    
        // Signed distance from point to plane
        let dist = |p: &Vector3<f32>| -> f32 { plane_n.dot(p) - plane_n.dot(&plane_p) };
    
        let v1 = in_tri.v1;
        let v2 = in_tri.v2;
        let v3 = in_tri.v3;
    
        let p1 = to_v3(&v1);
        let p2 = to_v3(&v2);
        let p3 = to_v3(&v3);
    
        let d1 = dist(&p1);
        let d2 = dist(&p2);
        let d3 = dist(&p3);
    
        let mut inside_points = vec![];
        let mut outside_points = vec![];
        let mut inside_ws = vec![];
        let mut outside_ws = vec![];
    
        if d1 >= 0.0 {
            inside_points.push(p1);
            inside_ws.push(v1.w);
        } else {
            outside_points.push(p1);
            outside_ws.push(v1.w);
        }
    
        if d2 >= 0.0 {
            inside_points.push(p2);
            inside_ws.push(v2.w);
        } else {
            outside_points.push(p2);
            outside_ws.push(v2.w);
        }
    
        if d3 >= 0.0 {
            inside_points.push(p3);
            inside_ws.push(v3.w);
        } else {
            outside_points.push(p3);
            outside_ws.push(v3.w);
        }
    
        match inside_points.len() {
            0 => vec![], // Fully clipped
            3 => vec![in_tri.clone()], // Fully inside
            1 => {
                let p0 = inside_points[0];
                let w0 = inside_ws[0];
                let p1 = Renderer3D::intersect_plane(plane_p, plane_n, p0, outside_points[0]);
                let p2 = Renderer3D::intersect_plane(plane_p, plane_n, p0, outside_points[1]);
    
                let w1 = w0; // Approximate w for interpolated points
                let w2 = w0;
    
                vec![Triangle {
                    v1: to_v4(p0, w0),
                    v2: to_v4(p1, w1),
                    v3: to_v4(p2, w2),
                    light_color: in_tri.light_color,
                }]
            }
            2 => {
                let p0 = inside_points[0];
                let w0 = inside_ws[0];
                let p1 = inside_points[1];
                let w1 = inside_ws[1];
                let i0 = Renderer3D::intersect_plane(plane_p, plane_n, p0, outside_points[0]);
                let i1 = Renderer3D::intersect_plane(plane_p, plane_n, p1, outside_points[0]);
    
                let w2 = w0; // Approximate w
                let w3 = w1;
    
                vec![
                    Triangle {
                        v1: to_v4(p0, w0),
                        v2: to_v4(p1, w1),
                        v3: to_v4(i0, w2),
                        light_color: in_tri.light_color,
                    },
                    Triangle {
                        v1: to_v4(p1, w1),
                        v2: to_v4(i1, w3),
                        v3: to_v4(i0, w2),
                        light_color: in_tri.light_color,
                    },
                ]
            }
            _ => vec![], // Should never happen
        }
    }
    

    pub fn draw_mesh(&mut self, _mesh: Mesh) {
        todo!()
    }

    pub fn make_projection(fov_degrees: f32, aspect_ratio: f32, near: f32, far: f32) -> Matrix4<f32> {
        let fov_rad = 1.0 / (fov_degrees * 0.5 / 180.0 * PI).tan();
        
        // Crear una matriz 4x4 inicializada en cero
        let mut matrix = Matrix4::zeros();
        
        // Asignar los elementos no nulos según la fórmula de proyección
        matrix[(0, 0)] = aspect_ratio * fov_rad; // m[0][0]
        matrix[(1, 1)] = fov_rad;                // m[1][1]
        matrix[(2, 2)] = far / (far - near);     // m[2][2]
        matrix[(3, 2)] = (-far * near) / (far - near); // m[3][2]
        matrix[(2, 3)] = 1.0;                    // m[2][3]
        matrix[(3, 3)] = 0.0;                    // m[3][3]
        
        matrix
    }

    pub fn point_at(camera: &Vector3<f32>, target: &Vector3<f32>, up: &Vector3<f32>) -> Matrix4<f32> {
        // Calcular el vector forward (dirección desde la cámara al objetivo)
        let forward = (*target - *camera).normalize();
        
        // Calcular el vector right (perpendicular a up y forward)
        let right = up.cross(&forward).normalize();
        
        // Calcular el vector up corregido (perpendicular a forward y right)
        let up = forward.cross(&right).normalize();
        
        // Construir la matriz de rotación
        let rotation = Matrix4::new(
            right.x, right.y, right.z, 0.0,
            up.x, up.y, up.z, 0.0,
            forward.x, forward.y, forward.z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );
        
        // Construir la matriz de traslación
        let translation = Matrix4::new_translation(&camera);
        
        // Combinar: rotación seguida de traslación
        translation * rotation
    }
    pub fn quick_inverse(matrix: &Matrix4<f32>) -> Matrix4<f32> {
        // Extraer la matriz de rotación (3x3)
        let rotation = Matrix3::from_iterator([
            matrix[(0,0)], matrix[(0,1)], matrix[(0,2)],
            matrix[(1,0)], matrix[(1,1)], matrix[(1,2)],
            matrix[(2,0)], matrix[(2,1)], matrix[(2,2)],
        ]);
        
        // La inversa de la rotación es su transpuesta (porque es ortogonal)
        let rotation_inv = rotation.transpose();
        
        // Extraer la traslación
        let translation = Vector3::new(matrix[(0,3)], matrix[(1,3)], matrix[(2,3)]);
        
        // La traslación inversa es -R^T * t
        let translation_inv = -(rotation_inv * translation);
        
        // Construir la matriz inversa
        Matrix4::new(
            rotation_inv[(0,0)], rotation_inv[(0,1)], rotation_inv[(0,2)], translation_inv.x,
            rotation_inv[(1,0)], rotation_inv[(1,1)], rotation_inv[(1,2)], translation_inv.y,
            rotation_inv[(2,0)], rotation_inv[(2,1)], rotation_inv[(2,2)], translation_inv.z,
            0.0, 0.0, 0.0, 1.0,
        )
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

    fn draw_square(&mut self, a: Vector2<i32>, b: Vector2<i32>, color: u32, filled: bool, fill_color: u32) {
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
