use crate::{color::Color, math::Vector3};

#[derive(Debug, Clone, Copy,)]
pub struct Material {
    pub color: Vector3,
    pub refl: f32,
    pub diff: f32,
    pub transparency: f32,
    pub base_illumination: f32,
}
impl Material {
    pub const BACKWALLS: Material = Material {
        color: Vector3::new(1.0, 1.0, 1.0),
        refl: 0.1,
        diff: 0.9,
        transparency: 0.0,
        base_illumination: 0.05,
    };
    pub const LEFTWALL: Material = Material {
        color: Vector3::new(1.0, 0.2, 0.2),
        refl: 0.1,
        diff: 0.9,
        transparency: 0.0,
        base_illumination: 0.05,
    };
    pub const RIGHTWALL: Material = Material {
        color: Vector3::new(0.2, 0.2, 1.0),
        refl: 0.1,
        diff: 0.9,
        transparency: 0.0,
        base_illumination: 0.05,
    };
    pub const CUBE: Material = Material {
        color: Vector3::new(1.0, 1.0, 0.2),
        refl: 0.1,
        diff: 0.9,
        transparency: 0.0,
        base_illumination: 0.05,
    };
    pub const CUBEMETALIC: Material = Material {
        color: Vector3::new(0.9, 0.9, 0.9),
        refl: 0.4,
        diff: 0.6,
        transparency: 0.0,
        base_illumination: 0.05,
    };
}
