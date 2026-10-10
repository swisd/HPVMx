use core::convert::Infallible;
use core::iter;
use core::ops::DerefMut;
use crate::Color;
use crate::{hpvm_log, hpvm_warn};

pub struct XorShiftRng {
    state: u64,
}

impl XorShiftRng {
    // Function to create a new RNG with a starting seed
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    // The core PRNG logic (Xorshift* variant)
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12; // a
        x ^= x << 25; // b
        x ^= x >> 27; // c
        self.state = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }

    pub fn rand(&mut self, len: u8) -> u64 {
        if len > 19 {
            hpvm_warn!("RNG", "length out of range for u64");
            0
        } else {
            let it = 1;
            let mut num = 0;
            while it <= len {
                num += self.next_u64() * ((10 * it) as u64);
            }
            num
        }
    }

    pub fn rand_range(&mut self, min: u64, max: u64) -> u64 {
        assert!(min < max, "min must be less than max");
        let range = max - min;
        min + (self.next_u64() % range)
    }

    pub fn rand_bool(&mut self) -> bool {
        self.next_u64() % 2 == 0
    }

    pub fn rand_ratio(&mut self, numerator: u32, denominator: u32) -> bool {
        assert!(numerator <= denominator, "numerator must be less than or equal to denominator");
        self.next_u64() % (denominator as u64) < (numerator as u64)
    }
}


// custom version of the rand crate's Rng trait, to avoid the dependency on std and alloc.
// also useful for custom functions of the XorShiftRng as well as any cryptography

pub trait Rng: BaseRng<Error = Infallible> {
    fn next_u32(&mut self) -> u32;

    fn next_u64(&mut self) -> u64;

    fn fill_bytes(&mut self, dst: &mut [u8]);
}

impl<R> Rng for R
where
    R: BaseRng<Error = Infallible> + ?Sized,
{
    #[inline]
    fn next_u32(&mut self) -> u32 {
        match self.try_next_u32() {
            Ok(x) => x,
            _ => {0}
        }
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        match self.try_next_u64() {
            Ok(x) => x,
            _ => {0}
        }
    }

    #[inline]
    fn fill_bytes(&mut self, dst: &mut [u8]) {
        match self.try_fill_bytes(dst) {
            Ok(()) => (),
        }
    }
}

pub trait BaseRng {
    type Error: core::error::Error;
    
    fn try_next_u32(&mut self) -> Result<u32, Self::Error>;

    fn try_next_u64(&mut self) -> Result<u64, Self::Error>;

    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error>;
}

impl<R: DerefMut> BaseRng for R
where
    R::Target: BaseRng,
{
    type Error = <R::Target as BaseRng>::Error;

    #[inline]
    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        self.deref_mut().try_next_u32()
    }

    #[inline]
    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        self.deref_mut().try_next_u64()
    }

    #[inline]
    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        self.deref_mut().try_fill_bytes(dst)
    }
}

pub trait RngExt: Rng {

    #[inline]
    fn random<T>(&mut self) -> T
    where
        StandardUniform: Distribution<T>,
    {
        StandardUniform.sample(self)
    }

    #[inline]
    fn random_iter<T>(self) -> Iter<StandardUniform, Self, T>
    where
        Self: Sized,
        StandardUniform: Distribution<T>,
    {
        StandardUniform.sample_iter(self)
    }

    /*
    #[track_caller]
    fn random_range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>,
    {
        assert!(!range.is_empty(), "cannot sample empty range");
        range.sample_single(self).unwrap()
    }

    #[inline]
    #[track_caller]
    fn random_bool(&mut self, p: f64) -> bool {
        match Bernoulli::new(p) {
            Ok(d) => self.sample(d),
            Err(_) => panic!("p={:?} is outside range [0.0, 1.0]", p),
        }
    }

    #[inline]
    #[track_caller]
    fn random_ratio(&mut self, numerator: u32, denominator: u32) -> bool {
        match Bernoulli::from_ratio(numerator, denominator) {
            Ok(d) => self.sample(d),
            Err(_) => panic!(
                "p={}/{} is outside range [0.0, 1.0]",
                numerator, denominator
            ),
        }
    }
    */

    fn sample<T, D: Distribution<T>>(&mut self, distr: D) -> T {
        distr.sample(self)
    }

    fn sample_iter<T, D>(self, distr: D) -> Iter<D, Self, T>
    where
        D: Distribution<T>,
        Self: Sized,
    {
        distr.sample_iter(self)
    }

    #[track_caller]
    fn fill<T: Fill>(&mut self, dest: &mut [T]) {
        Fill::fill_slice(dest, self)
    }
}

impl<R: Rng + ?Sized> RngExt for R {}

pub trait Distribution<T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T;

    fn sample_iter<R>(self, rng: R) -> Iter<Self, R, T>
    where
        R: Rng,
        Self: Sized,
    {
        Iter {
            distr: self,
            rng,
            phantom: core::marker::PhantomData,
        }
    }

    fn map<F, S>(self, func: F) -> Map<Self, F, T, S>
    where
        F: Fn(T) -> S,
        Self: Sized,
    {
        Map {
            distr: self,
            func,
            phantom: core::marker::PhantomData,
        }
    }
}

impl<T, D: Distribution<T> + ?Sized> Distribution<T> for &D {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T {
        (*self).sample(rng)
    }
}


#[derive(Debug)]
pub struct Iter<D, R, T> {
    distr: D,
    rng: R,
    phantom: core::marker::PhantomData<T>,
}

impl<D, R, T> Iterator for Iter<D, R, T>
where
    D: Distribution<T>,
    R: Rng,
{
    type Item = T;

    #[inline(always)]
    fn next(&mut self) -> Option<T> {
        // Here, self.rng may be a reference, but we must take &mut anyway.
        // Even if sample could take an R: Rng by value, we would need to do this
        // since Rng is not copyable and we cannot enforce that this is "reborrowable".
        Some(self.distr.sample(&mut self.rng))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }
}

impl<D, R, T> iter::FusedIterator for Iter<D, R, T>
where
    D: Distribution<T>,
    R: Rng,
{
}


#[derive(Debug)]
pub struct Map<D, F, T, S> {
    distr: D,
    func: F,
    phantom: core::marker::PhantomData<fn(T) -> S>,
}

impl<D, F, T, S> Distribution<S> for Map<D, F, T, S>
where
    D: Distribution<T>,
    F: Fn(T) -> S,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> S {
        (self.func)(self.distr.sample(rng))
    }
}

pub struct StandardUniform;

pub trait Fill: Sized {

    fn fill_slice<R: Rng + ?Sized>(this: &mut [Self], rng: &mut R);
}

impl Fill for u8 {
    fn fill_slice<R: Rng + ?Sized>(this: &mut [Self], rng: &mut R) {
        rng.fill_bytes(this)
    }
}

pub struct Bernoulli {
    p_int: u64,
}