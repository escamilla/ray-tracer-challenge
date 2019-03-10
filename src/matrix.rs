use crate::f32_equal;
use crate::tuple::Tuple;
use std::ops::Mul;

#[derive(Clone, Debug)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    values: Vec<f32>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            values: vec![0.0; rows * cols],
        }
    }

    pub fn new_with_values(rows: usize, cols: usize, values: Vec<f32>) -> Self {
        assert_eq!(rows * cols, values.len());
        Matrix { rows, cols, values }
    }

    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        self.values[row * self.cols + col] = value;
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.values[row * self.cols + col]
    }

    pub fn identity_matrix(size: usize) -> Self {
        let mut m = Matrix::new(size, size);
        for i in 0..size {
            m.set(i, i, 1.0);
        }
        m
    }

    pub fn transpose(&self) -> Self {
        let mut m = Matrix::new(self.rows, self.cols);
        for row in 0..self.rows {
            for col in 0..self.cols {
                m.set(col, row, self.get(row, col));
            }
        }
        m
    }

    pub fn determinant(&self) -> f32 {
        assert_eq!(self.rows, self.cols);
        assert!(self.rows >= 2);
        if self.rows == 2 && self.cols == 2 {
            (self.get(0, 0) * self.get(1, 1)) - (self.get(1, 0) * self.get(0, 1))
        } else {
            let mut det = 0.0;
            for col in 0..self.cols {
                det += self.get(0, col) * self.cofactor(0, col);
            }
            det
        }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Self {
        assert!(row < self.rows);
        assert!(col < self.cols);
        let mut m = Matrix::new(self.rows - 1, self.cols - 1);
        let mut i = 0;
        for r in 0..self.rows {
            for c in 0..self.cols {
                if r == row || c == col {
                    continue;
                }
                m.values[i] = self.get(r, c);
                i += 1;
            }
        }
        m
    }

    pub fn minor(&self, row: usize, col: usize) -> f32 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        if (row + col) % 2 == 0 {
            self.minor(row, col)
        } else {
            -self.minor(row, col)
        }
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Matrix {
        let det = self.determinant();
        let mut inverse_matrix = Matrix::new(self.rows, self.cols);
        for row in 0..self.rows {
            for col in 0..self.cols {
                let c = self.cofactor(row, col);
                inverse_matrix.set(col, row, c / det);
            }
        }
        inverse_matrix
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        self.rows == other.rows
            && self.cols == other.cols
            && self
                .values
                .iter()
                .zip(other.values.iter())
                .all(|(a, b)| f32_equal(*a, *b))
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Self {
        assert_eq!(self.cols, other.rows);
        let mut result = Matrix::new(self.rows, other.cols);
        for row in 0..self.rows {
            for col in 0..other.cols {
                let mut value = 0.0;
                for i in 0..self.cols {
                    value += self.get(row, i) * other.get(i, col);
                }
                result.set(row, col, value);
            }
        }
        result
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, tuple: Tuple) -> Tuple {
        assert_eq!(self.cols, 4);
        Tuple::from_matrix(self * tuple.to_matrix())
    }
}

#[test]
fn test_constructing_and_inspecting_a_4x4_matrix() {
    #[rustfmt::skip]
    let m: Matrix = Matrix::new_with_values(
        4, 4,
        vec![
             1.0,  2.0,  3.0,  4.0,
             5.5,  6.5,  7.5,  8.5,
             9.0, 10.0, 11.0, 12.0,
            13.5, 14.5, 15.5, 16.5,
        ],
    );
    assert_eq!(m.get(0, 0), 1.0);
    assert_eq!(m.get(0, 3), 4.0);
    assert_eq!(m.get(1, 0), 5.5);
    assert_eq!(m.get(1, 2), 7.5);
    assert_eq!(m.get(2, 2), 11.0);
    assert_eq!(m.get(3, 0), 13.5);
    assert_eq!(m.get(3, 2), 15.5);
}

#[test]
fn test_a_2x2_matrix_ought_to_be_representable() {
    let m = Matrix::new_with_values(2, 2, vec![-3.0, 5.0, 1.0, -2.0]);
    assert_eq!(m.get(0, 0), -3.0);
    assert_eq!(m.get(0, 1), 5.0);
    assert_eq!(m.get(1, 0), 1.0);
    assert_eq!(m.get(1, 1), -2.0);
}

#[test]
fn test_a_3x3_matrix_ought_to_be_representable() {
    #[rustfmt::skip]
    let m = Matrix::new_with_values(
        3, 3,
        vec![
            -3.0,  5.0,  0.0,
             1.0, -2.0, -7.0,
             0.0,  1.0,  1.0,
        ],
    );
    assert_eq!(m.get(0, 0), -3.0);
    assert_eq!(m.get(0, 1), 5.0);
    assert_eq!(m.get(0, 2), 0.0);
    assert_eq!(m.get(1, 0), 1.0);
    assert_eq!(m.get(1, 1), -2.0);
    assert_eq!(m.get(1, 2), -7.0);
    assert_eq!(m.get(2, 0), 0.0);
    assert_eq!(m.get(2, 1), 1.0);
    assert_eq!(m.get(2, 2), 1.0);
}

#[test]
fn test_matrix_equality_with_identical_matrices() {
    #[rustfmt::skip]
    let a = Matrix::new_with_values(
        4, 4,
        vec![
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0,
        ],
    );
    #[rustfmt::skip]
    let b = Matrix::new_with_values(
        4, 4,
        vec![
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0,
        ],
    );
    assert_eq!(a, b);
}

#[test]
fn test_matrix_equality_with_different_matrices() {
    #[rustfmt::skip]
    let a = Matrix::new_with_values(
        4,
        4,
        vec![
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0,
        ],
    );
    #[rustfmt::skip]
    let b = Matrix::new_with_values(
        4, 4,
        vec![
            2.0, 3.0, 4.0, 5.0,
            6.0, 7.0, 8.0, 9.0,
            8.0, 7.0, 6.0, 5.0,
            4.0, 3.0, 2.0, 1.0,
        ],
    );
    assert_ne!(a, b);
}

#[test]
fn test_multiplying_two_matrices() {
    #[rustfmt::skip]
    let a = Matrix::new_with_values(
        4, 4,
        vec![
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0,
        ],
    );
    #[rustfmt::skip]
    let b = Matrix::new_with_values(
        4, 4,
        vec![
            -2.0, 1.0, 2.0,  3.0,
             3.0, 2.0, 1.0, -1.0,
             4.0, 3.0, 6.0,  5.0,
             1.0, 2.0, 7.0,  8.0,
        ],
    );
    #[rustfmt::skip]
    let m = Matrix::new_with_values(
        4, 4,
        vec![
            20.0, 22.0,  50.0,  48.0,
            44.0, 54.0, 114.0, 108.0,
            40.0, 58.0, 110.0, 102.0,
            16.0, 26.0,  46.0,  42.0,
        ],
    );
    assert_eq!(a * b, m);
}

#[test]
fn test_a_matrix_multiplied_by_a_tuple() {
    #[rustfmt::skip]
    let a = Matrix::new_with_values(
        4, 4,
        vec![
            1.0, 2.0, 3.0, 4.0,
            2.0, 4.0, 4.0, 2.0,
            8.0, 6.0, 4.0, 1.0,
            0.0, 0.0, 0.0, 1.0,
        ],
    );
    let b = Tuple::new(1.0, 2.0, 3.0, 1.0);
    assert_eq!(a * b, Tuple::new(18.0, 24.0, 33.0, 1.0));
}

#[test]
fn test_multiplying_a_matrix_by_the_identity_matrix() {
    #[rustfmt::skip]
    let a = Matrix::new_with_values(
        4, 4,
        vec![
            0.0, 1.0,  2.0,  4.0,
            1.0, 2.0,  4.0,  8.0,
            2.0, 4.0,  8.0, 16.0,
            4.0, 8.0, 16.0, 32.0,
        ],
    );
    assert_eq!(a.clone() * Matrix::identity_matrix(4), a);
}

#[test]
fn test_multiplying_the_identity_matrix_by_a_tuple() {
    let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
    assert_eq!(Matrix::identity_matrix(4) * a, a);
}

#[test]
fn test_transposing_a_matrix() {
    #[rustfmt::skip]
    let a = Matrix::new_with_values(
        4, 4,
        vec![
            0.0, 9.0, 3.0, 0.0,
            9.0, 8.0, 0.0, 8.0,
            1.0, 8.0, 5.0, 3.0,
            0.0, 0.0, 5.0, 8.0,
        ],
    );
    #[rustfmt::skip]
    let b = Matrix::new_with_values(
        4, 4,
        vec![
            0.0, 9.0, 1.0, 0.0,
            9.0, 8.0, 8.0, 0.0,
            3.0, 0.0, 5.0, 5.0,
            0.0, 8.0, 3.0, 8.0,
        ],
    );
    assert_eq!(a.transpose(), b);
}

#[test]
fn test_transposing_the_identity_matrix() {
    let a = Matrix::identity_matrix(4);
    assert_eq!(a.transpose(), a);
}

#[test]
fn test_calculating_the_determinant_of_a_2x2_matrix() {
    let a = Matrix::new_with_values(2, 2, vec![1.0, 5.0, -3.0, 2.0]);
    assert_eq!(a.determinant(), 17.0);
}

#[test]
fn test_a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
    #[rustfmt::skip]
    let a = Matrix::new_with_values(
        3, 3,
        vec![
             1.0, 5.0,  0.0,
            -3.0, 2.0,  7.0,
             0.0, 6.0, -3.0,
        ],
    );
    assert_eq!(
        a.submatrix(0, 2),
        Matrix::new_with_values(2, 2, vec![-3.0, 2.0, 0.0, 6.0])
    )
}

