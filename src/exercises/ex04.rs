use crate::num_traits::scalar::Scalar;
use crate::base_structs::vector::Vector;
use std::ops::Add;

impl<T: Scalar + Into<f32> + Add<f32, Output = f32>, const N: usize> Vector<T, N> {
    pub fn norm_1(&mut self) -> f32 {
        let mut res = f32::zero();
        let v = self.as_slice();
        println!("v {:?}", v);
        for i in 0..N {
            res = v[i].abs() + res;
        }
        res
    }

    pub fn norm(&mut self) -> f32 {
        let mut res = f32::zero();
        for i in self.as_vec().iter() {
            res = i.powi(2) + res;
        }
        f32::sqrt(res)
    }

    pub fn norm_inf(&mut self) -> f32 {
        let b = self.as_vec();
        let max = b
            .iter()
            .max_by(|x, y| x.abs().partial_cmp(&y.abs()).unwrap())
            .unwrap();
        let cast: f32 = max.clone().abs().into();
        cast
    }
}

#[cfg(test)]
mod norm {
    use super::*;
    #[test]
    fn test_norm() {
        let mut u = Vector::from([0., 0., 0.]);
        assert_eq!(u.norm_1(), 0.0);
        assert_eq!(u.norm(), 0.0);
        assert_eq!(u.norm_inf(), 0.0);
        let mut u = Vector::from([1., 2., 3.]);
        assert_eq!(u.norm_1(), 6.0);
        assert_eq!(u.norm(), f32::sqrt(14.0));
        assert_eq!(u.norm_inf(), 3.0);
        let mut u = Vector::from([-1., -2.]);
        assert_eq!(u.norm_1(), 3.0);
        assert_eq!(u.norm(), f32::sqrt(5.0));
        assert_eq!(u.norm_inf(), 2.0);
    }
}