use crate::base_structs::vector::Vector;
use crate::num_traits::scalar::Scalar;
use std::ops::{Add, Div};

pub fn angle_cos<
    T: Scalar + Into<f32> + Add<f32, Output = f32> + Div<f32, Output = f32>,
    const N: usize,
>(
    u: &Vector<T, N>,
    v: &Vector<T, N>,
) -> f32 {
    let num = u.dot(v);
    let den = u.clone().norm() * v.clone().norm();
    let res = num / den;
    res.into()
}

#[cfg(test)]
mod cosine {
    use super::*;
    use crate::utils::comp::floats_are_equal;

    #[test]
    fn test_cos_parallel_vectors() {
        let u = Vector::from([1., 0.]);
        let v = Vector::from([1., 0.]);
        assert_eq!(angle_cos(&u, &v), 1.0);
    }

    #[test]
    fn test_cos_perpendicular_vectors() {
        let u = Vector::from([1., 0.]);
        let v = Vector::from([0., 1.]);
        assert_eq!(angle_cos(&u, &v), 0.0);
    }

    #[test]
    fn test_cos_opposite_vectors() {
        let u = Vector::from([-1., 1.]);
        let v = Vector::from([1., -1.]);
        assert!(floats_are_equal(angle_cos(&u, &v), -1.0));
    }

    #[test]
    fn test_cos_collinear_vectors() {
        let u = Vector::from([2., 1.]);
        let v = Vector::from([4., 2.]);
        assert_eq!(angle_cos(&u, &v), 1.0);
    }

    #[test]
    fn test_cos_arbitrary_vectors() {
        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);
        assert_eq!(angle_cos(&u, &v), 0.9746318);
    }

    #[test]
    fn test_cos_orthogonal_vectors() {
        let u = Vector::from([1., 0., 0.]);
        let v = Vector::from([0., 1., 0.]);
        assert_eq!(angle_cos(&u, &v), 0.0);
    }

    #[test]
    fn test_cos_non_unit_vectors() {
        let u = Vector::from([3., 4.]);
        let v = Vector::from([4., 3.]);
        assert!(floats_are_equal(angle_cos(&u, &v), 0.96));
    }

    #[test]
    fn test_cos_negative_vectors() {
        let u = Vector::from([-1., -2.]);
        let v = Vector::from([-2., -4.]);
        assert_eq!(angle_cos(&u, &v), 1.0);
    }
}
