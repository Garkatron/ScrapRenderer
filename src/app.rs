#![allow(dead_code, unused_variables, unused_imports, unused_import_braces)]

use core::time;
use std::{default, f32::consts::PI, fs, usize, vec};

use crate::engine::{
    control::keyboard::KeyboardController, engine_3d::Engine3D, loader::obj_loader::ObjLoader, rendering::{camera::Camera3D, mesh::Mesh, renderer::Renderer, renderer_3d::Renderer3D}, types::{
        colour::COLOUR,
        object3d::Object3D,
        triangle::Triangle,
        vector::{matrix4x4::{self, Matrix4x4}, vector2i::Vector2i, vector3::Vector3, vector_ops::VectorOps},
    }
};

pub struct MyApp {
    pub engine: Engine3D,
    //pub kbcontroller: KeyboardController<'a>,
    pub objects: Vec<Mesh>,
    pub camera: Vector3,
    pub look_dir: Vector3,
    pub f_theta: f32,
    pub mat_proj: Matrix4x4,
}

impl MyApp {
    pub fn new(width: usize, height: usize, window: minifb::Window) -> Self {
        let mut objects = vec![];

        let mut obj = ObjLoader::from_file("/home/deus/Documents/models/axis.obj").unwrap();
        obj.obj.position.z += 20.0;
        obj.obj.rotation.y += 0.0;
        obj.obj.rotation.x += 0.0;
        obj.obj.rotation.z += 0.0;
        objects.push(obj);

        let mat_proj = Matrix4x4::project(0.1, 1000.0, 90.0, height, width);

        Self {
            engine: Engine3D {
                renderer: Renderer3D::new(vec![0; width * height], width, height, window),
            },
            //kbcontroller: KeyboardController::new(&window)
            objects,
            camera: Vector3::new(0.0, 0.0, 0.0),
            look_dir: Vector3::zero(),
            //camera: Camera3D::new(Vector3::new(0.0, 0.1, 5.0), width, height),
            f_theta: 0.0,
            mat_proj,
        }
    }

    pub fn render(&mut self, delta_time: f32) {
        //println!("FPS: {:.2}", 1.0 / delta_time);

        self.engine.renderer.clear(COLOUR::BLACK.to_u32());

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

                self.look_dir = Vector3::new(0.0, 0.0, 1.0);
                let v_up = Vector3::up();
                let v_target = self.camera + self.look_dir;

                let mat_camera = Matrix4x4::point_at(self.camera, v_target, v_up);
                let mat_view = Matrix4x4::quick_inverse(&mat_camera);

                let tri_transformed = Triangle {
                    v1: Matrix4x4::multiply_vec(&world_matrix, &tri.v1),
                    v2: Matrix4x4::multiply_vec(&world_matrix, &tri.v2),
                    v3: Matrix4x4::multiply_vec(&world_matrix, &tri.v3),
                    light_color: 0
                };

                // Calc Normal
                let l1 = tri_transformed.v2 - tri_transformed.v1;
                let l2 = tri_transformed.v3 - tri_transformed.v1;
                let normal = l1.cross(&l2).normalize(); // You normally need to normalize a normal

                // Avoid divide by 0 and triangles before camera.
                if tri_transformed.v1.z <= 0.0 || tri_transformed.v2.z <= 0.0 || tri_transformed.v3.z <= 0.0 {
                    continue;
                }

                // Get ray from triangle to camera
                let v_camera_ray = tri_transformed.v1 - self.camera;

                // If ray is aligned with normal, make it visible.
                if normal.dot(v_camera_ray) < 0.0
                {

                    let light_direction = Vector3::new(0.0, 1.0, -1.0).normalize();
                    let dp = light_direction.dot(normal); // How "aligned" are light direction and triangle sureface normal?

                    let colour: u32 = Renderer3D::get_shading_color(dp);

                    let viewed_triangle = Triangle {
                        v1: Matrix4x4::multiply_vec(&mat_view, &tri_transformed.v1),
                        v2: Matrix4x4::multiply_vec(&mat_view, &tri_transformed.v2),
                        v3: Matrix4x4::multiply_vec(&mat_view, &tri_transformed.v3),
                        light_color: 0
                    };

                    let mut projected = Triangle {
                        v1: Matrix4x4::multiply_vec(&self.mat_proj, &viewed_triangle.v1),
                        v2: Matrix4x4::multiply_vec(&self.mat_proj, &viewed_triangle.v2),
                        v3: Matrix4x4::multiply_vec(&self.mat_proj, &viewed_triangle.v3),
                        light_color: colour
                    };


                    // Convertir a coordenadas de pantalla
                    for v in [&mut projected.v1, &mut projected.v2, &mut projected.v3] {
                        v.x = (v.x + 1.0) * 0.5 * self.engine.renderer.width() as f32;
                        v.y = (1.0 - v.y) * 0.5 * self.engine.renderer.height() as f32;
                    }

                    
                    triangles_to_raster.push(projected);

                }
            }

            //  Sort back to front
            triangles_to_raster.sort_by(|t1, t2| {
                let z1 = (t1.v1.z + t1.v2.z + t1.v3.z) / 3.0;
                let z2 = (t2.v1.z + t2.v2.z + t2.v3.z) / 3.0;
                z1.partial_cmp(&z2).unwrap_or(std::cmp::Ordering::Equal).reverse() // Back to front (descending order)
            });

            for projected in triangles_to_raster {
                self.engine.renderer.fill_triangle(
                    projected.v1.into(),
                    projected.v2.into(),
                    projected.v3.into(),
                    projected.light_color
                );
               
                self.engine.renderer.draw_triangle(
                    projected.v1.into(),
                    projected.v2.into(),
                    projected.v3.into(),
                    COLOUR::BLACK.to_u32()
                );
            }
        }

        self.engine.render(delta_time);
    }

    pub fn update(&mut self, delta_time: f32) {}
}
