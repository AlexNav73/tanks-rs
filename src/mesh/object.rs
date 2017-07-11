
use std::path::Path;
use std::fs::File;
use std::io::BufReader;

use ::obj::*;
use ::obj::raw::object::Polygon;

use mesh::{Mesh, default_view};
use context::Context;
use defines::Vertex;

pub struct Object {
    mesh: Mesh
}

impl Object {
    pub fn new<P: AsRef<Path>>(context: &mut Context, path: P) -> Self {
        let file = BufReader::new(File::open(path).unwrap());
        let obj_data = load_obj(file).unwrap();
        let idxs = obj_data.indices.iter().map(|&i| i as u32).collect::<Vec<_>>();

        let texture = [0x20, 0xA0, 0xC0, 0x00];
        Object {
            mesh: context.create_mesh(default_view(), &texture, &obj_data.vertices, idxs.as_slice())
        }
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }
}

impl FromRawVertex for Vertex {
    fn process(vertices: Vec<(f32, f32, f32, f32)>,
               normals: Vec<(f32, f32, f32)>,
               polygons: Vec<Polygon>) -> ObjResult<(Vec<Self>, Vec<u16>)> {
        
        let vert = vertices.into_iter()
            .map(|v| Vertex::new([v.0, v.1, v.2], [0, 0]))
            .collect::<Vec<_>>();

        Ok((vert, Vec::new()))
    }
}

