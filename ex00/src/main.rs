
// use Matrix::Vector;
// use std::ops::Add;
// use std::fmt::Display;
pub mod vector;
use vector::Vector;


fn main() {
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.add(&v);
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


