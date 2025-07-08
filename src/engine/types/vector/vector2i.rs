use std::ops::{Add, Sub, Mul};

use crate::engine::types::vector::{vector_ops::VectorOps};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vector2i {
    pub x: i32,
    pub y: i32,
}

impl Vector2i {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn one() -> Self {
        Self { x: 1, y: 1 }
    }

    pub fn normalize(&self) -> Vector2i {
        let len = self.magnitude();
        if len > 0.0 {
            Vector2i {
                x: (self.x as f32 / len) as i32,
                y: (self.y as f32 / len) as i32,
            }
        } else {
            Vector2i::zero()
        }
    }
}

impl VectorOps<i32> for Vector2i {
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

    fn scale(self, factor: i32) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    fn dot(self, other: Self) -> i32 {
        self.x * other.x + self.y * other.y
    }

    fn magnitude(self) -> f32 {
        ((self.x * self.x + self.y * self.y) as f32).sqrt()
    }

    fn distance(self, other: Self) -> f32 {
        (self - other).magnitude()
    }

    fn normalize(&self) -> Self {
        let v2i = self.normalize();
        Self {
            x: v2i.x,
            y: v2i.y
        }
    }
}

impl Add for Vector2i {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2i {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i32> for Vector2i {
    type Output = Self;

    fn mul(self, factor: i32) -> Self::Output {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}