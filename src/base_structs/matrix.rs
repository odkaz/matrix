use crate::base_structs::vector::Vector;
use crate::num_traits::scalar::Scalar;
use std::clone::Clone;
use std::convert::TryInto;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::{Add, Mul, Sub};

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

impl<T: Scalar, const M: usize, const N: usize> Add<Matrix<T, M, N>> for Matrix<T, M, N> {
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

impl<T: Scalar, const M: usize, const N: usize> Sub<Matrix<T, M, N>> for Matrix<T, M, N> {
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

impl<T: Scalar, const M: usize, const N: usize, const H: usize> Mul<Matrix<T, N, H>>
    for Matrix<T, M, N>
{
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
