use crate::shapes::Ray;
use crate::types::Fp;
use crate::vecmath::{cross, Vec3F};
#[cfg(not(feature = "use-64bit-float"))]
use std::f32::consts::PI;
#[cfg(feature = "use-64bit-float")]
use std::f64::consts::PI;

pub struct Camera {
    postion: Vec3F,

    pixel_start_pos: Vec3F,
    viewport_delta_u: Vec3F,
    viewport_delta_v: Vec3F,

    defocus_disk_u: Vec3F,
    defocus_disk_v: Vec3F,
}

#[derive(Default)]
pub struct CameraBuilder {
    pixel_width: u32,
    pixel_height: u32,

    position: Vec3F,
    lookat: Vec3F,
    up: Vec3F,

    fov: Fp, // vertical field of view, in half turns.
    focus_length: Fp,

    defocus_angle: Fp,
}

impl Camera {
    pub fn builder() -> CameraBuilder {
        CameraBuilder {
            fov: 0.5,
            focus_length: 3.4,
            defocus_angle: 0.0277,
            ..Default::default()
        }
    }

    pub fn gen_ray<R: rand::Rng>(&self, w: u32, h: u32, rx: Fp, ry: Fp, rand: &mut R) -> Ray {
        // Generate a random ray bounded by the pixel cell.

        assert!(rx >= 0.0 && rx < 1.0, "rx: {}", rx);
        assert!(ry >= 0.0 && ry < 1.0, "ry: {}", ry);

        let random_sample = (rx - 0.5) * self.viewport_delta_u + (ry - 0.5) * self.viewport_delta_v;

        let pixel_center = self.pixel_start_pos
            + ((w as Fp) * self.viewport_delta_u)
            + ((h as Fp) * self.viewport_delta_v);

        let ray_origin = self.defocus_disk_sample(rand);

        Ray::new(ray_origin, (pixel_center + random_sample) - ray_origin)
    }

    fn defocus_disk_sample<R: rand::Rng>(&self, rand: &mut R) -> Vec3F {
        // Return a random point on the defocus disk.
        let (rx, ry) = loop {
            let x = rand.gen_range(0.0..1.0);
            let y = rand.gen_range(0.0..1.0);
            let p = Vec3F::new(x, y, 0.0);
            if p.length_squared() < 1.0 {
                break (x, y);
            }
        };

        self.postion + rx * self.defocus_disk_u + ry * self.defocus_disk_v
    }
}

impl CameraBuilder {
    pub fn pixel_dimension(mut self, width: u32, height: u32) -> CameraBuilder {
        self.pixel_width = width;
        self.pixel_height = height;
        self
    }

    /// Vertical field of view, in half turns [0.0, 2.0].
    pub fn fov(mut self, fov: Fp) -> CameraBuilder {
        assert!(fov > 0.0 && fov < 1.0); // (0, 180) degree
        self.fov = fov;
        self
    }

    /// Distance from camera to the plane of perfect focus.
    pub fn focus_length(mut self, focus_length: Fp) -> CameraBuilder {
        self.focus_length = focus_length;
        self
    }

    pub fn position(mut self, position: Vec3F) -> CameraBuilder {
        self.position = position;
        self
    }

    pub fn lookat(mut self, dir: Vec3F) -> CameraBuilder {
        self.lookat = dir;
        self
    }

    pub fn up(mut self, dir: Vec3F) -> CameraBuilder {
        self.up = dir;
        self
    }

    pub fn defocus_angle(mut self, angle: Fp) -> CameraBuilder {
        assert!(angle >= 0.0 && angle < 0.5);
        self.defocus_angle = angle;
        self
    }

    pub fn build(self) -> Camera {
        let aspect_ratio = (self.pixel_width as Fp) / (self.pixel_height as Fp);

        let fov_tangent = (self.fov / 2.0 * PI).tan();
        let viewport_height = 2.0 * self.focus_length * fov_tangent;
        let viewport_width = viewport_height * aspect_ratio;

        let camera_z = (self.position - self.lookat).normalized();
        let camera_x = cross(&self.up, &camera_z).normalized();
        let camera_y = cross(&camera_z, &camera_x);

        let viewport_u = viewport_width * camera_x;
        let viewport_v = -viewport_height * camera_y;

        let viewport_delta_u = viewport_u / (self.pixel_width as Fp);
        let viewport_delta_v = viewport_v / (self.pixel_height as Fp);

        let viewport_upper_left =
            self.position - (self.focus_length * camera_z) - 0.5 * (viewport_u + viewport_v);

        let pixel_start_pos = viewport_upper_left + 0.5 * (viewport_delta_u + viewport_delta_v);

        let defocus_angle = self.defocus_angle / 2.0;
        let defocus_disk_radius = if defocus_angle <= 0.0 {
            0.0
        } else {
            self.focus_length * (defocus_angle * PI).tan()
        };

        Camera {
            postion: self.position,
            pixel_start_pos,
            viewport_delta_u,
            viewport_delta_v,
            defocus_disk_u: camera_x * defocus_disk_radius,
            defocus_disk_v: camera_y * defocus_disk_radius,
        }
    }
}

unsafe impl Send for Camera {}
unsafe impl Sync for Camera {}
