use crate::num_traits::scalar::Scalar;
use crate::base_structs::matrix::Matrix;
use crate::base_structs::vector::Vector;


impl<T: Scalar, const M: usize, const N: usize> Matrix<T, M, N>{
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

    pub fn mul_vec(&mut self, rhs: &Vector<T, N>) -> Vector<T, N> {
        let mut res = [T::zero(); N];
        let d = self.as_arr();
        for j in 0..M {
            let mut sum: T = T::zero();
            for i in 0..N {
                println!("i{}", i);
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
    fn test_mul() {
        let mut u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(u.mul_vec(&v), Vector::from([4., 2.]));

        let mut u = Matrix::from([[2., 0.], [0., 2.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(u.mul_vec(&v), Vector::from([8., 4.]));

        let mut u = Matrix::from([[2., -2.], [-2., 2.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(u.mul_vec(&v), Vector::from([4., -4.]));

        let mut u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[1., 0.], [0., 1.]]);
        assert_eq!(u.mul_mat(&v), Matrix::from([[1., 0.], [0., 1.]]));

        let mut u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[2., 1.], [4., 2.]]);
        assert_eq!(u.mul_mat(&v), Matrix::from([[2., 1.], [4., 2.]]));

        let mut u = Matrix::from([[3., -5.], [6., 8.]]);
        let v = Matrix::from([[2., 1.], [4., 2.]]);
        assert_eq!(u.mul_mat(&v), Matrix::from([[-14., -7.], [44., 22.]]));
    }
}
