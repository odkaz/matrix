use std::fmt::Display;
use std::ops::{Add, Mul, Sub};
use std::default::Default;
extern crate num;
use num::{Num, Float};
// use std::num::FromPrimitive;

pub type TVector<T, const R: usize> = Vector<T, R>;
pub type TVector2<T> = TVector<T, 2>;
pub type TVector3<T> = TVector<T, 3>;
pub type TVector4<T> = TVector<T, 4>;

#[derive(Debug)]
pub struct Vector<T, const R: usize> {
    data: Vec<T>,
    len: usize,
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(s: [T; N]) -> Vector<T, N> {
        let d = Vec::from(s);
        let l = d.len();
        return Vector { data: d, len: l };
    }
}

// impl<T: Clone, const N: usize> Vector<T, N> {
//     fn from(v: Vec<T>) -> Vector<T, N> {
//         Vector {
//             data: v.clone(),
//             len: v.len(),
//         }
//     }
// }

impl<T: std::fmt::Debug, const N: usize> Vector<T, N> {
    pub fn out(&mut self) {
        println!("{:?}", self.data);
    }
}

impl<T: Add<Output = T> + Clone, const N: usize> Vector<T, N> {
    pub fn add(&mut self, v: &Vector<T, N>) {
        let it1 = self.data.iter();
        let it2 = v.data.iter();
        let iter = it1.zip(it2);
        let mut v = Vec::new();
        for (item1, item2) in iter {
            let value = item1.clone() + item2.clone();
            v.push(value);
        }
        self.data = v;
    }
}

impl<T: Sub<Output = T> + Clone, const N: usize> Vector<T, N> {
    pub fn sub(&mut self, v: &Vector<T, N>) {
        let it1 = self.data.iter();
        let it2 = v.data.iter();
        let iter = it1.zip(it2);
        let mut v = Vec::new();
        for (item1, item2) in iter {
            let value = item1.clone() - item2.clone();
            v.push(value);
        }
        self.data = v;
    }
}

impl<T: Mul<Output = T> + Clone + Copy, const N: usize> Vector<T, N> {
    pub fn scl(&mut self, a: T) {
        let it = self.data.iter();
        let mut v = Vec::new();
        for item in it {
            v.push(item.clone() * a);
        }
        self.data = v;
    }
}

impl<T: Display, const N: usize> Display for Vector<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[").unwrap();
        for i in 0..self.len {
            if i != 0 {
                write!(f, ", ").unwrap();
            }
            write!(f, "{}", self.data[i]).unwrap();
        }
        write!(f, "]").unwrap();
        Ok(())
    }
}

// impl<T: Display + Default> Vector::<T> {
//     fn dot::<T>(&self, v: Vector::<T>) -> T {
//         let x: T = Default::default;
//         println!("{}", x);
//     }
// }

impl<T: Clone + Add<Output = T>, const N: usize> Add<Vector<T, N>> for Vector<T, N> {
    type Output = Vector<T, N>;
    fn add(self, rhs: Vector<T, N>) -> Vector<T, N> {
        let mut res = Vec::new();
        for i in 0..self.len {
            res.push(self.data[i].clone() + rhs.data[i].clone());
        }
        Vector {
            data: res,
            len: self.len,
        }
    }
}

impl<T , const N: usize> Mul<Vector<T, N>> for Vector<T, N> {
    type Output = Vector<T, N>;
    fn mul(self, rhs: Vector<T, N>) -> Vector<T, N> {
        self
    }
}



impl<T: Float + Display, const N: usize> Vector<T, N> {
    pub fn normalize (&self) -> Vector<T, N> {
        let mut total = T::zero();
        for i in 0..self.data.len() {
            total = total + (self.data[i] * self.data[i]);
        }
        println!("total{}", total);
        let sq = total.sqrt();
        let mut res = Vec::new();
        for item in self.data.clone() {
            res.push(item / sq);
        }
        Vector {
            data: res.clone(),
            len: res.len(),
        }
    }
}