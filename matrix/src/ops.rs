pub use crate::mult::Matrix;
use std::ops::{Add, Sub};

impl<T: Clone + Add<Output = T>> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn add(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.number_of_rows() != other.number_of_rows() 
            || self.number_of_cols() != other.number_of_cols() {
            return None;
        }

        let mut result = Vec::new();
        for i in 0..self.number_of_rows() {
            let mut row = Vec::new();
            for j in 0..self.number_of_cols() {
                row.push(self.0[i][j].clone() + other.0[i][j].clone());
            }
            result.push(row);
        }
        Some(Matrix(result))
    }
}

impl<T: Clone + Sub<Output = T>> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.number_of_rows() != other.number_of_rows() 
            || self.number_of_cols() != other.number_of_cols() {
            return None;
        }

        let mut result = Vec::new();
        for i in 0..self.number_of_rows() {
            let mut row = Vec::new();
            for j in 0..self.number_of_cols() {
                row.push(self.0[i][j].clone() - other.0[i][j].clone());
            }
            result.push(row);
        }
        Some(Matrix(result))
    }
}
