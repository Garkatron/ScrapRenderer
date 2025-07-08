pub trait VectorOps<T> {
    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
    fn scale(self, factor: T) -> Self;
    fn dot(self, other: Self) -> T;
    fn magnitude(self) -> f32;
    fn distance(self, other: Self) -> f32;
    fn normalize(&self) -> Self;
}