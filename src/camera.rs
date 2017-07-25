
use cgmath::{Point3, InnerSpace, Quaternion, Vector3, Decomposed, Rotation, EuclideanSpace, Matrix4, Transform as Transform_, Rotation3, Rad, ElementWise};

type Transform = Decomposed<Vector3<f32>, Quaternion<f32>>;

const CAMERA_SPEED: f32 = 0.05;

pub struct Camera {
   transform: Transform,
   target: Vector3<f32>,
   rot: (f32, f32),
   pos: (f32, f32),

   front_dir: Vector3<f32>,
   up_dir: Vector3<f32>
}

impl Camera {
    pub fn new<P>(position: P, target: P) -> Self 
        where P: Into<Point3<f32>>
    {
        let position = position.into();
        let target = target.into();

        let dir = (target - position).normalize();
        let q = Quaternion::look_at(dir, Vector3::unit_y()).invert(); // Made Y axis as axis of the view (from screen to object)
        
        Camera {
           target: target.to_vec(),
           transform: Decomposed {
               disp: position.to_vec(),
               rot: q,
               scale: 1.0
           },
           rot: (0.0f32, 0.0f32),
           pos: (0.0f32, 0.0f32),

           front_dir: Vector3::unit_z(),
           up_dir: Vector3::unit_y()
        }
    }

    pub fn update(&mut self, x: f32, y: f32) -> Matrix4<f32> {
        let pre = Decomposed {
            disp: -self.transform.disp,
            .. Decomposed::one()
        };

        let q_hor = Quaternion::from_angle_y(Rad(self.rot.0 - x)); // Rotate around Z axis
        let axis = self.transform.rot * Vector3::unit_x(); // Rotate normalized X vec to new coordinate system with new rotation
        let q_ver = Quaternion::from_axis_angle(axis, Rad(self.rot.1 - y)); // Rotate around rotated X axis 

        let post = Decomposed {
            rot: q_hor * q_ver,
            disp: self.transform.disp,
            scale: 1.0,
        };

        self.rot.0 = x;
        self.rot.1 = y;
        self.transform = post.concat(&pre.concat(&self.transform));

        self.transform.into()
    }

    pub fn move_forward(&mut self) {
        self.transform.disp += self.front_dir.mul_element_wise(CAMERA_SPEED);
    }

    pub fn move_backward(&mut self) {
        self.transform.disp -= self.front_dir.mul_element_wise(CAMERA_SPEED);
    }

    pub fn move_left(&mut self) {
        let face_dir = (self.target - self.transform.disp).normalize();
        let right_dir = face_dir.cross(self.up_dir);

        self.transform.disp += right_dir.mul_element_wise(CAMERA_SPEED);
    }

    pub fn move_right(&mut self) {
        let face_dir = (self.target - self.transform.disp).normalize();
        let right_dir = face_dir.cross(self.up_dir);

        self.transform.disp -= right_dir.mul_element_wise(CAMERA_SPEED);
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        self.transform.disp[0] += self.pos.0 - x;
        self.transform.disp[2] += self.pos.1 - y;
        self.pos.0 = x;
        self.pos.1 = y;
    }

    pub fn print(&self) {
        // println!("face: {} {} {}", self.front_dir[0], self.front_dir[1], self.front_dir[2]);
        // println!("up: {} {} {}", self.up_dir[0], self.up_dir[1], self.up_dir[2]);
        // println!("right: {} {} {}", self.right_dir[0], self.right_dir[1], self.right_dir[2]);

        println!("position: {} {} {}", self.transform.disp[0], self.transform.disp[1], self.transform.disp[2]);
    }
}
