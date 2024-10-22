#[cfg(feature = "use-f64")]
pub type Fp = f64;
#[cfg(not(feature = "use-f64"))]
pub type Fp = f32;
