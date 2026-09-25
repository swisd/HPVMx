use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub fn unrolled_find_u16s(needle: u16, haystack: &[u16]) -> Option<usize> {
    let ptr = haystack.as_ptr();
    let mut start = haystack;

    // For performance reasons unfold the loop eight times max.
    while start.len() >= 8 {
        macro_rules! if_return {
            ($($n:literal,)+) => {
                $(
                    if start[$n] == needle {
                        return Some(((&start[$n] as *const u16).addr() - ptr.addr()) / 2);
                    }
                )+
            }
        }

        if_return!(0, 1, 2, 3, 4, 5, 6, 7,);

        start = &start[8..];
    }

    for c in start {
        if *c == needle {
            return Some(((c as *const u16).addr() - ptr.addr()) / 2);
        }
    }
    None
}

pub fn encode_wide(s: String) -> Vec<u16> {
    todo!()
}

pub fn to_u16s(s: String) -> Result<Vec<u16>, String> {
    fn inner(s: String) -> Result<Vec<u16>, String> {
        let mut maybe_result = Vec::with_capacity(s.len() + 1);
        maybe_result.extend(encode_wide(s));

        if unrolled_find_u16s(0, &maybe_result).is_some() {
            return Err("strings passed cannot contain NULs".parse().unwrap());
        }
        maybe_result.push(0);
        Ok(maybe_result)
    }
    inner(s)
    // Err("unavailable".to_owned())
}

pub fn truncate_utf16_at_nul(v: &[u16]) -> &[u16] {
    match unrolled_find_u16s(0, v) {
        // don't include the 0
        Some(i) => &v[..i],
        None => v,
    }
}

pub fn ensure_no_nuls(s: String) -> Result<String, String> {
    if encode_wide(s.clone()).iter().any(|b| *b == 0) {
        Err("nul byte found in provided data".parse().unwrap())
    } else {
        Ok(s)
    }
}

// Basic type aliases
pub type _BYTE = i8;
pub type _WORD = i16;
pub type _DWORD = i32;
pub type _QWORD = i64;

pub type BOOL = bool;
pub type _BOOL8 = i64;

// _DWORD_DWORD structure
#[repr(C)]
pub struct _DWORD_DWORD {
    DWord1: _DWORD,
    DWord2: _DWORD,
}

pub type DWORD_DWORD = _DWORD_DWORD;
pub type LPDWORD_DWORD = *mut _DWORD_DWORD;
pub type DWORD2 = DWORD_DWORD;

// __m128i union
#[repr(C, align(16))]
pub union __m128i {
    m128i_i8: [i8; 16],
    m128i_i16: [i16; 8],
    m128i_i32: [i32; 4],
    m128i_i64: [i64; 2],
    m128i_u8: [u8; 16],
    m128i_u16: [u16; 8],
    m128i_u32: [u32; 4],
    m128i_u64: [u64; 2],
    m1281_i128: [i128; 1],
    m1281_u128: [u128; 1],
}

impl Copy for __m128i {}
impl Clone for __m128i {
    fn clone(&self) -> Self { *self }
}

pub type _OWORD = __m128i;

// Pair128 structure
#[repr(C)]
pub struct Pair128<T> {
    low: T,
    high: T,
}

pub fn __PAIR128__<T>(high: T, low: T) -> Pair128<i64>
where
    T: Into<i64>,
{
    Pair128 {
        low: low.into(),
        high: high.into(),
    }
}

pub fn from_pair128(p: Pair128<i64>) -> i128 {
    let mut result: i128 = 0;
    result += p.low as i128;
    result += p.high as i128;
    result
}

pub fn __PAIR64__(high: u32, low: u32) -> u64 {
    ((high as u64) << 32) | (low as u64)
}

// memset implementation
pub unsafe fn memset(s: *mut core::ffi::c_void, c: i32, n: u64) -> *mut core::ffi::c_void {
    let mut p = s as *mut u8;
    let mut count = n;
    while count > 0 {
        *p = c as u8;
        p = p.add(1);
        count -= 1;
    }
    s
}

// SIMD intrinsics
pub unsafe fn _mm_shuffle_epi32(a: __m128i, imm8: i32) -> __m128i {
    let mut result = __m128i { m128i_i32: [0; 4] };
    let idx0 = ((imm8 >> 0) & 0x03) as usize;
    let idx1 = ((imm8 >> 2) & 0x03) as usize;
    let idx2 = ((imm8 >> 4) & 0x03) as usize;
    let idx3 = ((imm8 >> 6) & 0x03) as usize;

    result.m128i_i32[0] = a.m128i_i32[idx0];
    result.m128i_i32[1] = a.m128i_i32[idx1];
    result.m128i_i32[2] = a.m128i_i32[idx2];
    result.m128i_i32[3] = a.m128i_i32[idx3];

    result
}

