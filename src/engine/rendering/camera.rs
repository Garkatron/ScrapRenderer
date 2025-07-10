#![allow(unused_variables)]
use crate::engine::types::vector::{matrix4x4::Matrix4x4, vector_ops::VectorOps, vector3::Vector3};

pub struct Camera3D {
    pub position: Vector3<f32>,
    pub look_dir: Vector3<f32>,
    pub f_yaw: f32,
    //pub v_target: Vector3<f32>,
}

impl Camera3D {
    pub fn new(position: Vector3<f32>, width: usize, height: usize) -> Self {
        Self {
            f_yaw: 0.0,
            position,
            look_dir: Vector3::zero(),
            
        }
    }

    pub fn calc_view(&mut self) -> Matrix4x4 {
        let v_up = Vector3::up();

        let mut v_target = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };

        let mat_camera_rot = Matrix4x4::rotation_y(self.f_yaw);
        self.look_dir = Matrix4x4::multiply_vec(&mat_camera_rot, &v_target);

        v_target = self.position + self.look_dir;

        let mat_camera = Matrix4x4::point_at(self.position, v_target, v_up);
        Matrix4x4::quick_inverse(&mat_camera)
    }
}
