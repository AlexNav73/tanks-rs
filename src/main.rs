
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
    let mut cam = camera::Camera::new([1.0f32, 0.0, 2.0], [0.0, 0.0, -1.0]); 

    let mut cube = mesh::cube::Cube::new(&mut context, &cam, [0.0, 0.0, 0.0]);
    let mut cube2 = mesh::cube::Cube::new(&mut context, &cam, [3.0, 0.0, 0.0]);
    let mut object = mesh::model::Model::new(&mut context, &cam, "C:\\Users\\Aliaksandr\\Desktop\\Models\\sphere.obj");

    let window_size = context.get_viewport_size().unwrap();
    let mut obj_x: f32 = 0.0;
    let mut obj_y: f32 = 0.0;
    while context.is_running() {
        context.handle_event(|e| 
            match e {
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::W), _) => cam.move_forward(),
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::S), _) => cam.move_back(),
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::A), _) => cam.move_left(),
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::D), _) => cam.move_right(),

                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::I), _) => obj_x -= 1.0,
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::K), _) => obj_x += 1.0,
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::J), _) => obj_y -= 1.0,
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::L), _) => obj_y += 1.0,

                WindowEvent::MouseMoved(mouse_y, mouse_x) => {
                    let x = (mouse_x as f32 - (window_size.0 as f32 / 2.0)) / 100.0;
                    let y = (mouse_y as f32 - (window_size.1 as f32 / 2.0)) / 100.0;

                    cam.rotate(x, -y);
                },
                _ => {}
            }
        );

        let view = cam.view();

        use cgmath::{Matrix4, Vector3, Point3, SquareMatrix};
        let matrix = Matrix4::identity();

        let matrix2 = Matrix4::look_at(
            Point3::new(obj_x, obj_y, 3.0),
            Point3::new(0.0f32, 0.0, 0.0),
            Vector3::unit_y()
        );

        cube.transform(&context, matrix2, view);
        cube2.transform(&context, matrix, view);
        object.transform(&context, matrix, view);
        
        context.clear();

        context.render(&cube);
        context.render(&cube2);
        context.render(&object);

        context.flush();
    }
}
