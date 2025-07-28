use crate::shapes::{Ray, RayIntersection};
use crate::textures::{Texture, TextureChecker, TextureImage, TextureSolidColor};
use crate::types::Fp;
use crate::vecmath::{cross, dot, Color3F, Vec3F};
use std::path::Path;

#[cfg(not(feature = "use-f64"))]
use std::f32::consts::PI;
#[cfg(feature = "use-f64")]
use std::f64::consts::PI;

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

pub struct MaterialDiffuseLight {
    albedo: Color3F,
}

pub enum Material {
    Diffuse(MaterialDiffuse),
    Metal(MaterialMetal),
    Dielectric(MaterialDielectric),
    DiffuseLight(MaterialDiffuseLight),
}

pub struct ScatteredResult {
    pub ray: Ray,
    pub albedo: Color3F,
    pub probability: Fp,
}

fn reflect(in_dir: &Vec3F, normal: &Vec3F) -> Vec3F {
    in_dir - 2.0 * dot(in_dir, normal) * normal
}

/// `refrac_index` should be `in_refrac_index / out_refrac_index` where:
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
            tex: Texture::Solid(TextureSolidColor::new(albedo)),
        }
    }

    pub fn new_checker(even: Color3F, odd: Color3F, scale: Fp) -> Self {
        Self {
            tex: Texture::Checker(TextureChecker::new(even, odd, scale)),
        }
    }

    pub fn from_image<P: AsRef<Path>>(image_path: P) -> Self {
        Self {
            tex: Texture::Image(TextureImage::from_file(image_path)),
        }
    }

    pub fn scatter<R: rand::Rng>(
        &self,
        intersection: &RayIntersection,
        rand: &mut R,
    ) -> Option<ScatteredResult> {
        let pdf = PdfCosineHemisphere::new();
        let sample = pdf.gen_sample(rand);

        let scattered_ray = from_local_to_world_space(&intersection.normal, &sample.dir);

        Some(ScatteredResult {
            ray: Ray::new(intersection.hit_point, scattered_ray),
            albedo: self
                .tex
                .value(intersection.u, intersection.v, intersection.hit_point),
            probability: sample.probability,
        })
    }

    pub fn scattering_pdf(&self, surface_normal: &Vec3F, scattered_ray: &Ray) -> Fp {
        let cos_theta = dot(surface_normal, &scattered_ray.direction.normalized());
        if cos_theta < 0.0 {
            0.0
        } else {
            cos_theta / PI
        }
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
    ) -> Option<ScatteredResult> {
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
            Some(ScatteredResult {
                ray: Ray::new(intersection.hit_point, scattered_ray),
                albedo: self.albedo,
                probability: 1.0,
            })
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
    ) -> Option<ScatteredResult> {
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

        Some(ScatteredResult {
            ray: Ray::new(intersection.hit_point, out_dir),
            albedo: attenuation,
            probability: 1.0,
        })
    }

    fn reflectance(cos_in_angle: Fp, refrac_index: Fp) -> Fp {
        // Schlick's approximation for reflectance.
        let r0 = (1.0 - refrac_index) / (1.0 + refrac_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * Fp::powi(1.0 - cos_in_angle, 5)
    }
}

impl MaterialDiffuseLight {
    pub fn new(albedo: Color3F) -> Self {
        Self { albedo }
    }

    pub fn emit(&self, _u: Fp, _v: Fp, _p: Vec3F) -> Color3F {
        self.albedo
    }
}

impl Material {
    pub fn scatter<R: rand::Rng>(
        &self,
        incident_ray: &Ray,
        intersection: &RayIntersection,
        rand: &mut R,
    ) -> Option<ScatteredResult> {
        match self {
            Material::Diffuse(mat) => mat.scatter(intersection, rand),
            Material::Metal(mat) => mat.scatter(incident_ray, intersection, rand),
            Material::Dielectric(mat) => mat.scatter(incident_ray, intersection, rand),
            _ => None,
        }
    }

    pub fn scattering_pdf(&self, surface_normal: &Vec3F, scattered_ray: &Ray) -> Fp {
        match self {
            Material::Diffuse(mat) => mat.scattering_pdf(surface_normal, scattered_ray),
            _ => 1.0,
        }
    }

    pub fn emit(&self) -> Color3F {
        match self {
            Material::DiffuseLight(mat) => mat.emit(0.0, 0.0, Vec3F::zero()),
            _ => Color3F::zero(),
        }
    }
}

struct PdfSample {
    dir: Vec3F,
    probability: Fp,
}

// p(a, b) = cos(a)/PI, a is polar angle, b is azimuthal angle.
struct PdfCosineHemisphere {}

impl PdfCosineHemisphere {
    pub fn new() -> Self {
        Self {}
    }

    pub fn gen_sample<R: rand::Rng>(&self, rand: &mut R) -> PdfSample {
        // The section 7.1 in the third book shows the CDF^-1 that takes two uniform
        // random numbers r1, r2, to generate another two random spherical coordinates
        // a, b with the distribution p(a, b) = f(a), where polar angle is the only
        // input variable.
        //
        // Then the section 7.3 shows the equations to generate a, b with the distribution
        // p(a, b) = cos(a)/PI.

        let r1 = rand.gen_range(0.0..1.0);
        let r2 = rand.gen_range(0.0..1.0);

        let phi = 2.0 * PI * r1;
        let x = phi.cos() * Fp::sqrt(r2);
        let y = phi.sin() * Fp::sqrt(r2);
        let z = Fp::sqrt(1.0 - r2); // z == cos(a)

        let probability = z / PI;

        PdfSample {
            dir: Vec3F::new(x, y, z),
            probability,
        }
    }
}

// Transform a vector in local space (constructed from another vector in world
// space) to the world space.
pub fn from_local_to_world_space(n: &Vec3F, local_v: &Vec3F) -> Vec3F {
    // Construct the local space basis based `n`.

    let local_axis_z = n.normalized();
    // Check if world x-axis is almost parallel with n.
    let tmp = if local_axis_z.x.abs() > 0.9 {
        Vec3F::new(0.0, 1.0, 0.0)
    } else {
        Vec3F::new(1.0, 0.0, 0.0)
    };
    let local_axis_y = cross(&local_axis_z, &tmp).normalized();
    let local_axis_x = cross(&local_axis_z, &local_axis_y);

    // express local_v in the world space
    (local_v.x * local_axis_x) + (local_v.y * local_axis_y) + (local_v.z * local_axis_z)
}
