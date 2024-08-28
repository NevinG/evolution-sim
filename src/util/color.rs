#[derive(Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn random() -> Color {
        Color {
            r: rand::random(),
            g: rand::random(),
            b: rand::random(),
        }
    }
}
