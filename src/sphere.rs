use super::intersection::Intersection;
use super::ray::Ray;
use super::tuple::Tuple;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Sphere {
    pub origin: Tuple,
    pub radius: f32,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            origin: Tuple::point(0.0, 0.0, 0.0),
            radius: 1.0,
        }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin - self.origin;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = (b * b) - (4.0 * a * c);
        if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            if t1 < t2 {
                vec![Intersection::new(t1, *self), Intersection::new(t2, *self)]
            } else {
                vec![Intersection::new(t2, *self), Intersection::new(t1, *self)]
            }
        }
    }
}

#[test]
fn test_a_ray_intersects_a_sphere_at_two_points() {
    let r =
        Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(r);
    assert!(!xs.is_empty());
    assert_eq!(xs[0], Intersection::new(4.0, s));
    assert_eq!(xs[1], Intersection::new(6.0, s));
}

#[test]
fn test_a_ray_intersects_a_sphere_at_a_tangent() {
    let r =
        Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(r);
    assert!(!xs.is_empty());
    assert_eq!(xs[0], Intersection::new(5.0, s));
    assert_eq!(xs[1], Intersection::new(5.0, s));
}

#[test]
fn test_a_ray_misses_a_sphere() {
    let r =
        Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(r);
    assert!(xs.is_empty());
}

#[test]
fn test_a_ray_originates_inside_a_sphere() {
    let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(r);
    assert!(!xs.is_empty());
    assert_eq!(xs[0], Intersection::new(-1.0, s));
    assert_eq!(xs[1], Intersection::new(1.0, s));
}

#[test]
fn test_a_sphere_is_behind_a_ray() {
    let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(r);
    assert!(!xs.is_empty());
    assert_eq!(xs[0], Intersection::new(-6.0, s));
    assert_eq!(xs[1], Intersection::new(-4.0, s));
}
