use crate::base_structs::matrix::Matrix;
use crate::num_traits::scalar::Scalar;

impl<T: Scalar, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn rank(&mut self) -> usize {
        let mat = self.row_echelon().as_arr();
        let mut res: usize = 0;
        for m in 0..M {
            for n in m..N {
                if mat[m][n] != T::zero() {
                    res = res + 1;
                    break;
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_zero_matrix() {
        let mut matrix = Matrix::from([[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]);
        assert_eq!(matrix.rank(), 0);
    }

    #[test]
    fn test_rank_identity_matrix() {
        let mut matrix = Matrix::from([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
        assert_eq!(matrix.rank(), 3);
    }

    #[test]
    fn test_rank_rectangular_matrix() {
        let mut matrix = Matrix::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        assert_eq!(matrix.rank(), 2);
    }

    #[test]
    fn test_rank_singular_matrix() {
        let mut matrix = Matrix::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        assert_eq!(matrix.rank(), 2);
    }

    #[test]
    fn test_rank_non_square_matrix() {
        let mut matrix = Matrix::from([[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]);
        assert_eq!(matrix.rank(), 2);
    }
}
