
use genmesh;

use genmesh::generators::{IndexedPolygon, SharedVertex};
use genmesh::{Triangulate, Vertices};

use mesh::{Object, Mesh, default_view};
use defines::Vertex;
use context::Context;

pub struct Cube {
    mesh: Mesh
}

impl Cube {
    pub fn new(context: &mut Context, offset: [f32; 3]) -> Self {
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
        Cube {
            mesh: context.create_mesh(default_view(), &texels, &vertex_data, &index_data)
        }
    }
}

impl Object for Cube {
    fn mesh(&self) -> &Mesh { &self.mesh }
    fn mesh_mut(&mut self) -> &mut Mesh { &mut self.mesh }
}

