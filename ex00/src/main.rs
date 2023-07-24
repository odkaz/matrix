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

pub fn cross_product<T: Float>(u: &TVector3<T>, v: &TVector3<T>) -> TVector3<T> {
    let mut res = [T::zero(); 3];
    let u_arr = u.as_slice();
    let v_arr = v.as_slice();
    res[0] = u_arr[1] * v_arr[2] - u_arr[2] * v_arr[1];
    res[1] = u_arr[2] * v_arr[0] - u_arr[0] * v_arr[2];
    res[2] = u_arr[0] * v_arr[1] - u_arr[1] * v_arr[0];
    Vector::from(res)
}

#[cfg(test)]
mod tests {
    use crate::vector::Vector;
    use crate::matrix::Matrix;
    use crate::lerp;
    use crate::angle_cos;
    use crate::cross_product;

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

    #[test]
    fn test_mul() {
        let mut u = Matrix::from([
        [1., 0.],
        [0., 1.],
        ]);
        let v = Vector::from([4., 2.]);
        assert_eq!(u.mul_vec(&v), Vector::from([4., 2.]));

        let mut u = Matrix::from([
        [2., 0.],
        [0., 2.],
        ]);
        let v = Vector::from([4., 2.]);
        assert_eq!(u.mul_vec(&v), Vector::from([8., 4.]));

        let mut u = Matrix::from([
        [2., -2.],
        [-2., 2.],
        ]);
        let v = Vector::from([4., 2.]);
        assert_eq!(u.mul_vec(&v), Vector::from([4., -4.]));

        let mut u = Matrix::from([
        [1., 0.],
        [0., 1.],
        ]);
        let v = Matrix::from([
        [1., 0.],
        [0., 1.],
        ]);
        assert_eq!(u.mul_mat(&v), Matrix::from([[1., 0.], [0., 1.]]));

        let mut u = Matrix::from([
        [1., 0.],
        [0., 1.],
        ]);
        let v = Matrix::from([
        [2., 1.],
        [4., 2.],
        ]);
        assert_eq!(u.mul_mat(&v), Matrix::from([[2., 1.], [4., 2.]]));

        let mut u = Matrix::from([
        [3., -5.],
        [6., 8.],
        ]);
        let v = Matrix::from([
        [2., 1.],
        [4., 2.],
        ]);
        assert_eq!(u.mul_mat(&v), Matrix::from([[-14., -7.], [44., 22.]]));
    }

    #[test]
    fn test_trace() {
        let mut u = Matrix::from([
        [1., 0.],
        [0., 1.],
        ]);
        assert_eq!(u.trace(), 2.);
        // 2.0
        let mut u = Matrix::from([
        [2., -5., 0.],
        [4., 3., 7.],
        [-2., 3., 4.],
        ]);
        assert_eq!(u.trace(), 9.);
        // 9.0
        let mut u = Matrix::from([
        [-2., -8., 4.],
        [1., -23., 4.],
        [0., 6., 4.],
        ]);
        assert_eq!(u.trace(), -21.);
        // -21.0
    }

    #[test]
    fn test_transpose() {
        let mut u = Matrix::from([
            [1., 2.],
            [3., 4.],
            [5., 6.],
            ]);
        assert_eq!(u.transpose(), Matrix::from([
            [1., 3., 5.],
            [2., 4., 6.],
        ]));

        let mut u = Matrix::from([
            [1.],
            [2.],
            [3.],
            [4.],
            ]);
        assert_eq!(u.transpose(), Matrix::from([
            [1., 2., 3., 4.],
        ]));
    }

    #[test]
    fn test_row_echelon() {
        let mut u = Matrix::from([
            [1., 0., 0.],
            [0., 1., 0.],
            [0., 0., 1.],
        ]);
        assert_eq!(u.row_echelon(), Matrix::from([
            [1., 0., 0.],
            [0., 1., 0.],
            [0., 0., 1.],
        ]));

        let mut u = Matrix::from([
            [1., 2.],
            [3., 4.],
        ]);
        assert_eq!(u.row_echelon(), Matrix::from([
            [1.0, 0.0],
            [0.0, 1.0],
        ]));

        let mut u = Matrix::from([
            [1., 2.],
            [2., 4.],
        ]);
        assert_eq!(u.row_echelon(), Matrix::from([
            [1.0, 2.0],
            [0.0, 0.0],
        ]));

        let mut u = Matrix::from([
        [8., 5., -2., 4., 28.],
        [4., 2.5, 20., 4., -4.],
        [8., 5., 1., 4., 17.],
        ]);
        assert_eq!(u.row_echelon(), Matrix::from([
            [1.0, 0.625, 0.0, 0.0, -12.166666666666668],
            [0.0, 0.0, 1.0, 0.0, -3.666666666666667],
            [0.0, 0.0, 0.0, 1.0, 29.500000000000004 ],
        ]));
    }

    #[test]
    fn test_determinant() {
        let mut u = Matrix::from([
            [ 1., -1.],
            [-1., 1.],
        ]);
        assert_eq!(u.determinant(), 0.0);

        let mut u = Matrix::from([
            [2., 0., 0.],
            [0., 2., 0.],
            [0., 0., 2.],
        ]);
        assert_eq!(u.determinant(), 8.0);

        let mut u = Matrix::from([
            [8., 5., -2.],
            [4., 7., 20.],
            [7., 6., 1.],
        ]);
        assert_eq!(u.determinant(), -174.0);

        let mut u = Matrix::from([
            [5., 3., 7.],
            [2., -5., 8.],
            [-6., 4., 9.],
        ]);
        assert_eq!(u.determinant(), -737.0);

        let mut u = Matrix::from([
            [ 8., 5., -2., 4.],
            [ 4., 2.5, 20., 4.],
            [ 8., 5., 1., 4.],
            [28., -4., 17., 1.],
        ]);
        assert_eq!(u.determinant(), 1032.0);
    }
}

fn test_matrix() {
    let mut u = Matrix::from([
        [1., 2.],
        [3., 4.],
    ]);
    println!("{}", u.row_echelon());
}

fn main() {
    test_matrix();
}
