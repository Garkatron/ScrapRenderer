use num_traits::{Float, NumCast, One, Zero};
use std::ops::{Add, Div, Mul, Sub};

use crate::engine::types::vector::vector2i::Vector2i;

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

impl<T> Vector2<T>
where
    T: Float,
{
    pub fn magnitude(self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn distance(self, other: Self) -> T {
        (self - other).magnitude()
    }

    pub fn normalize(self) -> Self {
        let len = self.magnitude();
        if len > T::zero() {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            Self::zero()
        }
    }

    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }

    pub fn scale(self, factor: T) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }

    pub fn up() -> Self {
        Self {
            x: T::zero(),
            y: T::one(),
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

// Conversion from Vector2i
impl From<Vector2i> for Vector2<f32> {
    fn from(value: Vector2i) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}
