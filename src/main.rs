use std::{time::Instant, env::{Args, self}};

use color::Color;
use figure::FigureKind;
use material::Material;
use math::{Ray, Vector3};
use raytracer::{render, save_to_image};
use scene::{RenderSurface, Scene, LightSource};

pub mod color;
pub mod figure;
pub mod material;
pub mod math;
pub mod raytracer;
pub mod scene;

fn main() {
    let default_res = 500;
    let pixels = if let Some(t) = env::args().nth(1) {
        t.parse().unwrap_or(default_res)
    } else {default_res};
    let s = Scene::get_room();
    let x = pixels;
    let y = pixels;
    let begin = Instant::now();
    let t = render(&s, x, y);
    println!("Elapsed: {:?}", begin.elapsed());
    save_to_image(&t, x, y).save_with_format("./output.png", image::ImageFormat::Png).unwrap();
}
