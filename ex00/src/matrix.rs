use std::clone::Clone;
use std::fmt::Display;
use std::ops::{Add, Mul, Sub};

pub struct Matrix<T> {
    data: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T: Clone, const M: usize, const N: usize> From<[[T; N]; M]> for Matrix<T> {
    fn from(s: [[T; N]; M]) -> Matrix<T> {
        let mut d = Vec::new();
        for item in s.iter() {
            d.push(item.to_vec());
        }
        return Matrix {
            data: d,
            width: N,
            height: M,
        };
    }
}

impl<T: std::fmt::Debug> Matrix<T> {
    pub fn out(&mut self) {
        for item in self.data.iter() {
            println!("{:?}", item);
        }
    }
}

impl<T: Display + Add<Output = T> + Clone> Matrix<T> {
    pub fn add(&mut self, v: &Matrix<T>) {
        let mut res = Vec::new();
        for j in 0..self.height {
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

impl<T: Display + Sub<Output = T> + Clone> Matrix<T> {
    pub fn sub(&mut self, v: &Matrix<T>) {
        let mut res = Vec::new();
        for j in 0..self.height {
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

impl<T: Display + Mul<Output = T> + Clone + Copy> Matrix<T> {
    pub fn scl(&mut self, a: T) {
        let mut res = Vec::new();
        for j in 0..self.height {
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

impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for j in 0..self.height {
            if j != 0 {
                write!(f, "\n").unwrap();
            }
            write!(f, "[").unwrap();
            for i in 0..self.width {
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
