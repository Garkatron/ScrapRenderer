#![allow(dead_code, unused_variables, unused_imports, unused_import_braces)]

use core::time;
use std::{
    cell::RefCell, collections::btree_map::VacantEntry, default, f32::consts::PI, fs, rc::Rc,
    usize, vec,
};

use minifb::Key;

use crate::engine::{
    control::keyboard::KeyboardController,
    engine_3d::Engine3D,
    loader::obj_loader::ObjLoader,
    rendering::{
        camera::Camera3D,
        mesh::Mesh,
        palettes::{PALETTE_DEFAULT, PALETTE_PINK, PaletteDefault, PalettePink},
        renderer::Renderer,
        renderer_3d::Renderer3D,
    },
    types::{
        object3d::Object3D,
        triangle::Triangle,
        vector::{
            matrix4x4::{self, Matrix4x4},
            vector_ops::VectorOps,
            vector3::Vector3,
        },
    },
};

pub struct MyApp {
    pub window: Rc<RefCell<minifb::Window>>,
    pub engine: Engine3D,
    //pub kbcontroller: KeyboardController<'a>,
    pub objects: Vec<Mesh>,
    pub camera: Camera3D,
    pub f_theta: f32,
    pub mat_proj: Matrix4x4,
}

impl MyApp {
    pub fn new(width: usize, height: usize, window: minifb::Window) -> Self {
        let w = Rc::new(RefCell::new(window));
        let mut objects = vec![];

        let mut obj = ObjLoader::from_file("/home/deus/Documents/models/african_head.obj").unwrap();
        obj.obj.position.z += 20.0;
        obj.obj.rotation.y += 0.0;
        obj.obj.rotation.x += 0.0;
        obj.obj.rotation.z += 0.0;
        objects.push(obj);

        let mat_proj = Matrix4x4::project(0.1, 1000.0, 90.0, height, width);

        Self {
            window: w.clone(),
            engine: Engine3D {
                running: true,
                renderer: Renderer3D::new(vec![0; width * height], width, height, w.clone()),
                kbcontroller: KeyboardController::new(w.clone()),
            },
            //kbcontroller: KeyboardController::new(&window)
            objects,
            camera: Camera3D::new(Vector3::new(0.0, 0.0, 0.0), width, height),
            //camera: Camera3D::new(Vector3::new(0.0, 0.1, 5.0), width, height),
            f_theta: 0.0,
            mat_proj,
        }
    }

