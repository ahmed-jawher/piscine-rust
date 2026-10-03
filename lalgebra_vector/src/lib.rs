use std::ops::{Add, Mul};

pub trait Scalar: Copy + Add<Output = Self> + Mul<Output = Self> {
    fn zero() -> Self;
    fn one() -> Self;
}

macro_rules! impl_scalar {
    ($zero:expr, $one:expr; $($t:ty),*) => {
        $(
            impl Scalar for $t {
                fn zero() -> Self {
                    $zero
                }
                fn one() -> Self {
                    $one
                }
            }
        )*
    };
}

impl_scalar!(0, 1; i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_scalar!(0.0, 1.0; f32, f64);

#[derive(Debug, PartialEq, Clone)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }

        Some(Vector(
            self.0.into_iter().zip(rhs.0).map(|(a, b)| a + b).collect(),
        ))
    }
}

impl<T: Scalar> Vector<T> {
    pub fn dot(self, rhs: Self) -> Option<T> {
        if self.0.len() != rhs.0.len() {
            return None;
        }

        Some(
            self.0
                .into_iter()
                .zip(rhs.0)
                .fold(T::zero(), |acc, (a, b)| acc + a * b),
        )
    }
}
