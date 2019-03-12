use super::intersection::Intersection;
use super::matrix::Matrix4;
use super::ray::Ray;
use super::tuple::Tuple;
use std::f32::consts::{PI, SQRT_2};

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
                vec![
                    Intersection::new(t1, self.clone()),
                    Intersection::new(t2, self.clone()),
                ]
            } else {
                vec![
                    Intersection::new(t2, self.clone()),
                    Intersection::new(t1, self.clone()),
                ]
            }
        }
    }

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = self.transform.inverse() * world_point;
        let object_normal = object_point - self.origin;
        let mut world_normal =
            self.transform.inverse().transpose() * object_normal;
        world_normal.w = 0.0;
        world_normal.normalize()
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

#[test]
fn test_the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
    let s = Sphere::new();
    let n = s.normal_at(Tuple::point(1.0, 0.0, 0.0));
    assert_eq!(n, Tuple::vector(1.0, 0.0, 0.0));
}

#[test]
fn test_the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
    let s = Sphere::new();
    let n = s.normal_at(Tuple::point(0.0, 1.0, 0.0));
    assert_eq!(n, Tuple::vector(0.0, 1.0, 0.0));
}

#[test]
fn test_the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
    let s = Sphere::new();
    let n = s.normal_at(Tuple::point(0.0, 0.0, 1.0));
    assert_eq!(n, Tuple::vector(0.0, 0.0, 1.0));
}

#[test]
fn test_the_normal_on_a_sphere_at_a_nonaxial_point() {
    let sqrt3_over_3 = (3.0 as f32).sqrt() / 3.0;
    let s = Sphere::new();
    let n = s.normal_at(Tuple::point(sqrt3_over_3, sqrt3_over_3, sqrt3_over_3));
    assert_eq!(n, Tuple::vector(sqrt3_over_3, sqrt3_over_3, sqrt3_over_3));
}

#[test]
fn test_the_normal_is_a_normalized_vector() {
    let sqrt3_over_3 = (3.0 as f32).sqrt() / 3.0;
    let s = Sphere::new();
    let n = s.normal_at(Tuple::point(sqrt3_over_3, sqrt3_over_3, sqrt3_over_3));
    assert_eq!(n, n.normalize());
}

#[test]
fn test_computing_the_normal_on_a_translated_sphere() {
    let mut s = Sphere::new();
    s.transform = Matrix4::translation(0.0, 1.0, 0.0);
    let n = s.normal_at(Tuple::point(0.0, 1.70711, -0.70711));
    assert_eq!(n, Tuple::vector(0.0, 0.70711, -0.70711));
}

#[test]
fn test_computing_the_normal_on_a_transformed_sphere() {
    let mut s = Sphere::new();
    let m = Matrix4::scaling(1.0, 0.5, 1.0) * Matrix4::rotation_z(PI / 5.0);
    s.transform = m;
    let n = s.normal_at(Tuple::point(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0));
    assert_eq!(n, Tuple::vector(0.0, 0.97014, -0.24254));
}
