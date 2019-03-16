use crate::color::Color;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Material {
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub color: Color,
}

impl Default for Material {
    fn default() -> Material {
        Material {
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            color: Color::white(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::material::Material;

    #[test]
    fn test_the_default_material() {
        let m = Material::default();
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }
}
