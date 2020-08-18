use crate::types::basic_types::{ TVector2, TVector3, TVector4, TQuaternion };
use crate::types::basic_types::num_traits::Zero;

use std::ops::*;
use std::fmt::*;

impl<T> TVector2<T> where T : Zero<T> + Copy {
    /// Creates TVector2&lt;type&gt; whose x and y equal to zero
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
    
    /// Creates TVector2&lt;type&gt; whose x and y equal to scalar
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

    /// Creates TVector2&lt;type&gt; whose x and y equal to TVector3&lt;type&gt;'s x and y
    /// ```
    /// use iomath::vectors::{ Vector2, Vector3 };
    /// 
    /// let vector = Vector2::from_vector_3(Vector3::new(5.6, 3.8, 2.9));
    /// assert_eq!(vector, Vector2::new(5.6, 3.8))
    /// ```
    pub fn from_vector_3(vector: TVector3<T>) -> Self {
        Self {
            x: vector.x,
            y: vector.y
        }
    }

    /// Creates TVector2&lt;type&gt; whose x and y equal to TVector4&lt;type&gt;'s x and y
    /// ```
    /// use iomath::vectors::{ Vector2, Vector4 };
    /// 
    /// let vector = Vector2::from_vector_4(Vector4::new(0.3, 4.5, 2.9, 7.7));
    /// assert_eq!(vector, Vector2::new(0.3, 4.5))
    /// ```
    pub fn from_vector_4(vector: TVector4<T>) -> Self {
        Self {
            x: vector.x,
            y: vector.y
        }
    }

    /// Creates TVector2&lt;type&gt; whose x and y equal to TQuaternion&lt;type&gt;'s x and y
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
    /// Creates TVector3&lt;type&gt; whose x, y and z equal to zero
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
    
    /// Creates TVector3&lt;type&gt; whose x, y and z equal to scalar
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

    /// Creates TVector3&lt;type&gt; whose x, y and z equal to TVector4&lt;type&gt;'s x, y and z
    /// ```
    /// use iomath::vectors::{ Vector3, Vector4 };
    /// 
    /// let vector = Vector3::from_vector_4(Vector4::new(0.3, 4.5, 2.9, 7.7));
    /// assert_eq!(vector, Vector3::new(0.3, 4.5, 2.9))
    /// ```
    pub fn from_vector_4(vector: TVector4<T>) -> Self {
        Self {
            x: vector.x,
            y: vector.y,
            z: vector.z
        }
    }

    /// Creates TVector3&lt;type&gt; whose x equals to value, y and z equal to TVector2&lt;type&gt;'s x and y
    /// ```
    /// use iomath::vectors::{ Vector3, Vector2 };
    /// 
    /// let vector = Vector3::from_value_vector(-4.2, Vector2::new(7.3, 2.4));
    /// assert_eq!(vector, Vector3::new(-4.2, 7.3, 2.4));
    /// ```
    pub fn from_value_vector(value: T, vector: TVector2<T>) -> Self {
        Self {
            x: value,
            y: vector.x,
            z: vector.y
        }
    }

    /// Creates TVector3&lt;type&gt; whose x and y equal to TVector2&lt;type&gt;'s x and y, z equals to value
    /// ```
    /// use iomath::vectors::{ Vector3, Vector2 };
    /// 
    /// let vector = Vector3::from_vector_value(Vector2::new(7.3, 2.4), -4.2);
    /// assert_eq!(vector, Vector3::new(7.3, 2.4, -4.2));
    /// ```
    pub fn from_vector_value(vector: TVector2<T>, value: T) -> Self {
        Self {
            x: vector.x,
            y: vector.y,
            z: value
        }
    }

    /// Creates TVector3&lt;type&gt; whose x, y and z equals to TQuaternion&lt;type&gt;'s x, y and z
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
    /// Creates TVector4&lt;type&gt; whose x, y, z and w equal to zero
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

