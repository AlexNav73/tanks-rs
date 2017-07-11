
#[macro_use]
extern crate gfx;
extern crate gfx_core;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;

extern crate genmesh;
extern crate cgmath;

use gfx::traits::FactoryExt;
use glutin::{WindowEvent, VirtualKeyCode};

use genmesh::generators::{Cube, IndexedPolygon, SharedVertex};
use genmesh::{Triangulate, Vertices};

use cgmath::{Matrix4, Deg, Point3, Vector3};

mod defines;
mod context;

use defines::{Vertex, Locals};
use context::Context;

pub fn main() {

    let mut context = Context::new();

    let cube = Cube::new();
    let vertex_data: Vec<Vertex> = cube.shared_vertex_iter()
        .map(|v| Vertex::new([v.pos[0], v.pos[1], v.pos[2]], [0, 0]))
        .collect();
    let index_data: Vec<u32> = cube.indexed_polygon_iter()
        .triangulate()
        .vertices()
        .map(|i| i as u32)
        .collect();

    let (vertex_buffer, slice) = context.factory.create_vertex_buffer_with_slice(&vertex_data, index_data.as_slice());

    let mut running = true;
    let mut x: f32 = 0.0;
    let mut y: f32 = 3.0;
    while running {
        context.handle_event(|e| 
            match e {
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::Escape), _) |
                WindowEvent::Closed => running = false,
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::A), _) => {
                    x += 0.01;
                },
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::D), _) => {
                    x -= 0.01;
                },
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::W), _) => {
                    y += 0.01;
                },
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::S), _) => {
                    y -= 0.01;
                },
                _ => {}
            }
        );

        let matrix = Matrix4::look_at(
            Point3::new(x, y, 3.0),
            Point3::new(0.0f32, 0.0, 0.0),
            Vector3::unit_z()
        );
        
        let proj = cgmath::perspective(Deg(45.0f32), 1.333, 1.0, 10.0);
        let locals = Locals {
            transform: (proj * matrix).into()
        };

        context.clear();
        context.render_mesh(&locals, &vertex_buffer, &slice);
        context.flush();
    }
}
