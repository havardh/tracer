#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            x: x, y: y, z: z
        }
    }

    pub fn length(&self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    pub fn add(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn sub(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn multiply(&self, f: f32) -> Vec3 {
        Vec3::new(self.x * f, self.y * f, self.z * f)
    }

    pub fn cross(&self, o: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * o.z - self.z * o.y,
            -(self.x * o.z - self.z * o.x),
            self.x * o.y - self.y * o.x
        )
    }

    pub fn norm(&mut self) {
        let len = self.length();

        if len != 0.0 {
            self.x /= len;
            self.y /= len;
            self.z /= len;
        }
    }
}