    /// Creates TVector4&lt;type&gt; whose x, y, z, and w equal to scalar
    /// ```
    /// use iomath::vectors::Vector4;
    /// 
    /// let vector = Vector4::from_scalar(9.0);
    /// assert_eq!(vector, Vector4::new(9.0, 9.0, 9.0, 9.0));
    /// ```
    pub fn from_scalar(scalar: T) -> Self {
        Self {
            x: scalar,
            y: scalar,
            z: scalar,
            w: scalar
        }
    }

    /// Creates TVector4&lt;type&gt; whose x equals to first value, y to second value, z and w to TVector2&lt;type&gt;'s x and y
    /// ```
    /// use iomath::vectors::{ Vector4, Vector2 };
    /// 
    /// let vector = Vector4::from_two_values_vector(4.0, 5.0, Vector2::new(6.0, 7.0));
    /// assert_eq!(vector, Vector4::new(4.0, 5.0, 6.0, 7.0));
    /// ```
    pub fn from_two_values_vector(first_val: T, second_val: T, vector: TVector2<T>) -> Self {
        Self {
            x: first_val,
            y: second_val,
            z: vector.x,
            w: vector.y
        }
    }

    /// Creates TVector4&lt;type&gt; whose x equals to first value,y and z to TVector2&lt;type&gt;'s x and y, w to second value
    /// ```
    /// use iomath::vectors::{ Vector4, Vector2 };
    /// 
    /// let vector = Vector4::from_value_vector_value(4.0, Vector2::new(6.0, 7.0), 5.0);
    /// assert_eq!(vector, Vector4::new(4.0, 6.0, 7.0, 5.0));
    /// ```
    pub fn from_value_vector_value(first_val: T, vector: TVector2<T>, second_val: T) -> Self {
        Self {
            x: first_val,
            y: vector.x,
            z: vector.y,
            w: second_val
        }
    }

    /// Creates TVector4&lt;type&gt; whose x and y equal to TVector2&lt;type&gt;'s x and y, z to first value, w to second value
    /// ```
    /// use iomath::vectors::{ Vector4, Vector2 };
    /// 
    /// let vector = Vector4::from_vector_two_values(Vector2::new(6.0, 7.0), 4.0, 5.0);
    /// assert_eq!(vector, Vector4::new(6.0, 7.0, 4.0, 5.0));
    /// ```
    pub fn from_vector_two_values(vector: TVector2<T>, first_val: T, second_val: T) -> Self {
        Self {
            x: vector.x,
            y: vector.y,
            z: first_val,
            w: second_val
        }
    }

    /// Creates TVector4&lt;type&gt; whose x and y equal to first TVector2&lt;type&gt;'s x and y, z and w to second TVector2&lt;type&gt;'s x and y
    /// ```
    /// use iomath::vectors::{ Vector4, Vector2 };
    /// 
    /// let vector = Vector4::from_two_vectors(Vector2::new(4.0, 7.0), Vector2::new(5.0, 6.0));
    /// assert_eq!(vector, Vector4::new(4.0, 7.0, 5.0, 6.0));
    /// ```
    pub fn from_two_vectors(first_vector: TVector2<T>, second_vector: TVector2<T>) -> Self {
        Self {
            x: first_vector.x,
            y: first_vector.y,
            z: second_vector.x,
            w: second_vector.y
        }
    }

    /// Creates TVector4&lt;type&gt; whose x equals to value, y, z and w equal to TVector3&lt;type&gt;'s x, y and z
    /// ```
    /// use iomath::vectors::{ Vector4, Vector3 };
    /// 
    /// let vector = Vector4::from_value_vector(1.7, Vector3::new(1.4, 5.5, 2.3));
    /// assert_eq!(vector, Vector4::new(1.7, 1.4, 5.5, 2.3));
    /// ```
    pub fn from_value_vector(value: T, vector: TVector3<T>) -> Self {
        Self {
            x: value,
            y: vector.x,
            z: vector.y,
            w: vector.z
        }
    }

