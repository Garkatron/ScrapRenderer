#![allow(unused_variables)]

use crate::engine::types::vector::{Mat4, Vec3, Vec4};

pub struct Camera3D {
    pub position: Vec3,
    pub look_dir: Vec3,
    pub f_yaw: f32,
    //pub v_target: Vector3<f32>,
}

impl Camera3D {
    pub fn new(position: Vec3, width: usize, height: usize) -> Self {
        Self {
            f_yaw: 0.0,
            position,
            look_dir: Vec3::new(0.0, 0.0, 1.0),
        }
    }

    pub fn update_look_dir(&mut self) {
        self.look_dir = Vec3::new(self.f_yaw.sin(), 0.0, self.f_yaw.cos());
    }

    pub fn calc_view(&self) -> Mat4 {
        // Posición de la cámara
        let eye = self.position;

        // Rotación de la cámara solo por yaw
        let yaw = self.f_yaw;
        let look_dir = Vec3::new(yaw.sin(), 0.0, yaw.cos());

        // Punto al que mira la cámara
        let target = eye + look_dir;

        // Vector "up"
        let up = Vec3::y();

        // Forward, right y recalcular up
        let f = (target - eye).normalize(); // forward
        let r = f.cross(&up).normalize(); // right
        let u = r.cross(&f); // recalculado up

        Mat4::new(
            r.x,
            u.x,
            -f.x,
            0.0,
            r.y,
            u.y,
            -f.y,
            0.0,
            r.z,
            u.z,
            -f.z,
            0.0,
            -r.dot(&eye),
            -u.dot(&eye),
            f.dot(&eye),
            1.0,
        )
    }
}
