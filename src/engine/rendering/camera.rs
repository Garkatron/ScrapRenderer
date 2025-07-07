use crate::engine::types::{object3d::{Object3D, Transformable}, vector::{Matrix4x4, Vector3}};

pub struct Camera3D {
    base: Object3D,
    pub mat_view: Matrix4x4,
}

impl Camera3D {
    pub fn new(position: Vector3) -> Self {
        let inv_pos = Vector3 { x: -position.x, y: -position.y, z: -position.z, w: 0.0 };
        Self {
            base: Object3D { position },
            mat_view: Matrix4x4::translation(inv_pos),
        }
    }
}

impl Transformable for Camera3D {
    fn position(&self) -> &Vector3 {
        &self.base.position
    }
    fn position_mut(&mut self) -> &mut Vector3 {
        &mut self.base.position
    }
    fn translate(&mut self, to: Vector3) {
        self.base.position = to;
    
        let inv_pos = Vector3 { x: -to.x, y: -to.y, z: -to.z, w: 0.0 };
        self.mat_view = Matrix4x4::translation(inv_pos);
    }
    
}