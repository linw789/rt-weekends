mod types;
mod image;
mod vecmath;
mod camera;
mod shapes;
mod scene;

use types::Fp as Fp;
use image::Image;
use scene::Scene;
use std::path::Path;

fn main() {
    let mut image = Image::new(800, 600);

    let scene = Scene::from_file(Path::new("C:\\Projects\\rt-weekends\\src\\two-spheres.txt")).unwrap();

    for row in 0..image.height {
        for col in 0..image.width {
            image.write_pixel(row, col, [(row * 255 / image.height) as u8, 0, 0]);
        }
    }

    image.write_bmp(Path::new("C:\\Projects\\rt-weekends\\render.bmp")).unwrap();
}
