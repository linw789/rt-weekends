use crate::types::Fp;
use crate::vecmath::{Color3F, Vec3F};
use crate::Image;
use std::path::Path;

pub struct TextureSolidColor {
    color: Color3F,
}

pub struct TextureChecker {
    odd_color: Color3F,
    even_color: Color3F,
    inv_scale: Fp,
}

pub struct TextureImage {
    image: Image,
}

pub enum Texture {
    Solid(TextureSolidColor),
    Checker(TextureChecker),
    Image(TextureImage),
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

impl TextureImage {
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Self {
        Self {
            image: Image::from_file(file_path),
        }
    }

    pub fn value(&self, u: Fp, v: Fp) -> Color3F {
        if self.image.width == 0 || self.image.height == 0 {
            // Return solid cyan as debugging aid.
            return Color3F::new(0.0, 1.0, 1.0);
        }

        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // Flip to image coordinate.

        self.image.pixel_at_uv(u, v)
    }
}

impl Texture {
    pub fn value(&self, u: Fp, v: Fp, pos: Vec3F) -> Color3F {
        match self {
            Texture::Solid(tex) => tex.value(),
            Texture::Checker(tex) => tex.value(pos),
            Texture::Image(tex) => tex.value(u, v),
        }
    }
}
