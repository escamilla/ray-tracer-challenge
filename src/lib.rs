pub mod canvas;
pub mod matrix;
pub mod tuple;

const EPSILON: f32 = 0.00001;

fn f32_equal(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

fn i32_clamp(num: i32, min: i32, max: i32) -> i32 {
    if num < min {
        min
    } else if num > max {
        max
    } else {
        num
    }
}
