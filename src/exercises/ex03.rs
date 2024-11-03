use crate::num_traits::scalar::Scalar;
use crate::base_structs::vector::Vector;

impl<T: Scalar, const N: usize>
    Vector<T, N>
{
    pub fn dot(&self, v: &Vector<T, N>) -> T {
        let mut res = T::zero();
        for (item1, item2) in self.as_vec().iter().zip(v.as_vec().iter()) {
            res = res + item1.clone() * item2.clone();
        }
        res
    }
}

#[cfg(test)]
mod dot {
    use crate::base_structs::vector::Vector;

    #[test]
    fn test_dot() {
        let u = Vector::from([0., 0.]);
        let v = Vector::from([1., 1.]);
        assert_eq!(u.dot(&v), 0.0);

        let u = Vector::from([1., 1.]);
        let v = Vector::from([1., 1.]);
        assert_eq!(u.dot(&v), 2.0);

        let u = Vector::from([-1., 6.]);
        let v = Vector::from([3., 2.]);
        assert_eq!(u.dot(&v), 9.0);
    }

    #[test]
    fn test_dot_with_negative_numbers() {
        let u = Vector::from([-1., -1.]);
        let v = Vector::from([-1., -1.]);
        assert_eq!(u.dot(&v), 2.0);

        let u = Vector::from([-1., 1.]);
        let v = Vector::from([1., -1.]);
        assert_eq!(u.dot(&v), -2.0);
    }

    #[test]
    fn test_dot_with_zeros() {
        let u = Vector::from([0., 0.]);
        let v = Vector::from([0., 0.]);
        assert_eq!(u.dot(&v), 0.0);

        let u = Vector::from([0., 1.]);
        let v = Vector::from([1., 0.]);
        assert_eq!(u.dot(&v), 0.0);
    }

    #[test]
    fn test_dot_with_large_numbers() {
        let u = Vector::from([1e10, 1e10]);
        let v = Vector::from([1e10, 1e10]);
        assert_eq!(u.dot(&v), 2e20);

        let u = Vector::from([-1e10, 1e10]);
        let v = Vector::from([1e10, -1e10]);
        assert_eq!(u.dot(&v), -2e20);
    }

    #[test]
    fn test_dot_with_mixed_numbers() {
        let u = Vector::from([1.5, -2.5]);
        let v = Vector::from([-3.5, 4.5]);
        assert_eq!(u.dot(&v), -16.5);

        let u = Vector::from([0.5, 0.5]);
        let v = Vector::from([0.5, 0.5]);
        assert_eq!(u.dot(&v), 0.5);
    }
}