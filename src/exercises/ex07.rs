use crate::num_traits::scalar::Scalar;
use crate::base_structs::matrix::Matrix;
use crate::base_structs::vector::Vector;


impl<T: Scalar, const M: usize, const N: usize> Matrix<T, M, N>{
    //alteratively we can use mul_vec for each columns of the matrix
    pub fn mul_mat<const H: usize>(&mut self, rhs: &Matrix<T, N, H>) -> Matrix<T, M, H> {
        let mut res = [[T::zero(); H]; M];
        let d = self.as_arr();
        let r = rhs.clone().as_arr();
        for j in 0..M {
            for i in 0..H {
                let mut sum: T = T::zero();
                for k in 0..N {
                    sum = sum + (d[j][k] * r[k][i]);
                }
                res[j][i] = sum;
            }
        }
        Matrix::from(res)
    }
    //matrix shows where i hat and j hat are mapped to.
    //if you multiply a vector by a matrix, the vector is transformed by the matrix
    pub fn mul_vec(&mut self, rhs: &Vector<T, N>) -> Vector<T, N> {
        let mut res = [T::zero(); N];
        let d = self.as_arr();
        for j in 0..M {
            let mut sum: T = T::zero();
            for i in 0..N {
                sum = sum + (d[j][i] * rhs[i]);
            }
            res[j] = sum;
        }
        Vector::from(res)
    }
}

#[cfg(test)]
mod mul {
    use super::*;
    #[test]
    fn test_mul_vec_identity() {
        let mut u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(u.mul_vec(&v), Vector::from([4., 2.]));
    }

    #[test]
    fn test_mul_vec_scaling() {
        let mut u = Matrix::from([[2., 0.], [0., 2.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(u.mul_vec(&v), Vector::from([8., 4.]));
    }

    #[test]
    fn test_mul_vec_mixed() {
        let mut u = Matrix::from([[2., -2.], [-2., 2.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(u.mul_vec(&v), Vector::from([4., -4.]));
    }

    #[test]
    fn test_mul_mat_identity() {
        let mut u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[1., 0.], [0., 1.]]);
        assert_eq!(u.mul_mat(&v), Matrix::from([[1., 0.], [0., 1.]]));
    }

    #[test]
    fn test_mul_mat_simple() {
        let mut u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[2., 1.], [4., 2.]]);
        assert_eq!(u.mul_mat(&v), Matrix::from([[2., 1.], [4., 2.]]));
    }

    #[test]
    fn test_mul_mat_complex() {
        let mut u = Matrix::from([[3., -5.], [6., 8.]]);
        let v = Matrix::from([[2., 1.], [4., 2.]]);
        assert_eq!(u.mul_mat(&v), Matrix::from([[-14., -7.], [44., 22.]]));
    }
}
