use crate::materials::Material;
use crate::types::Fp;
use crate::vecmath::{dot, Vec3F};

pub struct Ray {
    pub origin: Vec3F,
    pub direction: Vec3F,
}

pub struct RayInterception {
    pub hit: bool,
    t: Fp,
    normal: Vec3F,
    is_front_face: bool,
}

pub struct Sphere {
    position: Vec3F,
    radius: Fp,
    material: Material,
}

impl Sphere {
    pub fn new(position: Vec3F, radius: Fp, material: Material) -> Sphere {
        Sphere {
            position,
            radius,
            material,
        }
    }

    pub fn ray_intercept(&self, ray: &Ray) -> RayInterception {
        let center_to_origin = ray.origin - self.position;

        // Calculate sphere quadratic coefficients.
        let a = dot(&ray.direction, &ray.direction);
        let half_b = dot(&center_to_origin, &ray.direction);
        let c = dot(&center_to_origin, &center_to_origin) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        // let discriminant_sqrt = Fp::sqrt(discriminant);
        let t = 0;

        RayInterception {
            hit: (discriminant > 0.0),
            t: 0.0,
            normal: Vec3F::new(0.0, 0.0, 0.0),
            is_front_face: false,
        }
    }
}
