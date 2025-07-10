use std::f32::consts::PI;

use crate::engine::types::vector::{vector3::Vector3, vector4::Vector4, vector_ops::VectorOps};

#[derive(Clone, Copy, Debug)]
pub struct Matrix4x4 {
    pub m: [[f32; 4]; 4],
}

impl Matrix4x4 {
    pub fn identity() -> Self {
        let mut m = [[0.0; 4]; 4];
        m[0][0] = 1.0;
        m[1][1] = 1.0;
        m[2][2] = 1.0;
        m[3][3] = 1.0;
        Self { m }
    }

    pub fn zero() -> Self {
        Self { m: [[0.0; 4]; 4] }
    }

    pub fn translation(to: Vector3<f32>) -> Self {
        let mut mat = Self::identity();
        mat.m[3][0] = to.x;
        mat.m[3][1] = to.y;
        mat.m[3][2] = to.z;
        mat
    }

    pub fn rotation_x(angle: f32) -> Self {
        let mut mat = Self::identity();
        let cos = angle.cos();
        let sin = angle.sin();
        mat.m[1][1] = cos;
        mat.m[1][2] = sin;
        mat.m[2][1] = -sin;
        mat.m[2][2] = cos;
        mat
    }

    pub fn rotation_y(angle: f32) -> Self {
        let mut mat = Self::identity();
        let cos = angle.cos();
        let sin = angle.sin();
        mat.m[0][0] = cos;
        mat.m[0][2] = -sin;
        mat.m[2][0] = sin;
        mat.m[2][2] = cos;
        mat
    }

    pub fn rotation_z(angle: f32) -> Self {
        let mut mat = Self::identity();
        let cos = angle.cos();
        let sin = angle.sin();
        mat.m[0][0] = cos;
        mat.m[0][1] = sin;
        mat.m[1][0] = -sin;
        mat.m[1][1] = cos;
        mat
    }

    pub fn multiply_matrix(a: &Self, b: &Self) -> Self {
        let mut result = Self::zero();
        for r in 0..4 {
            for c in 0..4 {
                result.m[r][c] = a.m[r][0] * b.m[0][c]
                    + a.m[r][1] * b.m[1][c]
                    + a.m[r][2] * b.m[2][c]
                    + a.m[r][3] * b.m[3][c];
            }
        }
        result
    }

    pub fn multiply_vec(mat: &Self, vec: &Vector4<f32>) -> Vector4<f32> {
        let nx = vec.x * mat.m[0][0] + vec.y * mat.m[1][0] + vec.z * mat.m[2][0] + vec.w * mat.m[3][0];
        let ny = vec.x * mat.m[0][1] + vec.y * mat.m[1][1] + vec.z * mat.m[2][1] + vec.w * mat.m[3][1];
        let nz = vec.x * mat.m[0][2] + vec.y * mat.m[1][2] + vec.z * mat.m[2][2] + vec.w * mat.m[3][2];
        let nw = vec.x * mat.m[0][3] + vec.y * mat.m[1][3] + vec.z * mat.m[2][3] + vec.w * mat.m[3][3];
        Vector4 { x: nx, y: ny, z: nz, w: nw }
    }

    pub fn project(f_near: f32, f_far: f32, f_fov: f32, height: usize, width: usize) -> Matrix4x4{
        let f_aspect_ratio = (height / width) as f32;
        let f_fov_rad = 1.0 / (f_fov * 0.5 / 180.0 * PI).tan();

        let mut mat_proj = Matrix4x4::identity();
        mat_proj.m[0][0] = f_aspect_ratio * f_fov_rad;
        mat_proj.m[1][1] = f_fov_rad;
        mat_proj.m[2][2] = f_far / (f_far - f_near);
        mat_proj.m[3][2] = (-f_far * f_near) / (f_far - f_near);
        mat_proj.m[2][3] = 1.0;
        mat_proj.m[3][3] = 0.0;
        mat_proj
    }

    pub fn point_at(pos: Vector3<f32>, target: Vector3<f32>, up: Vector3<f32>) -> Matrix4x4 {
        // Calculate new forward direction
        let new_forward = (target - pos).normalize();

        // Calculate new Up direction
        let a = new_forward * up.dot(new_forward);
        let new_up = (up - a).normalize();

        // Construct Dimensioning and Translation Matrix
        let new_right = new_up.cross(new_forward.cast::<f32>().expect("Error casting vi32 to f32"));
        let mut matrix = Matrix4x4::identity();
        matrix.m[0][0] = new_right.x;	matrix.m[0][1] = new_right.y;	matrix.m[0][2] = new_right.z;	matrix.m[0][3] = 0.0;
		matrix.m[1][0] = new_up.x;		matrix.m[1][1] = new_up.y;		matrix.m[1][2] = new_up.z;		matrix.m[1][3] = 0.0;
		matrix.m[2][0] = new_forward.x;	matrix.m[2][1] = new_forward.y;	matrix.m[2][2] = new_forward.z;	matrix.m[2][3] = 0.0;
		matrix.m[3][0] = pos.x;			matrix.m[3][1] = pos.y;			matrix.m[3][2] = pos.z;			matrix.m[3][3] = 1.0;
        matrix

    }

    pub fn quick_inverse(&self) -> Self {
        let mut matrix = Matrix4x4::zero();

        matrix.m[0][0] = self.m[0][0];
        matrix.m[0][1] = self.m[1][0];
        matrix.m[0][2] = self.m[2][0];
        matrix.m[0][3] = 0.0;
        matrix.m[1][0] = self.m[0][1];
        matrix.m[1][1] = self.m[1][1];
        matrix.m[1][2] = self.m[2][1];
        matrix.m[1][3] = 0.0;
        matrix.m[2][0] = self.m[0][2];
        matrix.m[2][1] = self.m[1][2];
        matrix.m[2][2] = self.m[2][2];
        matrix.m[2][3] = 0.0;

        // Calcular la traslación inversa: -(traslación * matriz de rotación transpuesta)
        matrix.m[3][0] = -(self.m[3][0] * matrix.m[0][0] + self.m[3][1] * matrix.m[1][0] + self.m[3][2] * matrix.m[2][0]);
        matrix.m[3][1] = -(self.m[3][0] * matrix.m[0][1] + self.m[3][1] * matrix.m[1][1] + self.m[3][2] * matrix.m[2][1]);
        matrix.m[3][2] = -(self.m[3][0] * matrix.m[0][2] + self.m[3][1] * matrix.m[1][2] + self.m[3][2] * matrix.m[2][2]);
        matrix.m[3][3] = 1.0;

        matrix
    }
}