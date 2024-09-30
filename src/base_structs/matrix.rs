use crate::base_structs::vector::Vector;
use crate::num_traits::scalar::Scalar;
// use num::Scalar;
use std::clone::Clone;
// use std::default::Default;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::{Add, Mul, Sub};
use std::convert::TryInto;

pub type TMatrix<T, const M: usize> = Matrix<T, M, M>;
pub type TMatrix2<T> = TMatrix<T, 2>;
pub type TMatrix3<T> = TMatrix<T, 3>;
pub type TMatrix4<T> = TMatrix<T, 4>;

#[derive(PartialEq, Debug, Clone)]
pub struct Matrix<T, const M: usize, const N: usize> {
    pub(crate) data: Vec<Vec<T>>,
}

impl<T: Clone, const M: usize, const N: usize> From<[[T; N]; M]> for Matrix<T, M, N> {
    fn from(s: [[T; N]; M]) -> Matrix<T, M, N> {
        let mut d = Vec::new();
        for item in s.iter() {
            d.push(item.to_vec());
        }
        return Matrix { data: d };
    }
}

impl<T: Debug, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn out(&self) {
        for item in self.data.iter() {
            println!("{:?}", item);
        }
    }
}

impl<T: Scalar, const M: usize, const N: usize> Matrix<T, M, N> {
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

impl<T: Scalar, const M: usize, const N: usize> Add<Matrix<T, M, N>>
    for Matrix<T, M, N>
{
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
        Matrix { data: res }
    }
}

impl<T: Scalar, const M: usize, const N: usize> Matrix<T, M, N> {
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

impl<T: Scalar, const M: usize, const N: usize> Sub<Matrix<T, M, N>>
    for Matrix<T, M, N>
{
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
        Matrix { data: res }
    }
}

impl<T: Scalar, const M: usize, const N: usize> Matrix<T, M, N> {
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



impl<T: Scalar, const M: usize, const N: usize, const H: usize> Mul<Matrix<T, N, H>> for Matrix<T, M, N> {
    type Output = Matrix<T, M, H>;
    fn mul(self, rhs: Matrix<T, N, H>) -> Matrix<T, M, H> {
        let mut res = Vec::new();
        for j in 0..M {
            let mut v = Vec::new();
            for i in 0..H {
                let mut sum: T = T::zero();
                for k in 0..N {
                    sum = sum + (self.data[j][k] * rhs.data[k][i]);
                }
                v.push(sum);
            }
            res.push(v);
        }
        Matrix { data: res }
    }
}

impl<T: Scalar, const M: usize, const N: usize> Mul<T> for Matrix<T, M, N> {
    type Output = Matrix<T, M, N>;
    fn mul(self, rhs: T) -> Matrix<T, M, N> {
        let mut res = Vec::new();
        for j in 0..M {
            let mut v = Vec::new();
            for i in 0..N {
                let sum = self.data[j][i] * rhs;
                v.push(sum);
            }
            res.push(v);
        }
        Matrix { data: res }
    }
}

impl<T: Scalar, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn transpose(&mut self) -> Matrix<T, N, M> {
        let mut res = Vec::new();
        for j in 0..N {
            let mut v = Vec::new();
            for i in 0..M {
                v.push(self.data[i][j]);
            }
            res.push(v);
        }
        Matrix { data: res }
    }
}

impl<T: Scalar, const M: usize, const N: usize> Matrix<T, M, N> {
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

impl<T: Scalar, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn as_vec(&self) -> Vec<Vec<T>> {
        self.data.clone()
    }
}

// impl<T: Scalar, const M: usize, const N: usize> Matrix<T, M, N> {
//     fn is_all_zero(&self, index: usize) -> bool {
//         for item in &self.data[index] {
//             if item != &T::zero() {
//                 return false;
//             }
//         }
//         true
//     }

//     pub fn row_echelon(&mut self) -> Matrix<T, M, N> {
//         let mut lead: usize = 0;
//         for j in 0..M {
//             if self.is_all_zero(j) {
//                 for k in 0..M - j {
//                     let index = M - k - 1;
//                     let l = self.as_vector(index);
//                     if !self.is_all_zero(index) {
//                         self.data[index] = self.data[j].clone();
//                         self.data[j] = l.as_vec();
//                         break;
//                     }
//                 }
//             }
//             for i in 0..N {
//                 let mut v = self.as_vector(j);

