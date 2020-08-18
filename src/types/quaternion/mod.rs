use crate::types::basic_types::TQuaternion;
use crate::types::basic_types::num_traits::*;

use std::ops::*;
use std::fmt::*;

impl<T> TQuaternion<T> where T : Zero<T> + One<T> {
    /// Creates TQuaternion whose members are (w: 1, x: 0, y: 0, z: 0)
    /// ```
    /// use iomath::quaternions::Quaternion;
    /// 
    /// let quaternion = Quaternion::identity();
    /// assert_eq!(quaternion, Quaternion { w: 1.0, x: 0.0, y: 0.0, z: 0.0 });
    /// ```
    pub fn identity() -> Self {
        Self {
            w: T::one(),
            y: T::zero(),
            x: T::zero(),
            z: T::zero()
        }
    }
}

impl<T> PartialEq for TQuaternion<T> where T : PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.w == other.w &&
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z
    }
}

impl<T> Debug for TQuaternion<T> where T : Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct(format!("TQuaternion<{}>", std::any::type_name::<T>()).as_str())
         .field("w", &self.w)
         .field("x", &self.x)
         .field("y", &self.y)
         .field("z", &self.z)
         .finish()
    }
}