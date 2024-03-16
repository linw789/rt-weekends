use crate::types::Fp;
use crate::vecmath::Color3F;

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

    pub fn scatter() -> Color3F {
        Color3F::new(0.0, 0.0, 0.0)
    }
}
