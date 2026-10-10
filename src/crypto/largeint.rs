use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::fmt;
use core::fmt::{Debug, Display};
use num_traits::{FromBytes, ToBytes};
use crate::rng::{Distribution, Rng, RngExt, StandardUniform};

#[macro_export]
macro_rules! cfg_32 {
    ($($any:tt)+) => {
        #[cfg(not(target_pointer_width = "64"))] $($any)+
    }
}

#[macro_export]
macro_rules! cfg_64 {
    ($($any:tt)+) => {
        #[cfg(target_pointer_width = "64")] $($any)+
    }
}

#[macro_export]
macro_rules! cfg_digit {
    ($item32:item $item64:item) => {
        cfg_32!($item32);
        cfg_64!($item64);
    };
}

#[macro_export]
macro_rules! cfg_digit_expr {
    ($expr32:expr, $expr64:expr) => {
        cfg_32!($expr32);
        cfg_64!($expr64);
    };
}

cfg_digit!(
    pub(crate) type LargeDigit = u32;
    pub(crate) type LargeDigit = u64;
);

cfg_digit!(
    pub(crate) type DoubleLargeDigit = u64;
    pub(crate) type DoubleLargeDigit = u128;
);

pub(crate) const BITS: u8 = LargeDigit::BITS as u8;
pub(crate) const HALF_BITS: u8 = BITS / 2;
pub(crate) const HALF: LargeDigit = (1 << HALF_BITS) - 1;

pub(crate) const MAX: LargeDigit = LargeDigit::MAX;
const LO_MASK: DoubleLargeDigit = MAX as DoubleLargeDigit;

#[inline]
fn highdigit(n: DoubleLargeDigit) -> LargeDigit {
    (n >> BITS) as LargeDigit
}
#[inline]
fn lowdigit(n: DoubleLargeDigit) -> LargeDigit {
    (n & LO_MASK) as LargeDigit
}

#[inline]
pub(crate) fn from_doublelargedigit(n: DoubleLargeDigit) -> (LargeDigit, LargeDigit) {
    (highdigit(n), lowdigit(n))
}

#[inline]
pub(crate) fn to_doublelargedigit(hi: LargeDigit, lo: LargeDigit) -> DoubleLargeDigit {
    DoubleLargeDigit::from(lo) | (DoubleLargeDigit::from(hi) << BITS)
}

#[derive(Clone, Debug)]
pub(crate) enum LargeDigits {
    Inline(Option<LargeDigit>),
    Heap(Vec<LargeDigit>),
}

impl LargeDigits {
    pub const ZERO: Self = Self::Inline(Some(0));
    pub const ONE: Self = Self::Inline(Some(1));
    
    pub fn from_digit(x: LargeDigit) -> Self {
        if x == 0 {
            LargeDigits::ZERO
        } else {
            LargeDigits::Inline(Some(x))
        }
    }
    
    pub fn from_slice(slice: &[LargeDigit]) -> Self {
        match slice {
            &[] => LargeDigits::ZERO,
            &[x] => LargeDigits::Inline(Some(x)),
            xs => LargeDigits::Heap(xs.to_vec()),
        }
    }
    
    pub fn from_vec_slice(vec: Vec<LargeDigit>) -> Self {
        match vec.as_slice() {
            &[] => LargeDigits::ZERO,
            &[x] => LargeDigits::Inline(Some(x)),
            xs => LargeDigits::Heap(xs.to_vec()),
        }
    }

    pub(crate) fn from_vec(xs: Vec<LargeDigit>) -> Self {
        LargeDigits::Heap(xs)
    }
    
    pub fn push(&mut self, x: LargeDigit) {
        match self {
            LargeDigits::Inline(Some(y)) => {
                let mut vec = Vec::with_capacity(2);
                vec.push(*y);
                vec.push(x);
                *self = LargeDigits::Heap(vec);
            }
            LargeDigits::Inline(None) => {
                *self = LargeDigits::Inline(Some(x));
            }
            LargeDigits::Heap(vec) => {
                vec.push(x);
            }
        }
    }
    
    pub fn pop(&mut self) -> Option<LargeDigit> {
        match self {
            LargeDigits::Inline(Some(x)) => {
                *self = LargeDigits::Inline(None);
                Some(*x)
            }
            LargeDigits::Inline(None) => None,
            LargeDigits::Heap(vec) => vec.pop(),
        }
    }
    
