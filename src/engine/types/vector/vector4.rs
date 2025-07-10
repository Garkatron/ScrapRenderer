use crate::engine::types::vector::vector3::Vector3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn from_vector3(v: Vector3<f32>, w: f32) -> Vector4 {
        Vector4 {
            x: v.x,
            y: v.y,
            z: v.z,
            w,
        }
    }

    pub fn to_vector3(&self) -> Vector3<f32> {
        if self.w != 0.0 {
            Vector3 {
                x: self.x / self.w,
                y: self.y / self.w,
                z: self.z / self.w,
            }
        } else {
            Vector3 {
                x: self.x,
                y: self.y,
                z: self.z,
            }
        }
    }
}
