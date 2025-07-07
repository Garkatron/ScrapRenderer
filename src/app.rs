#![allow(dead_code, unused_variables, unused_imports, unused_import_braces)]

use std::{f32::consts::PI, usize, vec};

use crate::engine::{
    engine_3d::Engine3D,
    rendering::{camera::Camera3D, mesh::Mesh, renderer::Renderer, renderer_3d::Renderer3D},
    types::{
        colour::COLOUR,
        object3d::Object3D,
        triangle::Triangle,
        vector::{Matrix4x4, Vector3},
    },
};

pub struct MyApp {
    pub engine: Engine3D,
    pub objects: Vec<Mesh>,
    pub camera: Camera3D,
    pub f_theta: f32,
}

impl MyApp {
    pub fn new(width: usize, height: usize, window: minifb::Window) -> Self {
        let mut objects = vec![];

        objects.push(Mesh {
            tris: vec![
                // SOUTH
                Triangle {
                    v1: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                        w: 0.0,
                    },
                    v2: Vector3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                        w: 0.0,
                    },
                    v3: Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 0.0,
                        w: 0.0,
                    },
                },
                Triangle {
                    v1: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                        w: 0.0,
                    },
                    v2: Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 0.0,
                        w: 0.0,
                    },
                    v3: Vector3 {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                        w: 0.0,
                    },
                },
                // EAST
                Triangle {
                    v1: Vector3 {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                        w: 0.0,
                    },
                    v2: Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 0.0,
                        w: 0.0,
                    },
                    v3: Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                        w: 0.0,
                    },
                },
                Triangle {
                    v1: Vector3 {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                        w: 0.0,
                    },
                    v2: Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                        w: 0.0,
                    },
                    v3: Vector3 {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                        w: 0.0,
                    },
                },
                // NORTH
                Triangle {
                    v1: Vector3 {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                        w: 0.0,
                    },
                    v2: Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                        w: 0.0,
                    },
                    v3: Vector3 {
                        x: 0.0,
                        y: 1.0,
                        z: 1.0,
                        w: 0.0,
                    },
                },
                Triangle {
                    v1: Vector3 {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                        w: 0.0,
                    },
                    v2: Vector3 {
                        x: 0.0,
                        y: 1.0,
                        z: 1.0,
                        w: 0.0,
                    },
                    v3: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                        w: 0.0,
                    },
                },
                // WEST
                Triangle {
                    v1: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                        w: 0.0,
                    },
                    v2: Vector3 {
                        x: 0.0,
                        y: 1.0,
                        z: 1.0,
                        w: 0.0,
                    },
                    v3: Vector3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                        w: 0.0,
                    },
                },
                Triangle {
                    v1: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                        w: 0.0,
                    },
                    v2: Vector3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                        w: 0.0,
                    },
                    v3: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                        w: 0.0,
                    },
                },
                // TOP
                Triangle {
                    v1: Vector3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                        w: 0.0,
                    },
                    v2: Vector3 {
                        x: 0.0,
                        y: 1.0,
                        z: 1.0,
                        w: 0.0,
                    },
                    v3: Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                        w: 0.0,
                    },
                },
                Triangle {
                    v1: Vector3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                        w: 0.0,
                    },
                    v2: Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                        w: 0.0,
                    },
                    v3: Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 0.0,
                        w: 0.0,
                    },
                },
                // BOTTOM
                Triangle {
                    v1: Vector3 {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                        w: 0.0,
                    },
                    v2: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                        w: 0.0,
                    },
                    v3: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                        w: 0.0,
                    },
                },
                Triangle {
                    v1: Vector3 {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                        w: 0.0,
                    },
                    v2: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                        w: 0.0,
                    },
                    v3: Vector3 {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                        w: 0.0,
                    },
                },
            ],
        });

        Self {
            engine: Engine3D {
                renderer: Renderer3D::new(vec![0; width * height], width, height, window),
            },
            objects,
            camera: Camera3D::new(Vector3::new(0.0, 0.1, 5.0, 0.0)),
            f_theta: 0.0,
        }
    }

    pub fn render(&mut self, delta_time: f32) {
        println!("FPS: {:.2}", 1.0 / delta_time);

        self.engine.renderer.clear(COLOUR::BLACK.to_u32());

        let mut mat_rot_z = Matrix4x4::identity();
        let mut mat_rot_x = Matrix4x4::identity();
        self.f_theta += 1.0 * delta_time;

        mat_rot_z.m[0][0] = self.f_theta.cos();
        mat_rot_z.m[0][1] = self.f_theta.sin();
        mat_rot_z.m[1][0] = -self.f_theta.sin();
        mat_rot_z.m[1][1] = self.f_theta.cos();
        mat_rot_z.m[2][2] = 1.0;
        mat_rot_z.m[3][3] = 1.0;

        mat_rot_x.m[0][0] = 1.0;
        mat_rot_x.m[1][1] = (self.f_theta * 0.5).cos();
        mat_rot_x.m[1][2] = (self.f_theta * 0.5).sin();
        mat_rot_x.m[2][1] = -(self.f_theta * 0.5).sin();
        mat_rot_x.m[2][2] = (self.f_theta * 0.5).cos();
        mat_rot_x.m[3][3] = 1.0;

        let mut mat_proj = Matrix4x4 {
            m: [[0.0; 4], [0.0; 4], [0.0; 4], [0.0; 4]],
        };

        let width = self.engine.renderer.width() as f32;
        let height = self.engine.renderer.height() as f32;

        let f_near = 0.1;
        let f_far = 1000.0;
        let f_fov = 90.0;
        let f_aspect_ratio =
            self.engine.renderer.height() as f32 / self.engine.renderer.width() as f32;
        let f_fov_rad = 1.0 / (f_fov * 0.5 / 180.0 * PI).tan();

        mat_proj.m[0][0] = f_aspect_ratio * f_fov_rad;
        mat_proj.m[1][1] = f_fov_rad;
        mat_proj.m[2][2] = f_far / (f_far - f_near);
        mat_proj.m[3][2] = (-f_far * f_near) / (f_far - f_near);
        mat_proj.m[2][3] = 1.0;
        mat_proj.m[3][3] = 0.0;

        for mesh in &self.objects {
            for tri in &mesh.tris {
                // 1. Rotar el triángulo
                let v1_rot_z = Matrix4x4::multiply_vec(&mat_rot_z, &tri.v1);
                let v2_rot_z = Matrix4x4::multiply_vec(&mat_rot_z, &tri.v2);
                let v3_rot_z = Matrix4x4::multiply_vec(&mat_rot_z, &tri.v3);

                let v1_rot_x = Matrix4x4::multiply_vec(&mat_rot_x, &v1_rot_z);
                let v2_rot_x = Matrix4x4::multiply_vec(&mat_rot_x, &v2_rot_z);
                let v3_rot_x = Matrix4x4::multiply_vec(&mat_rot_x, &v3_rot_z);

                // 2. Trasladar el triángulo hacia adelante
                let translated = Triangle {
                    v1: Vector3::new(v1_rot_x.x, v1_rot_x.y, v1_rot_x.z + 1.0, 1.0),
                    v2: Vector3::new(v2_rot_x.x, v2_rot_x.y, v2_rot_x.z + 1.0, 1.0),
                    v3: Vector3::new(v3_rot_x.x, v3_rot_x.y, v3_rot_x.z + 1.0, 1.0),
                };
                // 2. Aplicar la matriz de vista a cada vértice traducido
                let v1_camera = Matrix4x4::multiply_vec(&self.camera.mat_view, &translated.v1);
                let v2_camera = Matrix4x4::multiply_vec(&self.camera.mat_view, &translated.v2);
                let v3_camera = Matrix4x4::multiply_vec(&self.camera.mat_view, &translated.v3);

                // 3. Proyectar los vértices en espacio de cámara a espacio 2D
                let v1_projected = Matrix4x4::multiply_vec(&mat_proj, &v1_camera);
                let v2_projected = Matrix4x4::multiply_vec(&mat_proj, &v2_camera);
                let v3_projected = Matrix4x4::multiply_vec(&mat_proj, &v3_camera);

                // 4. Ya con v*_projected haces la conversión a pantalla
                let mut projected = Triangle {
                    v1: v1_projected,
                    v2: v2_projected,
                    v3: v3_projected,
                };

                for v in [&mut projected.v1, &mut projected.v2, &mut projected.v3] {
                    v.x = (v.x + 1.0) * 0.5 * width;
                    v.y = (1.0 - v.y) * 0.5 * height;
                }

                self.engine.renderer.draw_triangle(
                    projected.v1.to_vi2d(),
                    projected.v2.to_vi2d(),
                    projected.v3.to_vi2d(),
                    COLOUR::YELLOW.to_u32(),
                );
            }
        }

        self.engine.render(delta_time);
    }

    pub fn update(&mut self, delta_time: f32) {}
}
