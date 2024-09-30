use crate::base_structs::matrix::Matrix;
use crate::num_traits::scalar::Scalar;

impl<T: Scalar, const N: usize> Matrix<T, N, N> {
    pub fn trace(&mut self) -> T {
        let mut res = T::zero();
        for i in 0..N {
            res = res + self.data[i][i];
        }
        res
    }
}
#[cfg(test)]
mod trace {
    // use crate::base_structs::matrix::Matrix;
    use super::*;

    #[test]
    fn test_trace_1() {
        let mut u = Matrix::from([[1., 0.], [0., 1.]]);
        assert_eq!(u.trace(), 2.);
    }
    #[test]
    fn test_trace_2() {
        let mut u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
        assert_eq!(u.trace(), 9.);
    }
    #[test]
    fn test_trace_3() {
        let mut u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
        assert_eq!(u.trace(), -21.);
    }
}
