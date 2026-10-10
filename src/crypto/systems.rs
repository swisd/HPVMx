use core::fmt::{Debug, Display};
use core::hash::Hash;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use bitflags::Flags;
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use crate::crypto::largeint::{LargeInteger, LargeInteger256, Uniform};
use crate::crypto::math::{adc, mac_wc};


pub type Fr = Fp256<FrParameters>;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FrParameters;

impl FieldParameters for FrParameters {
    const MODULUS: Self::LargeInteger = ();
    const MODULUS_BITS: u32 = 0;
    const REPR_DROP_BITS: u32 = 0;
    const R: Self::LargeInteger = ();
    const R2: Self::LargeInteger = ();
    const INV: u64 = 0;
    const GEN: Self::LargeInteger = ();
    const CAP: u32 = 0;
    const T: Self::LargeInteger = ();
    const TM1D2: Self::LargeInteger = ();
    const MODM1D2: Self::LargeInteger = ();
}

pub type Fq = Fp384<FqParameters>;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FqParameters;

impl Fp384Parameters for FqParameters {}

impl FftParameters for FrParameters {
    type LargeInteger = LargeInteger256;
    const TWO_ADICITY: u32 = 0;
    const TWO_ADIC_ROOT_OF_UNITY: Self::LargeInteger = ();
    const POWERS_OF_RsOfU: &'static [Self::LargeInteger] = &[];
}

impl Fp256Parameters for FrParameters {}

pub trait FftParameters: 'static + Send + Sync + Sized {
    type LargeInteger: LargeInteger;

    /// Let `N` be the size of the multiplicative group defined by the field.
    /// Then `TWO_ADICITY` is the two-adicity of `N`, i.e. the integer `s`
    /// such that `N = 2^s * t` for some odd integer `t`.
    /// 2^s * t = MODULUS - 1 with t odd. This is the two-adicity of the prime.
    const TWO_ADICITY: u32;
    /// 2^s root of unity, defined as `GENERATOR^t`.
    const TWO_ADIC_ROOT_OF_UNITY: Self::LargeInteger;
    /// An integer `b` such that there exists a multiplicative subgroup
    /// of size `b^k` for some integer `k`.
    const SMALL_SUBGROUP_BASE: Option<u32> = None;
    /// The integer `k` such that there exists a multiplicative subgroup
    /// of size `Self::SMALL_SUBGROUP_BASE^k`.
    const SMALL_SUBGROUP_BASE_ADICITY: Option<u32> = None;
    /// GENERATOR^((MODULUS-1) / (2^s *
    /// SMALL_SUBGROUP_BASE^SMALL_SUBGROUP_BASE_ADICITY)) Used for mixed-radix FFT.
    const LARGE_SUBGROUP_ROfU: Option<Self::LargeInteger> = None;
    /// `TWO_ADIC_ROOT_OF_UNITY^2^i` for `i := 0..TWO_ADICITY-1`
    const POWERS_OF_RsOfU: &'static [Self::LargeInteger];
}

pub trait FieldParameters:
'static + FftParameters + Copy + Clone + Debug + Default + PartialEq + Eq + Hash
{
    /// The modulus of the field.
    const MODULUS: Self::LargeInteger;
    /// The number of bits needed to represent the `Self::MODULUS`.
    const MODULUS_BITS: u32;
    const REPR_DROP_BITS: u32;
    /// R = 2^256 % Self::MODULUS
    const R: Self::LargeInteger;
    /// R2 = R^2 % Self::MODULUS
    const R2: Self::LargeInteger;
    /// INV = -(MODULUS^{-1} mod MODULUS) mod MODULUS
    const INV: u64;
    const GEN: Self::LargeInteger;
    const CAP: u32;
    /// t for 2^s * t = MODULUS - 1
    const T: Self::LargeInteger;
    /// (t - 1) / 2
    const TM1D2: Self::LargeInteger;
    /// (Self::MODULUS - 1) / 2
    const MODM1D2: Self::LargeInteger;
}

pub trait Fp256Parameters: FieldParameters<LargeInteger = LargeInteger256> {}

pub struct Fp256<P: Fp256Parameters>(pub LargeInteger256, #[doc(hidden)] pub PhantomData<P>);

impl<P: Fp256Parameters> Fp256<P> {
    #[inline]
    fn is_valid(&self) -> bool {
        self.0 < P::MODULUS
    }

    #[inline]
    fn reduce(&mut self) {
        if !self.is_valid() {
            self.0.sub_noborrow(&P::MODULUS);
        }
    }
}

impl<P: Fp256Parameters> Add<Self> for Fp256<P> {
    type Output = ();

    fn add(mut self, rhs: Self) -> Self::Output {
        self.0.add_nocarry(&rhs.0);
        self.reduce();
    }
}

impl<P: Fp256Parameters> Zero for Fp256<P> {
    #[inline]
    fn zero() -> Self {
        Self(LargeInteger::from(0), PhantomData)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<P: Fp256Parameters> Mul<Self> for Fp256<P> {
    type Output = ();

    fn mul(mut self, rhs: Self) -> Self::Output {
        self.0.mul_nocarry(&rhs.0);
        self.reduce();
    }
}

impl<P: Fp256Parameters> One for Fp256<P> {
    #[inline]
    fn one() -> Self {
        Self(P::R, PhantomData)
    }

    #[inline]
    fn is_one(&self) -> bool {
        self.0 == P::R
    }
}

pub trait Fp384Parameters: FieldParameters<LargeInteger =dyn LargeInteger> {}

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct Fp384<P: Fp384Parameters>(pub dyn LargeInteger, #[doc(hidden)] pub PhantomData<P>);

impl<P: Fp384Parameters> Fp384<P> {
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.0 < P::MODULUS
    }

    #[inline]
    fn reduce(&mut self) {
        if !self.is_valid() {
            self.0.sub_noborrow(&P::MODULUS);
        }
    }
}

impl<P: Fp384Parameters> Add<Self> for Fp384<P> {
    type Output = ();

    fn add(mut self, rhs: Self) -> Self::Output {
        self.0.add_nocarry(rhs.0);
        self.reduce();
    }
}

impl<P: Fp384Parameters> Zero for Fp384<P> {
    #[inline]
    fn zero() -> Self {
        Self(LargeInteger::from(0), PhantomData)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<P: Fp384Parameters> Mul<Self> for Fp384<P> {
    type Output = ();

    fn mul(mut self, rhs: Self) -> Self::Output {
        self.0.mul_nocarry(&rhs.0);
        self.reduce();
    }
}

impl<P: Fp384Parameters> One for Fp384<P> {
    #[inline]
    fn one() -> Self {
        Self(P::R, PhantomData)
    }

    #[inline]
    fn is_one(&self) -> bool {
        self.0 == P::R
    }
}

// todo: add fp, fp2, fp6, fp12, and other field types as needed