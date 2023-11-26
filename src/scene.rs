use crate::{
    figure::FigureKind,
    math::{Ray, Vector3}, color::Color, material::Material,
};

#[derive(Debug, Clone)]
pub struct Scene {
    pub figures: Vec<FigureKind>,
    pub image: RenderSurface,
    pub lights: Vec<LightSource>,
}

impl Scene {
    //2, 2 
    pub fn get_room() -> Self{
        let r = RenderSurface {
            top_left: Vector3::new(-1.5, -1.5, -1.95),
            top_right: Vector3::new(1.5, -1.5, -1.95),
            down_left: Vector3::new(-1.5, 1.5, -1.95),
            foci_point: Vector3::new(0.0, 0.0, -4.95),
        };

        let top = FigureKind::new_side(
            &Vector3::new(-2.0, -2.0, -2.0),
            &Vector3::new(-2.0, -2.0, 2.0),
            &Vector3::new(2.0, -2.0, -2.0), 
            Material::FRONTWALLS);
        let front = FigureKind::new_side(
            &Vector3::new(-2.0, -2.0, 2.0),
            &Vector3::new(-2.0, 2.0, 2.0),
            &Vector3::new(2.0, -2.0, 2.0), 
            Material::FRONTWALLS);
        let down = FigureKind::new_side(
          &Vector3::new(-2.0, 2.0, -2.0),
          &Vector3::new(2.0, 2.0, -2.0),
          &Vector3::new(-2.0, 2.0, 2.0),
          Material::FRONTWALLS);
        let left = FigureKind::new_side(
            &Vector3::new(-2.0, -2.0, 2.0),
            &Vector3::new(-2.0, -2.0, -2.0),
            &Vector3::new(-2.0, 2.0, 2.0),
            Material::LEFTWALL);
        let right = FigureKind::new_side(
          &Vector3::new(2.0, 2.0, 2.0),
          &Vector3::new(2.0, 2.0, -2.0),
          &Vector3::new(2.0, -2.0, 2.0),
          Material::RIGHTWALL);
        let back = FigureKind::new_side(
            &Vector3::new(-2.0, -2.0, -2.0),
            &Vector3::new(2.0, -2.0, -2.0),
            &Vector3::new(-2.0, 2.0, -2.0),
            Material::BACKWALLS);
        let cube = FigureKind::new_cube(
            &Vector3::new(-1.5, 1.0, 1.5), 
            &Vector3::new(-0.5, 1.0, 1.5), 
            &Vector3::new(-1.5, 2.0, 1.5), 
            &Vector3::new(-1.5, 1.0, 0.5), Material::CUBE);
        let cube2 = FigureKind::new_cube_from_d(
            &Vector3::new(0.5, 0.5, 1.0), 
            &Vector3::new(1.0, 0.0, 0.0).rotate_y_axis(std::f32::consts::PI / 6.0 ), 
            &Vector3::new(0.0, 1.499, 0.0).rotate_y_axis(std::f32::consts::PI / 6.0 ), 
            &Vector3::new(0.0, 0.0, -1.0).rotate_y_axis(std::f32::consts::PI / 6.0 ), Material::CUBEMETALIC);
        let sphere = FigureKind::Sphere { r: 0.2, pos: Vector3 { x: -1.0, y: 1.5, z: -0.5 }, m: Material::CUBETRANSPARENT };
        
        let v = vec![
            top, 
            front, 
            down,
            left,
            right,
            back,
            cube,
            cube2,
            sphere
        ];

        let l1 = LightSource { pos: Vector3::new(1.6, -1.6, -0.1), color: Color::WHITE.to_vector3(), intencity: 1.5 };
        let l2 = LightSource { pos: Vector3::new(-1.6, -1.6, -0.1), color: Color::WHITE.to_vector3(), intencity: 1.5 };

        Scene { figures: v, image: r, lights: vec![l1, l2] }
    }
}

#[derive(Debug, Clone)]
pub struct RenderSurface {
    pub top_left: Vector3,
    pub top_right: Vector3,
    pub down_left: Vector3,
    pub foci_point: Vector3,
}
impl RenderSurface {
    pub fn get_rays(&self, x: usize, y: usize) -> Vec<Ray> {
        let mut r = vec![];
        let delta_y = (self.down_left - self.top_left).div(x as f32);
        let delta_x = (self.top_right - self.top_left).div(y as f32);
        let begin = self.top_left + delta_x.mult(0.5) + delta_y.mult(0.5);
        let origin = self.foci_point;

        for j in 0..y {
            for i in 0..x {
                let pos = begin + delta_x.mult(i as f32) + delta_y.mult(j as f32);
                let dir = (pos - origin).normalize();
                let ray = Ray { pos, dir };
                r.push(ray)
            }
        }
        r
    }
}
#[derive(Debug, Clone)]
pub struct LightSource {
    pub pos: Vector3,
    pub color: Vector3,
    pub intencity: f32,
}
