use crate::types::Fp as Fp;
use crate::vecmath::Vec3F;

pub struct Sphere {
    position: Vec3F,
    size: Fp,
}

impl Sphere {
    pub fn new(pos: Vec3F, size: Fp) -> Sphere {
        Sphere {
            position: pos,
            size: size,
        }
    }
}