    pub fn len(&self) -> usize {
        match self {
            LargeDigits::Inline(Some(_)) => 1,
            LargeDigits::Inline(None) => 0,
            LargeDigits::Heap(vec) => vec.len(),
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn last(&self) -> Option<&LargeDigit> {
        match self {
            LargeDigits::Inline(x) => x.as_ref(),
            LargeDigits::Heap(xs) => xs.last(),
        }
    }

    pub fn normalize(&mut self) {
        match self {
            LargeDigits::Inline(x) => {
                if let Some(0) = *x {
                    *x = None;
                }
            }
            LargeDigits::Heap(xs) => {
                if let [.., 0] = **xs {
                    let len = xs.iter().rposition(|&d| d != 0).map_or(0, |i| i + 1);
                    xs.truncate(len);
                }
                if xs.len() < xs.capacity() / 2 {
                    match **xs {
                        [] => *self = LargeDigits::ZERO,
                        [x] => *self = LargeDigits::Inline(Some(x)),
                        _ => xs.shrink_to(xs.len() + 1),
                    }
                }
            }
        }
    }
    
    pub(crate) fn capacity(&self) -> usize {
        match self {
            LargeDigits::Inline(_) => 1,
            LargeDigits::Heap(xs) => xs.capacity(),
        }
    }
    
    pub(crate) fn shrink(&mut self) {
        if let LargeDigits::Heap(xs) = self {
            if xs.len() < xs.capacity() / 2 {
                match **xs {
                    [] => *self = LargeDigits::ZERO,
                    [x] => *self = LargeDigits::Inline(Some(x)),
                    _ => xs.shrink_to(xs.len() + 1),
                }
            }
        }
    }
    pub(crate) fn drain_front(&mut self, len: usize) {
        match self {
            LargeDigits::Inline(x) => {
                assert!(len <= 1);
                if len == 1 {
                    *x = None;
                }
            }
            LargeDigits::Heap(xs) => {
                xs.drain(..len);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct LargeUint {
    data: LargeDigits,
}


pub trait LargeInteger:
// ToBits
// + FromBits
/*+ ToBytes
+ FromBytes
+ */Copy
+ Clone
+ Debug
+ Default
+ Display
+ Eq
+ Ord
+ Send
+ Sized
+ Sync
+ 'static
+ Uniform
+ AsMut<[u64]>
+ AsRef<[u64]>
+ From<u64>

{
    const NUM_LIMBS: usize;

    fn add_nocarry(&mut self, other: &Self) -> bool;
    fn sub_noborrow(&mut self, other: &Self) -> bool;
    fn mul_nocarry(&mut self, other: &Self) -> bool;
    fn div_norem(&mut self, other: &Self) -> bool;

    // lshift 2
    fn mul2(&mut self);
    
    //lshift n
    fn muln(&mut self, amt: u32);
    
    //rshift 2    
    fn div2(&mut self);
    
    //rshift n
    fn divn(&mut self, amt: u32);
    
    fn is_odd(&self) -> bool;
    
    fn is_even(&self) -> bool;
    
    fn is_zero(&self) -> bool;

    //encoding required number of bits, always multiple of 64
    fn num_bits(&self) -> u32;

    /// Compute the `i`-th bit of `self`.
    fn get_bit(&self, i: usize) -> bool;

    /// Returns the LargeUint representation.
    fn to_largeuint(&self) -> LargeUint;

    /// Returns a vector for wnaf.
    fn find_wnaf(&self) -> Vec<i64>;

    fn is_null(&self) -> bool {
        self.is_zero()
    }
}



pub trait Uniform: Sized {
    fn rand<R: RngExt + ?Sized>(rng: &mut R) -> Self;
}

impl<T> Uniform for T
where
    StandardUniform: Distribution<T>,
{
    #[inline]
    fn rand<R: RngExt + ?Sized>(rng: &mut R) -> Self {
        rng.random()
    }
}

// The rand 0.8 default implementation.
pub trait UniformExt: Uniform {
    fn rand_option<R: Rng + ?Sized>(rng: &mut R) -> Option<Self> {
        if !rng.random().is_null() {
            Some(Self::rand(rng)) } else { None }
    }
}

impl<T: Uniform> UniformExt for T {}

#[derive(Copy, Clone, PartialEq, Eq, Default, Hash)]
pub struct LargeInteger256(pub [u64; 4]);

impl LargeInteger256 {
    pub const fn new(value: [u64; 4]) -> Self {
        LargeInteger256(value)
    }
}

impl LargeInteger for LargeInteger256 {
    const NUM_LIMBS: usize = 4;

    fn add_nocarry(&mut self, other: &Self) -> bool {
        let mut carry = 0;
        for i in 0..Self::NUM_LIMBS {
            let (sum, c) = self.0[i].overflowing_add(other.0[i]);
            let (sum, c2) = sum.overflowing_add(carry);
            self.0[i] = sum;
            carry = (c as u64) + (c2 as u64);
        }
        carry != 0
    }

    fn sub_noborrow(&mut self, other: &Self) -> bool {
        let mut borrow = 0;
        for i in 0..Self::NUM_LIMBS {
            let (diff, b) = self.0[i].overflowing_sub(other.0[i]);
            let (diff, b2) = diff.overflowing_sub(borrow);
            self.0[i] = diff;
            borrow = (b as u64) + (b2 as u64);
        }
        borrow != 0
    }

    fn mul_nocarry(&mut self, other: &Self) -> bool {
        let mut carry = 0;
        for i in 0..Self::NUM_LIMBS {
            let (res, c) = self.0[i].overflowing_mul(other.0[i]);
            let (res, c2) = res.overflowing_add(carry);
            self.0[i] = res;
            carry = (c as u64) + (c2 as u64);
        }
        carry != 0
    }

    fn div_norem(&mut self, other: &Self) -> bool {
        let mut borrow = 0;
        for i in (0..Self::NUM_LIMBS).rev() {
            let (res, b) = self.0[i].overflowing_sub(other.0[i]);
            let (res, b2) = res.overflowing_sub(borrow);
            self.0[i] = res;
            borrow = (b as u64) + (b2 as u64);
        }
        borrow != 0
    }

    fn mul2(&mut self) {
        let mut carry = 0;
        for i in 0..Self::NUM_LIMBS {
            let (res, c) = self.0[i].overflowing_mul(2);
            let (res, c2) = res.overflowing_add(carry);
            self.0[i] = res;
            carry = (c as u64) + (c2 as u64);
        }
    }

    fn muln(&mut self, amt: u32) {
        for _ in 0..amt {
            self.mul2();
        }
    }

    fn div2(&mut self) {
        let mut carry = 0;
        for i in (0..Self::NUM_LIMBS).rev() {
            let new_carry = self.0[i] & 1;
            self.0[i] >>= 1;
            if carry != 0 {
                self.0[i] |= 1 << 63;
            }
            carry = new_carry;
        }
    }

    fn divn(&mut self, amt: u32) {
        for _ in 0..amt {
            self.div2();
        }
    }

    fn is_odd(&self) -> bool {
        self.0[0] & 1 == 1
    }

    fn is_even(&self) -> bool {
        !self.is_odd()
    }

    fn is_zero(&self) -> bool {
        self.0.iter().all(|&x| x == 0)
    }

    fn num_bits(&self) -> u32 {
        for i in (0..Self::NUM_LIMBS).rev() {
            if self.0[i] != 0 {
                return (i as u32) * 64 + (64 - self.0[i].leading_zeros());
            }
        }
        0
    }

    fn get_bit(&self, i: usize) -> bool {
        if i >= Self::NUM_LIMBS * 64 {
            return false;
        }
        let limb = i / 64;
        let bit = i % 64;
        (self.0[limb] & (1 << bit)) != 0
    }

    fn to_largeuint(&self) -> LargeUint {
        // Placeholder implementation
        LargeUint { data: LargeDigits::Inline(Some(LargeDigit::from(self.0[0]))) }
    }

    fn find_wnaf(&self) -> Vec<i64> {
        // Placeholder implementation
        vec![]
    }
}

impl Debug for LargeInteger256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in self.0.iter().rev() {
            write!(f, "{:016X}", *i)?;
        }
        Ok(())
    }
}

impl Display for LargeInteger256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.to_largeuint())
    }
}

impl Ord for LargeInteger256 {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
            match a.cmp(b) {
                Ordering::Less => return Ordering::Less,
                Ordering::Greater => return Ordering::Greater,
                _ => continue,
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for LargeInteger256 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Distribution<LargeInteger256> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LargeInteger256 {
        LargeInteger256(rng.random().as_ref().to_vec().try_into().unwrap_or([0u64; 4]))
    }
}

impl AsMut<[u64]> for LargeInteger256 {
    #[inline]
    fn as_mut(&mut self) -> &mut [u64] {
        &mut self.0
    }
}

impl AsRef<[u64]> for LargeInteger256 {
    #[inline]
    fn as_ref(&self) -> &[u64] {
        &self.0
    }
}

impl From<u64> for LargeInteger256 {
    #[inline]
    fn from(val: u64) -> LargeInteger256 {
        let mut repr = Self::default();
        repr.0[0] = val;
        repr
    }
}