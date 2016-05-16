use vec::Vec3;
use ray::Ray;

#[derive(Debug)]
pub struct Sphere {
    pub origin: Vec3,
    pub r: f32,
}

impl Sphere {
    pub fn new(origin: Vec3, r: f32) -> Sphere {
        Sphere {
            origin: origin, r: r
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Vec3> {

        let o = ray.origin;
        let l = ray.direction;
        let c = &self.origin;
        let r = self.r;

        let o_c = o.sub(&c);

        let l_dot_oc = l.dot(&o_c);

        let a = -l_dot_oc;
        let b = (l_dot_oc * l_dot_oc) - o_c.dot(&o_c) + (r*r);

        if b < 0.0 {
            None
        } else {
            let d = a - b.sqrt();

            let p = o.add(&l.multiply(d));

            if (o.sub(&p)).length().abs() > 0.001 {
                Some(p)
            } else {
                None
            }
        }
    }

    pub fn norm(&self, point: &Vec3) -> Vec3 {
        let mut direction = point.sub(&self.origin);
        direction.norm();
        direction
    }
}
