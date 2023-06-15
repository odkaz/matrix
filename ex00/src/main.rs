// use Matrix::Vector;
// use std::ops::Add;
// use std::fmt::Display;
pub mod vector;
use vector::Vector;

pub mod matrix;
use matrix::Matrix;

fn test_vector() {
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);

    u.add(&v);
    println!("{}", u);
    u.out();
    // [7.0]
    // [10.0]
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.sub(&v);
    u.out();
    // [-3.0]
    // [-4.0]
    let mut u = Vector::from([2., 3.]);
    u.scl(2.);
    u.out();
    // [4.0]
    // [6.0]
}

fn test_matrix() {
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.add(&v);
    println!("{}", u);
    // [8.0, 6.0]
    // [1.0, 6.0]

    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.sub(&v);
    println!("{}", u);
    // [-6.0, -2.0]
    // [5.0, 2.0]

    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    u.scl(2.);
    println!("{}", u);
    // [2.0, 4.0]
    // [6.0, 8.0]
}

fn main() {
    test_matrix();
}
