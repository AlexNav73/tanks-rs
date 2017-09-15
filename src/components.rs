
use cgmath::Vector3;
use specs::VecStorage;

#[derive(Debug, Component)]
#[component(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Position { x, y, z } 
    }

    pub fn into_vec(&self) -> Vector3<f32> {
        Vector3::new(self.x, self.y, self.z)
    }
}

#[derive(Component)]
#[component(VecStorage)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Velocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Velocity { x, y, z } 
    }
}

