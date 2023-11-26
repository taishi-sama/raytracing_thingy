use crate::math::Vector3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    pub const BLACK: Self = Color{r:0, g: 0, b: 0};
    pub const WHITE: Self = Color{r: 255, g: 255, b: 255};
    pub fn to_vector3(&self) -> Vector3 {
        Vector3 { x: self.r as f32 / 255.0, y: self.g as f32 / 255.0, z: self.b as f32 / 255.0 }
    }
    pub fn from_vector3(v: &Vector3) -> Self {
        Color { r: (v.x.clamp(0.0, 1.0) * 255.0) as u8, g: (v.y.clamp(0.0, 1.0) * 255.0) as u8, b: (v.z.clamp(0.0, 1.0) * 255.0) as u8 }
    }
}