    /// Creates TVector4&lt;type&gt; whose x, y and z equal to TVector3&lt;type&gt;'s x, y and z, w equals to value
    /// ```
    /// use iomath::vectors::{ Vector4, Vector3 };
    /// 
    /// let vector = Vector4::from_vector_value(Vector3::new(1.4, 5.5, 2.3), 1.7);
    /// assert_eq!(vector, Vector4::new(1.4, 5.5, 2.3, 1.7));
    /// ```
    pub fn from_vector_value(vector: TVector3<T>, value: T) -> Self {
        Self {
            x: vector.x,
            y: vector.y,
            z: vector.z,
            w: value
        }
    }

    /// Creates TVector4&lt;type&gt; whose x, y, z and w equal to TQuaternion&lt;type&gt;'s x, y, z and w
    /// ```
    /// use iomath::vectors::Vector4;
    /// use iomath::quaternions::Quaternion;
    /// 
    /// let vector = Vector4::from_quaternion(Quaternion::identity());
    /// assert_eq!(vector, Vector4::new(0.0, 0.0, 0.0, 1.0));
    /// ```
    pub fn from_quaternion(quat: TQuaternion<T>) -> Self {
        Self {
            x: quat.x,
            y: quat.y,
            z: quat.z,
            w: quat.w
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

impl<T> From<TVector2<T>> for TVector3<T> where T : Zero<T> {
    fn from(vector: TVector2<T>) -> Self {
        Self {
            x: vector.x,
            y: vector.y,
            z: T::zero()
        }
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

impl<T> Copy for TVector4<T> where T : Copy { }
impl<T> Clone for TVector4<T> where T : Copy {
    fn clone(&self) -> Self {
        *self
    }

    fn clone_from(&mut self, source: &Self) {
        self.x = source.x;
        self.y = source.y;
        self.z = source.z;
        self.w = source.w;
    }
}

impl<T> From<TVector2<T>> for TVector4<T> where T : Zero<T> {
    fn from(vector: TVector2<T>) -> Self {
        Self {
            x: vector.x,
            y: vector.y,
            z: T::zero(),
            w: T::zero()
        }
    }
}

impl<T> From<TVector3<T>> for TVector4<T> where T : Zero<T> {
    fn from(vector: TVector3<T>) -> Self {
        Self {
            x: vector.x,
            y: vector.y,
            z: vector.z,
            w: T::zero()
        }
    }
}

impl<T> Index<usize> for TVector4<T> {
    type Output = T;
    
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => &self.w
        }
    }
}

impl<T> IndexMut<usize> for TVector4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => &mut self.w
        }
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

impl<T> Add<T> for TVector4<T> where T : Add<Output = T> + Copy {
    type Output = Self;

    fn add(self, scalar: T) -> Self::Output {
        Self {
            x: self.x + scalar,
            y: self.y + scalar,
            z: self.z + scalar,
            w: self.w + scalar
        }
    }
}

impl<T> Add<TVector4<T>> for TVector4<T> where T : Add<Output = T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }
}

impl<T> AddAssign<T> for TVector4<T> where T : AddAssign + Copy {
    fn add_assign(&mut self, scalar: T) {
        self.x += scalar;
        self.y += scalar;
        self.z += scalar;
        self.w += scalar;
    }
}

impl<T> AddAssign<TVector4<T>> for TVector4<T> where T : AddAssign {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w
    }
}

impl<T> Sub<T> for TVector4<T> where T : Sub<Output = T> + Copy {
    type Output = Self;

    fn sub(self, scalar: T) -> Self::Output {
        Self {
            x: self.x - scalar,
            y: self.y - scalar,
            z: self.z - scalar,
            w: self.w - scalar
        }
    }
}

impl<T> Sub<TVector4<T>> for TVector4<T> where T : Sub<Output = T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w
        }
    }
}

impl<T> SubAssign<T> for TVector4<T> where T : SubAssign + Copy {
    fn sub_assign(&mut self, scalar: T) {
        self.x -= scalar;
        self.y -= scalar;
        self.z -= scalar;
        self.w -= scalar;
    }
}

