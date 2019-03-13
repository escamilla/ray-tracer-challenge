pub mod canvas;
pub mod color;
pub mod intersection;
pub mod light;
pub mod material;
pub mod matrix;
pub mod ray;
pub mod sphere;
pub mod tuple;
pub mod world;

const EPSILON: f32 = 0.00001;

fn equal_f32(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

fn clamp_i32(num: i32, min: i32, max: i32) -> i32 {
    if num < min {
        min
    } else if num > max {
        max
    } else {
        num
    }
}
