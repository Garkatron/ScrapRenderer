#![allow(dead_code, unused_variables, unused_imports, unused_import_braces)]

use std::{f32::consts::PI, usize, vec};

use crate::engine::{
    engine_3d::Engine3D,
    rendering::{camera::Camera3D, mesh::Mesh, renderer::Renderer, renderer_3d::Renderer3D},
    types::{
        colour::COLOUR,
        object3d::Object3D,
        triangle::Triangle,
        vector::{matrix4x4::Matrix4x4, vector3::Vector3},
    },
};

pub struct MyApp {
    pub engine: Engine3D,
    pub objects: Vec<Mesh>,
    pub camera: Vector3,
    pub f_theta: f32,
    pub mat_proj: Matrix4x4,
}

impl MyApp {
    pub fn new(width: usize, height: usize, window: minifb::Window) -> Self {
        let mut objects = vec![];

        objects.push(Mesh {
            obj: Object3D::new(Vector3::new(0.0, 0.0, 5.0), Vector3::new(1.0, 10.0, 0.0)),
            tris: vec![
                // SOUTH
                Triangle {
                    v1: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    v2: Vector3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    v3: Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 0.0,
                    },
                },
                Triangle {
                    v1: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    v2: Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    v3: Vector3 {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                },
                // EAST
                Triangle {
                    v1: Vector3 {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    v2: Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    v3: Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                },
                Triangle {
                    v1: Vector3 {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    v2: Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    v3: Vector3 {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                    },
                },
                // NORTH
                Triangle {
                    v1: Vector3 {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    v2: Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    v3: Vector3 {
                        x: 0.0,
                        y: 1.0,
                        z: 1.0,
                    },
                },
                Triangle {
                    v1: Vector3 {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    v2: Vector3 {
                        x: 0.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    v3: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                    },
                },
                // WEST
                Triangle {
                    v1: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    v2: Vector3 {
                        x: 0.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    v3: Vector3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                },
                Triangle {
                    v1: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    v2: Vector3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    v3: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                },
                // TOP
                Triangle {
                    v1: Vector3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    v2: Vector3 {
                        x: 0.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    v3: Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                },
                Triangle {
                    v1: Vector3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    v2: Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    v3: Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 0.0,
                    },
                },
                // BOTTOM
                Triangle {
                    v1: Vector3 {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    v2: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    v3: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                },
                Triangle {
                    v1: Vector3 {
                        x: 1.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    v2: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    v3: Vector3 {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                },
            ],
        });

        let f_near = 0.1;
        let f_far = 1000.0;
        let f_fov = 90.0;
        let f_aspect_ratio = (height / width) as f32;
        let f_fov_rad = 1.0 / (f_fov * 0.5 / 180.0 * PI).tan();

        let mut mat_proj = Matrix4x4::identity();
        mat_proj.m[0][0] = f_aspect_ratio * f_fov_rad;
        mat_proj.m[1][1] = f_fov_rad;
        mat_proj.m[2][2] = f_far / (f_far - f_near);
        mat_proj.m[3][2] = (-f_far * f_near) / (f_far - f_near);
        mat_proj.m[2][3] = 1.0;
        mat_proj.m[3][3] = 0.0;

        Self {
            engine: Engine3D {
                renderer: Renderer3D::new(vec![0; width * height], width, height, window),
            },
            objects,
            camera: Vector3::new(0.0, 0.0, 0.0),
            //camera: Camera3D::new(Vector3::new(0.0, 0.1, 5.0), width, height),
            f_theta: 0.0,
            mat_proj,
        }
    }

    pub fn render(&mut self, delta_time: f32) {
        println!("FPS: {:.2}", 1.0 / delta_time);

        self.engine.renderer.clear(COLOUR::BLACK.to_u32());

        self.f_theta += 1.0 * delta_time;

        for Mesh { obj, tris } in &self.objects {
            for tri in tris {
                // 1. Rotar triángulo (usando matriz de rotación de objeto)
                let v1_rot = Matrix4x4::multiply_vec(&obj.rotation_matrix(), &tri.v1);
                let v2_rot = Matrix4x4::multiply_vec(&obj.rotation_matrix(), &tri.v2);
                let v3_rot = Matrix4x4::multiply_vec(&obj.rotation_matrix(), &tri.v3);

                // 2. Trasladar triángulo
                let translated = Triangle {
                    v1: Vector3::new(
                        v1_rot.x + obj.position.x,
                        v1_rot.y + obj.position.y,
                        v1_rot.z + obj.position.z,
                    ),
                    v2: Vector3::new(
                        v2_rot.x + obj.position.x,
                        v2_rot.y + obj.position.y,
                        v2_rot.z + obj.position.z,
                    ),
                    v3: Vector3::new(
                        v3_rot.x + obj.position.x,
                        v3_rot.y + obj.position.y,
                        v3_rot.z + obj.position.z,
                    ),
                };

                // Calcular normal
                let l1 = translated.v2 - translated.v1;
                let l2 = translated.v3 - translated.v1;
                let normal = l1.cross(&l2).normalize();

                // Evitar dividir por cero o vértices detrás de cámara
                if translated.v1.z <= 0.0 || translated.v2.z <= 0.0 || translated.v3.z <= 0.0 {
                    continue;
                }

                // if normal.z < 0.0 {
                if normal.x * (translated.v1.x - self.camera.x)
                    + normal.y * (translated.v1.y - self.camera.y)
                    + normal.z * (translated.v1.z - self.camera.z) < 0.0
                {
                    let mut projected = Triangle {
                        v1: Matrix4x4::multiply_vec(&self.mat_proj, &translated.v1),
                        v2: Matrix4x4::multiply_vec(&self.mat_proj, &translated.v2),
                        v3: Matrix4x4::multiply_vec(&self.mat_proj, &translated.v3),
                    };

                    // Convertir a coordenadas de pantalla
                    for v in [&mut projected.v1, &mut projected.v2, &mut projected.v3] {
                        v.x = (v.x + 1.0) * 0.5 * self.engine.renderer.width() as f32;
                        v.y = (1.0 - v.y) * 0.5 * self.engine.renderer.height() as f32;
                    }

                    self.engine.renderer.draw_triangle(
                        projected.v1.into(),
                        projected.v2.into(),
                        projected.v3.into(),
                        COLOUR::GREEN.to_u32(),
                    );
                }
            }
        }

        self.engine.render(delta_time);
    }

    pub fn update(&mut self, delta_time: f32) {}
}
