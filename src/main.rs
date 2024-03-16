// #![feature(core_intrinsics)]

mod camera;
mod image;
mod materials;
mod scene;
mod shapes;
mod types;
mod vecmath;

use camera::Camera;
use image::Image;
use scene::Scene;
use std::path::Path;
use vecmath::{Color3U8, Vec3F};
// use std::intrinsics::breakpoint;

fn main() {
    // unsafe { breakpoint(); }

    let mut image = Image::new(800, 600);

    let scene = Scene::one_sphere();
    let camera = Camera::builder()
        .pixel_dimension(image.width, image.height)
        .fov(0.5)
        .focal_length(1.0)
        .position(Vec3F::new(0.0, 0.0, 0.0))
        .build();

    for row in 0..image.height {
        for col in 0..image.width {
            let ray = camera.gen_ray(col, row);
            let attenuation = scene.trace(&ray);

            image.write_pixel(row, col, Color3U8::from(attenuation),);
        }
    }

    image
        .write_bmp(Path::new("C:\\Projects\\rt-weekends\\render.bmp"))
        .unwrap();
}
