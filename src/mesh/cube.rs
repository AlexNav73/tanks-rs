
use std::sync::Arc;
use std::sync::RwLock;

use genmesh;

use genmesh::generators::{IndexedPolygon, SharedVertex};
use genmesh::{Triangulate, Vertices};
use cgmath::{Matrix4, SquareMatrix};
use specs::VecStorage;

use mesh::{Object, Mesh};
use defines::Vertex;
use context::Context;

#[derive(Component)]
#[component(VecStorage)]
pub struct Cube {
    mesh: Arc<RwLock<Mesh>>
}

impl Cube {
    pub fn new(context: &mut Context, view: Matrix4<f32>, offset: [f32; 3]) -> Self {
        let cube = genmesh::generators::Cube::new();
        let vertex_data: Vec<Vertex> = cube.shared_vertex_iter()
            .map(|v| Vertex::new([v.pos[0] + offset[0], v.pos[1] + offset[1], v.pos[2] + offset[1]], [0, 0]))
            .collect();
        let index_data: Vec<u32> = cube.indexed_polygon_iter()
            .triangulate()
            .vertices()
            .map(|i| i as u32)
            .collect();

        let texels = [0x20, 0xA0, 0xC0, 0x00];
        let position = Matrix4::identity().into();

        let mesh = context.create_mesh(position, view, &texels, &vertex_data, &index_data); 
        Cube {
            mesh: Arc::new(RwLock::new(mesh))
        }
    }
}

impl Object for Cube {
    fn mesh(&self) -> Arc<RwLock<Mesh>> { self.mesh.clone() }
}

