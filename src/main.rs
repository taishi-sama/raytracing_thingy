use std::{time::Instant, env::{self}};


use raytracer::{render, save_to_image};
use scene::Scene;

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
