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
            // Check if the current row is all zeros
            if self.is_all_zero(j) {
                // Find a non-zero row from the bottom and swap it with the current row
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

                // Find the lead (first non-zero element) in the current row
                if self.data[j][i] != T::zero() && i >= lead {
                    // Normalize the row so that the lead is 1
                    v = v.clone() / v[i];
                    self.data[j] = v.as_vec();

                    // Make all elements in the pivot column (except the lead) zero
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
        // Return the resulting matrix in row echelon form
        Matrix {
            data: self.data.clone(),
        }
    }
}

#[cfg(test)]
mod row_echelon {
    use super::*;
    use crate::utils::comp::matrices_are_equal;

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
}
