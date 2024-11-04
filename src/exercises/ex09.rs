use crate::base_structs::matrix::Matrix;
use crate::num_traits::scalar::Scalar;

impl<T: Scalar, const M: usize, const N: usize> Matrix<T, M, N> {
    //useful for changing rows to columns and columns to rows
    //opposite rotation matrix can be made by transposing the matrix, useful in computer graphics
    //can solve simultaneous equations for example
    pub fn transpose(&mut self) -> Matrix<T, N, M> {
        let mut res = Vec::new();
        for j in 0..N {
            let mut v = Vec::new();
            for i in 0..M {
                v.push(self.data[i][j]);
            }
            res.push(v);
        }
        Matrix { data: res }
    }
}

#[cfg(test)]
mod transpose {
    use super::*;

    #[test]
    fn test_transpose_1() {
        let mut u = Matrix::from([[1., 0.], [0., 1.]]);
        assert_eq!(u.transpose(), Matrix::from([[1., 0.], [0., 1.]]));
    }
    #[test]
    fn test_transpose_2() {
        let mut u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
        assert_eq!(
            u.transpose(),
            Matrix::from([[2., 4., -2.], [-5., 3., 3.], [0., 7., 4.]])
        );
    }
    #[test]
    fn test_transpose_3() {
        let mut u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
        assert_eq!(
            u.transpose(),
            Matrix::from([[-2., 1., 0.], [-8., -23., 6.], [4., 4., 4.]])
        );
    }

    #[test]
    fn test_transpose_4() {
        let mut u = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);
        assert_eq!(
            u.transpose(),
            Matrix::from([[1., 4.], [2., 5.], [3., 6.]])
        );
    }

    #[test]
    fn test_transpose_5() {
        let mut u = Matrix::from([[7., 8.], [9., 10.], [11., 12.]]);
        assert_eq!(
            u.transpose(),
            Matrix::from([[7., 9., 11.], [8., 10., 12.]])
        );
    }

    #[test]
    fn test_transpose_6() {
        let mut u = Matrix::from([[1.]]);
        assert_eq!(u.transpose(), Matrix::from([[1.]]));
    }

    #[test]
    fn test_transpose_7() {
        let mut u = Matrix::from([[1., 2.], [3., 4.], [5., 6.], [7., 8.]]);
        assert_eq!(
            u.transpose(),
            Matrix::from([[1., 3., 5., 7.], [2., 4., 6., 8.]])
        );
    }
}
