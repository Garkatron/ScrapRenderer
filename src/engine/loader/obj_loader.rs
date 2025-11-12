use std::{fs::File, io::Read};

use crate::engine::{
    rendering::mesh::Mesh,
    types::{object3d::Object3D, triangle::Triangle, vector::Vec3},
};

pub struct ObjLoader;

impl ObjLoader {
    pub fn from_file(path: &str) -> Result<Mesh, std::io::Error> {
        let mut mesh = Mesh {
            obj: Object3D::zero(),
            tris: vec![],
        };
        let mut file = File::open(path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Cache de vértices
        let mut verts: Vec<Vec3> = vec![];

        for line in contents.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let mut parts = line.split_whitespace();

            if line.starts_with("v ") {
                if let (Some(x), Some(y), Some(z)) = (parts.next(), parts.next(), parts.next()) {
                    if let (Ok(x), Ok(y), Ok(z)) =
                        (x.parse::<f32>(), y.parse::<f32>(), z.parse::<f32>())
                    {
                        verts.push(Vec3::new(x, y, z));
                    }
                } else {
                    eprintln!("Error parsing vertex: {}", line);
                }
            } else if line.starts_with("f ") {
                let indices: Vec<usize> = parts
                    .filter_map(|s| s.split('/').next()?.parse::<usize>().ok())
                    .map(|i| i - 1) // OBJ indices are 1-based
                    .collect();

                if indices.len() == 3 {
                    mesh.tris.push(Triangle::new(
                        verts[indices[0]],
                        verts[indices[1]],
                        verts[indices[2]],
                    ));
                } else if indices.len() == 4 {
                    mesh.tris.push(Triangle::new(
                        verts[indices[0]],
                        verts[indices[1]],
                        verts[indices[2]],
                    ));
                    mesh.tris.push(Triangle::new(
                        verts[indices[0]],
                        verts[indices[2]],
                        verts[indices[3]],
                    ));
                }
            }
        }

        Ok(mesh)
    }
}
