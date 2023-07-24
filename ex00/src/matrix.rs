use std::clone::Clone;
use std::f32::consts::PI;
use std::fmt::Display;
use std::ops::{Add, Mul, Sub};
use std::default::Default;
use std::fmt::Debug;
use std::result;
use num::{Float};
use crate::vector::Vector;

use std::convert::TryInto;

pub type TMatrix<T, const M: usize> = Matrix<T, M, M>;
pub type TMatrix2<T> = TMatrix<T, 2>;
pub type TMatrix3<T> = TMatrix<T, 3>;
pub type TMatrix4<T> = TMatrix<T, 4>;

#[derive(PartialEq, Debug, Clone)]
pub struct Matrix<T, const M: usize, const N: usize> {
    data: Vec<Vec<T>>,
}

impl<T: Clone, const M: usize, const N: usize> From<[[T; N]; M]> for Matrix<T, M, N> {
    fn from(s: [[T; N]; M]) -> Matrix<T, M, N> {
        let mut d = Vec::new();
        for item in s.iter() {
            d.push(item.to_vec());
        }
        return Matrix {
            data: d,
        };
    }
}

impl<T: std::fmt::Debug, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn out(&self) {
        for item in self.data.iter() {
            println!("{:?}", item);
        }
    }
}

impl<T: Display + Add<Output = T> + Clone, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn add(&mut self, v: &Matrix<T, M, N>) {
        let mut res = Vec::new();
        for j in 0..M {
            let it1 = self.data[j].iter();
            let it2 = v.data[j].iter();
            let iter = it1.zip(it2);
            let mut v = Vec::new();
            for (item1, item2) in iter {
                let value = item1.clone() + item2.clone();
                v.push(value);
            }
            res.push(v);
        }
        self.data = res;
    }
}

impl<T: Display + Add<Output = T> + Clone, const M: usize, const N: usize> Add<Matrix<T, M, N>> for Matrix<T, M, N> {
    type Output = Matrix<T, M, N>;
    fn add(self, v: Matrix<T, M, N>) -> Matrix<T, M, N> {
        let mut res = Vec::new();
        for j in 0..M {
            let it1 = self.data[j].iter();
            let it2 = v.data[j].iter();
            let iter = it1.zip(it2);
            let mut v = Vec::new();
            for (item1, item2) in iter {
                let value = item1.clone() + item2.clone();
                v.push(value);
            }
            res.push(v);
        }
        Matrix {
            data: res,
        }
    }
}

impl<T: Display + Sub<Output = T> + Clone, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn sub(&mut self, v: &Matrix<T, M, N>) {
        let mut res = Vec::new();
        for j in 0..M {
            let it1 = self.data[j].iter();
            let it2 = v.data[j].iter();
            let iter = it1.zip(it2);
            let mut v = Vec::new();
            for (item1, item2) in iter {
                let value = item1.clone() - item2.clone();
                v.push(value);
            }
            res.push(v);
        }
        self.data = res;
    }
}

impl<T: Display + Sub<Output = T> + Clone, const M: usize, const N: usize> Sub<Matrix<T, M, N>> for Matrix<T, M, N> {
    type Output = Matrix<T, M, N>;
    fn sub(self, v: Matrix<T, M, N>) -> Matrix<T, M, N> {
        let mut res = Vec::new();
        for j in 0..M {
            let it1 = self.data[j].iter();
            let it2 = v.data[j].iter();
            let iter = it1.zip(it2);
            let mut v = Vec::new();
            for (item1, item2) in iter {
                let value = item1.clone() - item2.clone();
                v.push(value);
            }
            res.push(v);
        }
        Matrix {
            data: res,
        }
    }
}

