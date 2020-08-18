pub trait Zero<T> {
    fn zero() -> T;
}

pub trait One<T> {
    fn one() -> T;
}

impl Zero<f32> for f32 {
    fn zero() -> f32 {
        0 as f32
    }
}

impl Zero<f64> for f64 {
    fn zero() -> f64 {
        0 as f64
    }
}

impl Zero<i8> for i8 {
    fn zero() -> i8 {
        0 as i8
    }
}

impl Zero<i16> for i16 {
    fn zero() -> i16 {
        0 as i16
    }
}

impl Zero<i32> for i32 {
    fn zero() -> i32 {
        0 as i32
    }
}

impl Zero<i64> for i64 {
    fn zero() -> i64 {
        0 as i64
    }
}

impl Zero<i128> for i128 {
    fn zero() -> i128 {
        0 as i128
    }
}

impl Zero<u16> for u16 {
    fn zero() -> u16 {
        0 as u16
    }
}

impl Zero<u32> for u32 {
    fn zero() -> u32 {
        0 as u32
    }
}

impl Zero<u64> for u64 {
    fn zero() -> u64 {
        0 as u64
    }
}

impl Zero<u128> for u128 {
    fn zero() -> u128 {
        0 as u128
    }
}

impl Zero<bool> for bool {
    fn zero() -> bool {
        false
    }
}

impl One<f32> for f32 {
    fn one() -> f32 {
        1 as f32
    }
}

impl One<f64> for f64 {
    fn one() -> f64 {
        1 as f64
    }
}

impl One<i8> for i8 {
    fn one() -> i8 {
        1 as i8
    }
}

impl One<i16> for i16 {
    fn one() -> i16 {
        1 as i16
    }
}

impl One<i32> for i32 {
    fn one() -> i32 {
        1 as i32
    }
}

impl One<i64> for i64 {
    fn one() -> i64 {
        1 as i64
    }
}

impl One<i128> for i128 {
    fn one() -> i128 {
        1 as i128
    }
}

impl One<u16> for u16 {
    fn one() -> u16 {
        1 as u16
    }
}

impl One<u32> for u32 {
    fn one() -> u32 {
        1 as u32
    }
}

impl One<u64> for u64 {
    fn one() -> u64 {
        1 as u64
    }
}

impl One<u128> for u128 {
    fn one() -> u128 {
        1 as u128
    }
}

impl One<bool> for bool {
    fn one() -> bool {
        true
    }
}