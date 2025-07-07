use std::ops::{Add, Mul, Sub};

pub trait VectorOps<T> {
    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
    fn scale(self, factor: T) -> Self;
    fn dot(self, other: Self) -> T;
    fn magnitude(self) -> f32;
    fn distance(self, other: Self) -> f32;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl VectorOps<f32> for Vector2 {
    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }

    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y }
    }

    fn scale(self, factor: f32) -> Self {
        Self { x: self.x * factor, y: self.y * factor }
    }

    fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn distance(self, other: Self) -> f32 {
        self.sub(other).magnitude()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w}
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0}
    }

    pub fn to_v2d(&self) -> Vector2 {
        Vector2 { x: self.x, y: self.y }
    }

    pub fn to_vi2d(&self) -> Vector2i {
        Vector2i { x: self.x as i32, y: self.y as i32}
    }
}

impl VectorOps<f32> for Vector3 {
    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z, w: 0.0 }
    }

    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z, w: 0.0 }
    }

    fn scale(self, factor: f32) -> Self {
        Self { x: self.x * factor, y: self.y * factor, z: self.z * factor, w: 0.0 }
    }

    fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn distance(self, other: Self) -> f32 {
        self.sub(other).magnitude()
    }
}

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
}

impl VectorOps<i32> for Vector2i {
    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }

    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y }
    }

    fn scale(self, factor: i32) -> Self {
        Self { x: self.x * factor, y: self.y * factor }
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

// Operador -
impl Sub for Vector2i {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// Operador * escalar
impl Mul<i32> for Vector2i {
    type Output = Self;

    fn mul(self, factor: i32) -> Self::Output {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

pub struct Matrix4x4 {
    pub m: [[f32; 4]; 4]
}


impl Matrix4x4 {
    pub fn identity() -> Self {
        let mut m = [[0.0; 4]; 4];
        m[0][0] = 1.0;
        m[1][1] = 1.0;
        m[2][2] = 1.0;
        m[3][3] = 1.0;
        Self { m }
    }

    pub fn zero() -> Matrix4x4 {
        Matrix4x4 { m: [[0.0; 4]; 4] }
    }

    pub fn multiply_vec(mat: &Matrix4x4, vec: &Vector3) -> Vector3 {
        let nx = vec.x * mat.m[0][0] + vec.y * mat.m[1][0] + vec.z * mat.m[2][0] + vec.w * mat.m[3][0];
        let ny = vec.x * mat.m[0][1] + vec.y * mat.m[1][1] + vec.z * mat.m[2][1] + vec.w * mat.m[3][1];
        let nz = vec.x * mat.m[0][2] + vec.y * mat.m[1][2] + vec.z * mat.m[2][2] + vec.w * mat.m[3][2];
        let nw = vec.x * mat.m[0][3] + vec.y * mat.m[1][3] + vec.z * mat.m[2][3] + vec.w * mat.m[3][3];

        if nw != 0.0 {
            return Vector3 { x: nx / nw, y: ny / nw, z: nz / nw, w: nw }
        }

        Vector3 { x: nx, y: ny, z: nz, w: nw }
    }

    pub fn translation(to: Vector3) -> Matrix4x4 {
        let mut mat = Matrix4x4::zero();
        mat.m[0][0] = 1.0;
        mat.m[1][1] = 1.0;
        mat.m[2][2] = 1.0;
        mat.m[3][3] = 1.0;

        mat.m[3][0] = to.x;
        mat.m[3][1] = to.y;
        mat.m[3][2] = to.z;

        mat
    }
}
