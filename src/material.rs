use crate::{color::Color, math::Vector3};

#[derive(Debug, Clone, Copy,)]
pub struct Material {
    pub color: Vector3,
    pub refl: f32,
    pub diff: f32,
    pub specular : f32,
    pub shininess : f32,
    pub transparency: f32,
    pub base_illumination: f32,
}
impl Material {
    pub const FRONTWALLS: Material = Material {
        color: Vector3::new(1.0, 1.0, 1.0),
        refl: 0.05,
        diff: 0.95,
        specular: 0.02,
        shininess: 1.0,
        transparency: 0.0,
        base_illumination: 0.05,
    };
    pub const BACKWALLS: Material = Material {
        color: Vector3::new(0.2, 0.5, 0.2),
        refl: 0.05,
        diff: 0.95,
        specular: 0.3,
        shininess: 1.0,
        transparency: 0.0,
        base_illumination: 0.05,
    };
    pub const LEFTWALL: Material = Material {
        color: Vector3::new(1.0, 0.2, 0.2),
        refl: 0.1,
        diff: 0.9,
        specular: 0.3,
        shininess: 1.0,
        transparency: 0.0,
        base_illumination: 0.05,
    };
    pub const RIGHTWALL: Material = Material {
        color: Vector3::new(0.2, 0.2, 1.0),
        refl: 0.1,
        diff: 0.9,
        specular: 0.3,
        shininess: 1.0,
        transparency: 0.0,
        base_illumination: 0.05,
    };
    pub const CUBE: Material = Material {
        color: Vector3::new(1.0, 1.0, 0.2),
        refl: 0.0,
        diff: 1.0,
        specular: 0.3,
        shininess: 1.0,
        transparency: 0.0,
        base_illumination: 0.05,
    };
    pub const CUBEMETALIC: Material = Material {
        color: Vector3::new(0.9, 0.9, 0.9),
        refl: 0.6,
        diff: 0.4,
        specular: 0.3,
        shininess: 1.0,
        transparency: 0.0,
        base_illumination: 0.05,
    };
    pub const MIRRORMATERIAL: Material = Material {
        color: Vector3::new(1.0, 1.0, 1.0),
        refl: 0.99,
        diff: 0.01,
        specular: 0.01,
        shininess: 1.0,
        transparency: 0.0,
        base_illumination: 0.01,  
    };
}
