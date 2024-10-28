use matrixlib::base_structs::matrix::{Matrix, TMatrix4};
use std::fs::OpenOptions;
use std::io::{self, Write};

pub fn projection(fov: f32, aspect: f32, znear: f32, zfar: f32) -> TMatrix4<f32> {
    let mut arr = [[0.; 4]; 4];
    arr[0][0] = aspect * (1. / f32::tan(fov / 2.));
    arr[1][1] = 1. / f32::tan(fov / 2.);
    arr[2][2] = zfar / (zfar - znear);
    arr[2][3] = (-zfar * znear) / (zfar - znear);
    arr[3][2] = 1.;
    Matrix::from(arr)
}

fn format_matrix(matrix: Vec<Vec<f32>>) -> String {
    matrix
        .iter()
        .map(|row| {
            row.iter()
                .map(|&value| format!("{:.7}", value)) // Format to 7 decimal places
                .collect::<Vec<String>>() // Collect values as strings
                .join(", ") // Join each row with commas
        })
        .collect::<Vec<String>>() // Collect rows as strings
        .join("\n") // Join each row with a newline
}

fn bonus() -> io::Result<()> {
    let znear = 0.1;
    let zfar = 100.0;
    let degree = 135.;
    let width = 600.;
    let height = 600.;
    let fov = degree / 360. * std::f32::consts::PI;
    let aspect = height / width;
    let res = projection(fov, aspect, znear, zfar).as_vec();
    let formatted_string = format_matrix(res);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true) // Creates the file if it doesn't exist
        .truncate(true) // Empties the file if it exists
        .open("src/bin/bonus/proj")?;

    file.write_all(formatted_string.as_bytes())?;
    Ok(())
}

fn main() {
    bonus().unwrap();
}
