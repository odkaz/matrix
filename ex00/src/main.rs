pub mod vector;
use vector::Vector;

pub mod matrix;
use matrix::Matrix;

fn test_vector() {
    let mut u = Vector::from([2., 3.]);
    let v: Vector<f64> = Vector::from([5., 7.]);

    u.add(&v);
    println!("{}", u);
    u.out();
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

fn test_matrix() {
    let mut u = Matrix::from([
        [1., 2., 3.],
        [4., 5., 6.],
        [7., 8., 9.]
    ]);
    let v = Matrix::from([
        [7., 4.,],
        [-2., 2.],
        [-4., -2.]
    ]);
    let res = u * v;
    println!("{}", res);
    // [8.0, 6.0]
    // [1.0, 6.0]

    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.sub(&v);
    // println!("{}", u);
    // [-6.0, -2.0]
    // [5.0, 2.0]

    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    u.scl(2.);
    // println!("{}", u);
    // [2.0, 4.0]
    // [6.0, 8.0]


    let mut trans = Matrix::translation(1.,2.,3.);
    trans.out();

    println!("-----");
    let mut rot_x = Matrix::rotation_x(30.);
    rot_x.out();
    println!("-----");

    let mut rot_y = Matrix::rotation_y(0.);
    rot_y.out();
    println!("-----");

    let mut rot_z = Matrix::rotation_z(0.);
    rot_z.out();
    println!("-----");

    let mut rot = Matrix::rotation(30., 0., 0.);
    rot.out();

    let mut t = rot_x * rot_y * rot_z;
    t.out();
}

fn main() {
    test_matrix();
}
