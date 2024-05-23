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
    albedo: Color3F,
    refrac_index: Fp,
}

pub enum Material {
    Diffuse(MaterialDiffuse),
    Metal(MaterialMetal),
    Dielectric(MaterialDielectric),
}

impl MaterialDiffuse {
    pub fn new(albedo: Color3F) -> Self {
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
        let reflected_ray = incident_ray.direction
            - dot(&incident_ray.direction, &intersection.normal) * intersection.normal * 2.0;

        let random_unit_dir = loop {
            let rand_dir = Vec3F::random_fp_range(rand, -1.0..1.0);
            let len_sqr = rand_dir.length_squared();
            if len_sqr <= 1.0 && len_sqr >= 1e-8 {
                break rand_dir / Fp::sqrt(len_sqr);
            }
        };

        let scattered_ray = reflected_ray.normalized() + self.fuzz * random_unit_dir;

        // If the `scattered_ray` points to the opposite direction as `intersection.normal`,
        // discard it (as if the surface absorbs the `incident_ray`).
        if dot(&scattered_ray, &intersection.normal) > 0.0 {
            Some((
                Ray {
                    origin: intersection.hit_point,
                    direction: scattered_ray,
                },
                self.albedo,
            ))
        } else {
            None
        }
    }
}

impl MaterialDielectric {
    pub fn new(albedo: Color3F, refrac_index: Fp) -> Self {
        Self {
            albedo,
            refrac_index,
        }
    }

    pub fn scatter<R: rand::Rng>(
        &self,
        incident_ray: &Ray,
        intersection: &RayIntersection,
        _rand: &mut R,
    ) -> Option<(Ray, Color3F)> {
        let attenuation = Color3F::new(1.0, 1.0, 1.0);
        let refrac_index = if intersection.is_normal_outward {
            self.refrac_index
        } else {
            -self.refrac_index
        };
        let refracted_dir =
            Self::refract(&incident_ray.direction, &intersection.normal, refrac_index);
        Some((
            Ray {
                origin: intersection.hit_point,
                direction: refracted_dir,
            },
            attenuation,
        ))
    }

    /// `rafrac_index_ratio` should be refrac_index_normal / refrac_index_inv_normal where:
    /// refrac_index_normal = refractive index of the surface the normal points to.
    /// refrac_index_inv_normal = refractive index of the surface the normal points away.
    fn refract(incident_dir: &Vec3F, surface_normal: &Vec3F, refrac_index_ratio: Fp) -> Vec3F {
        let refracted_dir_perpendicular = refrac_index_ratio
            * (incident_dir + &(dot(incident_dir, surface_normal) * surface_normal));
        
        let side_len = incident_dir.length_squared() - refracted_dir_perpendicular.length_squared();
        assert!(side_len > 0.0);
        let refracted_dir_parallel = (-1.0 * Fp::sqrt(side_len)) * surface_normal;

        let refracted_dir = refracted_dir_perpendicular + refracted_dir_parallel;

        refracted_dir
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
