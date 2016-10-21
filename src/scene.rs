use sphere::Sphere;
use ray::Ray;
use vec::Vec3;

pub struct Scene {
    shapes: Vec<Sphere>
}

impl Scene {

    pub fn new (shapes: Vec<Sphere>) -> Scene {
        Scene { shapes: shapes }
    }

    pub fn intersect(&self, ray: Ray) -> Option<(Sphere, Vec3)>  {
        let mut current = None;
        for shape in self.shapes.iter() {
            match shape.intersect(ray) {
                Some(point) => {
                    match current {
                        None => current = Some((*shape, point)),
                        Some((_, c)) => {
                            let d_current = (c.sub(ray.origin)).length();
                            let d_point = (point.sub(ray.origin)).length();

                            if d_point < d_current {
                                current = Some((*shape, point))
                            }
                        }
                    }
                },
                None => ()
            }
        }

        current
    }
}