impl<T: Display + Mul<Output = T> + Clone + Copy, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn scl(&mut self, a: T) {
        let mut res = Vec::new();
        for j in 0..M {
            let it = self.data[j].iter();
            let mut v = Vec::new();
            for item in it {
                v.push(item.clone() * a);
            }
            res.push(v);
        }
        self.data = res;
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
    where T: Default + Float + Copy {
    pub fn mul_mat<const H: usize>(&mut self, rhs: &Matrix<T, N, H>) -> Matrix<T, M, H>{
        let mut res = Vec::new();
        for j in 0..M {
            let mut v = Vec::new();
            for i in 0..H {
                let mut sum: T = Default::default();
                for k in 0..N {
                    sum = sum + (self.data[j][k] * rhs.data[k][i]);
                }
                v.push(sum);
            }
            res.push(v);
        }
        Matrix {
            data: res,
        }
    }

    pub fn mul_vec(&mut self, rhs: &Vector<T, N>) -> Vector<T, N> {
        let mut res = [T::default(); N];
        for j in 0..M {
            let mut sum: T = Default::default();
            for i in 0..N {
                sum = sum + (self.data[j][i] * rhs[i]);
            }
            res[j] = sum;
        }
        Vector::from(res)
    }
}

impl<T, const M: usize, const N: usize, const H: usize> Mul<Matrix<T, N, H>> for Matrix<T, M, N>
    where T: Default + Add<Output = T> + Mul<Output = T> + Copy {
    type Output = Matrix<T, M, H>;
     fn mul(self, rhs: Matrix<T, N, H>) -> Matrix<T, M, H>{
        let mut res = Vec::new();
        for j in 0..M {
            let mut v = Vec::new();
            for i in 0..H {
                let mut sum: T = Default::default();
                for k in 0..N {
                    sum = sum + (self.data[j][k] * rhs.data[k][i]);
                }
                v.push(sum);
            }
            res.push(v);
        }
        Matrix {
            data: res,
        }
    }
}

impl<T, const M: usize, const N: usize> Mul<T> for Matrix<T, M, N>
    where T: Float + Copy {
    type Output = Matrix<T, M, N>;
     fn mul(self, rhs: T) -> Matrix<T, M, N>{
        let mut res = Vec::new();
        for j in 0..M {
            let mut v = Vec::new();
            for i in 0..N {
                let sum = self.data[j][i] * rhs;
                v.push(sum);
            }
            res.push(v);
        }
        Matrix {
            data: res,
        }
    }
}

// impl<T, const M: usize, const N: usize> Mul<f32> for Matrix<T, M, N>
//     where T: Default + Add<Output = T> + Mul<f32, Output = T> + Copy {
//     type Output = Matrix<T, M, N>;
//      fn mul(self, rhs: f32) -> Matrix<T, M, N>{
//         let mut res = Vec::new();
//         for j in 0..M {
//             let mut v = Vec::new();
//             for i in 0..N {
//                 let sum = self.data[j][i] * rhs;
//                 v.push(sum);
//             }
//             res.push(v);
//         }
//         Matrix {
//             data: res,
//         }
//     }
// }

impl<T: Float, const N: usize> Matrix<T, N, N> {
    pub fn trace(&mut self) -> T {
        let mut res = T::zero();
        for i in 0..N {
            res = res + self.data[i][i];
        }
        res
    }
}

impl<T: Float, const M: usize, const N: usize> Matrix<T, M, N>
{
    pub fn transpose(&mut self) -> Matrix<T, N, M> {
        let mut res = Vec::new();
        for j in 0..N {
            let mut v = Vec::new();
            for i in 0..M {
                v.push(self.data[i][j]);
            }
            res.push(v);
        }
        Matrix {
            data: res,
        }
    }
}

impl<T: Float + Clone, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn as_arr(&mut self) -> [[T; N]; M] {
        let mut res = [[T::zero(); N]; M];
        for j in 0..M {
            for i in 0..N {
                res[j][i] = self.data[j][i];
            }
        }
        res
    }
}

impl<T: Float + Clone, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn as_vec(&self) -> Vec<Vec<T>> {
        self.data.clone()
    }
}

