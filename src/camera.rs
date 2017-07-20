
use cgmath::{Point3, InnerSpace, Quaternion, Vector3, Decomposed, Rotation, EuclideanSpace, Matrix4, Transform as Transform_, Rotation3, Rad};

type Transform = Decomposed<Vector3<f32>, Quaternion<f32>>;

pub struct Camera {
   transform: Transform,
   target: Point3<f32>,
   x: f32,
   y: f32
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
           target,
           transform: Decomposed {
               disp: position.to_vec(),
               rot: q,
               scale: 1.0
           },
           x: 0.0f32, y: 0.0f32
        }
    }

    pub fn update(&mut self, x: f32, y: f32) -> Matrix4<f32> {
        let pre = Decomposed {
            disp: -self.target.to_vec(),
            .. Decomposed::one()
        };

        let q_hor = Quaternion::from_angle_y(Rad(self.x - x)); // Rotate around Z axis
        let axis = self.transform.rot * Vector3::unit_x(); // Rotate normalized X vec to new coordinate system with new rotation
        let q_ver = Quaternion::from_axis_angle(axis, Rad(self.y - y)); // Rotate around rotated X axis 

        let post = Decomposed {
            rot: q_hor * q_ver,
            disp: self.target.to_vec(),
            scale: 1.0,
        };

        self.x = x;
        self.y = y;
        self.transform = post.concat(&pre.concat(&self.transform));

        self.transform.into()
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        self.target.x = x;
        self.target.z = y;
        self.transform.disp = self.target.to_vec();
    }
}
