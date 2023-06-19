use num::{Num, Float};

pub trait Rnum : Num + Copy {

}

// impl Rnum for Float {
//     fn norm(self) -> Float {
//         self.abs();
//     }
// }