// #![feature(core_intrinsics)]

extern crate rand;

mod camera;
mod image;
mod materials;
mod scene;
mod shapes;
mod types;
mod vecmath;

use camera::Camera;
use image::Image;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use scene::Scene;
use std::path::Path;
use types::Fp;
use vecmath::{Color3U8, Vec3F};
// use std::intrinsics::breakpoint;

fn main() {
    // unsafe { breakpoint(); }

    let mut image = Image::new(800, 600);

    let scene = Scene::two_spheres();

    let camera = Camera::builder()
        .pixel_dimension(image.width, image.height)
        .fov(0.5)
        .focal_length(1.0)
        .position(Vec3F::new(0.0, 0.0, 0.0))
        .build();

    let mut rand = SmallRng::seed_from_u64(131);
    let pixel_samples: [(Fp, Fp); 10] = std::array::from_fn(|_| {
        (
            rand.gen_range(0.0..1.0 as Fp),
            rand.gen_range(0.0..1.0 as Fp),
        )
    });

    for row in 0..image.height {
        for col in 0..image.width {
            let mut attenuation = Vec3F::zero();

            for rand_sample in pixel_samples.iter() {
                let ray = camera.gen_ray(col, row, rand_sample.0, rand_sample.1);
                attenuation += scene.trace(&ray, &mut rand, 0);
            }

            image.write_pixel(row, col, Color3U8::from(attenuation));
        }
    }

    image
        .write_bmp(Path::new("C:\\Projects\\rt-weekends\\render.bmp"))
        .unwrap();
}
