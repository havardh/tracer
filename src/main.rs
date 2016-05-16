extern crate bmp;

const W : u32 = 512;
const H : u32 = 512;

use bmp::*;
use sphere::Sphere;
use vec::Vec3;

pub mod sphere;
pub mod vec;

fn main() {
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, 4.0), 2.0);

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

fn gen_view_port() -> Vec<Vec<Vec3>> {
    let half_width = (W / 2) as f32;
    let half_height = (H / 2) as f32;

    let mut view_port = Vec::with_capacity(W as usize);
    for i in 0..W {
        let mut row = Vec::with_capacity(H as usize);
        for j in 0..H {
            let x = (i as f32 - half_width) / half_width;
            let y = (j as f32 - half_height) / half_height;
            let z = 1.0;

            let mut ray = Vec3::new(x, y, z);
            ray.norm();
            row.push(ray);
        }
        view_port.push(row);
    }
    view_port
}
