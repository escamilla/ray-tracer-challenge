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
    let mut m = Matrix::new(4, 4);
    m.set_value(0, 0, 1.0);
    m.set_value(0, 1, 2.0);
    m.set_value(0, 2, 3.0);
    m.set_value(0, 3, 4.0);
    m.set_value(1, 0, 5.5);
    m.set_value(1, 1, 6.5);
    m.set_value(1, 2, 7.5);
    m.set_value(1, 3, 8.5);
    m.set_value(2, 0, 9.0);
    m.set_value(2, 1, 10.0);
    m.set_value(2, 2, 11.0);
    m.set_value(2, 3, 12.0);
    m.set_value(3, 0, 13.5);
    m.set_value(3, 1, 14.5);
    m.set_value(3, 2, 15.5);
    m.set_value(3, 3, 16.5);
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
    let mut m = Matrix::new(2, 2);
    m.set_value(0, 0, -3.0);
    m.set_value(0, 1, 5.0);
    m.set_value(1, 0, 1.0);
    m.set_value(1, 1, -2.0);
    assert_eq!(m.value_at(0, 0), -3.0);
    assert_eq!(m.value_at(0, 1), 5.0);
    assert_eq!(m.value_at(1, 0), 1.0);
    assert_eq!(m.value_at(1, 1), -2.0);
}

#[test]
fn a_3_x_3_matrix_ought_to_be_representable() {
    let mut m = Matrix::new(3, 3);
    m.set_value(0, 0, -3.0);
    m.set_value(0, 1, 5.0);
    m.set_value(0, 2, 0.0);
    m.set_value(1, 0, 1.0);
    m.set_value(1, 1, -2.0);
    m.set_value(1, 2, -7.0);
    m.set_value(2, 0, 0.0);
    m.set_value(2, 1, 1.0);
    m.set_value(2, 2, 1.0);
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
    let mut a = Matrix::new(4, 4);
    a.set_value(0, 0, 1.0);
    a.set_value(0, 1, 2.0);
    a.set_value(0, 2, 3.0);
    a.set_value(0, 3, 4.0);
    a.set_value(1, 0, 5.0);
    a.set_value(1, 1, 6.0);
    a.set_value(1, 2, 7.0);
    a.set_value(1, 3, 8.0);
    a.set_value(2, 0, 9.0);
    a.set_value(2, 1, 8.0);
    a.set_value(2, 2, 7.0);
    a.set_value(2, 3, 6.0);
    a.set_value(3, 0, 5.0);
    a.set_value(3, 1, 4.0);
    a.set_value(3, 2, 3.0);
    a.set_value(3, 3, 2.0);
    let mut b = Matrix::new(4, 4);
    b.set_value(0, 0, 1.0);
    b.set_value(0, 1, 2.0);
    b.set_value(0, 2, 3.0);
    b.set_value(0, 3, 4.0);
    b.set_value(1, 0, 5.0);
    b.set_value(1, 1, 6.0);
    b.set_value(1, 2, 7.0);
    b.set_value(1, 3, 8.0);
    b.set_value(2, 0, 9.0);
    b.set_value(2, 1, 8.0);
    b.set_value(2, 2, 7.0);
    b.set_value(2, 3, 6.0);
    b.set_value(3, 0, 5.0);
    b.set_value(3, 1, 4.0);
    b.set_value(3, 2, 3.0);
    b.set_value(3, 3, 2.0);
    assert_eq!(a, b);
}

#[test]
fn test_matrix_equality_with_different_matrices() {
    let mut a = Matrix::new(4, 4);
    a.set_value(0, 0, 1.0);
    a.set_value(0, 1, 2.0);
    a.set_value(0, 2, 3.0);
    a.set_value(0, 3, 4.0);
    a.set_value(1, 0, 5.0);
    a.set_value(1, 1, 6.0);
    a.set_value(1, 2, 7.0);
    a.set_value(1, 3, 8.0);
    a.set_value(2, 0, 9.0);
    a.set_value(2, 1, 8.0);
    a.set_value(2, 2, 7.0);
    a.set_value(2, 3, 6.0);
    a.set_value(3, 0, 5.0);
    a.set_value(3, 1, 4.0);
    a.set_value(3, 2, 3.0);
    a.set_value(3, 3, 2.0);
    let mut b = Matrix::new(4, 4);
    b.set_value(0, 0, 2.0);
    b.set_value(0, 1, 3.0);
    b.set_value(0, 2, 4.0);
    b.set_value(0, 3, 5.0);
    b.set_value(1, 0, 6.0);
    b.set_value(1, 1, 7.0);
    b.set_value(1, 2, 8.0);
    b.set_value(1, 3, 9.0);
    b.set_value(2, 0, 8.0);
    b.set_value(2, 1, 7.0);
    b.set_value(2, 2, 6.0);
    b.set_value(2, 3, 5.0);
    b.set_value(3, 0, 4.0);
    b.set_value(3, 1, 3.0);
    b.set_value(3, 2, 2.0);
    b.set_value(3, 3, 1.0);
    assert_ne!(a, b);
}