impl<T: Float + Clone, const M: usize, const N: usize> Matrix<T, M, N> {
    fn is_all_zero(&self, v: Vector<T, N>) -> bool {
        for item in v.as_vec() {
            if item != T::zero() {
                return false
            }
        }
        true
    }

    pub fn row_echelon(&mut self) -> Matrix<T, M, N> {
        let mut lead: usize = 0;
        for j in 0..M {
            let mut v = self.as_vector(j);
            if self.is_all_zero(v.clone()) {
                self.data[M - 1] = v.as_vec();
            }
            for i in 0..N {
                //find the lead
                if self.data[j][i] != T::zero() && i >= lead {
                    //lead to 1
                    v = v.clone() / v[i];
                    self.data[j] = v.as_vec();

                    //pivot column to 0
                    for x in 0..M {
                        if x == j {
                            continue;
                        }
                        let coef = self.data[x][i];
                        if coef != T::zero() {
                            let mut tmp = self.as_vector(x);
                            tmp = tmp.clone() - v.clone() * coef;
                            self.data[x] = tmp.as_vec();
                        }
                    }
                    lead = i;
                    break;
                }
            }
        }
        Matrix {
            data: self.data.clone(),
        }
    }
}



fn vec_to_arr<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

impl<T: Float, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn as_vector(&self, h: usize) -> Vector<T, N> {
        let arr = vec_to_arr(self.data[h].clone());
        // let mut arr = [T::zero(); N];
        // for i in 0..N {
        //     arr[i] = self.data[h][i];
        // }
        Vector::from(arr)
    }
}

impl<T: Float, const M: usize> TMatrix<T, M> {
    fn _deter(&self, data: Vec<Vec<T>>, size: usize) -> T {
        if size == 1 {
            return data[0][0]
        }
        if size == 2 {
            return (data[0][0] * data[1][1]) - (data[0][1] * data[1][0])
        }
        let mut res = T::zero();
        for i in 0..size {
            let coef = data[0][i];
            let mut sign = T::one();
            if i % 2 == 1 {
                sign = -T::one();
            }
            let mut vec2d = Vec::new();
            for o in 1..size {
                let mut v = Vec::new();
                for l in 0..size {
                    if l != i {
                        v.push(data[o][l]);
                    }
                }
                vec2d.push(v);
            }
            let rhs = coef * sign * self._deter(vec2d, size - 1);
            res = res + rhs;
        }
        res
    }

    pub fn determinant(&mut self) -> T {
        self._deter(self.data.clone(), M)
    }
}

impl<T: Float + Display + Debug, const M: usize> TMatrix<T, M> {
    fn _get_minor(&self, data: Vec<Vec<T>>, y: usize, x: usize, size: usize) -> T {
        let mut vec2d = Vec::new();
        for j in 0..M {
            let mut v = Vec::new();
            for i in 0..M {
                if j != y && i != x {
                    println!("push{}", data[j][i]);
                    v.push(data[j][i]);
                }
            }
            if v.len() > 0 {
                vec2d.push(v);
            }
        }
        let mut sign = T::one();
        if (y + x) % 2 == 1 {
            sign = -T::one();
        }
        println!("vec2d{:?}", vec2d);
        sign * self._deter(vec2d, size - 1)
    }
    fn _minor(&self, data: Vec<Vec<T>>, size: usize) -> TMatrix<T, M> {
        let mut arr = [[T::zero(); M]; M];
        for j in 0..M {
            for i in 0..M {
                arr[j][i] = self._get_minor(data.clone(), j, i, size);
            }
        }
        Matrix::from(arr)
    }
    fn _get_adj(&mut self) -> TMatrix<T, M> {
        let trans = self.transpose();
        let adj = self._minor(trans.as_vec(), M);
        adj
    }
    pub fn inverse(&mut self) -> Result<TMatrix<T, M>, String> {
        let det = self.determinant();
        if det == T::zero() {
            panic!("determinant is zero")
        }
        let adj = self._get_adj();
        let res = adj * (T::one() / det);
        Ok(res)
    }
}




