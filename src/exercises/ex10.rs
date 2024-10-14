use crate::base_structs::matrix::Matrix;
use crate::num_traits::scalar::Scalar;

impl<T: Scalar, const M: usize, const N: usize> Matrix<T, M, N> {
    fn is_all_zero(&self, index: usize) -> bool {
        for item in &self.data[index] {
            if item != &T::zero() {
                return false;
            }
        }
        true
    }
    //reduced row echelon form
    pub fn row_echelon(&mut self) -> Matrix<T, M, N> {
        let mut lead: usize = 0;
        for j in 0..M {
            if self.is_all_zero(j) {
                for k in 0..M - j {
                    let index = M - k - 1;
                    let l = self.as_vector(index);
                    if !self.is_all_zero(index) {
                        self.data[index] = self.data[j].clone();
                        self.data[j] = l.as_vec();
                        break;
                    }
                }
            }
            for i in 0..N {
                let mut v = self.as_vector(j);

                //find the lead
                if self.data[j][i] != T::zero() && i >= lead {
                    //lead to 1
                    v = v.clone() / v[i];
                    self.data[j] = v.as_vec();

                    //pivot column to 0
                    for x in 0..M {
                        if x == j {
                            continue;
                        }
                        let coef = self.data[x][i];
                        if coef != T::zero() {
                            let mut tmp = self.as_vector(x);
                            tmp = tmp.clone() - v.clone() * coef;
                            self.data[x] = tmp.as_vec();
                        }
                    }
                    lead = i;
                    break;
                }
            }
        }
        Matrix {
            data: self.data.clone(),
        }
    }
}

#[test]
fn test_row_echelon_identity_matrix() {
    let mut matrix = Matrix::from([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
    let result = matrix.row_echelon();
    assert_eq!(
        result,
        Matrix::from([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0],])
    );
}

#[test]
fn test_row_echelon_non_square_matrix() {
    let mut matrix = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
    let result = matrix.row_echelon();
    assert_eq!(result, Matrix::from([[1.0, 0.0], [0.0, 1.0],]));
}

#[test]
fn test_row_echelon_singular_matrix() {
    let mut matrix = Matrix::from([[1.0, 2.0], [2.0, 4.0]]);
    let result = matrix.row_echelon();
    assert_eq!(result, Matrix::from([[1.0, 2.0], [0.0, 0.0],]));
}

#[test]

fn test_row_echelon_larger_matrix() {
    const EPSILON: f32 = 1e-6;

    fn floats_are_equal(a: f32, b: f32) -> bool {
        (a - b).abs() < EPSILON
    }

    fn matrices_are_equal<const M: usize, const N: usize>(
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
    let mut matrix = Matrix::from([
        [8.0, 5.0, -2.0, 4.0, 28.0],
        [4.0, 2.5, 20.0, 4.0, -4.0],
        [8.0, 5.0, 1.0, 4.0, 17.0],
    ]);
    let result = matrix.row_echelon();

    let expected = Matrix::from([
        [1.0, 0.625, 0.0, 0.0, -12.1666667],
        [0.0, 0.0, 1.0, 0.0, -3.6666667],
        [0.0, 0.0, 0.0, 1.0, 29.5],
    ]);

    assert!(
        matrices_are_equal(&result, &expected),
        "The matrices are not approximately equal: {:?} vs {:?}",
        result,
        expected
    );
}
