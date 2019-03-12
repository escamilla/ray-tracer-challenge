use super::intersection::Intersection;
use super::matrix::Matrix4;
use super::ray::Ray;
use super::tuple::Tuple;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Sphere {
    pub origin: Tuple,
    pub radius: f32,
    pub transform: Matrix4,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            origin: Tuple::point(0.0, 0.0, 0.0),
            radius: 1.0,
            transform: Matrix4::identity(),
        }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let transformed_ray = ray.transform(self.transform.inverse());
        let sphere_to_ray = transformed_ray.origin - self.origin;
        let a = transformed_ray.direction.dot(transformed_ray.direction);
        let b = 2.0 * transformed_ray.direction.dot(sphere_to_ray);
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

#[test]
fn test_a_spheres_default_transformation() {
    let s = Sphere::new();
    assert_eq!(s.transform, Matrix4::identity());
}

#[test]
fn test_changing_a_spheres_transformation() {
    let mut s = Sphere::new();
    let t = Matrix4::translation(2.0, 3.0, 4.0);
    s.transform = t;
    assert_eq!(s.transform, t);
}

#[test]
fn test_intersecting_a_scaled_sphere_with_a_ray() {
    let r =
        Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let mut s = Sphere::new();
    s.transform = Matrix4::scaling(2.0, 2.0, 2.0);
    let xs = s.intersect(r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 3.0);
    assert_eq!(xs[1].t, 7.0);
}

#[test]
fn test_intersecting_a_translated_sphere_with_a_ray() {
    let r =
        Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let mut s = Sphere::new();
    s.transform = Matrix4::translation(5.0, 0.0, 0.0);
    let xs = s.intersect(r);
    assert!(xs.is_empty());
}
