use crate::f32_equal;

#[derive(Debug)]
pub struct Matrix {
    rows: usize,
    cols: usize,
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
        self.values[row * self.rows + col] = value;
    }

    pub fn value_at(&self, row: usize, col: usize) -> f32 {
        self.values[row * self.rows + col]
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
