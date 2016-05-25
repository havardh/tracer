use vec::Vec3;
use color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub origin: Vec3,
    pub color: Color,
}

impl Light {
    pub fn new(origin: Vec3, color: Color) -> Light {
        Light { origin: origin, color: color }
    }
}
