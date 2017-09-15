
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{Sender, Receiver};

use specs::{ReadStorage, System, WriteStorage, Fetch};

use mesh::Object;
use mesh::cube::Cube;
use mesh::model::Model;
use camera::Camera;
use context::Command;
use components::*;

pub struct WallSystem;
pub struct ModelSystem;
pub struct CameraSystem;
pub struct RenderSystem;

impl<'a> System<'a> for WallSystem {
    type SystemData = (ReadStorage<'a, Camera>,
                       ReadStorage<'a, Position>,
                       WriteStorage<'a, Cube>);

    fn run(&mut self, (cam, poss, mut walls): Self::SystemData) {
        use cgmath::Matrix4;
        use specs::Join;

        let view = (&cam).join().next().unwrap().view();

        for (wall, p) in (&mut walls, &poss).join() {
            wall.transform(Matrix4::from_translation(p.into_vec()), view);
        }
    }
}

impl<'a> System<'a> for ModelSystem {
    type SystemData = (ReadStorage<'a, Camera>,
                       ReadStorage<'a, Position>,
                       WriteStorage<'a, Model>);

    fn run(&mut self, (cam, poss, mut models): Self::SystemData) {
        use cgmath::Matrix4;
        use specs::Join;

        let view = (&cam).join().next().unwrap().view();

        for (model, p) in (&mut models, &poss).join() {
            model.transform(Matrix4::from_translation(p.into_vec()), view);
        }
    }
}

impl<'a> System<'a> for CameraSystem {
    type SystemData = (Fetch<'a, Arc<Mutex<Receiver<Command>>>>,
                       WriteStorage<'a, Camera>);

    fn run(&mut self, (buf, mut cam): Self::SystemData) {
        use specs::Join;

        let cam = (&mut cam).join().next().unwrap();
        let channel = buf.lock().unwrap();
        for cmd in channel.try_iter() {
            match cmd {
                Command::Left(d) => cam.move_left(d),
                Command::Right(d) => cam.move_right(d),
                Command::Forward(d) => cam.move_forward(d),
                Command::Backward(d) => cam.move_back(d),
                Command::Rotate(x, y) => cam.rotate(x, y),
                _ => {}
            }
        }
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (Fetch<'a, Arc<Mutex<Sender<Command>>>>,
                       ReadStorage<'a, Cube>,
                       ReadStorage<'a, Model>);

    fn run(&mut self, (buf, walls, models): Self::SystemData) {
        use specs::Join;

        let channel = buf.lock().unwrap();

        for wall in walls.join() {
            channel.send(Command::Render(wall.mesh().clone())).unwrap();
        }
        for model in models.join() {
            channel.send(Command::Render(model.mesh().clone())).unwrap();
        }
    }
}

pub struct MovementSystem;
impl<'a> System<'a> for MovementSystem {
    type SystemData = (WriteStorage<'a, Position>,
                       ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut poss, vel): Self::SystemData) {
        use specs::Join;

        for (p, v) in (&mut poss, &vel).join() {
            p.x += v.x; p.y += v.y; p.z += v.z;
        }
    }
}
