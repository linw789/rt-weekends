use crate::types::Fp;
use crate::vecmath::{Color3F, Vec3F};

pub struct TextureSolidColor {
    color: Color3F,
}

pub struct TextureChecker {
    odd_color: Color3F,
    even_color: Color3F,
    inv_scale: Fp,
}

pub enum Texture {
    Solid(TextureSolidColor),
    Checker(TextureChecker),
}

impl TextureSolidColor {
    pub fn new(color: Color3F) -> Self {
        Self { color }
    }

    pub fn value(&self) -> Color3F {
        self.color
    }
}

impl TextureChecker {
    pub fn new(odd_color: Color3F, even_color: Color3F, scale: Fp) -> Self {
        Self {
            odd_color,
            even_color,
            inv_scale: 1.0 / scale,
        }
    }

    pub fn value(&self, pos: Vec3F) -> Color3F {
        let xi = Fp::floor(pos.x * self.inv_scale) as i32;
        let yi = Fp::floor(pos.y * self.inv_scale) as i32;
        let zi = Fp::floor(pos.z * self.inv_scale) as i32;

        // It's easy to visualize in 2D why the formula below forms a checker pattern, but I'm not
        // so sure about 3D.
        let is_even = (xi + yi + zi) % 2 == 0;
        if is_even { self.even_color } else { self.odd_color }
    }
}

impl Texture {
    pub fn value(&self, _u: Fp, _v: Fp, pos: Vec3F) -> Color3F {
        match self {
            Texture::Solid(tex) => tex.value(),
            Texture::Checker(tex) => tex.value(pos),
        }
    }
}
