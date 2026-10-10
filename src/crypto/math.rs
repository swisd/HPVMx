#[inline(always)]
pub fn adc(a: &mut u64, b: u64, carry: u64) -> u64 {
    let tmp = u128::from(*a) + u128::from(b) + u128::from(carry);
    *a = tmp as u64;
    (tmp >> 64) as u64
}

#[inline(always)]
pub fn sbb(a: &mut u64, b: u64, borrow: u64) -> u64 {
    let tmp = (1u128 << 64) + u128::from(*a) - u128::from(b) - u128::from(borrow);
    let carry = u64::from(tmp >> 64 == 0);
    *a = tmp as u64;
    carry
}

#[inline(always)]
pub fn mac_wc(a: u64, b: u64, c: u64, carry: &mut u64) -> u64 {
    let tmp = (u128::from(a)) + u128::from(b) * u128::from(c) + u128::from(*carry);

    *carry = (tmp >> 64) as u64;

    tmp as u64
}

#[inline(always)]
pub fn mac(a: u64, b: u64, c: u64, carry: &mut u64) -> u64 {
    let tmp = (u128::from(a)) + u128::from(b) * u128::from(c);

    *carry = (tmp >> 64) as u64;

    tmp as u64
}

#[inline(always)]
pub fn mac_discard(a: u64, b: u64, c: u64, carry: &mut u64) {
    let tmp = (u128::from(a)) + u128::from(b) * u128::from(c);

    *carry = (tmp >> 64) as u64;
}