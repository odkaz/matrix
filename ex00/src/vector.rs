use std::fmt::Display;
use std::ops::{Add, Mul, Sub};
use std::default::Default;

#[derive(Debug)]
pub struct Vector<T> {
    data: Vec<T>,
    len: usize,
}

impl<T, const N: usize> From<[T; N]> for Vector<T> {
    fn from(s: [T; N]) -> Vector<T> {
        let d = Vec::from(s);
        let l = d.len();
        return Vector { data: d, len: l };
    }
}

impl<T: std::fmt::Debug> Vector<T> {
    pub fn out(&mut self) {
        println!("{:?}", self.data);
    }
}

impl<T: Add<Output = T> + Clone> Vector<T> {
    pub fn add(&mut self, v: &Vector<T>) {
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

impl<T: Sub<Output = T> + Clone> Vector<T> {
    pub fn sub(&mut self, v: &Vector<T>) {
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

impl<T: Mul<Output = T> + Clone + Copy> Vector<T> {
    pub fn scl(&mut self, a: T) {
        let it = self.data.iter();
        let mut v = Vec::new();
        for item in it {
            v.push(item.clone() * a);
        }
        self.data = v;
    }
}

impl<T: Display> Display for Vector<T> {
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

impl<T: Clone+ Add<Output = T>> Add<Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: Vector<T>) -> Vector<T> {
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

impl<T> Mul<Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: Vector<T>) -> Vector<T> {
        self
    }
}