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

    pub fn set_value(&mut self, row: usize, col: usize, value: f32) {
        self.values[row * self.cols + col] = value;
    }

    pub fn value_at(&self, row: usize, col: usize) -> f32 {
        self.values[row * self.cols + col]
    }

    pub fn identity_matrix(size: usize) -> Self {
        let mut m = Matrix::new(size, size);
        for i in 0..size {
            m.set_value(i, i, 1.0);
        }
        m
    }

    pub fn transpose(&self) -> Self {
        let mut m = Matrix::new(self.rows, self.cols);
        for row in 0..self.rows {
            for col in 0..self.cols {
                m.set_value(col, row, self.value_at(row, col));
            }
        }
        m
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
                    value += self.value_at(row, i) * other.value_at(i, col);
                }
                result.set_value(row, col, value);
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
fn test_constructing_and_inspecting_a_4_x_4_matrix() {
    #[rustfmt::skip]
    let m: Matrix = Matrix::new_with_values(
        4, 4,
        vec![
            1.0, 2.0, 3.0, 4.0,
            5.5, 6.5, 7.5, 8.5,
            9.0, 10.0, 11.0, 12.0,
            13.5, 14.5, 15.5, 16.5,
        ],
    );
    assert_eq!(m.value_at(0, 0), 1.0);
    assert_eq!(m.value_at(0, 3), 4.0);
    assert_eq!(m.value_at(1, 0), 5.5);
    assert_eq!(m.value_at(1, 2), 7.5);
    assert_eq!(m.value_at(2, 2), 11.0);
    assert_eq!(m.value_at(3, 0), 13.5);
    assert_eq!(m.value_at(3, 2), 15.5);
}

#[test]
fn a_2_x_2_matrix_ought_to_be_representable() {
    let m = Matrix::new_with_values(2, 2, vec![-3.0, 5.0, 1.0, -2.0]);
    assert_eq!(m.value_at(0, 0), -3.0);
    assert_eq!(m.value_at(0, 1), 5.0);
    assert_eq!(m.value_at(1, 0), 1.0);
    assert_eq!(m.value_at(1, 1), -2.0);
}

#[test]
fn a_3_x_3_matrix_ought_to_be_representable() {
    #[rustfmt::skip]
    let m = Matrix::new_with_values(
        3, 3,
        vec![
            -3.0, 5.0, 0.0,
            1.0, -2.0, -7.0,
            0.0, 1.0, 1.0,
        ],
    );
    assert_eq!(m.value_at(0, 0), -3.0);
    assert_eq!(m.value_at(0, 1), 5.0);
    assert_eq!(m.value_at(0, 2), 0.0);
    assert_eq!(m.value_at(1, 0), 1.0);
    assert_eq!(m.value_at(1, 1), -2.0);
    assert_eq!(m.value_at(1, 2), -7.0);
    assert_eq!(m.value_at(2, 0), 0.0);
    assert_eq!(m.value_at(2, 1), 1.0);
    assert_eq!(m.value_at(2, 2), 1.0);
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
            -2.0, 1.0, 2.0, 3.0,
            3.0, 2.0, 1.0, -1.0,
            4.0, 3.0, 6.0, 5.0,
            1.0, 2.0, 7.0, 8.0,
        ],
    );
    #[rustfmt::skip]
    let m = Matrix::new_with_values(
        4, 4,
        vec![
            20.0, 22.0, 50.0, 48.0,
            44.0, 54.0, 114.0, 108.0,
            40.0, 58.0, 110.0, 102.0,
            16.0, 26.0, 46.0, 42.0,
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
            0.0, 1.0, 2.0, 4.0,
            1.0, 2.0, 4.0, 8.0,
            2.0, 4.0, 8.0, 16.0,
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
