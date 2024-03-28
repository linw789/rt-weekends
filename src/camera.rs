use crate::shapes::Ray;
use crate::types::Fp;
use crate::vecmath::Vec3F;
#[cfg(not(feature = "use-64bit-float"))]
use std::f32::consts::PI;
#[cfg(feature = "use-64bit-float")]
use std::f64::consts::PI;

pub struct Camera {
    postion: Vec3F,

    pixel_start_pos: Vec3F,
    viewport_delta_u: Vec3F,
    viewport_delta_v: Vec3F,
}

#[derive(Default)]
pub struct CameraBuilder {
    pixel_width: u32,
    pixel_height: u32,

    position: Vec3F,

    fov: Fp, // vertical field of view, in half turns
    focal_length: Fp,
}

impl Camera {
    pub fn builder() -> CameraBuilder {
        CameraBuilder {
            fov: 0.5,
            focal_length: 1.0,
            ..Default::default()
        }
    }

    pub fn gen_ray(&self, w: u32, h: u32, rx: Fp, ry: Fp) -> Ray {
        // Generate a random ray bounded by the pixel cell.

        assert!(rx >= 0.0 && rx < 1.0);
        assert!(ry >= 0.0 && ry < 1.0);

        let rand_sample = (rx - 0.5) * self.viewport_delta_u + (ry - 0.5) * self.viewport_delta_v;

        let pixel_center = self.pixel_start_pos
            + ((w as Fp) * self.viewport_delta_u)
            + ((h as Fp) * self.viewport_delta_v);

        Ray {
            origin: self.postion,
            direction: (pixel_center + rand_sample) - self.postion,
        }
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

    pub fn focal_length(mut self, focal_length: Fp) -> CameraBuilder {
        self.focal_length = focal_length;
        self
    }

    pub fn position(mut self, position: Vec3F) -> CameraBuilder {
        self.position = position;
        self
    }

    pub fn build(self) -> Camera {
        let aspect_ratio = (self.pixel_width as Fp) / (self.pixel_height as Fp);

        let fov_tangent = (self.fov / 2.0 * PI).tan();
        let viewport_height = 2.0 * self.focal_length * fov_tangent;
        let viewport_width = viewport_height * aspect_ratio;

        let viewport_u = Vec3F::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3F::new(0.0, -viewport_height, 0.0);

        let viewport_delta_u = viewport_u / (self.pixel_width as Fp);
        let viewport_delta_v = viewport_v / (self.pixel_height as Fp);

        let viewport_upper_left = self.position + (self.focal_length * Vec3F::new(0.0, 0.0, -1.0))
            - 0.5 * (viewport_u + viewport_v);

        let pixel_start_pos = viewport_upper_left + 0.5 * (viewport_delta_u + viewport_delta_v);

        Camera {
            postion: self.position,
            pixel_start_pos,
            viewport_delta_u,
            viewport_delta_v,
        }
    }
}
