pub mod vector;
use vector::{Vector, TVector3};

pub mod matrix;
use matrix::Matrix;
use std::{ops::{Add, Mul, Sub, Div, Index}, fmt::Display};
use num::{Float};
use crate::matrix::TMatrix4;

pub fn lerp<V: Clone + Add<Output = V> + Sub<Output = V> + Mul<f32, Output = V>>(u: V, v: V, t: f32) -> V {
    let res = u.clone() + ((v - u) * t);
    res
}

pub fn angle_cos<T: Float + Display + Default + Into<f32> + Add<f32, Output = f32> + Div<f32, Output = f32>, const N: usize>(u: &Vector<T, N>, v: &Vector<T, N>) -> f32 {
    let num = u.dot(v);
    let den = u.clone().norm() * v.clone().norm();
    let res = num / den;
    res
}

fn test_vector() {
    // let cameraPos = TVector3::from([0., 0., 0.]);
    // let cameraTarget = TVector3::from([0., 0., 0.]);
    // let cameraDirection = TVector3::normalize(cameraPos - cameraTarget);

    let mut u = Vector::from([2., 3., 4.]);
    let v = Vector::from([5., 6., 7.]);

    // let mut r = u - v;
    // println!("{}{}{}", &u[0], &u[1], &u[3]);
    // r.out();
    // u.add(&v);
    // let mut r = Vector::cross_product(&u, &v);
    // println!("{}", u.normalize());
    // r.out();
    // u.out();
    // [7.0]
    // [10.0]
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    let mut res = u + v;
    res.out();
    // [-3.0]
    // [-4.0]
    let mut u = Vector::from([2., 3.]);
    u.scl(2.);
    u.out();
    // [4.0]
    // [6.0]

}

#[cfg(test)]
mod tests {
    use crate::vector::Vector;
    use crate::matrix::Matrix;
    use crate::lerp;
    use crate::angle_cos;

    #[test]
    fn test01() {
        let e1 = Vector::from([1., 0., 0.]);
        let e2 = Vector::from([0., 1., 0.]);
        let e3 = Vector::from([0., 0., 1.]);
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([0., 10., -100.]);
        let res = Vector::linear_combination(&[e1, e2, e3], &[10., -2.,0.5]);
        let ans = Vector::from([10.0, -2.0, 0.5]);
        assert_eq!(res, ans);
        let res2 = Vector::linear_combination(&[v1, v2], &[10., -2.]);
        let ans2 = Vector::from([10.0, 0.0, 230.]);
        assert_eq!(res2, ans2);
    }

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0., 1., 0.), 0.0);
        assert_eq!(lerp(0., 1., 1.), 1.0);
        assert_eq!(lerp(0., 1., 0.5), 0.5);
        assert_eq!(lerp(21., 42., 0.3), 27.3);
        assert_eq!(lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3), Vector::from([2.6, 1.3]));
        assert_eq!(lerp(Matrix::from([[2., 1.], [3., 4.]]), Matrix::from([[20.,10.], [30., 40.]]), 0.5),
                    Matrix::from([[11., 5.5], [16.5, 22.]]));
    }

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

fn test_matrix() {
    let mut u = Matrix::from([
        [1., 2., 3.],
        [4., 5., 6.],
    ]);
    let v = Matrix::from([
        [7., 8.,],
        [9., 10.],
        [11., 12.]
    ]);
    // v.out();
    // let res = u * v;
    // println!("{}", res);

    // [8.0, 6.0]
    // [1.0, 6.0]

    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    // u.sub(&v);
    // u.out();
    // println!("{}", u);
    // [-6.0, -2.0]
    // [5.0, 2.0]

    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    // u.scl(2.);
    // println!("{}", u);
    // u.out();
    // [2.0, 4.0]
    // [6.0, 8.0]

    let mut trans = TMatrix4::translation(10., 0., 0.);
    let mut position = Matrix::from([
        [0.],
        [0.],
        [0.],
        [1.],
    ]);
    trans.out();
    position.out();
    let res = trans * position;
    res.out();


}

fn main() {
    test_matrix();
}
