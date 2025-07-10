use num_traits::{Float, NumCast, One, Zero};
use std::ops::{Add, Div, Mul, Sub};

use crate::engine::types::vector::{vector2::Vector2, vector_ops::VectorOps};


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    
}
impl<T> VectorOps<T> for Vector3<T>
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
        }
    }

    fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }
}

impl<T> Vector3<T>
where
    T: Float,
{

    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl<T> Vector3<T>
where
    T: Copy + NumCast,
{
    pub fn cast<G>(&self) -> Option<Vector3<G>>
    where
        G: NumCast,
    {
        Some(Vector3 {
            x: NumCast::from(self.x)?,
            y: NumCast::from(self.y)?,
            z: NumCast::from(self.z)?,
        })
    }
}

// Operator overloading

impl<T> Add for Vector3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}


impl<T> Mul for Vector3<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T> Sub for Vector3<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> Mul<T> for Vector3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, factor: T) -> Self::Output {
        Self {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }
}

impl<T> Add<T> for Vector3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, factor: T) -> Self::Output {
        Self {
            x: self.x + factor,
            y: self.y + factor,
            z: self.z + factor,
        }
    }
}

// Conversion from Vector3i
impl From<Vector3<i32>> for Vector3<f32> {
    fn from(value: Vector3<i32>) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
            z: value.z as f32,
        }
    }
}

impl<T> Div<T> for Vector3<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}


impl<T> Div for Vector3<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Into<Vector2<i32>> for Vector3<f32> {
    fn into(self) -> Vector2<i32> {
        Vector2 { x: self.x as i32, y: self.y as i32 }
    }
}