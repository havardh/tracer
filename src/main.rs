extern crate bmp;

const W : u32 = 1024;
const H : u32 = 1024;

use bmp::*;
use sphere::Sphere;
use vec::Vec3;
use camera::Camera;
use color::Color;
use ray::Ray;

pub mod camera;
pub mod color;
pub mod ray;
pub mod sphere;
pub mod vec;

fn main() {

    let lights = vec!(
        Vec3::new(3.0, -3.0, 5.0),
        Vec3::new(-3.0, -3.0, 1.0)
    );

    let shapes = vec!(Sphere::new(
        Vec3::new(0.0, 0.0, 10.0),
        4.0,
        Color::new(1.0, 1.0, 0.0)
    ));
    let camera = Camera::new();

    let view_port = gen_view_port(camera);

    let mut image = Image::new(W, H);
    for shape in shapes {
        for i in 0..W {
            for j in 0..H {
                let ray = &view_port[i as usize][j as usize];
                match shape.intersect(&ray) {
                    Some(point) => image.set_pixel(i, j, color(shape.color, &ray, point, shape.norm(&point), &lights)),
                    None => {},
                }
            }
        }
    }

    let _ = image.save("sphere.bmp");
}

fn color(base: Color, ray: &Ray, point: Vec3, normal: Vec3, lights: &Vec<Vec3>) -> Pixel {

    let ambient = 0.15;
    let direct = 0.5;


    let f1 = (ray.direction.dot(&normal) / (ray.direction.length() * normal.length())).abs();

    let mut f2 = 0.0;
    for light in lights {
        let v2 = point.sub(&light);
        let mut f = -(v2.dot(&normal) / (v2.length() * normal.length()));
        f = if f > 0.0 { f } else { 0.0 };
        f2 += f;
    }

    let mut f = f1 * ambient + f2 * direct;;

    f = if f > 1.0 { 1.0 } else { f };

    Pixel {
        r: (255.0 * base.red as f32 * f) as u8,
        g: (255.0 * base.green as f32 * f) as u8,
        b: (255.0 * base.blue as f32 * f) as u8
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
