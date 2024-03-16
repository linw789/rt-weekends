use crate::types::Fp;
use crate::vecmath::Vec3F;

struct Camera {
    fov: Fp, // vertical field of view
    focal_lenght: Fp,

    viewport_width: u32,
    viewport_height: u32,

    viewport_u: Vec3F,
    viewport_v: Vec3F,
}
