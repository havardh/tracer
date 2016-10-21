extern crate bmp;
extern crate rand;

const W : u32 = 512;
const H : u32 = 512;


use bmp::*;
use sphere::Sphere;
use scene::Scene;
use vec::Vec3;
use camera::Camera;
use color::Color;
use light::Light;
use ray::Ray;

pub mod camera;
pub mod color;
pub mod light;
pub mod ray;
pub mod scene;
pub mod sphere;
pub mod vec;

fn main() {

    let lights = vec!(
        Light::new(Vec3::new(3.0, -3.0, 1.0), Color::new(1.0, 1.0, 1.0)),
        Light::new(Vec3::new(-3.0, -3.0, 1.0), Color::new(1.0, 1.0, 1.0)),
        Light::new(Vec3::new(0.0, -10.0, 1.0), Color::new(1.0, 1.0, 1.0)),
    );

    let scene = Scene::new(vec!(
        Sphere::new(Vec3::new(0.0, 0.0, 10.0), 2.0, Color::new(1.0, 1.0, 1.0)),
        Sphere::new(Vec3::new(0.0, -4.0, 10.0), 2.0, Color::new(1.0, 1.0, 1.0)),
    ));
    let camera = Camera::new();

    let view_port = gen_view_port(camera);

    let mut image = Image::new(W, H);

    let (w_from, w_to) = (252, 253); //(0, W);
    let (h_from, h_to) = (300, 301); //(0, H);


    for i in w_from..w_to {
        for j in h_from..h_to {
            let ray = view_port[i as usize][j as usize];
            let c = color(ray, &scene, &lights, 10);
            image.set_pixel(i, j, c.into());
        }
    }
    let _ = image.save("sphere.bmp");
}

fn color(ray: Ray, scene: &Scene, lights: &Vec<Light>, n: i32) -> Color {
    if n < 0 {
        return Color::black();
    }


    match scene.intersect(ray) {
        Some((shape, point)) => {
            println!("{:?}", point);
            let base = shape.color;
            let normal = shape.norm(point);

            let ambient = 0.0;
            let direct = 0.05;
            let reflection = 1.0;

            let f1 = (ray.direction.dot(normal) / (ray.direction.length() * normal.length())).abs();

            let mut f2 = Color::new(0.0, 0.0, 0.0);
            for light in lights {
                let v2 = point.sub(light.origin);
                let mut f = -(v2.dot(normal) / (v2.length() * normal.length()));
                f = if f > 0.0 { f } else { 0.0 };
                f2 += light.color * f;
            }

            let f3 = color(shape.scatter(point), scene, lights, n-1);

            let f = f1 * ambient + f2 * direct + f3 * reflection;

            base * f
        },
        None => Color::black()
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

            let p = m.add(x.multiply(sx).add(y.multiply(sy)));

            let t = p.sub(e);
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

    let mut a = v.cross(u);
    let mut b = a.cross(v);

    a.norm();
    b.norm();

    let e = camera.position;
    let mut x = a.multiply(c * (delta / 2.0).tan());
    let mut y = b.multiply(c * (phi / 2.0).tan());
    let mut m = e.add(v.multiply(c));

    x.norm();
    y.norm();
    m.norm();

    (e, x, y, m)
}
