use alloc::format;
use alloc::string::String;
use core::sync::atomic::{Atomic, AtomicU64, Ordering};
use core::time::Duration;

const NANOS_PER_SEC: u64 = 1_000_000_000;
pub const INTERVALS_PER_SEC: u64 = NANOS_PER_SEC / 100;

pub struct FTIME {
    low: u32,
    high: u32,
}

pub struct ClockTime {
    inner: FTIME,
}

pub const UNIX_EPOCH: ClockTime = ClockTime::from_intervals(11_644_473_600 * INTERVALS_PER_SEC as i64);

impl ClockTime {
    pub const MAX: ClockTime = ClockTime::from_intervals(i64::MAX);
    pub const MIN: ClockTime = ClockTime::from_intervals(0);

    const fn from_intervals(intervals: i64) -> ClockTime {
        ClockTime {
            inner: FTIME {
                low: intervals as u32,
                high: (intervals >> 32) as u32,
            },
        }
    }

    fn intervals(&self) -> i64 {
        (self.inner.low as i64) | ((self.inner.high as i64) << 32)
    }

    pub fn intervals2dur(intervals: u64) -> Duration {
        Duration::new(intervals / INTERVALS_PER_SEC, ((intervals % INTERVALS_PER_SEC) * 100) as u32)
    }

    pub fn checked_dur2intervals(dur: &Duration) -> Option<i64> {
        dur.as_secs()
            .checked_mul(INTERVALS_PER_SEC)?
            .checked_add(dur.subsec_nanos() as u64 / 100)?
            .try_into()
            .ok()
    }

    pub fn dur2timeout(dur: Duration) -> u32 {
        dur.as_secs()
            .checked_mul(1000)
            .and_then(|ms| ms.checked_add((dur.subsec_nanos() as u64) / 1_000_000))
            .and_then(|ms| ms.checked_add(if dur.subsec_nanos() % 1_000_000 > 0 { 1 } else { 0 }))
            .map(|ms| if ms > <u32>::MAX as u64 { 4294967295u32 } else { ms as u32 })
            .unwrap_or(4294967295u32)
    }
}

pub type I32BOOL = i32;

pub fn now() -> i64 {
    let mut qpc_value: i64 = 0;
    cvt(unsafe { qpf(Some(&mut qpc_value)) }).unwrap();
    qpc_value
}

pub fn frequency() -> i64 {
    static FREQUENCY: Atomic<u64> = AtomicU64::new(0);

    let cached = FREQUENCY.load(Ordering::Relaxed);
    // If a previous thread has filled in this global state, use that.
    if cached != 0 {
        return cached as i64;
    }
    // ... otherwise learn for ourselves ...
    let mut frequency = 0;
    unsafe {
        cvt(qpf(Some(&mut frequency))).unwrap();
    }

    FREQUENCY.store(frequency as u64, Ordering::Relaxed);
    frequency
}

fn qpf(p0: Option<&mut i64>) -> I32BOOL {
    if p0.is_some() { *p0.unwrap() as I32BOOL } else { I32BOOL::default() }
}

pub fn epsilon() -> Duration {
    let epsilon = NANOS_PER_SEC / (frequency() as u64);
    Duration::from_nanos(epsilon)
}


pub trait IsZero {
    fn is_zero(&self) -> bool;
}

macro_rules! impl_is_zero {
    ($($t:ident)*) => ($(impl IsZero for $t {
        fn is_zero(&self) -> bool {
            *self == 0
        }
    })*)
    }

impl_is_zero! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }

pub trait IsMinusOne {
    fn is_minus_one(&self) -> bool;
}

macro_rules! impl_is_minus_one {
    ($($t:ident)*) => ($(impl IsMinusOne for $t {
        fn is_minus_one(&self) -> bool {
            *self == -1
        }
    })*)
}

impl_is_minus_one! { i8 i16 i32 i64 i128 isize }

pub fn cvt<I: IsZero>(i: I) -> Result<I, String> {
    if i.is_zero() { Err("zero".parse().unwrap()) } else { Ok(i) }
}

pub fn cvt_nz<I: IsZero>(i: I) -> Result<(), String> {
    if i.is_zero() { Ok(()) } else { Err("zero".parse().unwrap()) }
}

pub fn cvt_n<T: IsMinusOne>(t: T) -> Result<T, String> {
    if t.is_minus_one() { Err("minus one".parse().unwrap()) } else { Ok(t) }
}


pub fn cvt_gai(err: u64) -> Result<(), String> {
    if err == 0 { Ok(()) } else { Err(format!("cvt_gai: {}", err)) }
}

/// Just to provide the same interface as sys/pal/unix/net.rs
pub fn cvt_r<T, F>(mut f: F) -> Result<T, String>
where
    T: IsMinusOne + IsZero,
    F: FnMut() -> T,
{
    cvt(f())
}

pub mod types {
    use core::ops::{Add, Sub, Mul, Div, Rem};

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct i4(i8);

    impl i4 {
        pub const MIN: i8 = -8;
        pub const MAX: i8 = 7;

        /// Creates a new `I4` if the value is within bounds [-8, 7].
        #[inline]
        pub const fn new(value: i8) -> Option<Self> {
            if value >= Self::MIN && value <= Self::MAX {
                Some(Self(value))
            } else {
                None
            }
        }

        /// Unwraps the inner `i8` primitive value.
        #[inline]
        pub const fn get(self) -> i8 {
            self.0
        }

        /// Computes the absolute value of `self`.
        /// Note: `abs()` panics if called on `I4::MIN` (-8) because 8 exceeds `I4::MAX`.
        #[inline]
        pub fn abs(self) -> Self {
            let val = self.0.abs();
            Self::new(val).expect("Absolute value overflowed i4 bounds")
        }

        /// Raises `self` to the power of `exp`.
        #[inline]
        pub fn pow(self, exp: u32) -> Self {
            let val = self.0.pow(exp);
            Self::new(val).expect("Power calculation overflowed i4 bounds")
        }
    }

    // --- Arithmetic Traits ---

    impl Add for i4 {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Self::new(self.0 + rhs.0).expect("i4 addition overflowed")
        }
    }

    impl Sub for i4 {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self {
            Self::new(self.0 - rhs.0).expect("i4 subtraction overflowed")
        }
    }

    impl Mul for i4 {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self {
            Self::new(self.0 * rhs.0).expect("i4 multiplication overflowed")
        }
    }

    impl Div for i4 {
        type Output = Self;
        fn div(self, rhs: Self) -> Self {
            if rhs.0 == 0 {
                panic!("attempt to divide by zero");
            }
            Self::new(self.0 / rhs.0).expect("i4 division overflowed")
        }
    }

    impl Rem for i4 {
        type Output = Self;
        fn rem(self, rhs: Self) -> Self {
            if rhs.0 == 0 {
                panic!("attempt to calculate remainder with a divisor of zero");
            }
            Self::new(self.0 % rhs.0).unwrap()
        }
    }
}