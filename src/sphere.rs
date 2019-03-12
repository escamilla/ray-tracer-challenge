use super::tuple::Tuple;

#[derive(Copy, Clone, Debug)]
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
}
