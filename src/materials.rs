use crate::shapes::{Ray, RayIntersection};
use crate::types::Fp;
use crate::vecmath::{dot, Color3F, Vec3F};

pub struct MaterialDiffuse {
    albedo: Color3F,
}

pub struct MaterialMetal {
    albedo: Color3F,
    fuzz: Fp,
}

pub struct MaterialDielectric {
    refrac_index: Fp,
}

pub enum Material {
    Diffuse(MaterialDiffuse),
    Metal(MaterialMetal),
    Dielectric(MaterialDielectric),
}

impl MaterialDiffuse {
    pub fn new(albedo: Color3F) -> MaterialDiffuse {
        Self { albedo }
    }

    pub fn scatter<R: rand::Rng>(
        &self,
        intersection: &RayIntersection,
        rand: &mut R,
    ) -> Option<(Ray, Color3F)> {
        // Pick a random point on a unit sphere.
        let (rand_point, len_sqr) = loop {
            let random_ray = Vec3F::random_fp_range(rand, -1.0..1.0);
            let len_sqr = random_ray.length_squared();
            if len_sqr <= 1.0 {
                break (random_ray, len_sqr);
            }
        };
        let rand_point = if len_sqr > 0.0 {
            rand_point / Fp::sqrt(len_sqr)
        } else {
            rand_point
        };

        // unit_sphere_center = hit_point + normal
        // rand_sphere_point = rand_point + unit_sphere_center = rand_point + hit_point + normal
        // scattered_ray = rand_sphere_point - hit_point = (rand_point + hit_point + normal) - hit_point = rand_point + normal

        // Because `normal` always points to the opposite direction as the original intersecting
        // ray, so does the `scattered_ray`.
        let scattered_ray = rand_point + intersection.normal;
        let scattered_ray = if scattered_ray.approx_zero() {
            intersection.normal
        } else {
            scattered_ray
        };

        Some((
            Ray {
                origin: intersection.hit_point,
                direction: scattered_ray,
            },
            self.albedo,
        ))
    }
}

impl MaterialMetal {
    pub fn new(albedo: Color3F) -> MaterialMetal {
        Self { albedo, fuzz: 0.0 }
    }

    pub fn scatter<R: rand::Rng>(
        &self,
        original_ray: &Ray,
        intersection: &RayIntersection,
        rand: &mut R,
    ) -> Option<(Ray, Color3F)> {
        let scattered_ray = original_ray.direction
            - dot(&original_ray.direction, &intersection.normal) * intersection.normal * 2.0;

        Some((
            Ray {
                origin: intersection.hit_point,
                direction: scattered_ray,
            },
            self.albedo,
        ))
    }
}

impl MaterialDielectric {
    pub fn scatter<R: rand::Rng>(
        &self,
        intersection: &RayIntersection,
        rand: &mut R,
    ) -> Option<(Ray, Color3F)> {
        None
    }
}

impl Material {
    pub fn scatter<R: rand::Rng>(
        &self,
        original_ray: &Ray,
        intersection: &RayIntersection,
        rand: &mut R,
    ) -> Option<(Ray, Color3F)> {
        match self {
            Material::Diffuse(mat) => mat.scatter(intersection, rand),
            Material::Metal(mat) => mat.scatter(original_ray, intersection, rand),
            Material::Dielectric(mat) => mat.scatter(intersection, rand),
        }
    }
}
