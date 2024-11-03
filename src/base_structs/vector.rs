use std::fmt::Display;
use std::ops::{Add, Div, Index, Mul, Sub};
extern crate num;
use crate::num_traits::scalar::Scalar;

pub type TVector<T, const R: usize> = Vector<T, R>;
pub type TVector2<T> = TVector<T, 2>;
pub type TVector3<T> = TVector<T, 3>;
pub type TVector4<T> = TVector<T, 4>;

#[derive(PartialEq, Debug, Clone)]
pub struct Vector<T, const R: usize> {
    data: Vec<T>,
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(s: [T; N]) -> Vector<T, N> {
        let d = Vec::from(s);
        return Vector { data: d };
    }
}

impl<T: std::fmt::Debug, const N: usize> Vector<T, N> {
    pub fn out(&self) {
        println!("{:?}", self.data);
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        if index >= N {
            panic!("Vector: index out of bound");
        }
        &self.data[index]
    }
}

impl<T: Scalar, const N: usize> Vector<T, N> {
    pub fn add(&mut self, v: &Vector<T, N>) {
        for i in 0..N {
            self.data[i] = self.data[i] + v.data[i];
        }
    }
}

impl<T: Scalar, const N: usize> Add<Vector<T, N>> for Vector<T, N> {
    type Output = Vector<T, N>;
    fn add(self, rhs: Vector<T, N>) -> Vector<T, N> {
        let mut res = [T::zero(); N];
        for i in 0..N {
            res[i] = self.data[i] + rhs.data[i];
        }
        Vector::from(res)
    }
}

impl<T: Scalar, const N: usize> Mul<T> for Vector<T, N> {
    type Output = Vector<T, N>;
    fn mul(self, rhs: T) -> Vector<T, N> {
        let mut res = [T::zero(); N];
        for i in 0..N {
            res[i] = self.data[i] * rhs;
        }
        Vector::from(res)
    }
}

impl<T: Scalar, const N: usize> Div<T> for Vector<T, N> {
    type Output = Vector<T, N>;
    fn div(self, rhs: T) -> Vector<T, N> {
        let mut res = [T::zero(); N];
        for i in 0..N {
            res[i] = self.data[i] / rhs;
        }
        Vector::from(res)
    }
}

impl<T: Scalar, const N: usize> Vector<T, N> {
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

impl<T: Scalar, const N: usize> Sub<Vector<T, N>> for Vector<T, N> {
    type Output = Vector<T, N>;
    fn sub(self, rhs: Vector<T, N>) -> Vector<T, N> {
        let mut res = [T::zero(); N];
        let l = self.as_slice();
        let r = rhs.as_slice();
        for i in 0..N {
            res[i] = l[i] - r[i];
        }
        Vector::from(res)
    }
}

impl<T: Scalar + Default, const N: usize> Sub<&Vector<T, N>> for &Vector<T, N> {
    type Output = Vector<T, N>;
    fn sub(self, rhs: &Vector<T, N>) -> Vector<T, N> {
        let mut res = [Default::default(); N];
        let l = self.as_slice();
        let r = rhs.as_slice();
        for i in 0..N {
            res[i] = l[i] - r[i];
        }
        Vector::from(res)
    }
}

impl<T: Scalar, const N: usize> Vector<T, N> {
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
        for i in 0..N {
            if i != 0 {
                write!(f, ", ").unwrap();
            }
            write!(f, "{}", self.data[i]).unwrap();
        }
        write!(f, "]").unwrap();
        Ok(())
    }
}

impl<T, const N: usize> Vector<T, N> {
    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }
}

impl<T: Clone, const N: usize> Vector<T, N> {
    pub fn as_vec(&self) -> Vec<T> {
        self.data.clone()
    }
}

impl<T: Scalar, const N: usize> Vector<T, N> {
    pub fn abs(&self) -> T {
        let mut total = T::zero();
        for i in 0..N {
            total = total + (self.data[i] * self.data[i]);
        }
        total.sqrt()
    }
}

impl<T: Scalar, const N: usize> Vector<T, N> {
    pub fn normalize(&self) -> Vector<T, N> {
        let mut total = T::zero();
        for i in 0..N {
            total = total + (self.data[i] * self.data[i]);
        }
        let sq = total.sqrt();
        let mut res = Vec::new();
        for item in self.data.clone() {
            res.push(item / sq);
        }
        Vector { data: res.clone() }
    }
}
