use crate::base_structs::vector::Vector;
use crate::num_traits::scalar::Scalar;

impl<T: Scalar, const N: usize> Vector<T, N> {
    pub fn linear_combination(u: &[Vector<T, N>], coefs: &[T]) -> Vector<T, N> {
        let mut res = Vector::from([T::zero(); N]);
        for (i, item) in u.iter().enumerate() {
            let mut tmp = item.clone();
            tmp.scl(coefs[i]);
            res = res + tmp;
        }
        res
    }
}

#[cfg(test)]
mod linear_combination {
    use super::*;

    #[test]
    fn test01() {
        let e1 = Vector::from([1., 0., 0.]);
        let e2 = Vector::from([0., 1., 0.]);
        let e3 = Vector::from([0., 0., 1.]);
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([0., 10., -100.]);
        let res = Vector::linear_combination(&[e1, e2, e3], &[10., -2., 0.5]);
        let ans = Vector::from([10.0, -2.0, 0.5]);
        assert_eq!(res, ans);
        let res2 = Vector::linear_combination(&[v1, v2], &[10., -2.]);
        let ans2 = Vector::from([10.0, 0.0, 230.]);
        assert_eq!(res2, ans2);
    }
}
