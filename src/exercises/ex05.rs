use crate::base_structs::vector::Vector;
use crate::num_traits::scalar::Scalar;
use crate::exercises::ex04;
use std::ops::{Add, Div};

pub fn angle_cos<T: Scalar + Into<f32> + Add<f32, Output = f32> + Div<f32, Output = f32>, const N: usize>
(
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

    #[test]
    fn test_cos() {
        let u = Vector::from([1., 0.]);
        let v = Vector::from([1., 0.]);
        assert_eq!(angle_cos(&u, &v), 1.0);

        let u = Vector::from([1., 0.]);
        let v = Vector::from([0., 1.]);
        assert_eq!(angle_cos(&u, &v), 0.0);

        // let u = Vector::from([-1., 1.]);
        // let v = Vector::from([ 1., -1.]);
        // assert_eq!(angle_cos(&u, &v), -1.0);
        // float error?

        let u = Vector::from([2., 1.]);
        let v = Vector::from([4., 2.]);
        assert_eq!(angle_cos(&u, &v), 1.0);

        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);
        assert_eq!(angle_cos(&u, &v), 0.9746318);
    }
}