impl<T> SubAssign<TVector4<T>> for TVector4<T> where T : SubAssign {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

impl<T> Mul<T> for TVector4<T> where T : Mul<Output = T> + Copy {
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar
        }
    }
}

impl<T> Mul<TVector4<T>> for TVector4<T> where T : Mul<Output = T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w
        }
    }
}

impl<T> MulAssign<T> for TVector4<T> where T : MulAssign + Copy {
    fn mul_assign(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
        self.w *= scalar;
    }
}

impl<T> MulAssign<TVector4<T>> for TVector4<T> where T : MulAssign {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
        self.w *= other.w;
    }
}

impl<T> Div<T> for TVector4<T> where T : Div<Output = T> + Copy {
    type Output = Self;

    fn div(self, scalar: T) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar
        }
    }
}

impl<T> Div<TVector4<T>> for TVector4<T> where T : Div<Output = T> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w
        }
    }
}

impl<T> DivAssign<T> for TVector4<T> where T : DivAssign + Copy {
    fn div_assign(&mut self, scalar: T) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
        self.w /= scalar;
    }
}

impl<T> DivAssign<TVector4<T>> for TVector4<T> where T : DivAssign {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
        self.w /= other.w;
    }
}

impl<T> Neg for TVector4<T> where T : Neg<Output = T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w
        }
    }
}

impl<T> BitAnd<T> for TVector4<T> where T : BitAnd<Output = T> + Copy {
    type Output = Self;

    fn bitand(self, scalar: T) -> Self::Output {
        Self {
            x: self.x & scalar,
            y: self.y & scalar,
            z: self.z & scalar,
            w: self.w & scalar
        }
    }
}

impl<T> BitAnd<TVector4<T>> for TVector4<T> where T : BitAnd<Output = T> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x & rhs.x,
            y: self.y & rhs.y,
            z: self.z & rhs.z,
            w: self.w & rhs.w
        }
    }
}

impl<T> BitAndAssign<T> for TVector4<T> where T : BitAndAssign + Copy {
    fn bitand_assign(&mut self, scalar: T) {
        self.x &= scalar;
        self.y &= scalar;
        self.z &= scalar;
        self.w &= scalar;
    }
}

impl<T> BitAndAssign<TVector4<T>> for TVector4<T> where T : BitAndAssign {
    fn bitand_assign(&mut self, rhs: Self) {
        self.x &= rhs.x;
        self.y &= rhs.y;
        self.z &= rhs.z;
        self.w &= rhs.w;
    }
}

impl<T> BitOr<T> for TVector4<T> where T : BitOr<Output = T> + Copy {
    type Output = Self;

    fn bitor(self, scalar: T) -> Self::Output {
        Self {
            x: self.x | scalar,
            y: self.y | scalar,
            z: self.z | scalar,
            w: self.w | scalar
        }
    }
}

impl<T> BitOr<TVector4<T>> for TVector4<T> where T : BitOr<Output = T> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x | rhs.x,
            y: self.y | rhs.y,
            z: self.z | rhs.z,
            w: self.w | rhs.w
        }
    }
}

impl<T> BitOrAssign<T> for TVector4<T> where T : BitOrAssign + Copy {
    fn bitor_assign(&mut self, scalar: T) {
        self.x |= scalar;
        self.y |= scalar;
        self.z |= scalar;
        self.w |= scalar;
    }
}

impl<T> BitOrAssign<TVector4<T>> for TVector4<T> where T : BitOrAssign {
    fn bitor_assign(&mut self, rhs: Self) {
        self.x |= rhs.x;
        self.y |= rhs.y;
        self.z |= rhs.z;
        self.w |= rhs.w;
    }
}

impl<T> BitXor<T> for TVector4<T> where T : BitXor<Output = T> + Copy {
    type Output = Self;

    fn bitxor(self, scalar: T) -> Self::Output {
        Self {
            x: self.x ^ scalar,
            y: self.y ^ scalar,
            z: self.z ^ scalar,
            w: self.w ^ scalar
        }
    }
}

