extern crate bmp;

const W : u32 = 512;
const H : u32 = 512;

use bmp::*;

fn main() {
    let sphere = Sphere::new(Vect::new(0.0, 0.0, 4.0), 2.0);

    let view_port = gen_view_port();

    let mut image = Image::new(W, H);
    let color = Pixel { r: 255, g: 255, b: 255 };
    for i in 0..W {
        for j in 0..H {
            let ray = &view_port[i as usize][j as usize];
            if sphere.intersect(&ray) {
                image.set_pixel(i, j, color);
            }
        }
    }

    let _ = image.save("sphere.bmp");
}

fn gen_view_port() -> Vec<Vec<Vect>> {
    let half_width = (W / 2) as f32;
    let half_height = (H / 2) as f32;

    let mut view_port = Vec::with_capacity(W as usize);
    for i in 0..W {
        let mut row = Vec::with_capacity(H as usize);
        for j in 0..H {
            let x = (i as f32 - half_width) / half_width;
            let y = (j as f32 - half_height) / half_height;
            let z = 1.0;

            let mut ray = Vect::new(x, y, z);
            ray.norm();
            row.push(ray);
        }
        view_port.push(row);
    }
    view_port
}


#[derive(Debug)]
struct Sphere {
    pub origin: Vect,
    pub r: f32,
}

impl Sphere {
    pub fn new(origin: Vect, r: f32) -> Sphere {
        Sphere {
            origin: origin, r: r
        }
    }

    pub fn intersect(&self, ray: &Vect) -> bool {

        let o = Vect::new(0.0, 0.0, 0.0);
        let l = ray;
        let c = &self.origin;
        let r = self.r;

        let o_c = o.minus(&c);

        let l_dot_oc = l.dot(&o_c);

        let a = -l_dot_oc;
        let b = (l_dot_oc * l_dot_oc) - o_c.dot(&o_c) + (r*r);

        if b < 0.0 {
            false
        } else {
            let d = a - b.sqrt();

            let p = o.add(&l.multiply(d));

            if (o.minus(&p)).length().abs() > 0.001 {
                true
            } else {
                false
            }
        }

    }
}

#[derive(Debug)]
struct Vect {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vect {


    pub fn new(x: f32, y: f32, z: f32) -> Vect {
        Vect {
            x: x, y: y, z: z
        }
    }

    pub fn length(&self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    pub fn add(&self, other: &Vect) -> Vect {
        Vect::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn minus(&self, other: &Vect) -> Vect {
        Vect::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn dot(&self, other: &Vect) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn multiply(&self, f: f32) -> Vect {
        Vect::new(self.x * f, self.y * f, self.z * f)
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