//                 //find the lead
//                 if self.data[j][i] != T::zero() && i >= lead {
//                     //lead to 1
//                     v = v.clone() / v[i];
//                     self.data[j] = v.as_vec();

//                     //pivot column to 0
//                     for x in 0..M {
//                         if x == j {
//                             continue;
//                         }
//                         let coef = self.data[x][i];
//                         if coef != T::zero() {
//                             let mut tmp = self.as_vector(x);
//                             tmp = tmp.clone() - v.clone() * coef;
//                             self.data[x] = tmp.as_vec();
//                         }
//                     }
//                     lead = i;
//                     break;
//                 }
//             }
//         }
//         Matrix {
//             data: self.data.clone(),
//         }
//     }
// }

fn vec_to_arr<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

impl<T: Scalar, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn as_vector(&self, h: usize) -> Vector<T, N> {
        let arr = vec_to_arr(self.data[h].clone());
        Vector::from(arr)
    }
}

impl<T: Scalar, const M: usize> TMatrix<T, M> {
    fn _deter(&self, data: Vec<Vec<T>>, size: usize) -> T {
        if size == 1 {
            return data[0][0];
        }
        if size == 2 {
            return (data[0][0] * data[1][1]) - (data[0][1] * data[1][0]);
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

impl<T: Scalar, const M: usize> TMatrix<T, M> {
    fn _identity() -> [[T; M]; M] {
        let mut arr = [[T::zero(); M]; M];
        for i in 0..M {
            arr[i][i] = T::one();
        }
        arr
    }

    fn _swap_row(mat: &mut [[T; M]; M], x: usize, y: usize) {
        for i in 0..M {
            let tmp = mat[y][i];
            mat[y][i] = mat[x][i];
            mat[x][i] = tmp;
        }
    }

    pub fn inverse(&mut self) -> Result<TMatrix<T, M>, String> {
        let det = self.determinant();
        if det == T::zero() {
            panic!("determinant is zero")
        }
        let mut res = TMatrix::<T, M>::_identity();
        let mut d = self.as_arr();

        //set the row so that the pivot is different than zero
        for i in 0..M {
            if d[i][i] == T::zero() {
                let mut big = i;
                for j in 0..M {
                    if T::abs(&d[j][i]) > T::abs(&d[big][i]) {
                        big = j;
                    }
                }
                if big == i {
                    panic!("singular matrix");
                }
                TMatrix::<T, M>::_swap_row(&mut res, i, big);
                TMatrix::<T, M>::_swap_row(&mut d, i, big);
            }
        }

        //eliminate all numbers under the diagonal element
        for col in 0..M - 1 {
            for row in col + 1..M {
                let k = d[row][col] / d[col][col];
                for l in 0..M {
                    d[row][l] = d[row][l] - k * d[col][l];
                    res[row][l] = res[row][l] - k * res[col][l];
                }
                d[row][col] = T::zero();
            }
        }

        //scale all the pivots coefficients to 1
        for row in 0..M {
            let div = d[row][row];
            for col in 0..M {
                d[row][col] = d[row][col] / div;
                res[row][col] = res[row][col] / div;
            }
        }

        //eliminate all numbers above the diagonal
        for row in 0..M {
            for col in row + 1..M {
                let k = d[row][col];
                for l in 0..M {
                    d[row][l] = d[row][l] - d[col][l] * k;
                    res[row][l] = res[row][l] - res[col][l] * k;
                }
                d[row][col] = T::zero();
            }
        }
        Ok(Matrix::from(res))
    }
}

// impl<T: Scalar, const M: usize, const N: usize> Matrix<T, M, N> {
//     pub fn rank(&mut self) -> usize {
//         let mat = self.row_echelon().as_arr();
//         let mut res: usize = 0;
//         for m in 0..M {
//             for n in m..N {
//                 if mat[m][n] != T::zero() {
//                     res = res + 1;
//                     break;
//                 }
//             }
//         }
//         res
//     }
// }

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