    pub fn render(&mut self, delta_time: f32) {
        //println!("FPS: {:.2}", 1.0 / delta_time);

        self.engine.renderer.clear(PALETTE_DEFAULT::BLACK.to_u32());

        self.f_theta += 1.0 * delta_time;
        // self.objects.get_mut(0).unwrap().obj.rotation.y = self.f_theta;

        for Mesh { obj, tris } in &self.objects {
            let mut triangles_to_raster: Vec<Triangle> = vec![];

            for tri in tris {
                // 1. Rotar triángulo (usando matriz de rotación de objeto)
                let rotation_matrix = obj.rotation_matrix();

                // 2. Translate
                let transform_matrix = obj.transform_matrix();

                // 3.
                let world_matrix = Matrix4x4::multiply_matrix(&rotation_matrix, &transform_matrix);

                let camera_matrix = self.camera.calc_view();


                let tri_transformed = Triangle {
                    v1: Matrix4x4::multiply_vec(&world_matrix, &tri.v1),
                    v2: Matrix4x4::multiply_vec(&world_matrix, &tri.v2),
                    v3: Matrix4x4::multiply_vec(&world_matrix, &tri.v3),
                    light_color: 0,
                };

                // Calc Normal
                let l1 = tri_transformed.v2 - tri_transformed.v1;
                let l2 = tri_transformed.v3 - tri_transformed.v1;
                let normal = l1.cross(l2).normalize(); // You normally need to normalize a normal

                // Avoid divide by 0 and triangles before camera.
                if tri_transformed.v1.z <= 0.0
                    || tri_transformed.v2.z <= 0.0
                    || tri_transformed.v3.z <= 0.0
                {
                    continue;
                }

                // Get ray from triangle to camera
                let v_camera_ray = tri_transformed.v1 - self.camera.position;

                // If ray is aligned with normal, make it visible.
                if normal.dot(v_camera_ray) < 0.0 {
                    let light_direction = Vector3::new(0.0, 1.0, -1.0).normalize();
                    let dp = light_direction.dot(normal); // How "aligned" are light direction and triangle sureface normal?

                    let colour: u32 = Renderer3D::get_shading_color(dp, &PalettePink);

                    let viewed_triangle = Triangle {
                        v1: Matrix4x4::multiply_vec(&camera_matrix, &tri_transformed.v1),
                        v2: Matrix4x4::multiply_vec(&camera_matrix, &tri_transformed.v2),
                        v3: Matrix4x4::multiply_vec(&camera_matrix, &tri_transformed.v3),
                        light_color: colour,
                    };

                    // Clip viewed triangle againts near plane, this could form two aditional triangles.

                    let clipped = Renderer3D::triangle_clip_against_plane(
                        Vector3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.2,
                        },
                        Vector3 {
                            x: 0.0,
                            y: 0.0,
                            z: 1.0,
                        },
                        &viewed_triangle,
                    );

                    for tc in clipped {
                        let mut projected = Triangle {
                            v1: Matrix4x4::multiply_vec(&self.mat_proj, &tc.v1),
                            v2: Matrix4x4::multiply_vec(&self.mat_proj, &tc.v2),
                            v3: Matrix4x4::multiply_vec(&self.mat_proj, &tc.v3),
                            light_color: tc.light_color,
                        };

                        // Convertir a coordenadas de pantalla
                        for v in [&mut projected.v1, &mut projected.v2, &mut projected.v3] {
                            v.x = (v.x + 1.0) * 0.5 * self.engine.renderer.width() as f32;
                            v.y = (1.0 - v.y) * 0.5 * self.engine.renderer.height() as f32;
                        }

                        triangles_to_raster.push(projected);
                    }
                }
            }

            //  Sort back to front
            triangles_to_raster.sort_by(|t1, t2| {
                let z1 = (t1.v1.z + t1.v2.z + t1.v3.z) / 3.0;
                let z2 = (t2.v1.z + t2.v2.z + t2.v3.z) / 3.0;
                z1.partial_cmp(&z2)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .reverse() // Back to front (descending order)
            });

            // Loop through all transformed, viewed, projected, and sorted triangles
            for tri_to_raster in triangles_to_raster {
                let mut tri_queue: Vec<Triangle> = vec![tri_to_raster];

                for edge in 0..4 {
                    let mut new_triangles: Vec<Triangle> = vec![];

                    for test in tri_queue.drain(..) {
                        let clipped = match edge {
                            0 => Renderer3D::triangle_clip_against_plane(
                                Vector3 {
                                    x: 0.0,
                                    y: 0.0,
                                    z: 0.0,
                                },
                                Vector3 {
                                    x: 0.0,
                                    y: 1.0,
                                    z: 0.0,
                                },
                                &test,
                            ),
                            1 => Renderer3D::triangle_clip_against_plane(
                                Vector3 {
                                    x: 0.0,
                                    y: (self.engine.renderer.height() as f32) - 1.0,
                                    z: 0.0,
                                },
                                Vector3 {
                                    x: 0.0,
                                    y: -1.0,
                                    z: 0.0,
                                },
                                &test,
                            ),
                            2 => Renderer3D::triangle_clip_against_plane(
                                Vector3 {
                                    x: 0.0,
                                    y: 0.0,
                                    z: 0.0,
                                },
                                Vector3 {
                                    x: 1.0,
                                    y: 0.0,
                                    z: 0.0,
                                },
                                &test,
                            ),
                            3 => Renderer3D::triangle_clip_against_plane(
                                Vector3 {
                                    x: (self.engine.renderer.width() as f32) - 1.0,
                                    y: 0.0,
                                    z: 0.0,
                                },
                                Vector3 {
                                    x: -1.0,
                                    y: 0.0,
                                    z: 0.0,
                                },
                                &test,
                            ),
                            _ => vec![],
                        };
                        new_triangles.extend(clipped);
                    }

                    tri_queue = new_triangles;
                }
                for t in tri_queue {
                    self.engine.renderer.fill_triangle(
                        t.v1.into(),
                        t.v2.into(),
                        t.v3.into(),
                        t.light_color,
                    );
                    /*
                    self.engine.renderer.draw_triangle(
                        t.v1.into(),
                        t.v2.into(),
                        t.v3.into(),
                        PALETTE_DEFAULT::BLACK.to_u32(),
                    );*/
                }
            }
        }

        self.engine.render(delta_time);
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.engine.kbcontroller.is_key_down(Key::Escape) && self.window.borrow().is_open() {
            if self.engine.kbcontroller.is_key_down(Key::A) {
                self.camera.position.x -= 8.0 * delta_time;
            }

            if self.engine.kbcontroller.is_key_down(Key::D) {
                self.camera.position.x += 8.0 * delta_time;
            }

            if self.engine.kbcontroller.is_key_down(Key::LeftShift) {
                self.camera.position.y -= 8.0 * delta_time;
            }

            if self.engine.kbcontroller.is_key_down(Key::Space) {
                self.camera.position.y += 8.0 * delta_time;
            }

            let v_forward = self.camera.look_dir * (4.0 * delta_time);

            if self.engine.kbcontroller.is_key_down(Key::W) {
                self.camera.position = self.camera.position + v_forward;
            }

            if self.engine.kbcontroller.is_key_down(Key::S) {
                self.camera.position = self.camera.position - v_forward;
            }

            if self.engine.kbcontroller.is_key_down(Key::Right) {
                self.camera.f_yaw += 2.0 * delta_time;
            }

            if self.engine.kbcontroller.is_key_down(Key::Left) {
                self.camera.f_yaw -= 2.0 * delta_time;
            }
        } else {
            self.engine.running = false;
        }
    }
}
