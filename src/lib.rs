pub mod canvas;
pub mod tuples;

const EPSILON: f32 = 0.00001;

fn f32_equal(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}
