use crate::types::basic_types::{ TVector2, TVector3, TVector4, TQuaternion };
use crate::types::basic_types::num_traits::Zero;

use std::ops::*;
use std::fmt::*;

impl<T> TVector2<T> where T : Zero<T> + Copy {
    /// Creates TVector2&lt;type&gt; whose x and y equals to zero
    /// ```
    /// use iomath::vectors::Vector2;
    /// 
    /// let vector = Vector2::empty();
    /// assert_eq!(vector, Vector2 { x: 0.0, y: 0.0 });
    /// ```
    pub fn empty() -> Self {
        Self {
            x: T::zero(),
            y: T::zero()
        }
    }
    
    /// Creates TVector2&lt;type&gt; with x and y
    /// ```
    /// use iomath::vectors::Vector2;
    /// 
    /// let vector = Vector2::new(3.7, -1.3);
    /// assert_eq!(vector, Vector2 { x: 3.7, y: -1.3 });
    /// ```
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y
        }
    }
    
    /// Creates TVector2&lt;type&gt; whose x and y equals to scalar
    /// ```
    /// use iomath::vectors::Vector2;
    /// 
    /// let vector = Vector2::from_scalar(8.2);
    /// assert_eq!(vector, Vector2::new(8.2, 8.2));
    /// ```
    pub fn from_scalar(scalar: T) -> Self {
        Self {
            x: scalar,
            y: scalar
        }
    }

    /// Creates TVector2&lt;type&gt; whose x and y equals to other vector's x and y
    /// ```
    /// use iomath::vectors::{ Vector2, Vector3 };
    /// 
    /// let vector = Vector2::from_vector_3(Vector3::new(5.6, 3.8, 2.9));
    /// assert_eq!(vector, Vector2::new(5.6, 3.8))
    /// ```
    pub fn from_vector_3(other: TVector3<T>) -> Self {
        Self {
            x: other.x,
            y: other.y
        }
    }

    /// Creates TVector2&lt;type&gt; whose x and y equals to other vector's x and y
    /// ```
    /// use iomath::vectors::{ Vector2, Vector4 };
    /// 
    /// let vector = Vector2::from_vector_4(Vector4::new(0.3, 4.5, 2.9, 7.7));
    /// assert_eq!(vector, Vector2::new(0.3, 4.5))
    /// ```
    pub fn from_vector_4(other: TVector4<T>) -> Self {
        Self {
            x: other.x,
            y: other.y
        }
    }

    /// Creates TVector2&lt;type&gt; whose x and y equals to quaternion's x and y
    /// ```
    /// use iomath::vectors::{ Vector2, Vector4 };
    /// use iomath::quaternions::Quaternion;
    /// 
    /// let vector = Vector2::from_quaternion(Quaternion::identity());
    /// assert_eq!(vector, Vector2::new(0.0, 0.0))
    /// ```
    pub fn from_quaternion(quat: TQuaternion<T>) -> Self {
        Self {
            x: quat.x,
            y: quat.y
        }
    }
}

