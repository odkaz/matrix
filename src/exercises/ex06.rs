use crate::base_structs::vector::{TVector3, Vector};
use crate::num_traits::scalar::Scalar;

// creates a new vector that is perpendicular to the two input vectors
pub fn cross_product<T: Scalar>(u: &TVector3<T>, v: &TVector3<T>) -> TVector3<T> {
    let mut res = [T::zero(); 3];
    let u_arr = u.as_slice();
    let v_arr = v.as_slice();
    res[0] = u_arr[1] * v_arr[2] - u_arr[2] * v_arr[1];
    res[1] = u_arr[2] * v_arr[0] - u_arr[0] * v_arr[2];
    res[2] = u_arr[0] * v_arr[1] - u_arr[1] * v_arr[0];
    Vector::from(res)
}

#[cfg(test)]
mod cross_product {
    use super::*;

    #[test]
    fn test_cross() {
        let u = Vector::from([0., 0., 1.]);
        let v = Vector::from([1., 0., 0.]);
        println!("{}", cross_product(&u, &v));
        assert_eq!(cross_product(&u, &v), Vector::from([0., 1., 0.]));

        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);
        println!("{}", cross_product(&u, &v));
        assert_eq!(cross_product(&u, &v), Vector::from([-3., 6., -3.]));

        let u = Vector::from([4., 2., -3.]);
        let v = Vector::from([-2., -5., 16.]);
        println!("{}", cross_product(&u, &v));
        assert_eq!(cross_product(&u, &v), Vector::from([17., -58., -16.]));
    }
}
