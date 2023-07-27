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
}