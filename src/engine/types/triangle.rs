use crate::engine::types::vector::{vector3::Vector3, vector4::Vector4, vector_ops::VectorOps};


#[derive(Clone, PartialEq)]
pub struct Triangle {
    pub v1: Vector4<f32>,
    pub v2: Vector4<f32>,
    pub v3: Vector4<f32>,
    pub light_color: u32,
    pub uv: [Vector3<f32>; 3],
}

impl Triangle {
    pub fn new(v1: Vector4<f32>, v2: Vector4<f32>, v3: Vector4<f32>) -> Self {
        Self {
            v1,
            v2,
            v3,
            light_color: 0,
            uv: [Vector3::zero(); 3]
        }
    }

    pub fn set_uv(mut self, uv: [Vector3<f32>; 3]) -> Self {
        self.uv = uv;
        self
    }

    pub fn set_light_color(mut self, light_color: u32) -> Self {
        self.light_color = light_color;
        self
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            v1: Vector4::zero(),
            v2: Vector4::zero(),
            v3: Vector4::zero(),
            light_color: 0,
            uv: [Vector3::zero(); 3]
        }
    }
}

