use std::ops::*;
use std::fmt::*;

pub struct TVector2<T> {
    pub x: T,
    pub y: T
}

impl<T> TVector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y
        }
    }
}

// impl<T> Default for TVector2<T> where T : From<i32> {
//     fn default() -> Self {
//         let x: i32 = 0;
//         Self {
//             x: x as T,
//             y: 0
//         }
//     }
// }

impl<T> PartialEq for TVector2<T> where T : PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
        self.y == other.y
    }
}

impl<T> Add<T> for TVector2<T> where T : Add<Output = T> + Copy {
    type Output = Self;

    fn add(self, scalar: T) -> Self {
        Self {
            x: self.x + scalar,
            y: self.y + scalar
        }
    }
}

impl<T> Add<TVector2<T>> for TVector2<T> where T : Add<Output = T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<T> AddAssign<TVector2<T>> for TVector2<T> where T : AddAssign {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T> AddAssign<T> for TVector2<T> where T : AddAssign + Copy {
    fn add_assign(&mut self, scalar: T) {
        self.x += scalar;
        self.y += scalar;
    }
}

impl<T> Debug for TVector2<T> where T : Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct(format!("TVector2<{}>", std::any::type_name::<T>()).as_str())
         .field("x", &self.x)
         .field("y", &self.y)
         .finish()
    }
}