use std::ops::{Add, AddAssign, Mul};

use bmp::Pixel;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32
}

fn limit(v: f32) -> f32 {
    v.max(0.0).min(1.0)
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color {
            red: limit(red),
            green: limit(green),
            blue: limit(blue)
        }
    }

    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

impl From<Color> for Pixel {
    fn from(color: Color) -> Pixel {
        Pixel {
            r: (255.0 * color.red) as u8,
            g: (255.0 * color.green) as u8,
            b: (255.0 * color.blue) as u8
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue
        )
    }
}

impl Add<Color> for f32 {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color::new(
            self + rhs.red,
            self + rhs.green,
            self + rhs.blue
        )
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color::new(
            self.red * other,
            self.green * other,
            self.blue * other
        )
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color::new(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue
        )
    }
}
