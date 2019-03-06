use super::f32_equal;
use std::ops::{Add, Neg, Sub};

#[derive(Debug)]
struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tuple {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Tuple { x, y, z, w }
    }

    fn new_point(x: f32, y: f32, z: f32) -> Self {
        Tuple { x, y, z, w: 1.0 }
    }

    fn new_vector(x: f32, y: f32, z: f32) -> Self {
        Tuple { x, y, z, w: 0.0 }
    }

    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        f32_equal(self.x, other.x)
            && f32_equal(self.y, other.y)
            && f32_equal(self.z, other.z)
            && f32_equal(self.w, other.w)
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Self {
        Tuple::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Self {
        Tuple::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

#[test]
fn a_tuple_with_w_equals_1_is_a_point() {
    let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert_eq!(a.w, 1.0);
    assert!(a.is_point());
    assert!(!a.is_vector());
}

#[test]
fn a_tuple_with_w_equals_0_is_a_vector() {
    let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert_eq!(a.w, 0.0);
    assert!(a.is_vector());
    assert!(!a.is_point());
}

#[test]
fn point_function_creates_tuples_with_w_equals_1() {
    let p = Tuple::new_point(4.0, -4.0, 3.0);
    assert_eq!(p.w, 1.0);
    assert!(p.is_point());
    assert!(!p.is_vector());
}

#[test]
fn vector_function_creates_tuples_with_w_equals_0() {
    let v = Tuple::new_vector(4.0, -4.0, 3.0);
    assert_eq!(v.w, 0.0);
    assert!(v.is_vector());
    assert!(!v.is_point());
}

#[test]
fn adding_two_tuples() {
    let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
    let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
    assert_eq!(a1 + a2, Tuple::new(1.0, 1.0, 6.0, 1.0));
}

#[test]
fn subtracting_two_points() {
    let p1 = Tuple::new_point(3.0, 2.0, 1.0);
    let p2 = Tuple::new_point(5.0, 6.0, 7.0);
    assert_eq!(p1 - p2, Tuple::new_vector(-2.0, -4.0, -6.0));
}

#[test]
fn subtracting_a_vector_from_a_point() {
    let p = Tuple::new_point(3., 2., 1.);
    let v = Tuple::new_vector(5., 6., 7.);
    assert_eq!(p - v, Tuple::new_point(-2.0, -4.0, -6.0));
}

#[test]
fn subtracting_two_vectors() {
    let v1 = Tuple::new_vector(3.0, 2.0, 1.0);
    let v2 = Tuple::new_vector(5.0, 6.0, 7.0);
    assert_eq!(v1 - v2, Tuple::new_vector(-2.0, -4.0, -6.0));
}

#[test]
fn subtracting_a_vector_from_the_zero_vector() {
    let zero = Tuple::new_vector(0.0, 0.0, 0.0);
    let v = Tuple::new_vector(1.0, -2.0, 3.0);
    assert_eq!(zero - v, Tuple::new_vector(-1.0, 2.0, -3.0));
}

#[test]
fn negating_a_tuple() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0));
}
