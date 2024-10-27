use matrixlib::base_structs::matrix::{Matrix, TMatrix4};

pub fn projection(fov: f32, aspect: f32, znear: f32, zfar: f32) -> TMatrix4<f32> {
    let mut arr = [[0.; 4]; 4];
    arr[0][0] = aspect * (1. / f32::tan(fov / 2.));
    arr[1][1] = 1. / f32::tan(fov / 2.);
    arr[2][2] = zfar / (zfar - znear);
    arr[2][3] = (-zfar * znear) / (zfar - znear);
    arr[3][2] = 1.;
    Matrix::from(arr)
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
    println!("{}", res);
}

fn main() {
    println!("Running bonus.rs");
    bonus();
}
