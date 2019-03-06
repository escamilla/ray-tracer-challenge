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

#[test]
fn tuple_with_w_of_1_is_a_point() {
    let tuple = Tuple::new(4.3, -4.2, 3.1, 1.0);
    assert_eq!(tuple.x, 4.3);
    assert_eq!(tuple.y, -4.2);
    assert_eq!(tuple.z, 3.1);
    assert_eq!(tuple.w, 1.0);
    assert!(tuple.is_point());
    assert!(!tuple.is_vector());
}

#[test]
fn tuple_with_w_of_0_is_a_vector() {
    let tuple = Tuple::new(4.3, -4.2, 3.1, 0.0);
    assert_eq!(tuple.x, 4.3);
    assert_eq!(tuple.y, -4.2);
    assert_eq!(tuple.z, 3.1);
    assert_eq!(tuple.w, 0.0);
    assert!(tuple.is_vector());
    assert!(!tuple.is_point());
}

#[test]
fn point_function_creates_tuples_with_w_of_1() {
    let tuple = Tuple::new_point(4.0, -4.0, 3.0);
    assert_eq!(tuple.w, 1.0);
    assert!(tuple.is_point());
    assert!(!tuple.is_vector());
}

#[test]
fn vector_function_creates_tuples_with_w_of_0() {
    let tuple = Tuple::new_vector(4.0, -4.0, 3.0);
    assert_eq!(tuple.w, 0.0);
    assert!(tuple.is_vector());
    assert!(!tuple.is_point());
}
