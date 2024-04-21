use crate::shapes::{Ray, RayInterception};
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
        interception: &RayInterception,
        rand: &mut R,
    ) -> Option<(Ray, Color3F)> {
        let scattered_ray = loop {
            let random_ray = Vec3F::random_fp_range(rand, -1.0..1.0 as Fp);
            if random_ray.length_squared() < 1.0 {
                break random_ray;
            }
        };

        let scattered_ray = scattered_ray.normalized();
        let scattered_ray = if dot(&scattered_ray, &interception.normal) < 0.0 {
            scattered_ray * -1.0
        } else {
            scattered_ray
        };

        Some((
            Ray {
                origin: interception.hit_point,
                direction: scattered_ray,
            },
            self.albedo,
        ))
    }
}

impl MaterialMetal {
    pub fn scatter<R: rand::Rng>(
        &self,
        interception: &RayInterception,
        rand: &mut R,
    ) -> Option<(Ray, Color3F)> {
        None
    }
}

impl MaterialDielectric {
    pub fn scatter<R: rand::Rng>(
        &self,
        interception: &RayInterception,
        rand: &mut R,
    ) -> Option<(Ray, Color3F)> {
        None
    }
}

impl Material {
    pub fn scatter<R: rand::Rng>(
        &self,
        interception: &RayInterception,
        rand: &mut R,
    ) -> Option<(Ray, Color3F)> {
        match self {
            Material::Diffuse(mat) => mat.scatter(interception, rand),
            Material::Metal(mat) => mat.scatter(interception, rand),
            Material::Dielectric(mat) => mat.scatter(interception, rand),
        }
    }    
}
