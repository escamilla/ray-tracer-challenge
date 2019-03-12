use super::ray::Ray;
use super::sphere::Sphere;
use super::tuple::Tuple;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Intersection {
    pub t: f32,
    pub object: Sphere,
}

impl Intersection {
    pub fn new(t: f32, object: Sphere) -> Self {
        Intersection { t, object }
    }
}

#[test]
fn test_an_intersection_encapsulates_t_and_an_object() {
    let s = Sphere::new();
    let i = Intersection::new(3.5, s);
    assert_eq!(i.t, 3.5);
    assert_eq!(i.object, s);
}

#[test]
fn test_aggregating_intersections() {
    let s = Sphere::new();
    let i1 = Intersection::new(1.0, s);
    let i2 = Intersection::new(2.0, s);
    let intersections = vec![i1, i2];
    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].t, 1.0);
    assert_eq!(intersections[1].t, 2.0);
}

#[test]
fn test_intersect_sets_the_object_on_the_intersection() {
    let r =
        Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].object, s);
    assert_eq!(xs[1].object, s);
}
