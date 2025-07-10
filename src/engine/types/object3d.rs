use crate::engine::types::vector::{matrix4x4::Matrix4x4, vector3::Vector3, vector_ops::VectorOps};


pub struct Object3D {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>, // pitch (X), yaw (Y), roll (Z)
}

impl Object3D {

    pub fn new(position: Vector3<f32>, rotation: Vector3<f32>) -> Self {
        Self { position, rotation }
    }

    pub fn zero() -> Self {
        Self {
            position: Vector3::zero(), rotation: Vector3::zero()
        }
    }

    pub fn rotation_matrix(&self) -> Matrix4x4 {
        let (rx, ry, rz) = (
            self.rotation.x,
            self.rotation.y,
            self.rotation.z,
        );

        let rot_x = Matrix4x4::rotation_x(rx);
        let rot_y = Matrix4x4::rotation_y(ry);
        let rot_z = Matrix4x4::rotation_z(rz);

        // Orden típica: Z * X * Y o Y * X * Z dependiendo del motor
        Matrix4x4::multiply_matrix(&rot_y, &Matrix4x4::multiply_matrix(&rot_x, &rot_z))
    }

    pub fn transform_matrix(&self) -> Matrix4x4 {
        let trans = Matrix4x4::translation(self.position);
        let rot = self.rotation_matrix();
        Matrix4x4::multiply_matrix(&rot, &trans)
    }
    
}