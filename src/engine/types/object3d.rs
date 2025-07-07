use crate::engine::{types::{vector::Vector3}};

pub struct Object3D {
    pub position: Vector3
}

pub trait Transformable {
    fn position(&self) -> &Vector3;
    fn position_mut(&mut self) -> &mut Vector3;

    fn translate(&mut self, to: Vector3) {
        let pos = self.position_mut();
        pos.x += to.x;
        pos.y += to.y;
        pos.z += to.z;
    }
}

impl Transformable for Object3D {
    fn position(&self) -> &Vector3 {
        &self.position
    }
    fn position_mut(&mut self) -> &mut Vector3 {
        &mut self.position
    }
}