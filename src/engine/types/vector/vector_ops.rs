use num_traits::{Float, One, Zero};

pub trait VectorOps<T>: Sized + Copy
where
    T: Float,
{
    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
    fn scale(self, factor: T) -> Self;
    fn dot(self, other: Self) -> T;

    fn magnitude(self) -> T;
    fn distance(self, other: Self) -> T {
        self.sub(other).magnitude()
    }

    fn normalize(self) -> Self {
        let len = self.magnitude();
        if len > T::zero() {
            self.scale(T::one() / len)
        } else {
            Self::zero()
        }
    }

    fn up() -> Self;
    fn zero() -> Self;
}
