
#[macro_use]
extern crate gfx;
extern crate gfx_core;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;
extern crate obj;
extern crate specs;
#[macro_use]
extern crate specs_derive;

extern crate genmesh;
extern crate cgmath;

use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;

use glutin::{WindowEvent, VirtualKeyCode};
use specs::{World, DispatcherBuilder};

mod defines;
mod context;
mod texture;
mod mesh;
mod camera;
mod components;

use camera::Camera;
use context::Context;
use mesh::cube::Cube;
use mesh::model::Model;
use components::*;

pub fn main() {
    let mut context = Context::new();
    let mut world = World::new();
    let (tx, rx) = channel();

    world.register::<Cube>();
    world.register::<Model>();

    let cam = Camera::new([1.0f32, 0.0, 2.0], [0.0, 0.0, -1.0]);

    world.create_entity().with(Cube::new(&mut context, &cam, [0.0, 0.0, 0.0])).build();
    world.create_entity().with(Cube::new(&mut context, &cam, [3.0, 0.0, 0.0])).build();
    world.create_entity().with(Model::new(&mut context, &cam, ".\\assets\\objs\\sphere.obj")).build();

    world.add_resource(cam);
    world.add_resource(Arc::new(Mutex::new(tx)));

    let mut dispatcher = DispatcherBuilder::new()
        .add(WallSystem, "walls", &[])
        .add(RenderSystem, "render", &["walls"])
        .build();

    let window_size = context.get_viewport_size().unwrap();
    let mut obj_x: f32 = 0.0;
    let mut obj_y: f32 = 0.0;
    let mut running = true;
    while running {
        {
            let mut cam = world.write_resource::<Camera>();

            context.handle_event(|e, delta| {
                match e {
                    WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::W), _) => cam.move_forward(delta),
                    WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::S), _) => cam.move_back(delta),
                    WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::A), _) => cam.move_left(delta),
                    WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::D), _) => cam.move_right(delta),

                    WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::I), _) => obj_x -= 1.0,
                    WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::K), _) => obj_x += 1.0,
                    WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::J), _) => obj_y -= 1.0,
                    WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::L), _) => obj_y += 1.0,

                    WindowEvent::MouseMoved(mouse_y, mouse_x) => {
                        let x = ((window_size.0 as f32 / 2.0) - mouse_x as f32) / 100.0;
                        let y = ((window_size.1 as f32 / 2.0) - mouse_y as f32) / 100.0;

                        cam.rotate(x, y);
                    },
                    WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::Escape), _) | WindowEvent::Closed => running = false,
                    _ => {}
                }
            });
        }

        context.clear();

        dispatcher.dispatch(&mut world.res);

        for command in rx.try_iter() {
            context.handle(command);
        }

        context.flush();
    }
}
