use std::sync::Mutex;

use image::{RgbImage, Rgb, Rgba};
use rayon::prelude::*;

use crate::{color::Color, scene::{Scene, LightSource}, math::{Ray, Vector3, EPSILON}};

pub fn render(scene: &Scene, x: usize, y: usize) -> Vec<Color> {
    let p: Vec<_> = scene.image.get_rays(x, y).into_iter().enumerate().collect();

    let b = Mutex::new(vec![Color::BLACK; x * y]);
    p.par_chunks(500)
        .for_each(|x|x.iter()
            .for_each(|(i, y)| b.lock().unwrap()[*i] = Color::from_vector3(&raytrace(scene, y))));
    b.into_inner().unwrap()
}

pub fn save_to_image(i: &[Color], x:usize, y: usize) -> RgbImage {
    let t = i.iter().flat_map(|c| [c.r, c.g, c.b]).collect();
    RgbImage::from_vec(x as u32, y as u32, t).unwrap()
}
#[inline(always)]
pub fn intencity_distance(int: f32, dist: f32) -> f32 {
    int / (dist.powi(2) * 0.5 + dist * 0.5 )
}

//Цвет пикселя
pub fn raytrace(scene: &Scene, r: &Ray) -> Vector3 {
    let mut d = f32::MAX;
    let mut f_i = usize::MAX;
    for (i, f) in scene.figures.iter().enumerate() {
        if let Some(p) = f.intersect(r) {
            let d_new = (p - r.pos).len_sq();
            if d_new < d {
                d = d_new;
                f_i = i;
            }
        }
    }
    if f_i != usize::MAX {
        let f = &scene.figures[f_i];
        let (t, normal) = f.intersect_with_normal(&r).unwrap();
        let m = f.get_material();
        let mut int = m.base_illumination;
        let mut color = m.color.mult(int);
        if let Some((intencity, dist, c)) = shadowray(scene, &t, &normal, &scene.light) {
            let local = intencity_distance(intencity, dist) * m.diff;
            let c_res = c.mult_per_element(&m.color);
            int += local;
            color += c_res.mult(local);
        }
        color
    }
    else {
        Vector3{x:0.0, y:0.0, z:0.0}
    }
}
//Интенсивность источника, расстояние до источника, цвет как вектор
pub fn shadowray(scene: &Scene, point: &Vector3, side_normal: &Vector3, l: &LightSource) -> Option<(f32, f32, Vector3)> {
    let d = &l.pos - point;
    let d_len = d.len();
    let d_norm = d.div(d_len);
    //println!("Point of collision: {point}, vector to light: {d_norm}");
    
    if d.scalar_product(side_normal) > 0.0 {
        let r = &Ray { pos: *point, dir: d_norm };
        let mut intensity = l.intencity;
        let mut color = l.color;
        for i in &scene.figures {
            if let Some(p) = i.intersect(r) {
                if (&p - point).len() < d_len {
                    let m = i.get_material();
                    intensity *= m.transparency;
                    if intensity < EPSILON {return None};
                    color += color.mult_per_element(&m.color);
                }
            }
        }
        Some((intensity, d_len, color) )
    }
    else {None}
}
//pub fn mirrorray(scene: &Scene, point: &Vector3)