pub unsafe fn _mm_cvtsi32_si128(a: i32) -> __m128i {
    let mut result = __m128i { m128i_i32: [0; 4] };
    result.m128i_i32[0] = a;
    result
}

pub unsafe fn _mm_or_si128(a: __m128i, b: __m128i) -> __m128i {
    let mut result = __m128i { m128i_i64: [0; 2] };
    result.m128i_i64[0] = a.m128i_i64[0] | b.m128i_i64[0];
    result.m128i_i64[1] = a.m128i_i64[1] | b.m128i_i64[1];
    result
}

pub unsafe fn _mm_and_si128(a: __m128i, b: __m128i) -> __m128i {
    let mut result = __m128i { m128i_u64: [0; 2] };
    result.m128i_u64[0] = a.m128i_u64[0] & b.m128i_u64[0];
    result.m128i_u64[1] = a.m128i_u64[1] & b.m128i_u64[1];
    result
}

pub unsafe fn _mm_xor_si128(a: __m128i, b: __m128i) -> __m128i {
    let mut result = __m128i { m128i_i64: [0; 2] };
    result.m128i_i64[0] = a.m128i_i64[0] ^ b.m128i_i64[0];
    result.m128i_i64[1] = a.m128i_i64[1] ^ b.m128i_i64[1];
    result
}

pub unsafe fn _mm_load_si128(mem_addr: *const __m128i) -> __m128i {
    *mem_addr
}

pub unsafe fn _mm_add_epi32(a: __m128i, b: __m128i) -> __m128i {
    let mut result = __m128i { m128i_u32: [0; 4] };
    for i in 0..4 {
        result.m128i_u32[i] = a.m128i_u32[i].wrapping_add(b.m128i_u32[i]);
    }
    result
}

pub unsafe fn _mm_shuffle_epi8(a: __m128i, b: __m128i) -> __m128i {
    let mut result = __m128i { m128i_u8: [0; 16] };
    for i in 0..16 {
        if (b.m128i_u8[i] & 0x80) != 0 {
            result.m128i_u8[i] = 0;
        } else {
            let index = (b.m128i_u8[i] & 0x0F) as usize;
            result.m128i_u8[i] = a.m128i_u8[index];
        }
    }
    result
}

pub unsafe fn _mm_loadu_si128(p: *const __m128i) -> __m128i {
    *p
}

pub unsafe fn _mm_unpacklo_epi32(a: __m128i, b: __m128i) -> __m128i {
    let mut result = __m128i { m128i_u32: [0; 4] };
    result.m128i_u32[0] = a.m128i_u32[0];
    result.m128i_u32[1] = b.m128i_u32[0];
    result.m128i_u32[2] = a.m128i_u32[1];
    result.m128i_u32[3] = b.m128i_u32[1];
    result
}

pub unsafe fn _mm_unpackhi_epi32(a: __m128i, b: __m128i) -> __m128i {
    let mut result = __m128i { m128i_u32: [0; 4] };
    result.m128i_u32[0] = a.m128i_u32[2];
    result.m128i_u32[1] = b.m128i_u32[2];
    result.m128i_u32[2] = a.m128i_u32[3];
    result.m128i_u32[3] = b.m128i_u32[3];
    result
}

pub unsafe fn _mm_unpacklo_epi64(a: __m128i, b: __m128i) -> __m128i {
    let mut result = __m128i { m128i_u64: [0; 2] };
    result.m128i_u64[0] = a.m128i_u64[0];
    result.m128i_u64[1] = b.m128i_u64[0];
    result
}

pub unsafe fn _mm_unpackhi_epi64(a: __m128i, b: __m128i) -> __m128i {
    let mut result = __m128i { m128i_u64: [0; 2] };
    result.m128i_u64[0] = a.m128i_u64[1];
    result.m128i_u64[1] = b.m128i_u64[1];
    result
}

pub unsafe fn _mm_srli_epi32(a: __m128i, imm8: i32) -> __m128i {
    let mut result = __m128i { m128i_u32: [0; 4] };
    if imm8 > 31 {
        result.m128i_u64[0] = 0;
        result.m128i_u64[1] = 0;
    } else {
        result.m128i_u32[0] = a.m128i_u32[0] >> imm8;
        result.m128i_u32[1] = a.m128i_u32[1] >> imm8;
        result.m128i_u32[2] = a.m128i_u32[2] >> imm8;
        result.m128i_u32[3] = a.m128i_u32[3] >> imm8;
    }
    result
}

