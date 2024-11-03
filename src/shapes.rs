use crate::materials::Material;
use crate::types::Fp;
use crate::vecmath::{cross, dot, Vec3F};
use std::ops::Range;

#[cfg(not(feature = "use-f64"))]
use std::f32::consts::PI;
#[cfg(feature = "use-f64")]
use std::f64::consts::PI;

pub struct Ray {
    pub origin: Vec3F,
    pub direction: Vec3F,
    pub inv_dir: Vec3F,
    pub signs: [u8; 3],
}

#[derive(Copy, Clone, Default)]
pub struct RayIntersection {
    pub hit: bool,
    pub t: Fp,

    /// Whether the normal points outward away from the shape.
    /// This is needed to determine whether to invert the refractive index of a material.
    pub is_normal_outward: bool,

    pub hit_point: Vec3F,
    /// normal always points the opposite direction as the ray.
    pub normal: Vec3F,

    pub u: Fp,
    pub v: Fp,
}

pub struct Sphere {
    pub position: Vec3F,
    pub radius: Fp,
    pub material: Material,
}

pub struct Quad {
    pub corner: Vec3F,
    pub edges: [Vec3F; 2],
    pub normal: Vec3F,
    pub d: Fp,
}

#[derive(Copy, Clone)]
pub struct Aabb {
    bounds: [Vec3F; 2], // [min, max]
}

impl Ray {
    pub fn new(origin: Vec3F, dir: Vec3F) -> Self {
        let inv_dir = Vec3F::new(1.0 / dir.x, 1.0 / dir.y, 1.0 / dir.z); 
        Self {
            origin,
            direction: dir,
            inv_dir, 
            signs: [
                if inv_dir.x < 0.0 { 1 } else { 0 },
                if inv_dir.y < 0.0 { 1 } else { 0 },
                if inv_dir.z < 0.0 { 1 } else { 0 },
            ],
        }
    }
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

        let (u, v) = self.get_sphere_uv(&normal);

        RayIntersection {
            hit,
            t,
            hit_point,
            is_normal_outward,
            normal,
            u,
            v,
        }
    }

    // `unit_sphere_p` is the 3D position of a point on a unit sphere.
    fn get_sphere_uv(&self, unit_sphere_p: &Vec3F) -> (Fp, Fp) {
        // Imagine a cylinder whose radius is this sphere's radiu and whose height is 2 * radius.
        // The goal is to map a point's latitude (from -Y to +Y) and longitude (from -X to +X and
        // back to -X) on the sphere to the Y and X axis on the unfolded cylinder. 3Blue1Brown has a
        // good video explaining this: https://www.youtube.com/watch?v=GNcFjFmqEc8

        let phi = (-unit_sphere_p.z).atan2(unit_sphere_p.x) + PI; // longitude
        let theta = (-unit_sphere_p.y).acos(); // latitude
        (phi / (2.0 * PI), theta / PI)
    }
}

impl Quad {
    pub fn new(corner: Vec3F, edge0: Vec3F, edge1: Vec3F) -> Self {
        let normal = cross(&edge0, &edge1).normalized();
        let d = dot(&normal, &corner);
        Self {
            corner,
            edges: [edge0, edge1],
            normal,
            d,
        }
    }

    pub fn other_corner(&self) -> Vec3F {
        self.corner + self.edges[0] + self.edges[1]
    }

    pub fn ray_intersect(&self, ray: &Ray, limits: &Range<Fp>) -> RayIntersection {
        let mut intersection = RayIntersection {
            hit: false,
            ..Default::default()
        };

        let denominator = dot(&self.normal, &ray.direction);
        if Fp::abs(denominator) > 1e-8 {
            let t = (self.d - dot(&self.normal, &ray.origin)) / denominator;
            if limits.contains(&t) {
                intersection = RayIntersection {
                    hit: true,
                    t,
                    is_normal_outward: false,
                    hit_point: ray.origin + (t * ray.direction),
                    normal: self.normal,
                    u: 0.0,
                    v: 0.0,
                }
            } 
        }

        intersection
    }
}

impl Aabb {
    pub fn from_sphere(s: &Sphere) -> Self {
        let extent = Vec3F::new(s.radius, s.radius, s.radius);
        Self {
            bounds: [s.position - extent, s.position + extent],
        }
    }

