extern crate num_traits;

use crate::types::Fp;
use num_traits::identities::Zero;
use std::mem::transmute;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Range, Sub};

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
            // Soundness: because Vec3<T> is packed, its memory layout should be the same
            // as [T; 3].
            *transmute::<&Self, &[T; 3]>(&self)
        }
    }
}

impl<T: Copy> From<[T; 3]> for Vec3<T> {
    fn from(a: [T; 3]) -> Self {
        unsafe {
            // Soundness: because Vec3<T> is packed, its memory layout should be the same
            // as [T; 3].
            *transmute::<&[T; 3], &Self>(&a)
        }
    }
}

impl From<Color3U8> for Color3F {
    fn from(v: Color3U8) -> Self {
        let scale = 1.0 / 255.0;
        Color3F::new((v.x as Fp) * scale, (v.y as Fp) * scale, (v.z as Fp) * scale,)
    }
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

// Implements binary operator `T op &U`, `&T op U` and `&T op &U` based on `T op U`
// where `T` implements `Copy`.
// reference: Rust core library interanl_macro.rs
macro_rules! forward_ref_binop {
    (impl $operator:ident, $method:ident for $t:ident, $u:ty) => {
        impl<T> $operator<&$u> for $t<T>
        where
            T: Copy + $operator<Output = T>,
        {
            type Output = $t<T>;

            #[inline]
            #[track_caller]
            fn $method(self, other: &$u) -> Self::Output {
                $operator::$method(self, *other)
            }
        }

        impl<'a, T> $operator<$u> for &'a $t<T>
        where
            T: Copy + $operator<Output = T>,
        {
            type Output = $t<T>;

            #[inline]
            #[track_caller]
            fn $method(self, other: $u) -> Self::Output {
                $operator::$method(*self, other)
            }
        }

        impl<'a, T> $operator<&$u> for &'a $t<T>
        where
            T: Copy + $operator<Output = T>,
        {
            type Output = $t<T>;

            #[inline]
            #[track_caller]
            fn $method(self, other: &$u) -> Self::Output {
                $operator::$method(*self, other)
            }
        }
    };
}

macro_rules! vec3_impl_add {
    () => {
        impl<T> Add for Vec3<T>
        where
            T: Copy + Add<Output = T>
        {
            type Output = Vec3<T>;

            #[inline]
            #[track_caller]
            fn add(self, other: Vec3<T>) -> Self::Output {
                Vec3::<T>::new(self.x + other.x, self.y + other.y, self.z + other.z)
            }
        }

        forward_ref_binop!(impl Add, add for Vec3, Vec3<T>);
    }
}

macro_rules! vec3_impl_sub {
    () => {
        impl<T> Sub for Vec3<T>
        where
            T: Copy + Sub<Output = T>
        {
            type Output = Vec3<T>;

            #[inline]
            #[track_caller]
            fn sub(self, other: Vec3<T>) -> Self::Output {
                Vec3::<T>::new(self.x - other.x, self.y - other.y, self.z - other.z)
            }
        }

        forward_ref_binop!(impl Sub, sub for Vec3, Vec3<T>);
    }
}

macro_rules! vec3_impl_mul {
    () => {
        impl<T> Mul for Vec3<T>
        where
            T: Copy + Mul<Output = T>
        {
            type Output = Vec3<T>;

            #[inline]
            #[track_caller]
            fn mul(self, other: Vec3<T>) -> Self::Output {
                Vec3::<T>::new(self.x * other.x, self.y * other.y, self.z * other.z)
            }
        }

        forward_ref_binop!(impl Mul, mul for Vec3, Vec3<T>);
    }
}

macro_rules! vec3_impl_scaler_mul {
    () => {
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

        impl<T: Copy + Mul<Output = T>> Mul<T> for Vec3<T> {
            type Output = Vec3<T>;

            #[inline]
            #[track_caller]
            fn mul(self, s: T) -> Self::Output {
                Vec3::<T>::new(self.x * s, self.y * s, self.z * s)
            }
        }

        forward_ref_binop!(impl Mul, mul for Vec3, T);
    }
}

vec3_impl_add!();
vec3_impl_sub!();
vec3_impl_mul!();
vec3_impl_scaler_mul!();

impl<T: Copy + Add<Output = T>> AddAssign for Vec3<T> {
    fn add_assign(&mut self, other: Vec3<T>) {
        *self = other + *self;
    }
}

impl<T: Copy + Add<Output = T>> AddAssign<&Vec3<T>> for Vec3<T> {
    fn add_assign(&mut self, other: &Vec3<T>) {
        *self = other + *self;
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, s: T) -> Self::Output {
        Vec3::<T>::new(self.x / s, self.y / s, self.z / s)
    }
}

impl<T: Copy + Neg<Output = T>> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::<T>::new(-self.x, -self.y, -self.z)
    }
}

pub fn dot<T: Copy>(a: &Vec3<T>, b: &Vec3<T>) -> T
where
    T: Mul<Output = T> + Add<Output = T>,
{
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn cross<T>(a: &Vec3<T>, b: &Vec3<T>) -> Vec3<T>
where
    T: Copy + Mul<Output = T> + Sub<Output = T>,
{
    Vec3::<T>::new(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}
