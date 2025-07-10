use num_traits::{Float, NumCast, One, Zero};
use std::ops::{Add, Div, Mul, Sub};

use crate::engine::types::vector::vector_ops::VectorOps;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T> VectorOps<T> for Vector2<T>
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
        self.x * other.x + self.y * other.y
    }

    fn magnitude(self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn up() -> Self {
        Self {
            x: T::zero(),
            y: T::one(),
        }
    }

    fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }
}

impl<T> Vector2<T>
where
    T: Copy + NumCast,
{
    pub fn cast<G>(&self) -> Option<Vector2<G>>
    where
        G: NumCast,
    {
        Some(Vector2 {
            x: NumCast::from(self.x)?,
            y: NumCast::from(self.y)?,
        })
    }
}

// Operator overloading

impl<T> Add for Vector2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


impl<T> Mul for Vector2<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T> Sub for Vector2<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Mul<T> for Vector2<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, factor: T) -> Self::Output {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

impl<T> Add<T> for Vector2<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, factor: T) -> Self::Output {
        Self {
            x: self.x + factor,
            y: self.y + factor,
        }
    }
}


impl<T> Div<T> for Vector2<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}


impl<T> Div for Vector2<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}
