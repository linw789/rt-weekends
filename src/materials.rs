use crate::shapes::{Ray, RayIntersection};
use crate::types::Fp;
use crate::vecmath::{dot, Color3F, Vec3F};
use crate::textures::{Texture, TextureSolidColor, TextureChecker, TextureImage};
use std::path::Path;

pub struct MaterialDiffuse {
    tex: Texture,
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

fn reflect(in_dir: &Vec3F, normal: &Vec3F) -> Vec3F {
    in_dir - 2.0 * dot(in_dir, normal) * normal
}

/// `refrac_index` should be in_refrac_index / out_refrac_index where:
/// in_refrac_index = the refractive index of the surface of the incident ray
/// out_refrac_index = the refractive index of the surface of the outgoing ray
fn refract(in_dir: &Vec3F, normal: &Vec3F, refrac_index: Fp) -> Vec3F {
    let refrac_dir_perp = refrac_index * (in_dir - (dot(in_dir, normal) * normal));
    let side_len = in_dir.length_squared() - refrac_dir_perp.length_squared();
    let refrac_dir_parallel = -1.0 * Fp::sqrt(Fp::abs(side_len)) * normal;
    refrac_dir_perp + refrac_dir_parallel
}

impl MaterialDiffuse {
    pub fn new_solid_color(albedo: Color3F) -> Self {
        Self { 
            tex: Texture::Solid(TextureSolidColor::new(albedo))
        }
    }

    pub fn new_checker(even: Color3F, odd: Color3F, scale: Fp) -> Self {
        Self {
            tex: Texture::Checker(TextureChecker::new(even, odd, scale))
        }
    }

    pub fn from_image<P: AsRef<Path>>(image_path: P) -> Self {
        Self {
            tex: Texture::Image(TextureImage::from_file(image_path))
        }
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
            Ray::new(intersection.hit_point, scattered_ray),
            self.tex.value(intersection.u, intersection.v, intersection.hit_point),
        ))
    }
}

impl MaterialMetal {
    pub fn new(albedo: Color3F, fuzz: Fp) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < -1.0 {
                -1.0
            } else if fuzz > 1.0 {
                1.0
            } else {
                fuzz
            },
        }
    }

    pub fn scatter<R: rand::Rng>(
        &self,
        incident_ray: &Ray,
        intersection: &RayIntersection,
        rand: &mut R,
    ) -> Option<(Ray, Color3F)> {
        let reflected_dir = reflect(&incident_ray.direction, &intersection.normal);

        let random_unit_dir = loop {
            let rand_dir = Vec3F::random_fp_range(rand, -1.0..1.0);
            let len_sqr = rand_dir.length_squared();
            if len_sqr <= 1.0 && len_sqr >= 1e-8 {
                break rand_dir / Fp::sqrt(len_sqr);
            }
        };

        let scattered_ray = reflected_dir.normalized() + self.fuzz * random_unit_dir;

        // If the `scattered_ray` points to the opposite direction as `intersection.normal`,
        // discard it (as if the surface absorbs the `incident_ray`).
        if dot(&scattered_ray, &intersection.normal) > 0.0 {
            Some((
                Ray::new(intersection.hit_point, scattered_ray),
                self.albedo,
            ))
        } else {
            None
        }
    }
}

impl MaterialDielectric {
    pub fn new(refrac_index: Fp) -> Self {
        Self { refrac_index }
    }

    pub fn scatter<R: rand::Rng>(
        &self,
        incident_ray: &Ray,
        intersection: &RayIntersection,
        rand: &mut R,
    ) -> Option<(Ray, Color3F)> {
        let attenuation = Color3F::new(1.0, 1.0, 1.0);

        let refrac_index = if intersection.is_normal_outward {
            1.0 / self.refrac_index
        } else {
            self.refrac_index
        };

        let in_dir_normalized = incident_ray.direction.normalized();
        let cos_in_angle = Fp::min(dot(&in_dir_normalized, &(-intersection.normal)), 1.0);
        let sin_in_angle = Fp::sqrt(1.0 - cos_in_angle * cos_in_angle);

        let no_refract = (refrac_index * sin_in_angle) > 1.0;
        let no_refract = no_refract
            || (Self::reflectance(cos_in_angle, refrac_index) > rand.gen_range(0.0..1.0));
        let out_dir = if no_refract {
            reflect(&incident_ray.direction, &intersection.normal)
        } else {
            refract(&incident_ray.direction, &intersection.normal, refrac_index)
        };

        Some((
            Ray::new(intersection.hit_point, out_dir),
            attenuation,
        ))
    }

    fn reflectance(cos_in_angle: Fp, refrac_index: Fp) -> Fp {
        // Schlick's approximation for reflectance.
        let r0 = (1.0 - refrac_index) / (1.0 + refrac_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * Fp::powi(1.0 - cos_in_angle, 5)
    }
}

impl Material {
    pub fn scatter<R: rand::Rng>(
        &self,
        incident_ray: &Ray,
        intersection: &RayIntersection,
        rand: &mut R,
    ) -> Option<(Ray, Color3F)> {
        match self {
            Material::Diffuse(mat) => mat.scatter(intersection, rand),
            Material::Metal(mat) => mat.scatter(incident_ray, intersection, rand),
            Material::Dielectric(mat) => mat.scatter(incident_ray, intersection, rand),
        }
    }
}
