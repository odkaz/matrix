use crate::base_structs::matrix::Matrix;
use crate::base_structs::matrix::TMatrix;
use crate::num_traits::scalar::Scalar;

impl<T: Scalar, const M: usize> TMatrix<T, M> {
    fn _identity() -> [[T; M]; M] {
        let mut arr = [[T::zero(); M]; M];
        for i in 0..M {
            arr[i][i] = T::one();
        }
        arr
    }

    fn _swap_row(mat: &mut [[T; M]; M], x: usize, y: usize) {
        for i in 0..M {
            let tmp = mat[y][i];
            mat[y][i] = mat[x][i];
            mat[x][i] = tmp;
        }
    }

    pub fn inverse(&mut self) -> Result<TMatrix<T, M>, String> {
        let det = self.determinant();
        if det == T::zero() {
            panic!("determinant is zero")
        }
        let mut res = TMatrix::<T, M>::_identity();
        let mut d = self.as_arr();

        //set the row so that the pivot is different than zero
        for i in 0..M {
            if d[i][i] == T::zero() {
                let mut big = i;
                for j in 0..M {
                    if T::abs(&d[j][i]) > T::abs(&d[big][i]) {
                        big = j;
                    }
                }
                if big == i {
                    panic!("singular matrix");
                }
                TMatrix::<T, M>::_swap_row(&mut res, i, big);
                TMatrix::<T, M>::_swap_row(&mut d, i, big);
            }
        }

        //eliminate all numbers under the diagonal element
        for col in 0..M - 1 {
            for row in col + 1..M {
                let k = d[row][col] / d[col][col];
                for l in 0..M {
                    d[row][l] = d[row][l] - k * d[col][l];
                    res[row][l] = res[row][l] - k * res[col][l];
                }
                d[row][col] = T::zero();
            }
        }

        //scale all the pivots coefficients to 1
        for row in 0..M {
            let div = d[row][row];
            for col in 0..M {
                d[row][col] = d[row][col] / div;
                res[row][col] = res[row][col] / div;
            }
        }

        //eliminate all numbers above the diagonal
        for row in 0..M {
            for col in row + 1..M {
                let k = d[row][col];
                for l in 0..M {
                    d[row][l] = d[row][l] - d[col][l] * k;
                    res[row][l] = res[row][l] - res[col][l] * k;
                }
                d[row][col] = T::zero();
            }
        }
        Ok(Matrix::from(res))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverse_identity() {
        let mut identity_matrix = Matrix::from([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
        let inverse_matrix = identity_matrix.inverse().unwrap();
        assert_eq!(identity_matrix, inverse_matrix);
    }

    #[test]
    fn test_inverse_simple() {
        let mut matrix = Matrix::from([[4.0, 7.0], [2.0, 6.0]]);
        let expected_inverse = Matrix::from([[0.6, -0.7], [-0.2, 0.4]]);
        let inverse_matrix = matrix.inverse().unwrap();
        assert_eq!(inverse_matrix, expected_inverse);
    }

    #[test]
    #[should_panic(expected = "determinant is zero")]
    fn test_inverse_singular() {
        let mut singular_matrix = Matrix::from([[1.0, 2.0], [2.0, 4.0]]);
        singular_matrix.inverse().unwrap();
    }

    #[test]
    fn test_inverse_non_square() {
        // This test should fail to compile because the matrix is not square
        // let mut non_square_matrix = TMatrix::<f64, 2, 3>::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        // non_square_matrix.inverse().unwrap();
    }

    #[test]
    fn test_inverse_large_matrix() {
        let mut matrix = Matrix::from([[3.0, 0.0, 2.0], [2.0, 0.0, -2.0], [0.0, 1.0, 1.0]]);
        let expected_inverse = Matrix::from([[0.2, 0.2, 0.0], [-0.2, 0.3, 1.0], [0.2, -0.3, 0.0]]);
        let inverse_matrix = matrix.inverse().unwrap();
        assert_eq!(inverse_matrix, expected_inverse);
    }
}
