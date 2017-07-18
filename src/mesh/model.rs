#![allow(dead_code)]

use std::path::Path;
use std::fs::File;
use std::io::BufReader;

use obj::{self, load_obj};
use cgmath::{Matrix4, SquareMatrix};

use mesh::{Object, Mesh};
use context::Context;
use defines::Vertex;

pub struct Model {
    mesh: Mesh
}

impl Model {
    pub fn new<P: AsRef<Path>>(context: &mut Context, path: P) -> Self {
        let file = BufReader::new(File::open(path).unwrap());
        let obj_data = load_obj(file).unwrap();
        let idxs = obj_data.indices.iter().map(|&i| i as u32).collect::<Vec<_>>();
        let vert = obj_data.vertices.iter()
            .map(|v: &obj::Vertex| Vertex::new([v.position[0], v.position[1], v.position[2]], [0, 0]))
            .collect::<Vec<Vertex>>();

        let texture = [0x20, 0xA0, 0xC0, 0x00];
        Model {
            mesh: context.create_mesh(Matrix4::identity().into(), &texture, &vert, idxs.as_slice())
        }
    }
}

impl Object for Model {
    fn mesh(&self) -> &Mesh { &self.mesh }
    fn mesh_mut(&mut self) -> &mut Mesh { &mut self.mesh }
}

