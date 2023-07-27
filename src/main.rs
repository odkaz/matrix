use matrixlib::base_structs::matrix::{Matrix, TMatrix4};
use matrixlib::base_structs::vector::{Vector, TVector3};
use num::Float;
use std::fmt::Display;
use std::ops::{Add, Div};

// mod lib;
use matrixlib::exercises::ex04;







pub fn projection(fov: f32, aspect: f32, znear: f32, zfar: f32) -> TMatrix4<f32> {
    let mut arr = [[0.; 4]; 4];
    arr[0][0] = aspect * (1. / f32::tan(fov / 2.));
    arr[1][1] = 1. / f32::tan(fov / 2.);
    arr[2][2] = zfar / (zfar - znear);
    arr[2][3] = (-zfar * znear) / (zfar - znear);
    arr[3][2] = 1.;
    Matrix::from(arr)
}

#[cfg(test)]
mod tests {














    // #[test]
    // fn test_trace() {
    //     let mut u = Matrix::from([[1., 0.], [0., 1.]]);
    //     assert_eq!(u.trace(), 2.);
    //     // 2.0
    //     let mut u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
    //     assert_eq!(u.trace(), 9.);
    //     // 9.0
    //     let mut u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
    //     assert_eq!(u.trace(), -21.);
    //     // -21.0
    // }

    // #[test]
    // fn test_transpose() {
    //     let mut u = Matrix::from([[1., 2.], [3., 4.], [5., 6.]]);
    //     assert_eq!(u.transpose(), Matrix::from([[1., 3., 5.], [2., 4., 6.],]));

    //     let mut u = Matrix::from([[1.], [2.], [3.], [4.]]);
    //     assert_eq!(u.transpose(), Matrix::from([[1., 2., 3., 4.],]));
    // }

    // #[test]
    // fn test_row_echelon() {
    //     let mut u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    //     assert_eq!(
    //         u.row_echelon(),
    //         Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.],])
    //     );

    //     let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    //     assert_eq!(u.row_echelon(), Matrix::from([[1.0, 0.0], [0.0, 1.0],]));

    //     let mut u = Matrix::from([[1., 2.], [2., 4.]]);
    //     assert_eq!(u.row_echelon(), Matrix::from([[1.0, 2.0], [0.0, 0.0],]));

    //     let mut u = Matrix::from([
    //         [8., 5., -2., 4., 28.],
    //         [4., 2.5, 20., 4., -4.],
    //         [8., 5., 1., 4., 17.],
    //     ]);
    //     assert_eq!(
    //         u.row_echelon(),
    //         Matrix::from([
    //             [1.0, 0.625, 0.0, 0.0, -12.166666666666668],
    //             [0.0, 0.0, 1.0, 0.0, -3.666666666666667],
    //             [0.0, 0.0, 0.0, 1.0, 29.500000000000004],
    //         ])
    //     );
    // }

    // #[test]
    // fn test_determinant() {
    //     let mut u = Matrix::from([[1., -1.], [-1., 1.]]);
    //     assert_eq!(u.determinant(), 0.0);

    //     let mut u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    //     assert_eq!(u.determinant(), 8.0);

    //     let mut u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
    //     assert_eq!(u.determinant(), -174.0);

    //     let mut u = Matrix::from([[5., 3., 7.], [2., -5., 8.], [-6., 4., 9.]]);
    //     assert_eq!(u.determinant(), -737.0);

    //     let mut u = Matrix::from([
    //         [8., 5., -2., 4.],
    //         [4., 2.5, 20., 4.],
    //         [8., 5., 1., 4.],
    //         [28., -4., 17., 1.],
    //     ]);
    //     assert_eq!(u.determinant(), 1032.0);
    // }

    // #[test]
    // fn test_inverse() {
    //     let mut u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    //     assert_eq!(
    //         u.inverse().unwrap(),
    //         Matrix::from([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0],])
    //     );

    //     let mut u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    //     assert_eq!(
    //         u.inverse().unwrap(),
    //         Matrix::from([[0.5, 0.0, 0.0], [0.0, 0.5, 0.0], [0.0, 0.0, 0.5],])
    //     );

    //     let mut u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
    //     assert_eq!(
    //         u.inverse().unwrap(),
    //         Matrix::from([
    //             [0.6494252873563219, 0.0977011494252874, -0.6551724137931035],
    //             [-0.7816091954022988, 0.12643678160919547, 0.9655172413793105],
    //             [0.14367816091954022, 0.07471264367816093, -0.20689655172413796],
    //         ])
    //     );
    // }

    // #[test]
    // fn test_rank() {
    //     let mut u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    //     assert_eq!(u.rank(), 3);
    //     let mut u = Matrix::from([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]);
    //     assert_eq!(u.rank(), 2);
    //     let mut u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]]);
    //     assert_eq!(u.rank(), 3);
    // }
}

fn bonus() {
    let znear = 0.1;
    let zfar = 100.0;
    let degree = 135.;
    let width = 600.;
    let height = 600.;
    let fov = degree / 360. * std::f32::consts::PI;
    let aspect = height / width;
    let res = projection(fov, aspect, znear, zfar);
    println!("res{}", res);
}

fn test() {
    let mut u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Vector::from([4., 2.]);
    println!("u: {}", u.mul_vec(&v));
}
fn main() {
    // bonus();
    test();
}