pub unsafe fn _mm_slli_epi32(a: __m128i, imm8: i32) -> __m128i {
    let mut result = __m128i { m128i_u32: [0; 4] };
    if imm8 > 31 {
        result.m128i_u64[0] = 0;
        result.m128i_u64[1] = 0;
    } else {
        result.m128i_u32[0] = a.m128i_u32[0] << imm8;
        result.m128i_u32[1] = a.m128i_u32[1] << imm8;
        result.m128i_u32[2] = a.m128i_u32[2] << imm8;
        result.m128i_u32[3] = a.m128i_u32[3] << imm8;
    }
    result
}

pub unsafe fn _mm_srli_epi64(a: __m128i, imm8: i32) -> __m128i {
    let mut result = __m128i { m128i_u64: [0; 2] };
    if imm8 > 63 {
        result.m128i_u64[0] = 0;
        result.m128i_u64[1] = 0;
    } else {
        result.m128i_u64[0] = a.m128i_u64[0] >> imm8;
        result.m128i_u64[1] = a.m128i_u64[1] >> imm8;
    }
    result
}

pub unsafe fn _mm_slli_epi64(a: __m128i, imm8: i32) -> __m128i {
    let mut result = __m128i { m128i_u64: [0; 2] };
    if imm8 > 63 {
        result.m128i_u64[0] = 0;
        result.m128i_u64[1] = 0;
    } else {
        result.m128i_u64[0] = a.m128i_u64[0] << imm8;
        result.m128i_u64[1] = a.m128i_u64[1] << imm8;
    }
    result
}

pub unsafe fn _mm_alignr_epi8(a: __m128i, b: __m128i, imm8: i32) -> __m128i {
    let mut res = __m128i { m128i_u8: [0; 16] };
    let mut tmp = [0u8; 32];

    for i in 0..16 {
        tmp[i] = b.m128i_u8[i];
        tmp[i + 16] = a.m128i_u8[i];
    }

    for i in 0..16 {
        let shift_idx = imm8 + i as i32;
        res.m128i_u8[i] = if shift_idx < 32 { tmp[shift_idx as usize] } else { 0 };
    }

    res
}

pub const CHAR_BIT: usize = 8;

#[inline]
pub fn __ROL4__<T>(value: T, count: i32) -> T
where
    T: Copy + core::ops::Shl<u32, Output=T> + core::ops::Shr<u32, Output=T> + core::ops::BitOr<Output=T>,
{
    let mask = (CHAR_BIT * size_of::<T>() - 1) as i32;
    let count = (count & mask) as u32;
    let neg_count = ((-(count as i32)) & mask) as u32;
    (value << count) | (value >> neg_count)
}

#[inline]
pub fn __ROR4__<T>(value: T, count: i32) -> T
where
    T: Copy + core::ops::Shl<u32, Output=T> + core::ops::Shr<u32, Output=T> + core::ops::BitOr<Output=T>,
{
    let mask = (CHAR_BIT * size_of::<T>() - 1) as i32;
    let count = (count & mask) as u32;
    let neg_count = ((-(count as i32)) & mask) as u32;
    (value >> count) | (value << neg_count)
}

#[inline]
pub fn __ROL8__<T>(value: T, count: i32) -> T
where
    T: Copy + core::ops::Shl<u32, Output=T> + core::ops::Shr<u32, Output=T> + core::ops::BitOr<Output=T>,
{
    let mask = (CHAR_BIT * size_of::<T>() - 1) as i32;
    let count = (count & mask) as u32;
    let neg_count = ((-(count as i32)) & mask) as u32;
    (value << count) | (value >> neg_count)
}

#[inline]
pub fn __ROR8__<T>(value: T, count: i32) -> T
where
    T: Copy + core::ops::Shl<u32, Output=T> + core::ops::Shr<u32, Output=T> + core::ops::BitOr<Output=T>,
{
    let mask = (CHAR_BIT * size_of::<T>() - 1) as i32;
    let count = (count & mask) as u32;
    let neg_count = ((-(count as i32)) & mask) as u32;
    (value >> count) | (value << neg_count)
}

