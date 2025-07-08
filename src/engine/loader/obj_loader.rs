use std::{fs::File, io::Read};

use crate::engine::{rendering::mesh::Mesh, types::{object3d::Object3D, triangle::Triangle, vector::{vector3::Vector3, vector4::Vector4}}};

pub struct ObjLoader;

impl ObjLoader {
    pub fn from_file(path: &str) -> Result<Mesh, std::io::Error> {
        let mut mesh = Mesh { obj: Object3D::zero(), tris: vec![] };
        let mut file = File::open(path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Cache de vértices
        let mut verts: Vec<Vector3> = vec![];

        for line in contents.lines() {
            let data: Vec<&str> = line.split_whitespace().collect();

            match line.chars().nth(0) {
                Some('v') => {
                    if data.len() >= 4 {
                        verts.push(Vector3::new(
                            data[1].parse().unwrap_or(0.0),
                            data[2].parse().unwrap_or(0.0),
                            data[3].parse().unwrap_or(0.0),
                        ));
                    }
                }
                Some('f') => {
                    if data.len() >= 4 {
                        // Parsear los índices de la cara (ignorando texturas/normales)
                        let indices: Vec<usize> = data[1..]
                            .iter()
                            .map(|s| s.split('/').next().unwrap().parse::<usize>().unwrap_or(1) - 1)
                            .collect();

                        // Si es un triángulo (3 vértices)
                        if indices.len() == 3 {
                            mesh.tris.push(Triangle::new(
                                verts[indices[0]],
                                verts[indices[1]],
                                verts[indices[2]],
                            ));
                        }
                        // Si es un cuadrilátero (4 vértices), triangular
                        else if indices.len() == 4 {
                            // Triángulo 1: indices[0], indices[1], indices[2]
                            mesh.tris.push(Triangle::new(
                                verts[indices[0]],
                                verts[indices[1]],
                                verts[indices[2]],
                            ));
                            // Triángulo 2: indices[0], indices[2], indices[3]
                            mesh.tris.push(Triangle::new(
                                verts[indices[0]],
                                verts[indices[2]],
                                verts[indices[3]]
                            ));
                        }
                        // Ignorar polígonos con más de 4 vértices por ahora
                    }
                }
                _ => {}
            }
        }

        Ok(mesh)
    }
}