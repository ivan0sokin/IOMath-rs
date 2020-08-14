use std::ops::*;
use std::fmt::*;

pub struct TVector2<T> {
    pub x: T,
    pub y: T
}

impl<T> TVector2<T> where T : Copy {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y
        }
    }
    
    pub fn from_scalar(scalar: T) -> Self {
        Self {
            x: scalar,
            y: scalar
        }
    }
}

impl<T> Copy for TVector2<T> where T : Copy { }
impl<T> Clone for TVector2<T> where T : Copy {
    fn clone(&self) -> Self {
        *self
    }

    fn clone_from(&mut self, source: &Self) {
        self.x = source.x;
        self.y = source.y;
    }
}

impl<T> Index<usize> for TVector2<T> {
    type Output = T;
    
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => &self.y
        }
    }
}

impl<T> IndexMut<usize> for TVector2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => &mut self.y
        }
    }
}

impl<T> PartialEq for TVector2<T> where T : PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
        self.y == other.y
    }
}

impl<T> Add<T> for TVector2<T> where T : Add<Output = T> + Copy {
    type Output = Self;

    fn add(self, scalar: T) -> Self::Output {
        Self {
            x: self.x + scalar,
            y: self.y + scalar
        }
    }
}

impl<T> Add<TVector2<T>> for TVector2<T> where T : Add<Output = T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<T> AddAssign<T> for TVector2<T> where T : AddAssign + Copy {
    fn add_assign(&mut self, scalar: T) {
        self.x += scalar;
        self.y += scalar;
    }
}

impl<T> AddAssign<TVector2<T>> for TVector2<T> where T : AddAssign {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T> Sub<T> for TVector2<T> where T : Sub<Output = T> + Copy {
    type Output = Self;

    fn sub(self, scalar: T) -> Self::Output {
        Self {
            x: self.x - scalar,
            y: self.y - scalar
        }
    }
}

impl<T> Sub<TVector2<T>> for TVector2<T> where T : Sub<Output = T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl<T> SubAssign<T> for TVector2<T> where T : SubAssign + Copy {
    fn sub_assign(&mut self, scalar: T) {
        self.x -= scalar;
        self.y -= scalar;
    }
}

impl<T> SubAssign<TVector2<T>> for TVector2<T> where T : SubAssign {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T> Mul<T> for TVector2<T> where T : Mul<Output = T> + Copy {
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar
        }
    }
}

impl<T> Mul<TVector2<T>> for TVector2<T> where T : Mul<Output = T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }
}

impl<T> MulAssign<T> for TVector2<T> where T : MulAssign + Copy {
    fn mul_assign(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl<T> MulAssign<TVector2<T>> for TVector2<T> where T : MulAssign {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl<T> Div<T> for TVector2<T> where T : Div<Output = T> + Copy {
    type Output = Self;

    fn div(self, scalar: T) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar
        }
    }
}

impl<T> Div<TVector2<T>> for TVector2<T> where T : Div<Output = T> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y
        }
    }
}

impl<T> DivAssign<T> for TVector2<T> where T : DivAssign + Copy {
    fn div_assign(&mut self, scalar: T) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl<T> DivAssign<TVector2<T>> for TVector2<T> where T : DivAssign {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
    }
}

impl<T> Neg for TVector2<T> where T : Neg<Output = T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y
        }
    }
}

impl<T> BitAnd<T> for TVector2<T> where T : BitAnd<Output = T> + Copy {
    type Output = Self;

    fn bitand(self, scalar: T) -> Self::Output {
        Self {
            x: self.x & scalar,
            y: self.y & scalar
        }
    }
}

impl<T> BitAnd<TVector2<T>> for TVector2<T> where T : BitAnd<Output = T> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x & rhs.x,
            y: self.y & rhs.y
        }
    }
}

impl<T> BitAndAssign<T> for TVector2<T> where T : BitAndAssign + Copy {
    fn bitand_assign(&mut self, scalar: T) {
        self.x &= scalar;
        self.y &= scalar;
    }
}

impl<T> BitAndAssign<TVector2<T>> for TVector2<T> where T : BitAndAssign {
    fn bitand_assign(&mut self, rhs: Self) {
        self.x &= rhs.x;
        self.y &= rhs.y;
    }
}

