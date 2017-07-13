
#[macro_use]
extern crate gfx;
extern crate gfx_core;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;
extern crate obj;

extern crate genmesh;
extern crate cgmath;

use glutin::{WindowEvent, VirtualKeyCode};

use cgmath::{Matrix4, Point3, Vector3};

mod defines;
mod context;
mod texture;
mod mesh;

use context::Context;
use mesh::Object;

pub fn main() {
    let mut context = Context::new();

    let mut cube = mesh::cube::Cube::new(&mut context, [0.0, 0.0, 0.0]);
    let mut cube2 = mesh::cube::Cube::new(&mut context, [3.0, 0.0, 0.0]);
    let mut object = mesh::model::Model::new(&mut context, "C:\\Users\\Aliaksandr\\Desktop\\Models\\sphere.obj");

    let mut x: f32 = 0.0;
    let mut y: f32 = 3.0;
    while context.is_running() {
        context.handle_event(|e| 
            match e {
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::A), _) => x += 0.01,
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::D), _) => x -= 0.01,
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::W), _) => y += 0.01,
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::S), _) => y -= 0.01,
                _ => {}
            }
        );

        let matrix = Matrix4::look_at(
            Point3::new(x, y, 3.0),
            Point3::new(0.0f32, 0.0, 0.0),
            Vector3::unit_z()
        );

        cube.transform(&context, matrix);
        cube2.transform(&context, matrix);
        object.transform(&context, matrix);
        
        context.clear();

        context.render(&cube);
        context.render(&cube2);
        context.render(&object);

        context.flush();
    }
}
