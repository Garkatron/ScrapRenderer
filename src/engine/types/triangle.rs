use nalgebra::{Vector3, Vector4};


#[derive(Clone, PartialEq)]
pub struct Triangle {
    pub v1: Vector4<f32>,
    pub v2: Vector4<f32>,
    pub v3: Vector4<f32>,
    pub light_color: u32
}

impl Triangle {
    pub fn new(v1: Vector4<f32>, v2: Vector4<f32>, v3: Vector4<f32>,) -> Self {
        Self {
            v1,
            v2,
            v3,
            light_color: 0
        }
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            v1: Vector4::zeros(),
            v2: Vector4::zeros(),
            v3: Vector4::zeros(),
            light_color: 0
        }
    }
}

