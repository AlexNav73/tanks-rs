
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Sender;

use specs::{ReadStorage, System, WriteStorage, Fetch};

use mesh::Object;
use mesh::cube::Cube;
use mesh::model::Model;
use camera::Camera;
use context::Command;

pub struct WallSystem;
pub struct ModelSystem;
pub struct RenderSystem;

impl<'a> System<'a> for WallSystem {
    type SystemData = (Fetch<'a, Camera>,
                       WriteStorage<'a, Cube>);

    fn run(&mut self, (cam, mut walls): Self::SystemData) {
        use cgmath::{Matrix4, SquareMatrix};
        use specs::Join;

        let view = cam.view();
        let matrix = Matrix4::identity();

        for wall in (&mut walls).join() {
            wall.transform(matrix, view);
        }
    }
}

impl<'a> System<'a> for ModelSystem {
    type SystemData = (Fetch<'a, Camera>,
                       WriteStorage<'a, Model>);

    fn run(&mut self, (cam, mut models): Self::SystemData) {
        use cgmath::{Matrix4, SquareMatrix};
        use specs::Join;

        let view = cam.view();
        let matrix = Matrix4::identity();

        for model in (&mut models).join() {
            model.transform(matrix, view);
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
