use alloc::vec::Vec;
use core::cmp::{max, min};

pub fn fmax(a: f32, b: f32) -> f32 {
    // if a > b {
    //     a
    // } else {
    //     b
    // }
    a.max(b)
}

pub fn fmin(a: f32, b: f32) -> f32 {
    // if a < b {
    //     a
    // } else {
    //     b
    // }
    a.min(b)
}

/// Trap a value between 0 and 1, where `x ≤ 0.0 = 0.0` and `x ≥ 1.0 = 1.0`
pub fn trap(x: f32) -> f32 {
    fmax(0.0, fmin(x, 1.0))
}

/// Variable Trap function that can be used as a fast approximation of a sigmoid:
///
/// sig(x) = trap_var(x, 0.2, 0.5)\
/// where
/// 0 ≤ 0.2x+0.5 ≤ 1
///
pub fn trap_var(x: f32, slope: f32, offset: f32) -> f32 {
    fmax(0.0, fmin((slope * x) + offset, 1.0))
}

/// Dynamic offset version of `trap_var`.\
/// Use slope=1 and offset=0 to recreate `trap` with configurable bounds
pub fn trap_dyn(x: f32, lower: f32, upper: f32, slope: f32, offset: f32) -> f32 {
    fmax(lower, fmin((slope * x) + offset, upper))
}

pub fn weight(x: f32, w: f32) -> f32 {
    x * w
}

pub fn sum_weighted_vec(inp: Vec<WeightPair>) -> f32 {
    let mut sum = 0.0;
    for (x, w) in inp {
        sum += weight(x, w)
    }
    sum
}

/// Optimized sum using borrowed slices and internal iterators for SIMD auto-vectorization
pub fn sum_weighted(inp: &[WeightPair]) -> f32 {
    inp.iter().map(|&(x, w)| weight(x, w)).sum()
}

pub fn sum_weighted_vec_bias(inp: Vec<WeightPair>, b: f32) -> f32 {
    let mut sum = 0.0;
    for (x, w) in inp {
        sum += weight(x, w)
    }
    sum + b
}

/// Optimized sum using borrowed slices and internal iterators for SIMD auto-vectorization
pub fn sum_weighted_bias(inp: &[WeightPair], b: f32) -> f32 {
    inp.iter().map(|&(x, w)| weight(x, w)).sum::<f32>() + b
}

pub fn compress_val(x: f32) -> f32 {
    trap_var(x, 0.2, 0.5)
}

pub fn compress_val_pos(x: f32) -> f32 {
    trap_var(x, 1.0, 0.0)
}

pub fn compress_val_pos_wide(x: f32) -> f32 {
    trap_dyn(x, 0.0, 2.0, 1.0, 0.0)
}

pub type WeightPair = (f32, f32);
pub type WeightPairIdentifiable = (WeightPair, usize);

#[test]
pub fn fmax_test() {
    assert_eq!(fmax(20.0, 15.0), 20.0);
    assert_eq!(fmax(15.0, 30.0), 30.0);
}

#[test]
pub fn fmin_test() {
    assert_eq!(fmin(20.0, 15.0), 15.0);
    assert_eq!(fmin(10.0, 30.0), 10.0);
}

#[test]
pub fn trap_test() {
    // limit testing
    assert_eq!(trap(20.0), 1.0);
    assert_eq!(trap(-20.0), 1.0);

    //midpoint testing
    assert_eq!(trap(0.5), 0.5);
}

#[test]
pub fn trap_var_test() {
    assert_eq!(trap_var(1.0, 0.2, 0.5), 0.7);
    assert_eq!(trap_var(-1.0, 0.2, 0.5), 0.3);
    assert_eq!(trap_var(0.5, 0.2, 0.5), 0.5);
    assert_eq!(trap_var(-2.5, 0.2, 0.5), 0.0);
    assert_eq!(trap_var(2.5, 0.2, 0.5), 1.0);
}
