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
    fn test00() {
        assert_eq!(lerp(0., 1., 0.), 0.0);
        assert_eq!(lerp(0., 1., 1.), 1.0);
        assert_eq!(lerp(0., 1., 0.5), 0.5);
        assert_eq!(lerp(21., 42., 0.3), 27.3);
        assert_eq!(
            lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3),
            Vector::from([2.6, 1.3])
        );
        assert_eq!(
            lerp(
                Matrix::from([[2., 1.], [3., 4.]]),
                Matrix::from([[20., 10.], [30., 40.]]),
                0.5
            ),
            Matrix::from([[11., 5.5], [16.5, 22.]])
        );
    }
}