#[test]
fn test_a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
    #[rustfmt::skip]
    let a = Matrix::new_with_values(
        4, 4,
        vec![
            -6.0, 1.0,  1.0, 6.0,
            -8.0, 5.0,  8.0, 6.0,
            -1.0, 0.0,  8.0, 2.0,
            -7.0, 1.0, -1.0, 1.0,
        ],
    );
    #[rustfmt::skip]
    assert_eq!(
        a.submatrix(2, 1),
        Matrix::new_with_values(
            3, 3,
            vec![
                -6.0,  1.0, 6.0,
                -8.0,  8.0, 6.0,
                -7.0, -1.0, 1.0,
            ],
        ),
    );
}

#[test]
fn test_calculating_a_minor_of_a_3x3_matrix() {
    #[rustfmt::skip]
    let a = Matrix::new_with_values(
        3, 3,
        vec![
            3.0,  5.0,  0.0,
            2.0, -1.0, -7.0,
            6.0, -1.0,  5.0,
        ],
    );
    let b = a.submatrix(1, 0);
    assert_eq!(b.determinant(), 25.0);
    assert_eq!(a.minor(1, 0), 25.0);
}

#[test]
fn test_calculating_a_cofactor_of_a_3x3_matrix() {
    #[rustfmt::skip]
    let a = Matrix::new_with_values(
        3, 3,
        vec![
            3.0,  5.0,  0.0,
            2.0, -1.0, -7.0,
            6.0, -1.0,  5.0,
        ],
    );
    assert_eq!(a.minor(0, 0), -12.0);
    assert_eq!(a.cofactor(0, 0), -12.0);
    assert_eq!(a.minor(1, 0), 25.0);
    assert_eq!(a.cofactor(1, 0), -25.0);
}

