use crate::engine::types::vector::{vector3::Vector3, vector_ops::VectorOps};

#[derive(Clone, PartialEq)]
pub struct Triangle {
    pub v1: Vector3,
    pub v2: Vector3,
    pub v3: Vector3,
    pub light_color: u32
}

impl Triangle {
    pub fn new(v1: Vector3, v2: Vector3, v3: Vector3,) -> Self {
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
            v1: Vector3::zero(),
            v2: Vector3::zero(),
            v3: Vector3::zero(),
            light_color: 0
        }
    }
}

