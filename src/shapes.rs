use crate::materials::Material;
use crate::types::Fp;
use crate::vecmath::{dot, Vec3F};
use std::ops::Range;

pub struct Ray {
    pub origin: Vec3F,
    pub direction: Vec3F,
}

#[derive(Copy, Clone, Default)]
pub struct RayIntersection {
    pub hit: bool,
    pub t: Fp,

    /// Whether the normal points outward away from the shape.
    /// This is needed to determine whether to invert the refractive index of a material.
    pub is_normal_outward: bool,

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

    pub fn ray_intersect(&self, ray: &Ray, limits: &Range<Fp>) -> RayIntersection {
        let center_to_origin = ray.origin - self.position;

        // Calculate sphere quadratic coefficients.
        let a = dot(&ray.direction, &ray.direction);
        let half_b = dot(&center_to_origin, &ray.direction);
        let c = center_to_origin.length_squared() - self.radius * self.radius;

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
        let normal = (&hit_point - &self.position) / self.radius;
        let (normal, is_normal_outward) = if dot(&normal, &ray.direction) > 0.0 {
            // Make `normal` point to the opposite direction as `ray`.
            (normal * -1.0, false)
        } else {
            (normal, true)
        };

        RayIntersection {
            hit,
            t,
            hit_point,
            is_normal_outward,
            normal,
        }
    }
}
