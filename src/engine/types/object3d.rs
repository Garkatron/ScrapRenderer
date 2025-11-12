use nalgebra::{Rotation3, Translation3};

use crate::engine::types::vector::{Mat4, Vec3};

pub struct Object3D {
    pub position: Vec3,
    pub rotation: Vec3, // pitch (X), yaw (Y), roll (Z)
}

impl Object3D {
    pub fn new(position: Vec3, rotation: Vec3) -> Self {
        Self { position, rotation }
    }

    pub fn zero() -> Self {
        Self {
            position: Vec3::zeros(),
            rotation: Vec3::zeros(),
        }
    }

    pub fn rotation_matrix(&self) -> Mat4 {
        let (pitch, yaw, roll) = (self.rotation.x, self.rotation.y, self.rotation.z);

        let rotation = Rotation3::from_euler_angles(pitch, yaw, roll);

        rotation.to_homogeneous()
    }

    pub fn transform_matrix(&self) -> Mat4 {
        // Future scale here

        let translation = Translation3::from(self.position).to_homogeneous();
        let rotation = self.rotation_matrix();
        translation * rotation
    }
}
