
use cgmath::{Point3, Vector3, Matrix4, EuclideanSpace, InnerSpace, Rad};

const CAMERA_SPEED: f32 = 1.0;

pub struct Camera {
    position: Vector3<f32>,
    front: Vector3<f32>,
    up: Vector3<f32>,
    pitch: f32,
    yaw: f32,
    last: (f32, f32)
}

impl Camera {
    pub fn new<P>(position: P, target: P) -> Self 
        where P: Into<Point3<f32>>
    {
        Camera {
            position: position.into().to_vec(),
            front: target.into().to_vec(),
            up: Vector3::unit_y(),
            pitch: 0.0,
            yaw: 0.0,
            last: (0.0, 0.0)
        }
    }

    pub fn view(&self) -> Matrix4<f32> {
        let pos = Point3::new(self.position.x, self.position.y, self.position.z);
        let front = self.position + self.front;
        let front = Point3::new(front.x, front.y, front.z);

        Matrix4::look_at(pos, front, self.up)
    }

    pub fn rotate(&mut self, x: f32, y: f32) {
        use cgmath::{Zero, Angle};

        let pitch = x - self.last.0;
        let yaw = self.last.1 - y;

        self.last.0 = x;
        self.last.1 = y;

        self.pitch += pitch;
        self.yaw += yaw;

        let mut direction = Vector3::zero();
        direction.x = Rad(self.pitch).cos() * Rad(self.yaw).cos();
        direction.y = Rad(self.pitch).sin();
        direction.z = Rad(self.pitch).cos() * Rad(self.yaw).sin();

        self.front = direction.normalize();
    }

    pub fn move_forward(&mut self) {
        self.position += self.front * CAMERA_SPEED;
    }

    pub fn move_back(&mut self) {
        self.position -= self.front * CAMERA_SPEED;
    }

    pub fn move_left(&mut self) {
        self.position -= self.front.cross(self.up).normalize() * CAMERA_SPEED;
    }

    pub fn move_right(&mut self) {
        self.position += self.front.cross(self.up).normalize() * CAMERA_SPEED;
    }
}
