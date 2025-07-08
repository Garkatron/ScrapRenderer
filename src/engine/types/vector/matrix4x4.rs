use crate::engine::types::vector::{vector3::Vector3, vector4::Vector4};

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

    pub fn translation(to: Vector3) -> Self {
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

    pub fn multiply_vec(mat: &Self, vec: &Vector3) -> Vector3 {
        let v4 = vec.to_vector4(1.0);
        let nx = v4.x * mat.m[0][0] + v4.y * mat.m[1][0] + v4.z * mat.m[2][0] + v4.w * mat.m[3][0];
        let ny = v4.x * mat.m[0][1] + v4.y * mat.m[1][1] + v4.z * mat.m[2][1] + v4.w * mat.m[3][1];
        let nz = v4.x * mat.m[0][2] + v4.y * mat.m[1][2] + v4.z * mat.m[2][2] + v4.w * mat.m[3][2];
        let nw = v4.x * mat.m[0][3] + v4.y * mat.m[1][3] + v4.z * mat.m[2][3] + v4.w * mat.m[3][3];
        Vector4 { x: nx, y: ny, z: nz, w: nw }.to_vector3()
    }
}