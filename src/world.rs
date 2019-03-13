use crate::intersection::{find_hit, Intersection};
use crate::light::PointLight;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;

pub struct World {
    pub light: Option<PointLight>,
    pub objects: Vec<Sphere>,
}

impl World {
    pub fn new() -> Self {
        World {
            light: None,
            objects: vec![],
        }
    }

    pub fn default() -> Self {
        let light = PointLight::new(
            Tuple::point(-10.0, 10.0, -10.0),
            Tuple::color(1.0, 1.0, 1.0),
        );

        let mut s1 = Sphere::new();
        s1.material.color = Tuple::color(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::new();
        s2.transform = Matrix4::scaling(0.5, 0.5, 0.5);

        World {
            objects: vec![s1, s2],
            light: Some(light),
        }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections = Vec::new();
        for object in &self.objects {
            intersections.extend(object.intersect(ray));
        }
        intersections.sort();
        intersections
    }

    pub fn color_at(&self, ray: Ray) -> Tuple {
        let intersections = self.intersect(ray);
        let hit = find_hit(intersections);
        match hit {
            Some(mut intersection) => {
                intersection.prepare_hit(ray);
                intersection.shade_hit(self)
            }
            None => Tuple::color(0.0, 0.0, 0.0),
        }
    }
}

#[test]
fn test_creating_a_world() {
    let w = World::new();
    assert!(w.objects.is_empty());
    assert!(w.light.is_none());
}

#[test]
fn test_the_default_world() {
    let light = PointLight::new(
        Tuple::point(-10.0, 10.0, -10.0),
        Tuple::color(1.0, 1.0, 1.0),
    );
    let mut s1 = Sphere::new();
    s1.material.color = Tuple::color(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;
    let mut s2 = Sphere::new();
    s2.transform = Matrix4::scaling(0.5, 0.5, 0.5);
    let w = World::default();
    assert!(w.light.is_some());
    assert_eq!(w.light.unwrap(), light);
    assert!(w.objects.contains(&s1));
    assert!(w.objects.contains(&s2));
}

#[test]
fn test_intersect_a_world_with_a_ray() {
    let w = World::default();
    let r =
        Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let xs = w.intersect(r);
    assert_eq!(xs.len(), 4);
    assert_eq!(xs[0].t, 4.0);
    assert_eq!(xs[1].t, 4.5);
    assert_eq!(xs[2].t, 5.5);
    assert_eq!(xs[3].t, 6.0);
}

#[test]
fn test_the_color_when_a_ray_misses() {
    let w = World::default();
    let r =
        Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));
    let c = w.color_at(r);
    assert_eq!(c, Tuple::color(0.0, 0.0, 0.0));
}

#[test]
fn test_the_color_when_a_ray_hits() {
    let w = World::default();
    let r =
        Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let c = w.color_at(r);
    assert_eq!(c, Tuple::color(0.38066, 0.47583, 0.2855));
}

#[test]
fn test_the_color_with_an_intersection_behind_the_ray() {
    let mut w = World::default();
    w.objects[0].material.ambient = 1.0;
    w.objects[1].material.ambient = 1.0;
    let inner = w.objects[1];
    let r =
        Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));
    let c = w.color_at(r);
    assert_eq!(c, inner.material.color);
}
