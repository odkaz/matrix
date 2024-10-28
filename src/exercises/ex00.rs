#[cfg(test)]
mod arithmetic {
    use crate::base_structs::matrix::Matrix;
    use crate::base_structs::vector::Vector;

    #[test]
    fn test_vector_addition() {
        let mut u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        u.add(&v);
        assert_eq!(u, Vector::from([7., 10.]));
    }

    #[test]
    fn test_vector_subtraction() {
        let mut u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        u.sub(&v);
        assert_eq!(u, Vector::from([-3., -4.]));
    }

    #[test]
    fn test_vector_scalar_multiplication() {
        let mut u = Vector::from([2., 3.]);
        u.scl(2.);
        assert_eq!(u, Vector::from([4., 6.]));
    }

    #[test]
    fn test_matrix_addition() {
        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        u.add(&v);
        assert_eq!(u, Matrix::from([[8., 6.], [1., 6.]]));
    }

    #[test]
    fn test_matrix_subtraction() {
        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        u.sub(&v);
        assert_eq!(u, Matrix::from([[-6., -2.], [5., 2.]]));
    }

    #[test]
    fn test_matrix_scalar_multiplication() {
        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        u.scl(2.);
        assert_eq!(u, Matrix::from([[2., 4.], [6., 8.]]));
    }
}