    pub fn from_quad(q: &Quad) -> Self {
        let p0 = q.corner;
        let p1 = q.other_corner();
        let mut bounds = [
            Vec3F::new(
                Fp::min(p0.x, p1.x),
                Fp::min(p0.y, p1.y),
                Fp::min(p0.z, p1.z)),
            Vec3F::new(
                Fp::max(p0.x, p1.x),
                Fp::max(p0.y, p1.y),
                Fp::max(p0.z, p1.z)),
        ];

        // pad the AABB if any side is too narrow.
        let delta = 0.0001;
        if (bounds[1].x - bounds[0].x) < delta {
            bounds[0].x -= delta / 2.0;
            bounds[1].x += delta / 2.0;
        }
        if (bounds[1].y - bounds[0].y) < delta {
            bounds[0].y -= delta / 2.0;
            bounds[1].y += delta / 2.0;
        }
        if (bounds[1].z - bounds[0].z) < delta {
            bounds[0].z -= delta / 2.0;
            bounds[1].z += delta / 2.0;
        }
        Self { bounds }
    }

    pub fn merge(a: &Aabb, b: &Aabb) -> Self {
        const MIN: usize = 0;
        const MAX: usize = 1;

        Self {
            bounds: [
                Vec3F::new(
                    Fp::min(a.bounds[MIN].x, b.bounds[MIN].x),
                    Fp::min(a.bounds[MIN].y, b.bounds[MIN].y),
                    Fp::min(a.bounds[MIN].z, b.bounds[MIN].z)),
                Vec3F::new(
                    Fp::max(a.bounds[MAX].x, b.bounds[MAX].x),
                    Fp::max(a.bounds[MAX].y, b.bounds[MAX].y),
                    Fp::max(a.bounds[MAX].z, b.bounds[MAX].z)),
            ]
        }
    }

    pub fn ray_intersect(&self, ray: &Ray) -> bool {
        // Intersection exists only if all three segments overlap. I can intuitively, visually understand
        // this in 2D, but I'm not sure about this in 3D.
        //
        // https://people.csail.mit.edu/amy/papers/box-jgt.pdf (An Efficient and Robust Ray-Box Intersection Algorithm)
        // Note, this paper doesn't address a degenerate case where the origin of the ray lies on
        // one of the planes of the AABB. This post mentions a way to handle the issue:
        // https://tavianator.com/2015/ray_box_nan.html

        debug_assert!((self.bounds[0].x < self.bounds[1].x) &&
                      (self.bounds[0].y < self.bounds[1].y) &&
                      (self.bounds[0].z < self.bounds[1].z));

        const AXIS_X: usize = 0;
        const AXIS_Y: usize = 1;
        const AXIS_Z: usize = 2;

        let mut tmin = (self.bounds[ray.signs[AXIS_X] as usize].x - ray.origin.x) * ray.inv_dir.x; 
        let mut tmax = (self.bounds[1 - ray.signs[AXIS_X] as usize].x - ray.origin.x) * ray.inv_dir.x; 

        let ty_min = (self.bounds[ray.signs[AXIS_Y] as usize].y - ray.origin.y) * ray.inv_dir.y; 
        let ty_max = (self.bounds[1 - ray.signs[AXIS_Y] as usize].y - ray.origin.y) * ray.inv_dir.y; 

        let tz_min = (self.bounds[ray.signs[AXIS_Z] as usize].z - ray.origin.z) * ray.inv_dir.z; 
        let tz_max = (self.bounds[1 - ray.signs[AXIS_Z] as usize].z - ray.origin.z) * ray.inv_dir.z; 

        // If either tmin or ty_min is NaN, Fp::max returns the non-NaN value. tmin and ty_min
        // can't be both NaN, because we assert that AABB can't be degenerate. Specifically,
        //
        // if: ray.inv_dir.x == INFINITY and tmin == NaN,
        // then: tmax == INFINITY must hold
        // then: Fp::max(tmin, ty_min) -> ty_min, and Fp::min(tmax, ty_max) -> ty_max.
        //
        // if: ray.inv_dir.x == INFINITY and tmax == NaN,
        // then: tmin must == NEG_INFINITY must hold
        // then: Fp::max(tmin, ty_min) -> ty_min, and Fp::min(tmax, ty_max) -> ty_max.
        tmin = Fp::max(tmin, ty_min);
        tmax = Fp::min(tmax, ty_max);
        
        tmin = Fp::max(tmin, tz_min);
        tmax = Fp::min(tmax, tz_max);

        tmin < tmax
    }
}
