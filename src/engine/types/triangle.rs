use crate::engine::types::vector::vector3::Vector3;

#[derive(Clone, PartialEq)]
pub struct Triangle {
    pub v1: Vector3,
    pub v2: Vector3,
    pub v3: Vector3,
}

