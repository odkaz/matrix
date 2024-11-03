use crate::base_structs::vector::Vector;
use crate::num_traits::scalar::Scalar;

impl<T: Scalar, const N: usize> Vector<T, N> {
    pub fn linear_combination(u: &[Vector<T, N>], coefs: &[T]) -> Vector<T, N> {
        if u.len() != coefs.len() {
            panic!("The length of vectors and coefficients must match");
        }
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

    #[test]
    fn test02() {
        let e1 = Vector::from([1., 0.]);
        let e2 = Vector::from([0., 1.]);
        let res = Vector::linear_combination(&[e1, e2], &[3., 4.]);
        let ans = Vector::from([3.0, 4.0]);
        assert_eq!(res, ans);
    }

    #[test]
    fn test03() {
        let e1 = Vector::from([1., 0., 0., 0.]);
        let e2 = Vector::from([0., 1., 0., 0.]);
        let e3 = Vector::from([0., 0., 1., 0.]);
        let e4 = Vector::from([0., 0., 0., 1.]);
        let res = Vector::linear_combination(&[e1, e2, e3, e4], &[1., 2., 3., 4.]);
        let ans = Vector::from([1.0, 2.0, 3.0, 4.0]);
        assert_eq!(res, ans);
    }

    #[test]
    fn test04() {
        let e1 = Vector::from([1., 1., 1.]);
        let e2 = Vector::from([2., 2., 2.]);
        let res = Vector::linear_combination(&[e1, e2], &[1., 1.]);
        let ans = Vector::from([3.0, 3.0, 3.0]);
        assert_eq!(res, ans);
    }

    #[test]
    #[should_panic(expected = "The length of vectors and coefficients must match")]
    fn test_panic() {
        let e1 = Vector::from([1., 0., 0.]);
        let e2 = Vector::from([0., 1., 0.]);
        Vector::linear_combination(&[e1, e2], &[1.]);
    }
}
