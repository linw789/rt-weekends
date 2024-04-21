use crate::materials::Material;
use crate::types::Fp;
use crate::vecmath::{dot, Vec3F, Color3F};
use std::ops::Range;

pub struct Ray {
    pub origin: Vec3F,
    pub direction: Vec3F,
}

#[derive(Copy, Clone, Default)]
pub struct RayInterception {
    pub hit: bool,
    pub t: Fp,
    pub hit_point: Vec3F,
    pub normal: Vec3F,
}

pub struct Sphere {
    position: Vec3F,
    radius: Fp,
    pub material: Material,
}

impl Sphere {
    pub fn new(position: Vec3F, radius: Fp, material: Material) -> Sphere {
        Sphere {
            position,
            radius,
            material,
        }
    }

    pub fn ray_intercept(&self, ray: &Ray, limits: &Range<Fp>) -> RayInterception {
        let center_to_origin = ray.origin - self.position;

        // Calculate sphere quadratic coefficients.
        let a = dot(&ray.direction, &ray.direction);
        let half_b = dot(&center_to_origin, &ray.direction);
        let c = dot(&center_to_origin, &center_to_origin) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        let hit: bool;
        let mut t: Fp;

        // Find the closest `t` that's within `limits`.
        if discriminant < 0.0 {
            t = 0.0;
            hit = false;
        } else {
            let discriminant_sqrt = Fp::sqrt(discriminant);
            t = (-half_b - discriminant_sqrt) / a;
            if limits.contains(&t) {
                hit = true;
            } else {
                t = (-half_b + discriminant_sqrt) / a;
                hit = limits.contains(&t);
            }
        }

        let hit_point = ray.origin + (t * ray.direction);
        let normal = (hit_point - self.position) / self.radius;

        RayInterception { hit, t, hit_point, normal, }
    }

    pub fn scatter<R: rand::Rng>(
        &self,
        interception: &RayInterception,
        rand: &mut R,
    ) -> Option<(Ray, Color3F)> {
        self.scatter(interception, rand)
    }
}
