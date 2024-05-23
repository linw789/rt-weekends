extern crate num_traits;

use crate::types::Fp;
use num_traits::identities::Zero;
use std::mem::transmute;
use std::ops::{Add, AddAssign, Div, Mul, Range, Sub};

#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Default)]
pub struct Vec3<T: Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Vec3F = Vec3<Fp>;
pub type Color3F = Vec3<Fp>;
pub type Color3U8 = Vec3<u8>;

impl<T: Copy> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: Copy + Mul<Output = T> + Add<Output = T>> Vec3<T> {
    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl<T: Copy + Zero> Vec3<T> {
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }
}

impl Vec3<Fp> {
    pub fn random_fp_range<R: rand::Rng>(rand: &mut R, range: Range<Fp>) -> Self {
        Self {
            x: rand.gen_range(range.clone()),
            y: rand.gen_range(range.clone()),
            z: rand.gen_range(range.clone()),
        }
    }

    pub fn length(&self) -> Fp {
        self.length_squared().sqrt()
    }

    pub fn normalized(&self) -> Self {
        *self / self.length()
    }

    pub fn approx_zero(&self) -> bool {
        let threshold = 1e-8;

        (self.x > -threshold && self.x < threshold)
            && (self.y > -threshold && self.y < threshold)
            && (self.z > -threshold && self.z < threshold)
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

impl<T: Copy + Add<Output = T>> Add for Vec3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vec3::<T>::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T: Copy + Add<Output = T>> Add for &Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, other: Self) -> Self::Output {
        Vec3::<T>::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T: Copy + Add<Output = T>> AddAssign for Vec3<T> {
    fn add_assign(&mut self, other: Vec3<T>) {
        *self = other + *self;
    }
}

impl<T: Copy + Sub<Output = T>> Sub for &Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, other: Self) -> Self::Output {
        Vec3::<T>::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, other: Self) -> Self::Output {
        Vec3::<T>::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, s: T) -> Self::Output {
        Vec3::<T>::new(self.x * s, self.y * s, self.z * s)
    }
}

impl<T: Copy + Mul<Output = T>> Mul<Vec3<T>> for Vec3<T> {
    type Output = Self;

    fn mul(self, other: Vec3<T>) -> Self::Output {
        Vec3::<T>::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, s: T) -> Self::Output {
        Vec3::<T>::new(self.x / s, self.y / s, self.z / s)
    }
}

impl Mul<&Vec3<Fp>> for Fp {
    type Output = Vec3<Fp>;

    fn mul(self, v: &Vec3<Fp>) -> Self::Output {
        Vec3::<Fp>::new(self * v.x, self * v.y, self * v.z)
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
        Color3U8::new(
            (v.x * 255.0) as u8,
            (v.y * 255.0) as u8,
            (v.z * 255.0) as u8,
        )
    }
}
