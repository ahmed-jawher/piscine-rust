pub use matrix::{Matrix, Scalar};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Wrapper<const W: usize, const H: usize, T>(pub Matrix<W, H, T>);

impl<const W: usize, const H: usize, T> From<[[T; W]; H]> for Wrapper<W, H, T> {
    fn from(arr: [[T; W]; H]) -> Self {
        Wrapper(Matrix(arr))
    }
}

impl<const W: usize, const H: usize, T: Copy + Add<Output = T>> Add for Wrapper<W, H, T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let (a, b) = (self.0 .0, other.0 .0);
        Wrapper(Matrix(std::array::from_fn(|i| {
            std::array::from_fn(|j| a[i][j] + b[i][j])
        })))
    }
}

impl<const W: usize, const H: usize, T: Copy + Sub<Output = T>> Sub for Wrapper<W, H, T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let (a, b) = (self.0 .0, other.0 .0);
        Wrapper(Matrix(std::array::from_fn(|i| {
            std::array::from_fn(|j| a[i][j] - b[i][j])
        })))
    }
}

impl<const S: usize, T> Mul for Wrapper<S, S, T>
where
    T: Scalar<Item = T> + Copy + Add<Output = T> + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let (a, b) = (self.0 .0, other.0 .0);
        Wrapper(Matrix(std::array::from_fn(|i| {
            std::array::from_fn(|j| (0..S).fold(T::zero(), |acc, k| acc + a[i][k] * b[k][j]))
        })))
    }
}