impl<T> TVector3<T> where T : Zero<T> + Copy {
    /// Creates TVector3&lt;type&gt; whose x, y and z equals to zero
    /// ```
    /// use iomath::vectors::Vector3;
    /// 
    /// let vector = Vector3::empty();
    /// assert_eq!(vector, Vector3 { x: 0.0, y: 0.0, z: 0.0 });
    /// ```
    pub fn empty() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero()
        }
    }
    
    /// Creates TVector3&lt;type&gt; with x, y and z
    /// ```
    /// use iomath::vectors::Vector3;
    /// 
    /// let vector = Vector3::new(3.7, -1.3, 0.5);
    /// assert_eq!(vector, Vector3 { x: 3.7, y: -1.3, z: 0.5 });
    /// ```
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z
        }
    }
    
    /// Creates TVector3&lt;type&gt; whose x, y and z equals to scalar
    /// ```
    /// use iomath::vectors::Vector3;
    /// 
    /// let vector = Vector3::from_scalar(8.2);
    /// assert_eq!(vector, Vector3::new(8.2, 8.2, 8.2));
    /// ```
    pub fn from_scalar(scalar: T) -> Self {
        Self {
            x: scalar,
            y: scalar,
            z: scalar
        }
    }

    /// Creates TVector3&lt;type&gt; whose x, y and z equals to other vector's x, y and z
    /// ```
    /// use iomath::vectors::{ Vector3, Vector4 };
    /// 
    /// let vector = Vector3::from_vector_4(Vector4::new(0.3, 4.5, 2.9, 7.7));
    /// assert_eq!(vector, Vector3::new(0.3, 4.5, 2.9))
    /// ```
    pub fn from_vector_4(other: TVector4<T>) -> Self {
        Self {
            x: other.x,
            y: other.y,
            z: other.z
        }
    }

    /// Creates TVector3&lt;type&gt; whose x equals to value, y and z equals to other vector's x and y
    /// ```
    /// use iomath::vectors::{ Vector3, Vector2 };
    /// 
    /// let vector = Vector3::from_value_vector(-4.2, Vector2::new(7.3, 2.4));
    /// assert_eq!(vector, Vector3::new(-4.2, 7.3, 2.4));
    /// ```
    pub fn from_value_vector(value: T, other: TVector2<T>) -> Self {
        Self {
            x: value,
            y: other.x,
            z: other.y
        }
    }

    /// Creates TVector3&lt;type&gt; whose x and y equals to other vector's x and y, z equals to value
    /// ```
    /// use iomath::vectors::{ Vector3, Vector2 };
    /// 
    /// let vector = Vector3::from_vector_value(Vector2::new(7.3, 2.4), -4.2);
    /// assert_eq!(vector, Vector3::new(7.3, 2.4, -4.2));
    /// ```
    pub fn from_vector_value(other: TVector2<T>, value: T) -> Self {
        Self {
            x: other.x,
            y: other.y,
            z: value
        }
    }

    /// Creates TVector3&lt;type&gt; whose x, y and z equals to quaternion's x, y and z
    /// ```
    /// use iomath::vectors::Vector3;
    /// use iomath::quaternions::Quaternion;
    /// 
    /// let vector = Vector3::from_quaternion(Quaternion::identity());
    /// assert_eq!(vector, Vector3::new(0.0, 0.0, 0.0))
    /// ```
    pub fn from_quaternion(quat: TQuaternion<T>) -> Self {
        Self {
            x: quat.x,
            y: quat.y,
            z: quat.z
        }
    }
}

impl<T> TVector4<T> where T : Zero<T> + Copy {
    /// Creates TVector4&lt;type&gt; whose x, y, z and w equals to zero
    /// ```
    /// use iomath::vectors::Vector4;
    /// 
    /// let vector = Vector4::empty();
    /// assert_eq!(vector, Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 });
    /// ```
    pub fn empty() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
            w: T::zero()
        }
    }

    /// Creates TVector4&lt;type&gt; with x, y, z and w
    /// ```
    /// use iomath::vectors::Vector4;
    /// 
    /// let vector = Vector4::new(3.7, -1.3, 0.5, 6.4);
    /// assert_eq!(vector, Vector4 { x: 3.7, y: -1.3, z: 0.5, w: 6.4 });
    /// ```
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self {
            x,
            y,
            z,
            w
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

impl<T> Copy for TVector3<T> where T : Copy { }
impl<T> Clone for TVector3<T> where T : Copy {
    fn clone(&self) -> Self {
        *self
    }

    fn clone_from(&mut self, source: &Self) {
        self.x = source.x;
        self.y = source.y;
        self.z = source.z;
    }
}

impl<T> Index<usize> for TVector3<T> {
    type Output = T;
    
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &self.z
        }
    }
}

impl<T> IndexMut<usize> for TVector3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => &mut self.z
        }
    }
}

impl<T> PartialEq for TVector3<T> where T : PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z
    }
}

impl<T> Add<T> for TVector3<T> where T : Add<Output = T> + Copy {
    type Output = Self;

    fn add(self, scalar: T) -> Self::Output {
        Self {
            x: self.x + scalar,
            y: self.y + scalar,
            z: self.z + scalar
        }
    }
}

impl<T> Add<TVector3<T>> for TVector3<T> where T : Add<Output = T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl<T> AddAssign<T> for TVector3<T> where T : AddAssign + Copy {
    fn add_assign(&mut self, scalar: T) {
        self.x += scalar;
        self.y += scalar;
        self.z += scalar;
    }
}