#[inline]
pub fn __ROL2__<T>(value: T, count: i32) -> T
where
    T: Copy + core::ops::Shl<u32, Output=T> + core::ops::Shr<u32, Output=T> + core::ops::BitOr<Output=T>,
{
    let mask = (CHAR_BIT * size_of::<T>() - 1) as i32;
    let count = (count & mask) as u32;
    let neg_count = ((-(count as i32)) & mask) as u32;
    (value << count) | (value >> neg_count)
}

#[inline]
pub fn __ROR2__<T>(value: T, count: i32) -> T
where
    T: Copy + core::ops::Shl<u32, Output=T> + core::ops::Shr<u32, Output=T> + core::ops::BitOr<Output=T>,
{
    let mask = (CHAR_BIT * size_of::<T>() - 1) as i32;
    let count = (count & mask) as u32;
    let neg_count = ((-(count as i32)) & mask) as u32;
    (value >> count) | (value << neg_count)
}

pub fn __ROR1__<T>(value: T, count: i32) -> T
where
    T: Copy + core::ops::Shl<u32, Output=T> + core::ops::Shr<u32, Output=T> + core::ops::BitOr<Output=T>,
{
    let mask = (CHAR_BIT * size_of::<T>() - 1) as i32;
    let count = (count & mask) as u32;
    let neg_count = ((-(count as i32)) & mask) as u32;
    (value >> count) | (value << neg_count)
}

pub fn __ROL1__<T>(value: T, count: i32) -> T
where
    T: Copy + core::ops::Shl<u32, Output=T> + core::ops::Shr<u32, Output=T> + core::ops::BitOr<Output=T>,
{
    let mask = (CHAR_BIT * size_of::<T>() - 1) as i32;
    let count = (count & mask) as u32;
    let neg_count = ((-(count as i32)) & mask) as u32;
    (value << count) | (value >> neg_count)
}

pub fn __CFADD__<T, U>(x: T, y: U) -> i8
where
    T: Copy + core::ops::Add<U, Output=T> + PartialOrd,
    U: Copy,
{
    if x > x + y { 1 } else { 0 }
}



// Macros for byte extraction
macro_rules! BYTE0 { ($val:expr) => { (($val as i64 >> 0) & 0xFF) as i8 }; }
macro_rules! BYTE1 { ($val:expr) => { (($val as i64 >> 8) & 0xFF) as i8 }; }
macro_rules! BYTE2 { ($val:expr) => { (($val as i64 >> 16) & 0xFF) as i8 }; }
macro_rules! BYTE3 { ($val:expr) => { (($val as i64 >> 24) & 0xFF) as i8 }; }
macro_rules! BYTE4 { ($val:expr) => { (($val as i64 >> 32) & 0xFF) as i8 }; }
macro_rules! BYTE5 { ($val:expr) => { (($val as i64 >> 40) & 0xFF) as i8 }; }
macro_rules! BYTE6 { ($val:expr) => { (($val as i64 >> 48) & 0xFF) as i8 }; }
macro_rules! BYTE7 { ($val:expr) => { (($val as i64 >> 56) & 0xFF) as i8 }; }

pub(crate) use BYTE0;
pub(crate) use BYTE1;
pub(crate) use BYTE2;
pub(crate) use BYTE3;
pub(crate) use BYTE4;
pub(crate) use BYTE5;
pub(crate) use BYTE6;
pub(crate) use BYTE7;

macro_rules! LOBYTE { ($val:expr) => { (($val as i16) & 0xFF) as i8 }; }
macro_rules! HIBYTE { ($val:expr) => { ((($val as i16) >> 8) & 0xFF) as i8 }; }

pub(crate) use LOBYTE;
pub(crate) use HIBYTE;

macro_rules! LOWORD { ($val:expr) => { (($val as i32) & 0xFFFF) as i16 }; }
macro_rules! HIWORD { ($val:expr) => { ((($val as i32) >> 16) & 0xFFFF) as i16 }; }

pub(crate) use LOWORD;
pub(crate) use HIWORD;

macro_rules! LODWORD { ($val:expr) => { (($val as i64) & 0xFFFFFFFF) as i32 }; }
macro_rules! HIDWORD { ($val:expr) => { ((($val as i64) >> 32) & 0xFFFFFFFF) as i32 }; }

pub (crate) use LODWORD;
pub (crate) use HIDWORD;


pub fn __fastfail(Code: u32) /*-> !*/ {
    vdebug_autoprefix!("fastfail {Code}")
}


macro_rules! __offset {
    ($x:expr) => {};
}

macro_rules! __hex {
    ($($args:tt)*) => { $($args)* };
}


pub(crate) use __offset;
pub(crate) use __hex;
use crate::vdebug_autoprefix;