use num_traits::{Float, NumCast, One, Zero};
use std::ops::{Add, Div, Mul, Sub};

use crate::engine::types::vector::{vector3::Vector3, vector_ops::VectorOps};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vector4<T>
where
    T: Float,
{
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
            w: T::one(), // comúnmente w=1 en vectores homogéneos
        }
    }

    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub fn from_vector3(v: Vector3<T>, w: T) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            w,
        }
    }

    /// Divide x, y, z por w si w != 0, para hacer proyección en espacio 3D
    pub fn perspective_divide(&self) -> Vector3<T> {
        if self.w != T::zero() {
            Vector3 {
                x: self.x / self.w,
                y: self.y / self.w,
                z: self.z / self.w,
            }
        } else {
            Vector3 {
                x: self.x,
                y: self.y,
                z: self.z,
            }
        }
    }
}
impl<T> VectorOps<T> for Vector4<T>
where
    T: Float,
{
    fn add(self, other: Self) -> Self {
        self + other
    }

    fn sub(self, other: Self) -> Self {
        self - other
    }

    fn scale(self, factor: T) -> Self {
        self * factor
    }

    fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn magnitude(self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn up() -> Self {
        Self {
            x: T::zero(),
            y: T::one(),
            z: T::zero(),
            w: T::one(),
        }
    }

    fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
            w: T::one(),
        }
    }
}



impl<T> Add for Vector4<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<T> Mul for Vector4<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl<T> Sub for Vector4<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl<T> Mul<T> for Vector4<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, factor: T) -> Self::Output {
        Self {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
            w: self.w * factor,
        }
    }
}

impl<T> Add<T> for Vector4<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, factor: T) -> Self::Output {
        Self {
            x: self.x + factor,
            y: self.y + factor,
            z: self.z + factor,
            w: self.w + factor,
        }
    }
}

// Conversion from Vector4<i32> to Vector4<f32>
impl From<Vector4<i32>> for Vector4<f32> {
    fn from(value: Vector4<i32>) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
            z: value.z as f32,
            w: value.w as f32,
        }
    }
}

impl<T> Div<T> for Vector4<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl<T> Div for Vector4<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
        }
    }
}
