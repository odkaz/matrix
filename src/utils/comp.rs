use crate::base_structs::matrix::Matrix;

const EPSILON: f32 = 1e-6;

fn floats_are_equal(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

pub fn matrices_are_equal<const M: usize, const N: usize>(
    a: &Matrix<f32, M, N>,
    b: &Matrix<f32, M, N>,
) -> bool {
    for i in 0..a.data.len() {
        for j in 0..a.data[i].len() {
            if !floats_are_equal(a.data[i][j], b.data[i][j]) {
                return false;
            }
        }
    }
    true
}
