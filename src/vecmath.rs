use crate::types::Fp;
use std::mem::transmute;
use std::ops::{Add, Div, Mul, Sub};

#[repr(C, packed)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Vec3F = Vec3<Fp>;
pub type Color3F = Vec3<Fp>;
pub type Color3U8 = Vec3<u8>;

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3::<T> { x, y, z }
    }
}

impl<T: Copy> Into<[T; 3]> for Vec3<T> {
    fn into(self) -> [T; 3] {
        unsafe {
            // Soundness: because Vec3<T> is packed, its memory layout should be the same as [T; 3].
            *transmute::<&Self, &[T; 3]>(&self)
        }
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vec3::<T>::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vec3::<T>::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, s: T) -> Self::Output {
        Vec3::<T>::new(self.x * s, self.y * s, self.z * s)
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, s: T) -> Self::Output {
        Vec3::<T>::new(self.x / s, self.y / s, self.z / s)
    }
}

impl Mul<Vec3<Fp>> for Fp {
    type Output = Vec3<Fp>;

    fn mul(self, v: Vec3<Fp>) -> Self::Output {
        Vec3::<Fp>::new(self * v.x, self * v.y, self * v.z)
    }
}

pub fn dot<T: Copy>(a: &Vec3<T>, b: &Vec3<T>) -> T
where
    T: Mul<Output = T> + Add<Output = T>,
{
    a.x * b.x + a.y * b.y + a.z * b.z
}

impl From<Vec3F> for Color3U8 {
    fn from(v: Vec3F) -> Self {
        Color3U8::new((v.x * 255.0) as u8, (v.y * 255.0) as u8, (v.z * 255.0) as u8,)
    }
}
