extern crate bmp;

const W : u32 = 1024;
const H : u32 = 1024;

use bmp::*;
use sphere::Sphere;
use vec::Vec3;
use camera::Camera;
use ray::Ray;

pub mod camera;
pub mod ray;
pub mod sphere;
pub mod vec;

fn main() {
    let shapes = vec!(
        Sphere::new(Vec3::new(20.0, 0.0, 20.0), 4.0),
        Sphere::new(Vec3::new(0.0, 0.0, 20.0), 4.0)
    );
    let camera = Camera::new();

    let view_port = gen_view_port(camera);

    let mut image = Image::new(W, H);
    for shape in shapes {
        for i in 0..W {
            for j in 0..H {
                let ray = &view_port[i as usize][j as usize];
                match shape.intersect(&ray) {
                    Some(p) => image.set_pixel(i, j, color(&ray, shape.norm(&p))),
                    None => {},
                }
            }
        }
    }

    let _ = image.save("sphere.bmp");
}

fn color(ray: &Ray, normal: Vec3) -> Pixel {

    let f = (ray.direction.dot(&normal) / (ray.direction.length() * normal.length())).abs();

    Pixel {
        r: (255.0 * f) as u8,
        g: (255.0 * f) as u8,
        b: (255.0 * f) as u8
    }
}

fn gen_view_port(camera: Camera) -> Vec<Vec<Ray>> {
    let half_width = (W / 2) as f32;
    let half_height = (H / 2) as f32;

    let (e, x, y, m) = calculate_vectors(camera);

    let mut view_port = Vec::with_capacity(W as usize);
    for i in 0..W {
        let mut row = Vec::with_capacity(H as usize);
        for j in 0..H {
            let sx = (i as f32 - half_width) / half_width;
            let sy = (j as f32 - half_height) / half_height;

            let p = m.add(&x.multiply(sx).add(&y.multiply(sy)));

            let t = p.sub(&e);
            let mut d = t;
            d.norm();

            row.push(Ray::new(e, d));
        }
        view_port.push(row);
    }
    view_port
}

fn calculate_vectors(camera: Camera) -> (Vec3, Vec3, Vec3, Vec3) {
    let mut u = camera.up;
    let mut v = camera.direction;

    let phi = 3.14 / 2.0;
    let delta = phi * (W as f32/H as f32);
    let c = 1.1;

    u.norm();
    v.norm();

    let mut a = v.cross(&u);
    let mut b = a.cross(&v);

    a.norm();
    b.norm();

    let e = camera.position;
    let mut x = a.multiply(c * (delta / 2.0).tan());
    let mut y = b.multiply(c * (phi / 2.0).tan());
    let mut m = e.add(&v.multiply(c));

    x.norm();
    y.norm();
    m.norm();

    (e, x, y, m)
}
