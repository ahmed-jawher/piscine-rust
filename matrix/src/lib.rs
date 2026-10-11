pub use lalgebra_scalar::Scalar;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Matrix<const W: usize, const H: usize, T>(pub [[T; W]; H]);

impl<const W: usize, const H: usize, T: Scalar<Item = T>> Matrix<W, H, T> {
    pub fn zero() -> Self {
        Matrix(std::array::from_fn(|_| std::array::from_fn(|_| T::zero())))
    }
}

impl<const S: usize, T: Scalar<Item = T>> Matrix<S, S, T> {
    pub fn identity() -> Self {
        Matrix(std::array::from_fn(|i| {
            std::array::from_fn(|j| if i == j { T::one() } else { T::zero() })
        }))
    }
}
