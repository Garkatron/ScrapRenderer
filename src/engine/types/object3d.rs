use nalgebra::{Matrix4, Rotation3, Vector3};


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
            position: Vector3::zeros(), rotation: Vector3::zeros()
        }
    }

    pub fn rotation_matrix(&self) -> Matrix4<f32> {
        // Extraer los ángulos de rotación en radianes
        let (rx, ry, rz) = (
            self.rotation.x,
            self.rotation.y,
            self.rotation.z,
        );

        // Crear matrices de rotación alrededor de los ejes X, Y, Z
        let rot_x = Rotation3::from_axis_angle(&nalgebra::Vector3::x_axis(), rx).to_homogeneous();
        let rot_y = Rotation3::from_axis_angle(&nalgebra::Vector3::y_axis(), ry).to_homogeneous();
        let rot_z = Rotation3::from_axis_angle(&nalgebra::Vector3::z_axis(), rz).to_homogeneous();

        // Combinar las rotaciones en el orden Y * X * Z
        rot_y * rot_x * rot_z
    }

    pub fn transform_matrix(&self) -> Matrix4<f32> {
        // Crear matriz de traslación
        let trans = Matrix4::new_translation(&self.position);
        // Obtener matriz de rotación
        let rot = self.rotation_matrix();
        // Combinar rotación y traslación: T * R
        trans * rot
    }
}
    
