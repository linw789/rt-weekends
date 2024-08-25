use crate::materials::Material;
use crate::types::Fp;
use crate::vecmath::{dot, Vec3F};
use std::ops::Range;

pub struct Ray {
    pub origin: Vec3F,
    pub direction: Vec3F,
    // pub inv_dir: Vec3F,
}

impl Ray {
    pub fn new(origin: Vec3F, dir: Vec3F) -> Self {
        Self {
            origin,
            direction: dir,
            // inv_dir: Vec3F::new(1.0 / dir.x, 1.0 / dir.y, 1.0 / dir.z),
        }
    }
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
    pub position: Vec3F,
    pub radius: Fp,
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

#[derive(Copy, Clone)]
pub struct Aabb {
    min: Vec3F,
    max: Vec3F,
}

impl Aabb {
    pub fn new(min: Vec3F, max: Vec3F) -> Self {
        Self {
            min,
            max,
        }
    }

    pub fn from_sphere(s: &Sphere) -> Self {
        let extent = Vec3F::new(s.radius, s.radius, s.radius);
        Self {
            min: s.position - extent,
            max: s.position + extent,
        }
    }

    pub fn merge(a: &Aabb, b: &Aabb) -> Self {
        Self {
            min: Vec3F::new(Fp::min(a.min.x, b.min.x), Fp::min(a.min.y, b.min.y), Fp::min(a.min.z, b.min.z)),
            max: Vec3F::new(Fp::max(a.max.x, b.max.x), Fp::max(a.max.y, b.max.y), Fp::max(a.max.z, b.max.z)),
        }
    }

    pub fn ray_intersect(&self, ray: &Ray) -> bool {
        // TODO: how do we handle divide-by-zero?

        let yt0 = (self.min.x - ray.origin.y) / ray.direction.y;
        let yt1 = (self.max.x - ray.origin.y) / ray.direction.y;

        let zt0 = (self.min.y - ray.origin.z) / ray.direction.z;
        let zt1 = (self.max.y - ray.origin.z) / ray.direction.z;

        let xt0 = (self.min.z - ray.origin.x) / ray.direction.x;
        let xt1 = (self.max.z - ray.origin.x) / ray.direction.x;

        let mut t_min = Fp::NEG_INFINITY;
        let mut t_max = Fp::INFINITY;

        // Intersection exists only if all three segments overlap. I can intuitively, visually understand
        // this in 2D, but I'm not sure about this in 3D.

        t_min = Fp::max(t_min, Fp::min(xt0, xt1));
        t_max = Fp::min(t_max, Fp::max(xt0, xt1));

        t_min = Fp::max(t_min, Fp::min(yt0, yt1));
        t_max = Fp::min(t_max, Fp::max(yt0, yt1));

        t_min = Fp::max(t_min, Fp::min(zt0, zt1));
        t_max = Fp::min(t_max, Fp::max(zt0, zt1));

        t_min < t_max
    }
}
