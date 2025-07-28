use crate::shapes::{Ray, RayIntersection};
use crate::textures::{Texture, TextureChecker, TextureImage, TextureSolidColor};
use crate::types::Fp;
use crate::vecmath::{dot, Color3F, Vec3F, from_local_to_world_space};
use std::path::Path;

#[cfg(not(feature = "use-f64"))]
use std::f32::consts::PI;
#[cfg(feature = "use-f64")]
use std::f64::consts::PI;

pub struct MaterialDiffuse {
    tex: Texture,
}

pub struct MaterialMetal {
    pub albedo: Color3F,
    pub fuzz: Fp,
}

pub struct MaterialDielectric {
    pub refrac_index: Fp,
}

pub struct MaterialDiffuseLight {
    pub albedo: Color3F,
}

pub enum Material {
    Diffuse(MaterialDiffuse),
    Metal(MaterialMetal),
    Dielectric(MaterialDielectric),
    DiffuseLight(MaterialDiffuseLight),
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

    pub fn tex_color(&self, u: Fp, v: Fp, pos: Vec3F) -> Color3F {
        self.tex.value(u, v, pos)
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
}

impl MaterialDielectric {
    pub fn new(refrac_index: Fp) -> Self {
        Self { refrac_index }
    }

    pub fn reflectance(cos_in_angle: Fp, refrac_index: Fp) -> Fp {
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
    pub fn scattering_pdf(&self, surface_normal: &Vec3F, scattered_ray: &Ray) -> Fp {
        match self {
            Material::Diffuse(mat) => mat.scattering_pdf(surface_normal, scattered_ray),
            _ => 0.0,
        }
    }

    pub fn emit(&self) -> Color3F {
        match self {
            Material::DiffuseLight(mat) => mat.emit(0.0, 0.0, Vec3F::zero()),
            _ => Color3F::zero(),
        }
    }
}

