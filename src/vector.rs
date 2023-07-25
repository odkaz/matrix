use std::fmt::Display;
use std::ops::{Add, Div, Index, Mul, Sub};
extern crate num;

use num::Float;
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

impl<T: Clone + Add<Output = T>, const N: usize> Add<Vector<T, N>> for Vector<T, N> {
    type Output = Vector<T, N>;
    fn add(self, rhs: Vector<T, N>) -> Vector<T, N> {
        let mut res = Vec::new();
        for i in 0..N {
            res.push(self.data[i].clone() + rhs.data[i].clone());
        }
        Vector { data: res }
    }
}

impl<T: Clone + Add<Output = T>, const N: usize> Add<&Vector<T, N>> for &Vector<T, N> {
    type Output = Vector<T, N>;
    fn add(self, rhs: &Vector<T, N>) -> Vector<T, N> {
        let mut res = Vec::new();
        for i in 0..N {
            res.push(self.data[i].clone() + rhs.data[i].clone());
        }
        Vector { data: res }
    }
}

// impl<T , const N: usize> Mul<Vector<T, N>> for Vector<T, N> {
//     type Output = Vector<T, N>;
//     fn mul(self, rhs: Vector<T, N>) -> Vector<T, N> {
//         self
//     }
// }

impl<T: Clone + Float, const N: usize> Mul<T> for Vector<T, N> {
    type Output = Vector<T, N>;
    fn mul(self, rhs: T) -> Vector<T, N> {
        let mut res = Vec::new();
        for i in 0..N {
            res.push(self.data[i].clone() * rhs);
        }
        Vector { data: res }
    }
}

impl<T: Clone + Float, const N: usize> Div<T> for Vector<T, N> {
    type Output = Vector<T, N>;
    fn div(self, rhs: T) -> Vector<T, N> {
        let mut res = Vec::new();
        for i in 0..N {
            res.push(self.data[i].clone() / rhs);
        }
        Vector { data: res }
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

impl<T: Float + Copy, const N: usize> Sub<Vector<T, N>> for Vector<T, N> {
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

impl<T: Default + Sub<Output = T> + Copy, const N: usize> Sub<&Vector<T, N>> for &Vector<T, N> {
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

impl<T: Display + Default + Clone + Add<T, Output = T> + Mul<T, Output = T>, const N: usize>
    Vector<T, N>
{
    pub fn dot(&self, v: &Vector<T, N>) -> T {
        let mut res = T::default();
        for (item1, item2) in self.data.iter().zip(v.data.iter()) {
            res = res + item1.clone() * item2.clone();
        }
        res
    }
}

impl<T: Float + Into<f32> + Add<f32, Output = f32>, const N: usize> Vector<T, N> {
    pub fn norm_1(&mut self) -> f32 {
        let mut res = f32::default();
        for i in self.data.iter() {
            res = i.clone().abs() + res;
        }
        res
    }

    pub fn norm(&mut self) -> f32 {
        let mut res = f32::default();
        for i in self.data.iter() {
            res = i.clone().powi(2) + res;
        }
        f32::sqrt(res)
    }

    pub fn norm_inf(&mut self) -> f32 {
        let max = self
            .data
            .iter()
            .max_by(|x, y| x.abs().partial_cmp(&y.abs()).unwrap())
            .unwrap();
        let cast: f32 = max.clone().abs().into();
        cast
    }
}

impl<T, const N: usize> Vector<T, N> {
    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }
}

impl<T: Copy, const N: usize> Vector<T, N> {
    pub fn as_vec(&self) -> Vec<T> {
        let mut res = Vec::new();
        for i in 0..N {
            res.push(self[i]);
        }
        res
    }
}

impl<T: Float, const N: usize> Vector<T, N> {
    pub fn abs(&self) -> T {
        let mut total = T::zero();
        for i in 0..N {
            total = total + (self.data[i] * self.data[i]);
        }
        total.sqrt()
    }
}

impl<T: Float, const N: usize> Vector<T, N> {
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

impl<T: Float, const N: usize> Vector<T, N> {
    pub fn linear_combination(u: &[Vector<T, N>], coefs: &[T]) -> Vector<T, N> {
        let mut res = Vector::from([T::zero(); N]);
        for (i, item) in u.iter().enumerate() {
            let mut tmp = item.clone();
            tmp.scl(coefs[i]);
            res = res + tmp;
        }
        res
    }
}

// impl<T: Float> TVector3<T> {
//     pub fn cross_product(u: &TVector3<T>, v: &TVector3<T>) -> TVector3<T> {
//         let mut res = [T::zero(); 3];
//         let u_arr = u.as_slice();
//         let v_arr = v.as_slice();
//         res[0] = u_arr[1] * v_arr[2] - u_arr[2] * v_arr[1];
//         res[1] = u_arr[2] * v_arr[0] - u_arr[0] * v_arr[2];
//         res[2] = u_arr[0] * v_arr[1] - u_arr[1] * v_arr[0];
//         Vector::from(res)
//     }
// }