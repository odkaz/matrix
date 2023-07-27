use std::ops::{Add, Mul, Div, Sub};
use std::fmt::{Display, Debug};

pub trait Scalar: Display + Debug + Clone + Copy + Add<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Sub<Output = Self> + PartialEq + PartialOrd {
    fn zero() -> Self;
    fn one() -> Self;
    fn abs(&self) -> Self;
    fn sqrt(&self) -> Self;
    fn powi(&self, i: i32) -> Self;
    fn scl(&self, s: Self) -> Self;
}

impl Scalar for f32 {
    fn zero() -> Self {
        0.0_f32
    }
    fn one() -> Self {
        1.0_f32
    }
    fn abs(&self) -> f32 {
        f32::abs(*self)
    }
    fn sqrt(&self) -> f32 {
        f32::sqrt(*self)
    }
    fn powi(&self, i: i32) -> f32 {
        f32::powi(*self, i)
    }
    fn scl(&self, s: Self) -> f32 {
        f32::scl(self, s)
    }
}