use vec::Vec3;

pub struct Camera {
    pub position: Vec3,
    pub direction: Vec3,
    pub up: Vec3,
}

impl Camera {

    pub fn new() -> Camera {
        Camera {
            position: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
            up: Vec3::new(0.0, 1.0, 0.0)
        }
    }

}
