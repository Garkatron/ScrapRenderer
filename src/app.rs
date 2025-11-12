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
        texture_poll::TexturePool,
    },
    types::{
        object3d::Object3D,
        triangle::Triangle,
        vector::{Mat4, Vec3, Vec4},
    },
};

pub struct MyApp {
    pub window: Rc<RefCell<minifb::Window>>,
    pub engine: Engine3D,
    //pub kbcontroller: KeyboardController<'a>,
    pub objects: Vec<Mesh>,
    pub camera: Camera3D,
    pub f_theta: f32,
    pub mat_proj: Mat4,
}

impl MyApp {
    pub fn new(width: usize, height: usize, window: minifb::Window) -> Self {
        let w = Rc::new(RefCell::new(window));
        let mut objects = vec![];

        /*let mut obj = ObjLoader::from_file("/home/bazzite/Documentos/models/teapot.obj").unwrap();
        obj.obj.position.z += 10.0;
        obj.obj.rotation.y += 0.0;
        obj.obj.rotation.x += 0.0;
        objects.push(obj);*/

        objects.push(Mesh {
            obj: Object3D::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0)),
            tris: vec![
                // SOUTH
                Triangle::new(
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.0, 1.0, 0.0),
                    Vec3::new(1.0, 1.0, 0.0),
                )
                .set_uv([
                    Vec3::new(0.0, 1.0, 1.0),
                    Vec3::new(0.0, 0.0, 1.0),
                    Vec3::new(1.0, 0.0, 1.0),
                ]),
                Triangle::new(
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(1.0, 1.0, 0.0),
                    Vec3::new(1.0, 0.0, 0.0),
                )
                .set_uv([
                    Vec3::new(0.0, 1.0, 1.0),
                    Vec3::new(1.0, 0.0, 1.0),
                    Vec3::new(1.0, 1.0, 1.0),
                ]),
                // EAST
                Triangle::new(
                    Vec3::new(1.0, 0.0, 0.0),
                    Vec3::new(1.0, 1.0, 0.0),
                    Vec3::new(1.0, 1.0, 1.0),
                )
                .set_uv([
                    Vec3::new(0.0, 1.0, 1.0),
                    Vec3::new(0.0, 0.0, 1.0),
                    Vec3::new(1.0, 0.0, 1.0),
                ]),
                Triangle::new(
                    Vec3::new(1.0, 0.0, 0.0),
                    Vec3::new(1.0, 1.0, 1.0),
                    Vec3::new(1.0, 0.0, 1.0),
                )
                .set_uv([
                    Vec3::new(0.0, 1.0, 1.0),
                    Vec3::new(1.0, 0.0, 1.0),
                    Vec3::new(1.0, 1.0, 1.0),
                ]),
                // NORTH
                Triangle::new(
                    Vec3::new(1.0, 0.0, 1.0),
                    Vec3::new(1.0, 1.0, 1.0),
                    Vec3::new(0.0, 1.0, 1.0),
                )
                .set_uv([
                    Vec3::new(0.0, 1.0, 1.0),
                    Vec3::new(0.0, 0.0, 1.0),
                    Vec3::new(1.0, 0.0, 1.0),
                ]),
                Triangle::new(
                    Vec3::new(1.0, 0.0, 1.0),
                    Vec3::new(0.0, 1.0, 1.0),
                    Vec3::new(0.0, 0.0, 1.0),
                )
                .set_uv([
                    Vec3::new(0.0, 1.0, 1.0),
                    Vec3::new(1.0, 0.0, 1.0),
                    Vec3::new(1.0, 1.0, 1.0),
                ]),
                // WEST
                Triangle::new(
                    Vec3::new(0.0, 0.0, 1.0),
                    Vec3::new(0.0, 1.0, 1.0),
                    Vec3::new(0.0, 1.0, 0.0),
                )
                .set_uv([
                    Vec3::new(0.0, 1.0, 1.0),
                    Vec3::new(0.0, 0.0, 1.0),
                    Vec3::new(1.0, 0.0, 1.0),
                ]),
                Triangle::new(
                    Vec3::new(0.0, 0.0, 1.0),
                    Vec3::new(0.0, 1.0, 0.0),
                    Vec3::new(0.0, 0.0, 0.0),
                )
                .set_uv([
                    Vec3::new(0.0, 1.0, 1.0),
                    Vec3::new(1.0, 0.0, 1.0),
                    Vec3::new(1.0, 1.0, 1.0),
                ]),
                // TOP
                Triangle::new(
                    Vec3::new(0.0, 1.0, 0.0),
                    Vec3::new(0.0, 1.0, 1.0),
                    Vec3::new(1.0, 1.0, 1.0),
                )
                .set_uv([
                    Vec3::new(0.0, 1.0, 1.0),
                    Vec3::new(0.0, 0.0, 1.0),
                    Vec3::new(1.0, 0.0, 1.0),
                ]),
                Triangle::new(
                    Vec3::new(0.0, 1.0, 0.0),
                    Vec3::new(1.0, 1.0, 1.0),
                    Vec3::new(1.0, 1.0, 0.0),
                )
                .set_uv([
                    Vec3::new(0.0, 1.0, 1.0),
                    Vec3::new(1.0, 0.0, 1.0),
                    Vec3::new(1.0, 1.0, 1.0),
                ]),
                // BOTTOM
                Triangle::new(
                    Vec3::new(1.0, 0.0, 1.0),
                    Vec3::new(0.0, 0.0, 1.0),
                    Vec3::new(0.0, 0.0, 0.0),
                )
                .set_uv([
                    Vec3::new(0.0, 1.0, 1.0),
                    Vec3::new(0.0, 0.0, 1.0),
                    Vec3::new(1.0, 0.0, 1.0),
                ]),
                Triangle::new(
                    Vec3::new(1.0, 0.0, 1.0),
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(1.0, 0.0, 0.0),
                )
                .set_uv([
                    Vec3::new(0.0, 1.0, 1.0),
                    Vec3::new(1.0, 0.0, 1.0),
                    Vec3::new(1.0, 1.0, 1.0),
                ]),
            ],
        });

        // let mat_proj = Mat4::project(0.1, 1000.0, 90.0, height, width);
        let aspect = width as f32 / height as f32;
        let fovy_radians = 90.0f32.to_radians();
        let znear = 0.1f32;
        let zfar = 1000.0f32;

        let mat_proj = Mat4::new_perspective(aspect, fovy_radians, znear, zfar);

        let mut texture_poll = TexturePool::new();
        //texture_poll.reg_from_path("test", "/home/deus/Documents/textures/test/facebooklogo.png");

        // dbg!(texture_poll.get("test").expect("msg").get_pixel_as_u32(0, 0, false).unwrap());

        Self {
            window: w.clone(),
            engine: Engine3D {
                running: true,
                renderer: Renderer3D::new(vec![0; width * height], width, height, w.clone()),
                kbcontroller: KeyboardController::new(w.clone()),
                texture_poll,
            },
            //kbcontroller: KeyboardController::new(&window)
            objects,
            camera: Camera3D::new(Vec3::new(0.0, 0.0, 0.0), width, height),
            //camera: Camera3D::new(Vector3::new(0.0, 0.1, 5.0), width, height),
            f_theta: 0.0,
            mat_proj,
        }
    }
    fn transform_vertex(mat: &Mat4, v: Vec3) -> Vec3 {
        let v_hom = mat * Vec4::new(v.x, v.y, v.z, 1.0); // convertir a Vector4 homogéneo
        if v_hom.w != 0.0 {
            Vec3::new(v_hom.x / v_hom.w, v_hom.y / v_hom.w, v_hom.z / v_hom.w)
        } else {
            Vec3::new(v_hom.x, v_hom.y, v_hom.z)
        }
    }
    fn project_vertex(mat: &Mat4, v: Vec3) -> Vec3 {
        let v_hom = mat * v.to_homogeneous(); // Vector3 -> Vector4 (w=1)
        Vec3::new(v_hom.x / v_hom.w, v_hom.y / v_hom.w, v_hom.z / v_hom.w)
    }
    pub fn render(&mut self, delta_time: f32) {
        //println!("FPS: {:.2}", 1.0 / delta_time);
        self.engine.renderer.clear(0);

        self.f_theta += 1.0 * delta_time;
        // self.objects.get_mut(0).unwrap().obj.rotation.y = self.f_theta;

        for Mesh { obj, tris } in &self.objects {
            let mut triangles_to_raster: Vec<Triangle> = vec![];

            // 1. Rotar triángulo (usando matriz de rotación de objeto)
            let rotation_matrix = obj.rotation_matrix();

            // 2. Translate
            let transform_matrix = obj.transform_matrix();

            // 3. World Matrix
            let world_matrix = rotation_matrix * transform_matrix;

            // 4. Camera
            let camera_matrix = self.camera.calc_view();

            println!("=== Frame Debug ===");
            println!("Camera position: {:?}", self.camera.position);
            println!("Camera yaw: {}", self.camera.f_yaw);
            println!("Total triangles in mesh: {}", tris.len());

            for (i, tri) in tris.iter().enumerate() {
                let tri_world = Triangle::new(
                    MyApp::transform_vertex(&world_matrix, tri.v1),
                    MyApp::transform_vertex(&world_matrix, tri.v2),
                    MyApp::transform_vertex(&world_matrix, tri.v3),
                );

                // 2. Calcular normal en espacio world
                let l1 = tri_world.v2 - tri_world.v1;
                let l2 = tri_world.v3 - tri_world.v1;
                let normal = l1.cross(&l2).normalize();

                // let normal = l1.cross(l2).to_vector3().normalize(); // You normally need to normalize a normal
                let v_camera_ray = tri_world.v1 - self.camera.position;
                let dot_prod = normal.dot(&v_camera_ray);

                if i == 0 {
                    // Solo para el primer triángulo
                    println!("Triangle 0:");
                    println!("  World pos v1: {:?}", tri_world.v1);
                    println!("  Normal: {:?}", normal);
                    println!("  Camera ray: {:?}", v_camera_ray);
                    println!("  Dot product: {}", dot_prod);
                }

                // 4. Calcular iluminación (en espacio world)
                let light_direction = Vec3::new(0.0, 1.0, -1.0).normalize();
                let dp = light_direction.dot(&normal);
                let colour: u32 = Renderer3D::get_shading_color(dp, &PalettePink);

                // 5. Transformar a espacio de vista (cámara)
                let tri_viewed = Triangle::new(
                    MyApp::transform_vertex(&camera_matrix, tri_world.v1),
                    MyApp::transform_vertex(&camera_matrix, tri_world.v2),
                    MyApp::transform_vertex(&camera_matrix, tri_world.v3),
                )
                .set_light_color(colour);

                // Clip viewed triangle againts near plane, this could form two aditional triangles.

                let clipped = Renderer3D::triangle_clip_against_plane(
                    Vec4::new(0.0, 0.0, 0.1, 1.0),
                    Vec4::new(0.0, 0.0, 1.0, 1.0),
                    &tri_viewed,
                );
                if i == 0 {
                    println!("  Clipped result: {} triangles", clipped.len());
                }

                for tc in clipped {
                    let mut projected = Triangle::new(
                        MyApp::project_vertex(&self.mat_proj, tc.v1),
                        MyApp::project_vertex(&self.mat_proj, tc.v2),
                        MyApp::project_vertex(&self.mat_proj, tc.v3),
                    )
                    .set_light_color(tc.light_color);

                    // 8. Convertir a coordenadas de pantalla
                    for v in [&mut projected.v1, &mut projected.v2, &mut projected.v3] {
                        v.x = (v.x + 1.0) * 0.5 * self.engine.renderer.width() as f32;
                        v.y = (1.0 - v.y) * 0.5 * self.engine.renderer.height() as f32;
                    }

                    triangles_to_raster.push(projected);
                }
            }

            println!("Triangles after clipping: {}", triangles_to_raster.len());

            //  Sort back to front
            triangles_to_raster.sort_by(|t1, t2| {
                let z1 = (t1.v1.z + t1.v2.z + t1.v3.z) / 3.0;
                let z2 = (t2.v1.z + t2.v2.z + t2.v3.z) / 3.0;
                z2.partial_cmp(&z1).unwrap_or(std::cmp::Ordering::Equal)
            });

            self.engine.renderer.depth_buffer.fill(0.0);

            // Loop through all transformed, viewed, projected, and sorted triangles
            for tri_to_raster in triangles_to_raster {
                let mut tri_queue: Vec<Triangle> = vec![tri_to_raster];

                for edge in 0..4 {
                    let mut new_triangles: Vec<Triangle> = vec![];

                    for test in tri_queue.drain(..) {
                        let clipped = match edge {
                            0 => Renderer3D::triangle_clip_against_plane(
                                Vec4::new(0.0, 0.0, 0.0, 1.0),
                                Vec4::new(0.0, 1.0, 0.0, 1.0),
                                &test,
                            ),
                            1 => Renderer3D::triangle_clip_against_plane(
                                Vec4::new(
                                    0.0,
                                    (self.engine.renderer.height() as f32) - 1.0,
                                    0.0,
                                    1.0,
                                ),
                                Vec4::new(0.0, -1.0, 0.0, 1.0),
                                &test,
                            ),
                            2 => Renderer3D::triangle_clip_against_plane(
                                Vec4::new(0.0, 0.0, 0.0, 1.0),
                                Vec4::new(1.0, 0.0, 0.0, 1.0),
                                &test,
                            ),
                            3 => Renderer3D::triangle_clip_against_plane(
                                Vec4::new(
                                    (self.engine.renderer.width() as f32) - 1.0,
                                    0.0,
                                    0.0,
                                    1.0,
                                ),
                                Vec4::new(-1.0, 0.0, 0.0, 1.0),
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
                        t.v1.xy().map(|x| x as i32),
                        t.v2.xy().map(|x| x as i32),
                        t.v3.xy().map(|x| x as i32),
                        t.light_color,
                    );

                    // self.engine.renderer.textured_triangle(t.v1.into(), t.uv[0], t.v2.into(), t.uv[1], t.v3.into(), t.uv[2], self.engine.texture_poll.get_or_panic("test"));

                    self.engine.renderer.draw_triangle(
                        t.v1.xy().map(|x| x as i32),
                        t.v2.xy().map(|x| x as i32),
                        t.v3.xy().map(|x| x as i32),
                        PALETTE_DEFAULT::RED.to_u32(),
                    );
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
                self.camera.update_look_dir();
            }

            if self.engine.kbcontroller.is_key_down(Key::Left) {
                self.camera.f_yaw -= 2.0 * delta_time;
                self.camera.update_look_dir();
            }
        } else {
            self.engine.running = false;
        }
    }
}