#[test]
fn test_calculating_the_determinant_of_a_3x3_matrix() {
    #[rustfmt::skip]
    let a = Matrix::new_with_values(
        3, 3,
        vec![
             1.0, 2.0,  6.0,
            -5.0, 8.0, -4.0,
             2.0, 6.0,  4.0,
        ],
    );
    assert_eq!(a.cofactor(0, 0), 56.0);
    assert_eq!(a.cofactor(0, 1), 12.0);
    assert_eq!(a.cofactor(0, 2), -46.0);
    assert_eq!(a.determinant(), -196.0);
}

#[test]
fn test_calculating_the_determinant_of_a_4x4_matrix() {
    #[rustfmt::skip]
    let a = Matrix::new_with_values(
        4, 4,
        vec![
            -2.0, -8.0,  3.0,  5.0,
            -3.0,  1.0,  7.0,  3.0,
             1.0,  2.0, -9.0,  6.0,
            -6.0,  7.0,  7.0, -9.0,
        ],
    );
    assert_eq!(a.cofactor(0, 0), 690.0);
    assert_eq!(a.cofactor(0, 1), 447.0);
    assert_eq!(a.cofactor(0, 2), 210.0);
    assert_eq!(a.cofactor(0, 3), 51.0);
    assert_eq!(a.determinant(), -4071.0);
}

#[test]
fn test_testing_an_invertible_matrix_for_invertibility() {
    #[rustfmt::skip]
    let a = Matrix::new_with_values(
        4, 4,
        vec![
            6.0,  4.0, 4.0,  4.0,
            5.0,  5.0, 7.0,  6.0,
            4.0, -9.0, 3.0, -7.0,
            9.0,  1.0, 7.0, -6.0,
        ],
    );
    assert_eq!(a.determinant(), -2120.0);
    assert!(a.is_invertible());
}

