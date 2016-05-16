use vec::Vec3;

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

    pub fn intersect(&self, ray: &Vec3) -> bool {

        let o = Vec3::new(0.0, 0.0, 0.0);
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
