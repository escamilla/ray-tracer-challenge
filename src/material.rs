use crate::tuple::Tuple;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Material {
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub color: Tuple,
}

impl Material {
    pub fn new() -> Self {
        Material {
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            color: Tuple::color(1.0, 1.0, 1.0),
        }
    }
}

#[test]
fn test_the_default_material() {
    let m = Material::new();
    assert_eq!(m.ambient, 0.1);
    assert_eq!(m.diffuse, 0.9);
    assert_eq!(m.specular, 0.9);
    assert_eq!(m.shininess, 200.0);
}
