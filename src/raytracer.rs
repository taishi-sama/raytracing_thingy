use std::sync::Mutex;

use image::{RgbImage, Rgb, Rgba};
use rayon::prelude::*;

use crate::{color::Color, scene::{Scene, LightSource}, math::{Ray, Vector3, EPSILON}, material::Material};

pub fn render(scene: &Scene, x: usize, y: usize) -> Vec<Color> {
    let p: Vec<_> = scene.image.get_rays(x, y).into_iter().enumerate().collect();

    let b = Mutex::new(vec![Color::BLACK; x * y]);
    let chunk_size = 1000;
    p.par_chunks(chunk_size)
        .for_each(|x|{
            let temp_buffer : Vec<_> = 
            x.iter()
            .map(|(i, y)|(i, Color::from_vector3(&raytrace(scene, y, 1.0)))).collect();
            let mut t =  b.lock().unwrap();
            for (i, c) in temp_buffer {
                t[*i] = c;
            }
            //drop(t)
        });
    b.into_inner().unwrap()
}

pub fn save_to_image(i: &[Color], x:usize, y: usize) -> RgbImage {
    let t = i.iter().flat_map(|c| [c.r, c.g, c.b]).collect();
    RgbImage::from_vec(x as u32, y as u32, t).unwrap()
}
#[inline(always)]
pub fn intencity_distance(int: f32, dist: f32) -> f32 {
    int / (dist.powi(2) * 0.3 + dist * 0.5 )
}

//Цвет пикселя

pub fn raytrace(scene: &Scene, r: &Ray, portion: f32) -> Vector3 {
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
        let (t, normal) = f.intersect_with_normal(r).unwrap();
        let m = f.get_material();
        let int = m.base_illumination;
        let mut color = m.color.mult(int);
        if let Some(c) = shadowray(scene, r,&t, &normal, &scene.light, m) {
            let c_res = c.mult_per_element(&m.color);
            color += c_res;
        }
        if m.refl > EPSILON {
            let c = mirrorray(scene, &t, r, &normal, portion * m.refl);
            color += c;
        }
        color.mult(portion)
    }
    else {
        Vector3{x:0.0, y:0.0, z:0.0}
    }
}
//Цвет как вектор
pub fn shadowray(scene: &Scene, t: &Ray, point: &Vector3, side_normal: &Vector3, l: &LightSource, m: &Material) -> Option<Vector3> {
    let d = &l.pos - point;
    let d_len = d.len();
    let d_norm = d.div(d_len);
    //println!("Point of collision: {point}, vector to light: {d_norm}");
    let diff = d_norm.scalar_product(side_normal);
    if diff > 0.0 {
        let light_ray = &Ray { pos: *point, dir: d_norm };
        let mut intensity = l.intencity;
        let mut color = l.color;
        for i in &scene.figures {
            if let Some(p) = i.intersect(light_ray) {
                if (&p - point).len() < d_len {
                    let m = i.get_material();
                    intensity *= m.transparency;
                    if intensity < EPSILON {return None};
                    color = color.mult_per_element(&m.color);
                }
            }
        }
        let refl = light_ray.reflect(point, side_normal);

        let diff_part = m.diff * diff;
        let spec_part = m.specular * refl.dir.scalar_product(&t.dir).max(-0.2).powf(m.shininess);
        //let spec_part = 0.0;
        let local = intencity_distance(intensity, d_len);
        Some(color.mult((diff_part + spec_part) * local))
    }
    else {None}
}
pub fn mirrorray(scene: &Scene, point: &Vector3, r: &Ray, side_normal: &Vector3, portion: f32) -> Vector3 {
    if portion < EPSILON { return Vector3::new(0.0, 0.0, 0.0); }
    let t = r.reflect(point, side_normal);
    raytrace(scene, &t, portion)
}