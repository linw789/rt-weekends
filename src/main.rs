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
use image::{Image, IMAGE_PIXEL_SIZE};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use scene::Scene;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use types::Fp;
use vecmath::{Color3F, Color3U8, Vec3F};

fn linear_to_gamma(color: &Color3F) -> Color3F {
    Color3F::new(
        if color.x > 0.0 { color.x.sqrt() } else { 0.0 },
        if color.y > 0.0 { color.y.sqrt() } else { 0.0 },
        if color.z > 0.0 { color.z.sqrt() } else { 0.0 },
    )
}

fn trace_row<R: rand::Rng>(
    scene: &Scene,
    camera: &Camera,
    row_index: u32,
    row_pixels: &mut [[u8; IMAGE_PIXEL_SIZE]],
    pixel_samples: &[(Fp, Fp)],
    rand: &mut R,
) {
    for col in 0..row_pixels.len() {
        let mut pixel_color = Vec3F::zero();

        for rand_sample in pixel_samples.iter() {
            let ray = camera.gen_ray(col as u32, row_index, rand_sample.0, rand_sample.1, rand);
            pixel_color += scene.trace(&ray, rand, 0);
        }

        pixel_color = pixel_color / (pixel_samples.len() as Fp);
        pixel_color = linear_to_gamma(&pixel_color);

        row_pixels[col] = Color3U8::from(pixel_color).into();
    }
}

fn main() {
    const IMAGE_WIDTH: u32 = 1200;
    const IMAGE_HEIGHT: u32 = 800;

    let image = Arc::new(Mutex::new(Image::new(IMAGE_WIDTH, IMAGE_HEIGHT)));

    let scene = Arc::new(Scene::many_spheres());

    let camera = Arc::new(
        Camera::builder()
            .pixel_dimension(IMAGE_WIDTH, IMAGE_HEIGHT)
            .fov(20.0 / 180.0)
            .focus_length(10.0)
            .defocus_angle(0.6 / 180.0)
            .position(Vec3F::new(13.0, 2.0, 3.0))
            .lookat(Vec3F::zero())
            .up(Vec3F::new(0.0, 1.0, 0.0))
            .build(),
    );

    let threads_num = thread::available_parallelism().unwrap().get() as u32;
    let rows_per_thread = IMAGE_HEIGHT / threads_num;

    let mut trace_threads = Vec::with_capacity(threads_num as usize);

    for thread_index in 0..threads_num {
        let rows_num = if thread_index < (threads_num - 1) {
            rows_per_thread
        } else {
            IMAGE_HEIGHT - rows_per_thread * (threads_num - 1)
        };
        let row_start_index = thread_index * rows_per_thread;

        let image = Arc::clone(&image);
        let scene = Arc::clone(&scene);
        let camera = Arc::clone(&camera);

        trace_threads.push(thread::spawn(move || {
            let mut rand = SmallRng::seed_from_u64(131);
            let pixel_samples: [(Fp, Fp); 500] = std::array::from_fn(|_| {
                (
                    rand.gen_range(0.0..1.0 as Fp),
                    rand.gen_range(0.0..1.0 as Fp),
                )
            });

            let mut row_pixels: Vec<[u8; IMAGE_PIXEL_SIZE]> = vec![];
            row_pixels.resize(IMAGE_WIDTH as usize, [0, 0, 0]);

            for r in 0..rows_num {
                let row_index = row_start_index + r;
                trace_row(
                    &scene,
                    &camera,
                    row_index,
                    &mut row_pixels,
                    &pixel_samples,
                    &mut rand,
                );
                image.lock().unwrap().write_row(row_index, &row_pixels);
            }
        }));
    }

    for thread in trace_threads.into_iter() {
        assert!(thread.join().is_ok());
    }

    image
        .lock()
        .unwrap()
        // .write_bmp(Path::new("/home/linw/Projects/rt-weekends/render.bmp"))
        .write_bmp(Path::new("C:\\Projects\\rt-weekends\\render.bmp"))
        .unwrap();
}
