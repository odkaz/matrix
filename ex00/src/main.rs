pub mod vector;
use vector::{Vector, TVector3};

pub mod matrix;
use matrix::Matrix;

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
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
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



}

fn main() {
    test_vector();
}
