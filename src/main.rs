
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

mod defines;
mod context;
mod texture;
mod mesh;
mod camera;

use context::Context;
use mesh::Object;

pub fn main() {
    let mut context = Context::new();

    let mut cube = mesh::cube::Cube::new(&mut context, [0.0, 0.0, 0.0]);
    let mut cube2 = mesh::cube::Cube::new(&mut context, [3.0, 0.0, 0.0]);
    //let mut object = mesh::model::Model::new(&mut context, "C:\\Users\\Aliaksandr\\Desktop\\Models\\sphere.obj");
    let mut cam = camera::Camera::new([4.0f32, 0.0, 0.0], [0.0, 0.0, 0.0]);

    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut pos_x: f32 = 1.7;
    let mut pos_y: f32 = -6.7;
    while context.is_running() {
        context.handle_event(|e| 
            match e {
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::A), _) => x += 0.1,
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::D), _) => x -= 0.1,
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::W), _) => y += 0.1,
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::S), _) => y -= 0.1,

                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::Up), _) => pos_y += 0.1,
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::Down), _) => pos_y -= 0.1,

                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::Left), _) => pos_x += 0.1,
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::Right), _) => pos_x -= 0.1,
                _ => {}
            }
        );

        cam.move_to(pos_x, pos_y);
        let matrix = cam.update(x, y);

        cube.transform(&context, matrix);
        cube2.transform(&context, matrix);
        //object.transform(&context, matrix);
        
        context.clear();

        context.render(&cube);
        context.render(&cube2);
        //context.render(&object);

        context.flush();
    }
}