pub fn identity_array() -> [[f32; 4]; 4] {
    let mut trans:[[f32; 4]; 4] = [[0.; 4]; 4];
    trans[0][0] = 1.;
    trans[1][1] = 1.;
    trans[2][2] = 1.;
    trans[3][3] = 1.;
    trans
}

impl TMatrix4<f32> {
    pub fn identity() -> TMatrix4<f32> {
        let mut trans:[[f32; 4]; 4] = identity_array();
        Matrix::from(trans)
    }

    pub fn translation(x: f32, y: f32, z:f32) -> TMatrix4<f32> {
        let mut trans:[[f32; 4]; 4] = identity_array();
        trans[0][3] = x;
        trans[1][3] = y;
        trans[2][3] = z;
        Matrix::from(trans)
    }

    pub fn scale(x: f32, y: f32, z:f32) -> TMatrix4<f32> {
        let mut scale:[[f32; 4]; 4] = identity_array();
        scale[0][0] = x;
        scale[1][1] = y;
        scale[2][2] = z;
        Matrix::from(scale)
    }
}

impl TMatrix4<f32>{
    pub fn as_mut_arr(&self) -> [[f32; 4]; 4] {
        let iter = self.data.iter();
        let mut res:[[f32; 4]; 4] = [[0.; 4]; 4];
        for (i , row) in iter.enumerate() {
            let slice = row.as_slice();
            res[i] = slice.try_into().unwrap();
        }
        res
    }
}

fn degree_to_radians(degree: f32) -> f32 {
    degree / 180. * PI
}

impl TMatrix4<f32> {
    pub fn rotation_x(degrees: f32) -> TMatrix4<f32> {
        let t: f32 = degree_to_radians(degrees);
        let mut rot_x:[[f32; 4]; 4] = identity_array();
        rot_x[1][1] = f32::cos(t);
        rot_x[1][2] = -f32::sin(t);
        rot_x[2][1] = f32::sin(t);
        rot_x[2][2] = f32::cos(t);
        Matrix::from(rot_x)
    }

    pub fn rotation_y(degrees: f32) -> TMatrix4<f32> {
        let t: f32 = degree_to_radians(degrees);
        let mut rot_y:[[f32; 4]; 4] = identity_array();
        rot_y[0][0] = f32::cos(t);
        rot_y[0][2] = f32::sin(t);
        rot_y[2][0] = -f32::sin(t);
        rot_y[2][2] = f32::cos(t);
        Matrix::from(rot_y)
    }

    pub fn rotation_z(degrees: f32) -> TMatrix4<f32> {
        let t: f32 = degree_to_radians(degrees);
        let mut rot_z:[[f32; 4]; 4] = identity_array();
        rot_z[0][0] = f32::cos(t);
        rot_z[0][1] = -f32::sin(t);
        rot_z[1][0] = f32::sin(t);
        rot_z[1][1] = f32::cos(t);
        Matrix::from(rot_z)
    }

    pub fn rotation(x: f32, y: f32, z:f32) -> TMatrix4<f32> {
        let mut rot_x = Self::rotation_x(x);
        let mut rot_y = Self::rotation_y(y);
        let mut rot_z = Self::rotation_z(z);
        // Self::rotation_x(x) * Self::rotation_y(t_y) * Self::rotation_z(t_z)
        rot_x * rot_y * rot_z
    }
}

impl<T: Display, const M: usize, const N: usize> Display for Matrix<T, M, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for j in 0..M {
            if j != 0 {
                write!(f, "\n").unwrap();
            }
            write!(f, "[").unwrap();
            for i in 0..N {
                if i != 0 {
                    write!(f, ", ").unwrap();
                }
                write!(f, "{}", self.data[j][i]).unwrap();
            }
            write!(f, "]").unwrap();
        }

        Ok(())
    }
}