impl<T> AddAssign<TVector3<T>> for TVector3<T> where T : AddAssign {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T> Sub<T> for TVector3<T> where T : Sub<Output = T> + Copy {
    type Output = Self;

    fn sub(self, scalar: T) -> Self::Output {
        Self {
            x: self.x - scalar,
            y: self.y - scalar,
            z: self.z - scalar
        }
    }
}

impl<T> Sub<TVector3<T>> for TVector3<T> where T : Sub<Output = T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl<T> SubAssign<T> for TVector3<T> where T : SubAssign + Copy {
    fn sub_assign(&mut self, scalar: T) {
        self.x -= scalar;
        self.y -= scalar;
        self.z -= scalar;
    }
}

impl<T> SubAssign<TVector3<T>> for TVector3<T> where T : SubAssign {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T> Mul<T> for TVector3<T> where T : Mul<Output = T> + Copy {
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }
}

impl<T> Mul<TVector3<T>> for TVector3<T> where T : Mul<Output = T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl<T> MulAssign<T> for TVector3<T> where T : MulAssign + Copy {
    fn mul_assign(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl<T> MulAssign<TVector3<T>> for TVector3<T> where T : MulAssign {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl<T> Div<T> for TVector3<T> where T : Div<Output = T> + Copy {
    type Output = Self;

    fn div(self, scalar: T) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar
        }
    }
}

impl<T> Div<TVector3<T>> for TVector3<T> where T : Div<Output = T> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}

impl<T> DivAssign<T> for TVector3<T> where T : DivAssign + Copy {
    fn div_assign(&mut self, scalar: T) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl<T> DivAssign<TVector3<T>> for TVector3<T> where T : DivAssign {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl<T> Neg for TVector3<T> where T : Neg<Output = T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl<T> BitAnd<T> for TVector3<T> where T : BitAnd<Output = T> + Copy {
    type Output = Self;

    fn bitand(self, scalar: T) -> Self::Output {
        Self {
            x: self.x & scalar,
            y: self.y & scalar,
            z: self.z & scalar
        }
    }
}

impl<T> BitAnd<TVector3<T>> for TVector3<T> where T : BitAnd<Output = T> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x & rhs.x,
            y: self.y & rhs.y,
            z: self.z & rhs.z
        }
    }
}

impl<T> BitAndAssign<T> for TVector3<T> where T : BitAndAssign + Copy {
    fn bitand_assign(&mut self, scalar: T) {
        self.x &= scalar;
        self.y &= scalar;
        self.z &= scalar;
    }
}

impl<T> BitAndAssign<TVector3<T>> for TVector3<T> where T : BitAndAssign {
    fn bitand_assign(&mut self, rhs: Self) {
        self.x &= rhs.x;
        self.y &= rhs.y;
        self.z &= rhs.z;
    }
}

impl<T> BitOr<T> for TVector3<T> where T : BitOr<Output = T> + Copy {
    type Output = Self;

    fn bitor(self, scalar: T) -> Self::Output {
        Self {
            x: self.x | scalar,
            y: self.y | scalar,
            z: self.z | scalar
        }
    }
}

impl<T> BitOr<TVector3<T>> for TVector3<T> where T : BitOr<Output = T> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x | rhs.x,
            y: self.y | rhs.y,
            z: self.z | rhs.z
        }
    }
}

impl<T> BitOrAssign<T> for TVector3<T> where T : BitOrAssign + Copy {
    fn bitor_assign(&mut self, scalar: T) {
        self.x |= scalar;
        self.y |= scalar;
        self.z |= scalar;
    }
}

impl<T> BitOrAssign<TVector3<T>> for TVector3<T> where T : BitOrAssign {
    fn bitor_assign(&mut self, rhs: Self) {
        self.x |= rhs.x;
        self.y |= rhs.y;
        self.z |= rhs.z;
    }
}

impl<T> BitXor<T> for TVector3<T> where T : BitXor<Output = T> + Copy {
    type Output = Self;

    fn bitxor(self, scalar: T) -> Self::Output {
        Self {
            x: self.x ^ scalar,
            y: self.y ^ scalar,
            z: self.z ^ scalar
        }
    }
}

