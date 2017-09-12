
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
mod systems;

use camera::Camera;
use context::{Context, Command};
use mesh::cube::Cube;
use mesh::model::Model;
use systems::*;

pub fn main() {
    let mut context = Context::new();
    let mut world = World::new();
    let (tx, rx) = channel();
    let (ty, ry) = channel();

    world.register::<Cube>();
    world.register::<Model>();
    world.register::<Camera>();

    let cam = Camera::new([1.0f32, 0.0, 2.0], [0.0, 0.0, -1.0]);
    world.create_entity().with(Cube::new(&mut context, &cam, [0.0, 0.0, 0.0])).build();
    world.create_entity().with(Cube::new(&mut context, &cam, [3.0, 0.0, 0.0])).build();
    world.create_entity().with(Model::new(&mut context, &cam, ".\\assets\\objs\\sphere.obj")).build();
    world.create_entity().with(cam).build();

    world.add_resource(Arc::new(Mutex::new(ry)));
    world.add_resource(Arc::new(Mutex::new(tx)));

    let mut dispatcher = DispatcherBuilder::new()
        .add(WallSystem, "walls", &[])
        .add(ModelSystem, "models", &[])
        .add(CameraSystem, "camera", &[])
        .add_thread_local(RenderSystem)
        .build();

    let window_size = context.get_viewport_size().unwrap();
    let mut running = true;
    while running {
        context.handle_event(|e, delta| {
            match e {
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::W), _) => ty.send(Command::Forward(delta)).unwrap(),
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::S), _) => ty.send(Command::Backward(delta)).unwrap(),
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::A), _) => ty.send(Command::Left(delta)).unwrap(),
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::D), _) => ty.send(Command::Right(delta)).unwrap(),

                WindowEvent::MouseMoved(mouse_y, mouse_x) => {
                    let x = ((window_size.0 as f32 / 2.0) - mouse_x as f32) / 100.0;
                    let y = ((window_size.1 as f32 / 2.0) - mouse_y as f32) / 100.0;

                    ty.send(Command::Rotate(x, y)).unwrap();
                },
                WindowEvent::KeyboardInput(_, _, Some(VirtualKeyCode::Escape), _) | WindowEvent::Closed => running = false,
                _ => {}
            }
        });

        context.clear();

        dispatcher.dispatch(&mut world.res);

        for command in rx.try_iter() {
            context.handle(command);
        }

        context.flush();
    }
}
