#![allow(dead_code)]

use std::sync::Arc;
use std::sync::RwLock;

use std::path::Path;
use std::fs::File;
use std::io::BufReader;

use obj::{self, load_obj};
use cgmath::{Matrix4, SquareMatrix};
use specs::VecStorage;

use mesh::{Object, Mesh};
use context::Context;
use defines::Vertex;
use camera::Camera;

#[derive(Component)]
#[component(VecStorage)]
pub struct Model {
    mesh: Arc<RwLock<Mesh>>
}

impl Model {
    pub fn new<P: AsRef<Path>>(context: &mut Context, cam: &Camera, path: P) -> Self {
        let file = BufReader::new(File::open(path).unwrap());
        let obj_data = load_obj(file).unwrap();
        let idxs = obj_data.indices.iter().map(|&i| i as u32).collect::<Vec<_>>();
        let vert = obj_data.vertices.iter()
            .map(|v: &obj::Vertex| Vertex::new([v.position[0], v.position[1], v.position[2]], [0, 0]))
            .collect::<Vec<Vertex>>();

        let texture = [0x20, 0xA0, 0xC0, 0x00];
        let position = Matrix4::identity().into();

        let mesh = context.create_mesh(position, cam.view(), &texture, &vert, idxs.as_slice());
        Model {
            mesh: Arc::new(RwLock::new(mesh))
        }
    }
}

impl Object for Model {
    fn mesh(&self) -> Arc<RwLock<Mesh>> { self.mesh.clone() }
}