#[test]
fn test_testing_a_noninvertible_matrix_for_invertibility() {
    #[rustfmt::skip]
    let a = Matrix::new_with_values(
        4, 4,
        vec![
            -4.0,  2.0, -2.0, -3.0,
             9.0,  6.0,  2.0,  6.0,
             0.0, -5.0,  1.0, -5.0,
             0.0,  0.0,  0.0,  0.0,
        ],
    );
    assert_eq!(a.determinant(), 0.0);
    assert!(!a.is_invertible());
}

#[test]
fn test_calculating_the_inverse_of_a_matrix() {
    #[rustfmt::skip]
    let a = Matrix::new_with_values(
        4, 4,
        vec![
            -5.0,  2.0,  6.0, -8.0,
             1.0, -5.0,  1.0,  8.0,
             7.0,  7.0, -6.0, -7.0,
             1.0, -3.0,  7.0,  4.0,
        ],
    );
    let b = a.inverse();
    assert_eq!(a.determinant(), 532.0);
    assert_eq!(a.cofactor(2, 3), -160.0);
    assert!(f32_equal(b.get(3, 2), -160.0 / 532.0));
    assert_eq!(a.cofactor(3, 2), 105.0);
    assert!(f32_equal(b.get(2, 3), 105.0 / 532.0));
    #[rustfmt::skip]
    assert_eq!(b, Matrix::new_with_values(
        4,
        4,
        vec![
             0.21805,  0.45113,  0.24060, -0.04511,
            -0.80827, -1.45677, -0.44361,  0.52068,
            -0.07895, -0.22368, -0.05263,  0.19737,
            -0.52256, -0.81391, -0.30075,  0.30639,
        ],
    ));
}

#[test]
fn test_calculating_the_inverse_of_another_matrix() {
    #[rustfmt::skip]
    let a = Matrix::new_with_values(
        4, 4,
        vec![
             8.0, -5.0,  9.0,  2.0,
             7.0,  5.0,  6.0,  1.0,
            -6.0,  0.0,  9.0,  6.0,
            -3.0,  0.0, -9.0, -4.0,
        ],
    );
    #[rustfmt::skip]
    assert_eq!(a.inverse(), Matrix::new_with_values(
        4,
        4,
        vec![
            -0.15385, -0.15385, -0.28205, -0.53846,
            -0.07692,  0.12308,  0.02564,  0.03077,
             0.35897,  0.35897,  0.43590,  0.92308,
            -0.69231, -0.69231, -0.76923, -1.92308,
        ],
    ));
}

#[test]
fn test_calculating_the_inverse_of_a_third_matrix() {
    #[rustfmt::skip]
    let a = Matrix::new_with_values(
        4, 4,
        vec![
             9.0,  3.0,  0.0,  9.0,
            -5.0, -2.0, -6.0, -3.0,
            -4.0,  9.0,  6.0,  4.0,
            -7.0,  6.0,  6.0,  2.0,
        ],
    );
    #[rustfmt::skip]
    assert_eq!(a.inverse(), Matrix::new_with_values(
        4,
        4,
        vec![
            -0.04074, -0.07778,  0.14444, -0.22222,
            -0.07778,  0.03333,  0.36667, -0.33333,
            -0.02901, -0.14630, -0.10926,  0.12963,
             0.17778,  0.06667, -0.26667,  0.33333,
        ],
    ));
}

#[test]
fn test_multiplying_a_product_by_its_inverse() {
    #[rustfmt::skip]
    let a = Matrix::new_with_values(
        4, 4,
        vec![
             3.0, -9.0,  7.0,  3.0,
             3.0, -8.0,  2.0, -9.0,
            -4.0,  4.0,  4.0,  1.0,
            -6.0,  5.0, -1.0,  1.0,
        ],
    );
    #[rustfmt::skip]
    let b = Matrix::new_with_values(
        4, 4,
        vec![
            8.0,  2.0, 2.0, 2.0,
            3.0, -1.0, 7.0, 0.0,
            7.0,  0.0, 5.0, 4.0,
            6.0, -2.0, 0.0, 4.0,
        ],
    );
    let c = a.clone() * b.clone();
    assert_eq!(c * b.inverse(), a);
}
