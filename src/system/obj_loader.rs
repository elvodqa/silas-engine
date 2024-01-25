use std::{self, fs::File, io::{BufRead, BufReader}};



pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
    normal: [f32; 3],
}

#[derive(Default)]
pub struct ObjLoader {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Default for Vertex {
    fn default() -> Vertex {
        Vertex {
            position: [0.0; 3],
            tex_coords: [0.0; 2],
            normal: [0.0; 3],
        }
    }
}

impl ObjLoader {
    pub fn new(path: &str) -> ObjLoader {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let file: File = File::open(path).unwrap();
        let reader: BufReader<File> = BufReader::new(file);

        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut tex_coords: Vec<[f32; 2]> = Vec::new();
        let mut normals: Vec<[f32; 3]> = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            let mut words = line.split_whitespace();

            match words.next() {
                Some("v") => {
                    let x: f32 = words.next().unwrap().parse().unwrap();
                    let y: f32 = words.next().unwrap().parse().unwrap();
                    let z: f32 = words.next().unwrap().parse().unwrap();
                    positions.push([x, y, z]);
                }
                Some("vt") => {
                    let u: f32 = words.next().unwrap().parse().unwrap();
                    let v: f32 = words.next().unwrap().parse().unwrap();
                    tex_coords.push([u, v]);
                }
                Some("vn") => {
                    let x: f32 = words.next().unwrap().parse().unwrap();
                    let y: f32 = words.next().unwrap().parse().unwrap();
                    let z: f32 = words.next().unwrap().parse().unwrap();
                    normals.push([x, y, z]);
                }
                Some("f") => {
                    let mut vertex_indices: Vec<u32> = Vec::new();
                    let mut tex_coord_indices: Vec<u32> = Vec::new();
                    let mut normal_indices: Vec<u32> = Vec::new();

                    for word in words {
                        let mut indices = word.split('/');
                        let vertex_index: u32 = indices.next().unwrap().parse().unwrap();
                        let tex_coord_index: u32 = indices.next().unwrap().parse().unwrap();
                        let normal_index: u32 = indices.next().unwrap().parse().unwrap();

                        vertex_indices.push(vertex_index);
                        tex_coord_indices.push(tex_coord_index);
                        normal_indices.push(normal_index);
                    }

                    for i in 0..3 {
                        let vertex_index = vertex_indices[i] as usize;
                        let tex_coord_index = tex_coord_indices[i] as usize;
                        let normal_index = normal_indices[i] as usize;

                        let position = positions[vertex_index - 1];
                        let tex_coords = tex_coords[tex_coord_index - 1];
                        let normal = normals[normal_index - 1];

                        vertices.push(Vertex {
                            position,
                            tex_coords,
                            normal,
                        });
                        indices.push(i as u32);
                    }
                }

                _ => {}
            }
        }

        ObjLoader { vertices, indices }
    }
}
