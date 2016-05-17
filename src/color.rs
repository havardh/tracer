#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        assert!(red >= 0.0 && red <= 1.0);
        assert!(green >= 0.0 && green <= 1.0);
        assert!(blue >= 0.0 && blue <= 1.0);

        Color { red: red, green: green, blue: blue }
    }
}
