use super::f32_equal;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Tuple { x, y, z, w }
    }

    pub fn point(x: f32, y: f32, z: f32) -> Self {
        Tuple::new(x, y, z, 1.0)
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Self {
        Tuple::new(x, y, z, 0.0)
    }

    pub fn color(red: f32, green: f32, blue: f32) -> Self {
        Tuple::vector(red, green, blue)
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn magnitude(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Tuple::new(
            self.x / magnitude,
            self.y / magnitude,
            self.z / magnitude,
            self.w / magnitude,
        )
    }

    pub fn dot(&self, other: Tuple) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
    }

    pub fn cross(&self, other: Tuple) -> Tuple {
        Tuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn red(&self) -> f32 {
        self.x
    }

    pub fn green(&self) -> f32 {
        self.y
    }

    pub fn blue(&self) -> f32 {
        self.z
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

impl Mul<f32> for Tuple {
    type Output = Tuple;

    fn mul(self, scalar: f32) -> Self {
        Tuple::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            self.w * scalar,
        )
    }
}

impl Mul<Tuple> for Tuple {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Self {
        Tuple::new(
            self.x * other.x,
            self.y * other.y,
            self.z * other.z,
            self.w * other.w,
        )
    }
}

impl Div<f32> for Tuple {
    type Output = Tuple;

    fn div(self, scalar: f32) -> Self {
        Tuple::new(
            self.x / scalar,
            self.y / scalar,
            self.z / scalar,
            self.w / scalar,
        )
    }
}

#[test]
fn test_a_tuple_with_w_equals_1_is_a_point() {
    let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert_eq!(a.w, 1.0);
    assert!(a.is_point());
    assert!(!a.is_vector());
}

#[test]
fn test_a_tuple_with_w_equals_0_is_a_vector() {
    let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert_eq!(a.w, 0.0);
    assert!(a.is_vector());
    assert!(!a.is_point());
}

#[test]
fn test_point_function_creates_tuples_with_w_equals_1() {
    let p = Tuple::point(4.0, -4.0, 3.0);
    assert_eq!(p.w, 1.0);
    assert!(p.is_point());
    assert!(!p.is_vector());
}

#[test]
fn test_vector_function_creates_tuples_with_w_equals_0() {
    let v = Tuple::vector(4.0, -4.0, 3.0);
    assert_eq!(v.w, 0.0);
    assert!(v.is_vector());
    assert!(!v.is_point());
}

#[test]
fn test_adding_two_tuples() {
    let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
    let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
    assert_eq!(a1 + a2, Tuple::new(1.0, 1.0, 6.0, 1.0));
}

#[test]
fn test_subtracting_two_points() {
    let p1 = Tuple::point(3.0, 2.0, 1.0);
    let p2 = Tuple::point(5.0, 6.0, 7.0);
    assert_eq!(p1 - p2, Tuple::vector(-2.0, -4.0, -6.0));
}

#[test]
fn test_subtracting_a_vector_from_a_point() {
    let p = Tuple::point(3., 2., 1.);
    let v = Tuple::vector(5., 6., 7.);
    assert_eq!(p - v, Tuple::point(-2.0, -4.0, -6.0));
}

#[test]
fn test_subtracting_two_vectors() {
    let v1 = Tuple::vector(3.0, 2.0, 1.0);
    let v2 = Tuple::vector(5.0, 6.0, 7.0);
    assert_eq!(v1 - v2, Tuple::vector(-2.0, -4.0, -6.0));
}

#[test]
fn test_subtracting_a_vector_from_the_zero_vector() {
    let zero = Tuple::vector(0.0, 0.0, 0.0);
    let v = Tuple::vector(1.0, -2.0, 3.0);
    assert_eq!(zero - v, Tuple::vector(-1.0, 2.0, -3.0));
}

#[test]
fn test_negating_a_tuple() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0));
}

#[test]
fn test_multiplying_a_tuple_by_a_scalar() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
}

#[test]
fn test_multiplying_a_tuple_by_a_fraction() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
}

