
use std::ops::{Add, Sub, Mul};
use std::fmt::Display;


pub struct Vector<T> {
    data: Vec<T>,
    len: usize,
}

impl<T, const N: usize> From<[T; N]> for Vector<T> {
    fn from(s: [T; N]) -> Vector<T> {
        let d = Vec::from(s);
        let l = d.len();
        return Vector {
            data: d,
            len: l,
        }
    }
}

impl<T: std::fmt::Debug> Vector<T> {
    pub fn out(&mut self) {
        println!("{:?}", self.data);
    }
}

impl<T: Display + Add<Output = T> + Clone> Vector<T>{
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

impl<T: Display + Sub<Output = T> + Clone> Vector<T>{
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

impl<T: Display + Mul<Output = T> + Clone + Copy> Vector<T>{
    pub fn scl(&mut self, a: T) {
        let it = self.data.iter();
        let mut v = Vec::new();
        for item in it {
            v.push(item.clone() * a);
        }
        self.data = v;
    }
}