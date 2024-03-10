#[cfg(feature = "use-64bit-float")]
pub type Fp = f64;
#[cfg(not(feature = "use-64bit-float"))]
pub type Fp = f32;