#[test]
fn test_dividing_a_tuple_by_a_scalar() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
}

#[test]
fn test_computing_the_magnitude_of_vector_1_0_0() {
    let v = Tuple::vector(1.0, 0.0, 0.0);
    assert_eq!(v.magnitude(), 1.0);
}

#[test]
fn test_computing_the_magnitude_of_vector_0_1_0() {
    let v = Tuple::vector(0.0, 1.0, 0.0);
    assert_eq!(v.magnitude(), 1.0);
}

#[test]
fn test_computing_the_magnitude_of_vector_0_0_1() {
    let v = Tuple::vector(0.0, 0.0, 1.0);
    assert_eq!(v.magnitude(), 1.0);
}

#[test]
fn test_computing_the_magnitude_of_vector_1_2_3() {
    let v = Tuple::vector(1.0, 2.0, 3.0);
    assert!(f32_equal(v.magnitude(), (14.0 as f32).sqrt()));
}

#[test]
fn test_computing_the_magnitude_of_vector_neg_1_neg_2_neg_3() {
    let v = Tuple::vector(-1.0, -2.0, -3.0);
    assert!(f32_equal(v.magnitude(), (14.0 as f32).sqrt()));
}

#[test]
fn test_normalizing_vector_4_0_0_gives_1_0_0() {
    let v = Tuple::vector(4.0, 0.0, 0.0);
    assert_eq!(v.normalize(), Tuple::vector(1.0, 0.0, 0.0));
}

#[test]
fn test_normalizing_vector_1_2_3() {
    let v = Tuple::vector(1.0, 2.0, 3.0);
    // vector(1/sqrt(14), 2/sqrt(14), 3/sqrt(14))
    assert_eq!(v.normalize(), Tuple::vector(0.26726, 0.53452, 0.80178));
}

#[test]
fn test_the_magnitude_of_a_normalized_vector() {
    let v = Tuple::vector(1.0, 2.0, 3.0);
    let norm = v.normalize();
    assert!(f32_equal(norm.magnitude(), 1.0));
}

#[test]
fn test_the_dot_product_of_two_tuples() {
    let a = Tuple::vector(1.0, 2.0, 3.0);
    let b = Tuple::vector(2.0, 3.0, 4.0);
    assert!(f32_equal(a.dot(b), 20.0));
}

#[test]
fn test_the_cross_product_of_two_vectors() {
    let a = Tuple::vector(1.0, 2.0, 3.0);
    let b = Tuple::vector(2.0, 3.0, 4.0);
    assert_eq!(a.cross(b), Tuple::vector(-1.0, 2.0, -1.0));
    assert_eq!(b.cross(a), Tuple::vector(1.0, -2.0, 1.0));
}

#[test]
fn test_colors_are_red_green_blue_tuples() {
    let c = Tuple::color(-0.5, 0.4, 1.7);
    assert_eq!(c.red(), -0.5);
    assert_eq!(c.green(), 0.4);
    assert_eq!(c.blue(), 1.7);
}

#[test]
fn test_adding_colors() {
    let c1 = Tuple::color(0.9, 0.6, 0.75);
    let c2 = Tuple::color(0.7, 0.1, 0.25);
    assert_eq!(c1 + c2, Tuple::color(1.6, 0.7, 1.0));
}

#[test]
fn test_subtracting_colors() {
    let c1 = Tuple::color(0.9, 0.6, 0.75);
    let c2 = Tuple::color(0.7, 0.1, 0.25);
    assert_eq!(c1 - c2, Tuple::color(0.2, 0.5, 0.5));
}

#[test]
fn test_multiplying_a_color_by_a_scalar() {
    let c = Tuple::color(0.2, 0.3, 0.4);
    assert_eq!(c * 2.0, Tuple::color(0.4, 0.6, 0.8));
}

#[test]
fn test_multiplying_colors() {
    let c1 = Tuple::color(1.0, 0.2, 0.4);
    let c2 = Tuple::color(0.9, 1.0, 0.1);
    assert_eq!(c1 * c2, Tuple::color(0.9, 0.2, 0.04));
}