#[test]
fn test_multiplying_two_matrices() {
    let mut a = Matrix::new(4, 4);
    a.set_value(0, 0, 1.0);
    a.set_value(0, 1, 2.0);
    a.set_value(0, 2, 3.0);
    a.set_value(0, 3, 4.0);
    a.set_value(1, 0, 5.0);
    a.set_value(1, 1, 6.0);
    a.set_value(1, 2, 7.0);
    a.set_value(1, 3, 8.0);
    a.set_value(2, 0, 9.0);
    a.set_value(2, 1, 8.0);
    a.set_value(2, 2, 7.0);
    a.set_value(2, 3, 6.0);
    a.set_value(3, 0, 5.0);
    a.set_value(3, 1, 4.0);
    a.set_value(3, 2, 3.0);
    a.set_value(3, 3, 2.0);
    let mut b = Matrix::new(4, 4);
    b.set_value(0, 0, -2.0);
    b.set_value(0, 1, 1.0);
    b.set_value(0, 2, 2.0);
    b.set_value(0, 3, 3.0);
    b.set_value(1, 0, 3.0);
    b.set_value(1, 1, 2.0);
    b.set_value(1, 2, 1.0);
    b.set_value(1, 3, -1.0);
    b.set_value(2, 0, 4.0);
    b.set_value(2, 1, 3.0);
    b.set_value(2, 2, 6.0);
    b.set_value(2, 3, 5.0);
    b.set_value(3, 0, 1.0);
    b.set_value(3, 1, 2.0);
    b.set_value(3, 2, 7.0);
    b.set_value(3, 3, 8.0);
    let mut m = Matrix::new(4, 4);
    m.set_value(0, 0, 20.0);
    m.set_value(0, 1, 22.0);
    m.set_value(0, 2, 50.0);
    m.set_value(0, 3, 48.0);
    m.set_value(1, 0, 44.0);
    m.set_value(1, 1, 54.0);
    m.set_value(1, 2, 114.0);
    m.set_value(1, 3, 108.0);
    m.set_value(2, 0, 40.0);
    m.set_value(2, 1, 58.0);
    m.set_value(2, 2, 110.0);
    m.set_value(2, 3, 102.0);
    m.set_value(3, 0, 16.0);
    m.set_value(3, 1, 26.0);
    m.set_value(3, 2, 46.0);
    m.set_value(3, 3, 42.0);
    assert_eq!(a * b, m);
}

#[test]
fn test_a_matrix_multiplied_by_a_tuple() {
    let mut a = Matrix::new(4, 4);
    a.set_value(0, 0, 1.0);
    a.set_value(0, 1, 2.0);
    a.set_value(0, 2, 3.0);
    a.set_value(0, 3, 4.0);
    a.set_value(1, 0, 2.0);
    a.set_value(1, 1, 4.0);
    a.set_value(1, 2, 4.0);
    a.set_value(1, 3, 2.0);
    a.set_value(2, 0, 8.0);
    a.set_value(2, 1, 6.0);
    a.set_value(2, 2, 4.0);
    a.set_value(2, 3, 1.0);
    a.set_value(3, 0, 0.0);
    a.set_value(3, 1, 0.0);
    a.set_value(3, 2, 0.0);
    a.set_value(3, 3, 1.0);
    let b = Tuple::new(1.0, 2.0, 3.0, 1.0);
    assert_eq!(a * b, Tuple::new(18.0, 24.0, 33.0, 1.0));
}

#[test]
fn test_multiplying_a_matrix_by_the_identity_matrix() {
    let mut a = Matrix::new(4, 4);
    a.set_value(0, 0, 0.0);
    a.set_value(0, 1, 1.0);
    a.set_value(0, 2, 2.0);
    a.set_value(0, 3, 4.0);
    a.set_value(1, 0, 1.0);
    a.set_value(1, 1, 2.0);
    a.set_value(1, 2, 4.0);
    a.set_value(1, 3, 8.0);
    a.set_value(2, 0, 2.0);
    a.set_value(2, 1, 4.0);
    a.set_value(2, 2, 8.0);
    a.set_value(2, 3, 16.0);
    a.set_value(3, 0, 4.0);
    a.set_value(3, 1, 8.0);
    a.set_value(3, 2, 16.0);
    a.set_value(3, 3, 32.0);
    assert_eq!(a.clone() * Matrix::identity_matrix(4), a);
}

#[test]
fn test_multiplying_the_identity_matrix_by_a_tuple() {
    let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
    assert_eq!(Matrix::identity_matrix(4) * a, a);
}
