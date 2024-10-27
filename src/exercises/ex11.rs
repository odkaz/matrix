use crate::base_structs::matrix::TMatrix;
use crate::num_traits::scalar::Scalar;

impl<T: Scalar, const M: usize> TMatrix<T, M> {
    fn _deter(&self, data: Vec<Vec<T>>, size: usize) -> T {
        if size == 1 {
            return data[0][0];
        }
        if size == 2 {
            return (data[0][0] * data[1][1]) - (data[0][1] * data[1][0]);
        }
        let mut res = T::zero();
        for i in 0..size {
            let coef = data[0][i];
            let mut sign = T::one();
            if i % 2 == 1 {
                sign = -T::one();
            }
            let mut vec2d = Vec::new();
            for o in 1..size {
                let mut v = Vec::new();
                for l in 0..size {
                    if l != i {
                        v.push(data[o][l]);
                    }
                }
                vec2d.push(v);
            }
            let rhs = coef * sign * self._deter(vec2d, size - 1);
            res = res + rhs;
        }
        res
    }

    pub fn determinant(&mut self) -> T {
        self._deter(self.data.clone(), M)
    }
}

#[cfg(test)]
mod determinant {
    use crate::base_structs::matrix::Matrix;

    #[test]
    fn test_determinant_1x1() {
        let mut matrix = Matrix::from([[1.0]]);
        assert_eq!(matrix.determinant(), 1.0);
    }

    #[test]
    fn test_determinant_2x2() {
        let mut matrix = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        assert_eq!(matrix.determinant(), -2.0);
    }

    #[test]
    fn test_determinant_3x3() {
        let mut matrix = Matrix::from([[6.0, 1.0, 1.0], [4.0, -2.0, 5.0], [2.0, 8.0, 7.0]]);
        assert_eq!(matrix.determinant(), -306.0);
    }

    #[test]
    fn test_determinant_4x4() {
        let mut matrix = Matrix::from([
            [1.0, 0.0, 2.0, -1.0],
            [3.0, 0.0, 0.0, 5.0],
            [2.0, 1.0, 4.0, -3.0],
            [1.0, 0.0, 5.0, 0.0],
        ]);
        assert_eq!(matrix.determinant(), 30.0);
    }

    #[test]
    fn test_determinant_zero_matrix() {
        let mut matrix = Matrix::from([[0.0, 0.0], [0.0, 0.0]]);
        assert_eq!(matrix.determinant(), 0.0);
    }

    #[test]
    fn test_determinant_identity_matrix() {
        let mut matrix = Matrix::from([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
        assert_eq!(matrix.determinant(), 1.0);
    }
}
