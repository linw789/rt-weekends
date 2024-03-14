use crate::types::Fp as Fp;
use crate::vecmath::{Vec3F, dot};

pub struct Sphere {
    position: Vec3F,
    radius: Fp,
}

pub struct Ray {
    origin: Vec3F,
    direction: Vec3F,
}

pub struct RayInterception {
    hit: bool,
    t: Fp,
    normal: Vec3F,
    is_front_face: bool,
}

impl Sphere {
    pub fn new(pos: Vec3F, radius: Fp) -> Sphere {
        Sphere {
            position: pos,
            radius: radius,
        }
    }

    pub fn ray_intercept(&self, ray: &Ray) -> RayInterception {
        // TODO: is it ray-to-sphere or sphere-to-ray?
        let ray_to_sphere = ray.origin - self.position;

        // Calculate sphere quadratic coefficients.
        let a = dot(&ray.direction, &ray.direction);
        let half_b = dot(&ray_to_sphere, &ray.direction);
        let c = dot(&ray_to_sphere, &ray_to_sphere) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        let sqrt_discriminant = Fp::sqrt(discriminant);
        let t = 0;

        RayInterception {
            hit: false,
            t: 0.0,
            normal: Vec3F::new(0.0, 0.0, 0.0),
            is_front_face: false,
        }
    }
}