impl<T> BitOr<T> for TVector2<T> where T : BitOr<Output = T> + Copy {
    type Output = Self;

    fn bitor(self, scalar: T) -> Self::Output {
        Self {
            x: self.x | scalar,
            y: self.y | scalar
        }
    }
}

impl<T> BitOr<TVector2<T>> for TVector2<T> where T : BitOr<Output = T> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x | rhs.x,
            y: self.y | rhs.y
        }
    }
}

impl<T> BitOrAssign<T> for TVector2<T> where T : BitOrAssign + Copy {
    fn bitor_assign(&mut self, scalar: T) {
        self.x |= scalar;
        self.y |= scalar;
    }
}

impl<T> BitOrAssign<TVector2<T>> for TVector2<T> where T : BitOrAssign {
    fn bitor_assign(&mut self, rhs: Self) {
        self.x |= rhs.x;
        self.y |= rhs.y;
    }
}

impl<T> BitXor<T> for TVector2<T> where T : BitXor<Output = T> + Copy {
    type Output = Self;

    fn bitxor(self, scalar: T) -> Self::Output {
        Self {
            x: self.x ^ scalar,
            y: self.y ^ scalar
        }
    }
}

impl<T> BitXor<TVector2<T>> for TVector2<T> where T : BitXor<Output = T> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y
        }
    }
}

impl<T> BitXorAssign<T> for TVector2<T> where T : BitXorAssign + Copy {
    fn bitxor_assign(&mut self, scalar: T) {
        self.x ^= scalar;
        self.y ^= scalar;
    }
}

impl<T> BitXorAssign<TVector2<T>> for TVector2<T> where T : BitXorAssign {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.x ^= rhs.x;
        self.y ^= rhs.y;
    }
}

impl<T> Rem<T> for TVector2<T> where T : Rem<Output = T> + Copy {
    type Output = Self;

    fn rem(self, scalar: T) -> Self::Output {
        Self {
            x: self.x % scalar,
            y: self.y % scalar
        }
    }
}

impl<T> Rem<TVector2<T>> for TVector2<T> where T : Rem<Output = T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y
        }
    }
}

impl<T> RemAssign<T> for TVector2<T> where T : RemAssign + Copy {
    fn rem_assign(&mut self, scalar: T) {
        self.x %= scalar;
        self.y %= scalar;
    }
}

impl<T> RemAssign<TVector2<T>> for TVector2<T> where T : RemAssign {
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
    }
}

impl<T> Shl<T> for TVector2<T> where T : Shl<Output = T> + Copy {
    type Output = Self;

    fn shl(self, scalar: T) -> Self::Output {
        Self {
            x: self.x << scalar,
            y: self.y << scalar
        }
    }
}

impl<T> Shl<TVector2<T>> for TVector2<T> where T : Shl<Output = T> {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x << rhs.x,
            y: self.y << rhs.y
        }
    }
}

impl<T> ShlAssign<T> for TVector2<T> where T : ShlAssign + Copy {
    fn shl_assign(&mut self, scalar: T) {
        self.x <<= scalar;
        self.y <<= scalar;
    }
}

impl<T> ShlAssign<TVector2<T>> for TVector2<T> where T : ShlAssign {
    fn shl_assign(&mut self, rhs: Self) {
        self.x <<= rhs.x;
        self.y <<= rhs.y;
    }
}

impl<T> Shr<T> for TVector2<T> where T : Shr<Output = T> + Copy {
    type Output = Self;

    fn shr(self, scalar: T) -> Self::Output {
        Self {
            x: self.x >> scalar,
            y: self.y >> scalar
        }
    }
}

impl<T> Shr<TVector2<T>> for TVector2<T> where T : Shr<Output = T> {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x >> rhs.x,
            y: self.y >> rhs.y
        }
    }
}

impl<T> ShrAssign<T> for TVector2<T> where T : ShrAssign + Copy {
    fn shr_assign(&mut self, scalar: T) {
        self.x >>= scalar;
        self.y >>= scalar;
    }
}

impl<T> ShrAssign<TVector2<T>> for TVector2<T> where T : ShrAssign {
    fn shr_assign(&mut self, rhs: Self) {
        self.x >>= rhs.x;
        self.y >>= rhs.y;
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