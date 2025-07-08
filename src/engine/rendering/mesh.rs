use crate::engine::types::{object3d::Object3D, triangle::Triangle};

pub struct Mesh {
    pub obj: Object3D,
    pub tris: Vec<Triangle>
}