impl<T> BitXor<TVector3<T>> for TVector3<T> where T : BitXor<Output = T> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y,
            z: self.z ^ rhs.z
        }
    }
}

impl<T> BitXorAssign<T> for TVector3<T> where T : BitXorAssign + Copy {
    fn bitxor_assign(&mut self, scalar: T) {
        self.x ^= scalar;
        self.y ^= scalar;
        self.z ^= scalar;
    }
}

impl<T> BitXorAssign<TVector3<T>> for TVector3<T> where T : BitXorAssign {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.x ^= rhs.x;
        self.y ^= rhs.y;
        self.z ^= rhs.z;
    }
}

impl<T> Rem<T> for TVector3<T> where T : Rem<Output = T> + Copy {
    type Output = Self;

    fn rem(self, scalar: T) -> Self::Output {
        Self {
            x: self.x % scalar,
            y: self.y % scalar,
            z: self.z % scalar
        }
    }
}

impl<T> Rem<TVector3<T>> for TVector3<T> where T : Rem<Output = T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
            z: self.z % rhs.z
        }
    }
}

impl<T> RemAssign<T> for TVector3<T> where T : RemAssign + Copy {
    fn rem_assign(&mut self, scalar: T) {
        self.x %= scalar;
        self.y %= scalar;
        self.z %= scalar;
    }
}

impl<T> RemAssign<TVector3<T>> for TVector3<T> where T : RemAssign {
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
        self.z %= rhs.z;
    }
}

impl<T> Shl<T> for TVector3<T> where T : Shl<Output = T> + Copy {
    type Output = Self;

    fn shl(self, scalar: T) -> Self::Output {
        Self {
            x: self.x << scalar,
            y: self.y << scalar,
            z: self.z << scalar
        }
    }
}

impl<T> Shl<TVector3<T>> for TVector3<T> where T : Shl<Output = T> {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x << rhs.x,
            y: self.y << rhs.y,
            z: self.z << rhs.z
        }
    }
}

impl<T> ShlAssign<T> for TVector3<T> where T : ShlAssign + Copy {
    fn shl_assign(&mut self, scalar: T) {
        self.x <<= scalar;
        self.y <<= scalar;
        self.z <<= scalar;
    }
}

impl<T> ShlAssign<TVector3<T>> for TVector3<T> where T : ShlAssign {
    fn shl_assign(&mut self, rhs: Self) {
        self.x <<= rhs.x;
        self.y <<= rhs.y;
        self.z <<= rhs.z;
    }
}

impl<T> Shr<T> for TVector3<T> where T : Shr<Output = T> + Copy {
    type Output = Self;

    fn shr(self, scalar: T) -> Self::Output {
        Self {
            x: self.x >> scalar,
            y: self.y >> scalar,
            z: self.z >> scalar
        }
    }
}

impl<T> Shr<TVector3<T>> for TVector3<T> where T : Shr<Output = T> {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x >> rhs.x,
            y: self.y >> rhs.y,
            z: self.z >> rhs.z
        }
    }
}

impl<T> ShrAssign<T> for TVector3<T> where T : ShrAssign + Copy {
    fn shr_assign(&mut self, scalar: T) {
        self.x >>= scalar;
        self.y >>= scalar;
        self.z >>= scalar;
    }
}

impl<T> ShrAssign<TVector3<T>> for TVector3<T> where T : ShrAssign {
    fn shr_assign(&mut self, rhs: Self) {
        self.x >>= rhs.x;
        self.y >>= rhs.y;
        self.z >>= rhs.z;
    }
}

impl<T> Debug for TVector3<T> where T : Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {        
        f.debug_struct(format!("TVector3<{}>", std::any::type_name::<T>()).as_str())
         .field("x", &self.x)
         .field("y", &self.y)
         .field("z", &self.z)
         .finish()
    }
}

impl<T> PartialEq for TVector4<T> where T : PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z &&
        self.w == other.w
    }
}

impl<T> Debug for TVector4<T> where T : Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct(format!("TVector4<{}>", std::any::type_name::<T>()).as_str())
         .field("x", &self.x)
         .field("y", &self.y)
         .field("z", &self.z)
         .field("w", &self.w)
         .finish()
    }
}