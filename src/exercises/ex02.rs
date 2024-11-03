use std::ops::{Add, Mul, Sub};

pub fn lerp<T: Clone + Add<Output = T> + Sub<Output = T> + Mul<f32, Output = T>>(
    u: T,
    v: T,
    t: f32,
) -> T {
    let res = u.clone() + ((v - u) * t);
    res
}

#[cfg(test)]
mod lerp {
    use crate::base_structs::vector::Vector;
    use crate::base_structs::matrix::Matrix;
    use crate::exercises::ex02::lerp;

    #[test]
    fn test_numbers() {
        assert_eq!(lerp(0., 1., 0.), 0.0);
        assert_eq!(lerp(0., 1., 1.), 1.0);
        assert_eq!(lerp(0., 1., 0.5), 0.5);
        assert_eq!(lerp(21., 42., 0.3), 27.3);
        assert_eq!(lerp(10., 20., 0.), 10.0);
        assert_eq!(lerp(10., 20., 1.), 20.0);
        assert_eq!(lerp(10., 20., 0.5), 15.0);
        assert_eq!(lerp(-10., 10., 0.5), 0.0);
        assert_eq!(lerp(5., 15., 0.2), 7.0);
        assert_eq!(lerp(5., 15., 0.8), 13.0);
        assert_eq!(lerp(5., 15., 0.3), 8.0);
        assert_eq!(lerp(-5., 5., 0.5), 0.0);
    }

    #[test]
    fn test_vectors() {
        assert_eq!(
            lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3),
            Vector::from([2.6, 1.3])
        );
        assert_eq!(
            lerp(Vector::from([1., 2.]), Vector::from([3., 4.]), 0.5),
            Vector::from([2., 3.])
        );
        assert_eq!(
            lerp(Vector::from([0., 0.]), Vector::from([10., 10.]), 0.1),
            Vector::from([1., 1.])
        );
        assert_eq!(
            lerp(Vector::from([5., 5.]), Vector::from([15., 15.]), 0.5),
            Vector::from([10., 10.])
        );
        assert_eq!(
            lerp(Vector::from([-5., -5.]), Vector::from([5., 5.]), 0.5),
            Vector::from([0., 0.])
        );
    }

    #[test]
    fn test_matrices() {
        assert_eq!(
            lerp(
                Matrix::from([[2., 1.], [3., 4.]]),
                Matrix::from([[20., 10.], [30., 40.]]),
                0.5
            ),
            Matrix::from([[11., 5.5], [16.5, 22.]])
        );
        assert_eq!(
            lerp(
                Matrix::from([[1., 2.], [3., 4.]]),
                Matrix::from([[5., 6.], [7., 8.]]),
                0.25
            ),
            Matrix::from([[2., 3.], [4., 5.]])
        );
        assert_eq!(
            lerp(
                Matrix::from([[0., 0.], [0., 0.]]),
                Matrix::from([[10., 10.], [10., 10.]]),
                0.1
            ),
            Matrix::from([[1., 1.], [1., 1.]])
        );
        assert_eq!(
            lerp(
                Matrix::from([[5., 5.], [5., 5.]]),
                Matrix::from([[15., 15.], [15., 15.]]),
                0.5
            ),
            Matrix::from([[10., 10.], [10., 10.]])
        );
        assert_eq!(
            lerp(
                Matrix::from([[-5., -5.], [-5., -5.]]),
                Matrix::from([[5., 5.], [5., 5.]]),
                0.5
            ),
            Matrix::from([[0., 0.], [0., 0.]])
        );
    }
}
