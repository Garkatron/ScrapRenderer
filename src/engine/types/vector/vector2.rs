use std::ops::{Add, Mul, Sub};

use crate::engine::types::vector::{vector2i::Vector2i, vector_ops::VectorOps};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

}

impl VectorOps<f32> for Vector2 {
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn scale(self, factor: f32) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn distance(self, other: Self) -> f32 {
        (self - other).magnitude()
    }

    fn normalize(&self) -> Self {
        let len = self.magnitude();
        if len > 0.0 {
            Vector2 {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            Vector2::zero()
        }
    }
    fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    fn up() -> Self {
        Self { x: 0.0, y: 1.0 }
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, factor: f32) -> Self::Output {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

impl From<Vector2i> for Vector2 {
    fn from(value: Vector2i) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32
        }
    }
}