use rand::{ self, Rng };

use vec::Vec3;
use ray::Ray;
use color::Color;


#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub color: Color,
    pub origin: Vec3,
    pub r: f32,
}

impl Sphere {
    pub fn new(origin: Vec3, r: f32, color: Color) -> Sphere {
        Sphere {
            origin: origin,
            r: r,
            color: color
        }
    }

    pub fn unit() -> Sphere {
        Sphere::new(
            Vec3::new(0.0, 0.0, 0.0),
            1.0,
            Color::black()
        )
    }

    pub fn intersect(&self, ray: Ray) -> Option<Vec3> {

        let o = ray.origin;
        let l = ray.direction;
        let c = self.origin;
        let r = self.r;

        let o_c = o.sub(c);

        let l_dot_oc = l.dot(o_c);

        let a = -l_dot_oc;
        let b = (l_dot_oc * l_dot_oc) - o_c.dot(o_c) + (r*r);

        if b < 0.0 {
            None
        } else {
            let d = a - b.sqrt();

            let p = o.add(l.multiply(d));

            if (o.sub(p)).length().abs() > 0.001 {
                Some(p)
            } else {
                None
            }
        }
    }

    pub fn scatter(&self, point: Vec3) -> Ray {

        /*let mut rng = rand::thread_rng();

        let norm = self.norm(point);

        let Vec3 { x, y, z } = norm;

        let xs = Vec3::new(x, 0.0, 0.0).norm();
        let ys = Vec3::new(0.0, y, 0.0).norm();
        let zs = Vec3::new(0.0, 0.0, z).norm();

        Ray::new(point, norm
            .add(&xs.multiply(rng.next_f32() * 2.0 - 1.0))
            .add(&ys.multiply(rng.next_f32() * 2.0 - 1.0))
            .add(&zs.multiply(rng.next_f32() * 2.0 - 1.0)))*/

        let p = point.divide(point.length()).multiply(0.1);

        Ray::new(point.add(p), self.norm(point))
    }

    pub fn norm(&self, point: Vec3) -> Vec3 {
        let mut direction = point.sub(self.origin);
        direction.norm();
        direction
    }
}