impl<T> BitXor<TVector4<T>> for TVector4<T> where T : BitXor<Output = T> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y,
            z: self.z ^ rhs.z,
            w: self.w ^ rhs.w
        }
    }
}

impl<T> BitXorAssign<T> for TVector4<T> where T : BitXorAssign + Copy {
    fn bitxor_assign(&mut self, scalar: T) {
        self.x ^= scalar;
        self.y ^= scalar;
        self.z ^= scalar;
        self.w ^= scalar;
    }
}

impl<T> BitXorAssign<TVector4<T>> for TVector4<T> where T : BitXorAssign {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.x ^= rhs.x;
        self.y ^= rhs.y;
        self.z ^= rhs.z;
        self.w ^= rhs.w;
    }
}

impl<T> Rem<T> for TVector4<T> where T : Rem<Output = T> + Copy {
    type Output = Self;

    fn rem(self, scalar: T) -> Self::Output {
        Self {
            x: self.x % scalar,
            y: self.y % scalar,
            z: self.z % scalar,
            w: self.w % scalar
        }
    }
}

impl<T> Rem<TVector4<T>> for TVector4<T> where T : Rem<Output = T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
            z: self.z % rhs.z,
            w: self.w % rhs.w
        }
    }
}

impl<T> RemAssign<T> for TVector4<T> where T : RemAssign + Copy {
    fn rem_assign(&mut self, scalar: T) {
        self.x %= scalar;
        self.y %= scalar;
        self.z %= scalar;
        self.w %= scalar;
    }
}

impl<T> RemAssign<TVector4<T>> for TVector4<T> where T : RemAssign {
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
        self.z %= rhs.z;
        self.w %= rhs.w;
    }
}

impl<T> Shl<T> for TVector4<T> where T : Shl<Output = T> + Copy {
    type Output = Self;

    fn shl(self, scalar: T) -> Self::Output {
        Self {
            x: self.x << scalar,
            y: self.y << scalar,
            z: self.z << scalar,
            w: self.w << scalar
        }
    }
}

impl<T> Shl<TVector4<T>> for TVector4<T> where T : Shl<Output = T> {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x << rhs.x,
            y: self.y << rhs.y,
            z: self.z << rhs.z,
            w: self.w << rhs.w
        }
    }
}

impl<T> ShlAssign<T> for TVector4<T> where T : ShlAssign + Copy {
    fn shl_assign(&mut self, scalar: T) {
        self.x <<= scalar;
        self.y <<= scalar;
        self.z <<= scalar;
        self.w <<= scalar;
    }
}

impl<T> ShlAssign<TVector4<T>> for TVector4<T> where T : ShlAssign {
    fn shl_assign(&mut self, rhs: Self) {
        self.x <<= rhs.x;
        self.y <<= rhs.y;
        self.z <<= rhs.z;
        self.w <<= rhs.w;
    }
}

impl<T> Shr<T> for TVector4<T> where T : Shr<Output = T> + Copy {
    type Output = Self;

    fn shr(self, scalar: T) -> Self::Output {
        Self {
            x: self.x >> scalar,
            y: self.y >> scalar,
            z: self.z >> scalar,
            w: self.w >> scalar
        }
    }
}

impl<T> Shr<TVector4<T>> for TVector4<T> where T : Shr<Output = T> {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x >> rhs.x,
            y: self.y >> rhs.y,
            z: self.z >> rhs.z,
            w: self.w >> rhs.w
        }
    }
}

impl<T> ShrAssign<T> for TVector4<T> where T : ShrAssign + Copy {
    fn shr_assign(&mut self, scalar: T) {
        self.x >>= scalar;
        self.y >>= scalar;
        self.z >>= scalar;
        self.w >>= scalar;
    }
}

impl<T> ShrAssign<TVector4<T>> for TVector4<T> where T : ShrAssign {
    fn shr_assign(&mut self, rhs: Self) {
        self.x >>= rhs.x;
        self.y >>= rhs.y;
        self.z >>= rhs.z;
        self.w >>= rhs.w;
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