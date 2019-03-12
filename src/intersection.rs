use super::ray::Ray;
use super::sphere::Sphere;
use super::tuple::Tuple;
use std::cmp::Ordering;

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

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.t < other.t {
            Ordering::Less
        } else if self.t > other.t {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Intersection {}

pub fn find_hit(intersections: Vec<Intersection>) -> Option<Intersection> {
    intersections.iter().filter(|i| i.t >= 0.0).min().cloned()
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

#[test]
fn test_the_hit_when_all_intersections_have_positive_t() {
    let s = Sphere::new();
    let i1 = Intersection::new(1.0, s);
    let i2 = Intersection::new(2.0, s);
    let xs = vec![i2, i1];
    let i = find_hit(xs);
    assert!(i.is_some());
    assert_eq!(i.unwrap(), i1);
}

#[test]
fn test_the_hit_when_some_intersections_have_negative_t() {
    let s = Sphere::new();
    let i1 = Intersection::new(-1.0, s);
    let i2 = Intersection::new(1.0, s);
    let xs = vec![i2, i1];
    let i = find_hit(xs);
    assert!(i.is_some());
    assert_eq!(i.unwrap(), i2);
}

#[test]
fn test_the_hit_when_all_intersections_have_negative_t() {
    let s = Sphere::new();
    let i1 = Intersection::new(-2.0, s);
    let i2 = Intersection::new(-1.0, s);
    let xs = vec![i2, i1];
    let i = find_hit(xs);
    assert!(i.is_none());
}

#[test]
fn test_the_hit_is_always_the_lowest_nonnegative_intersection() {
    let s = Sphere::new();
    let i1 = Intersection::new(5.0, s);
    let i2 = Intersection::new(7.0, s);
    let i3 = Intersection::new(-3.0, s);
    let i4 = Intersection::new(2.0, s);
    let xs = vec![i1, i2, i3, i4];
    let i = find_hit(xs);
    assert!(i.is_some());
    assert_eq!(i.unwrap(), i4);
}
