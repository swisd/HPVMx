use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use ordered_float::OrderedFloat;

const PI: f64 = core::f64::consts::PI;
const TAU: f64 = core::f64::consts::TAU;
const FRAC_PI_2: f64 = core::f64::consts::FRAC_PI_2;
const LOG2_E: f64 = core::f64::consts::LOG2_E;
const LN2_HIGH: f64 = 0.6931471803691238;
const LN2_LOW: f64 = 1.9082149292705877e-10;


pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

pub fn sub(a: i64, b: i64) -> i64 {
    a - b
}

pub fn mul(a: i64, b: i64) -> i64 {
    a * b
}

pub fn div(a: i64, b: i64) -> i64 {
    a / b
}

pub fn pow(a: i64, b: i64) -> i64 {
    a.pow(b as u32)
}

pub fn fpowi(mut a: f64, b: i64) -> f64 {
    // .powi() not available
    if b == 0 {
        return 1.0;
    }

    let mut uexp = b.unsigned_abs();
    let mut acc = 1.0;

    while uexp > 1 {
        if uexp % 2 == 1 {
            acc *= a;
        }
        a *= a;
        uexp /= 2;
    }
    acc *= a;

    if b < 0 {
        1.0 / acc
    } else {
        acc
    }

}

pub fn modulo(a: i64, b: i64) -> i64 {
    a % b
}

pub fn sqrt(x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    let mut guess = x;
    for _ in 0..10 {
        guess = 0.5 * (guess + x / guess);
    }
    guess
}

pub fn abs(a: i64) -> i64 {
    a.abs()
}

pub fn sin(x: f64) -> f64 {
    let x = normalize_angle(x);
    let mut term = x;
    let mut sum = x;
    let x_sq = x * x;

    // 5 terms of Taylor series provides good precision
    for i in 1..=5 {
        let n = (2 * i * (2 * i + 1)) as f64;
        term = -term * x_sq / n;
        sum += term;
    }
    sum
}

pub fn cos(x: f64) -> f64 {
    let x = normalize_angle(x);
    let mut term = 1.0;
    let mut sum = 1.0;
    let x_sq = x * x;

    for i in 1..=5 {
        let n = ((2 * i - 1) * (2 * i)) as f64;
        term = -term * x_sq / n;
        sum += term;
    }
    sum
}

pub fn tan(x: f64) -> f64 {
    let c = cos(x);
    if c.abs() < 1e-6 {
        0.0 // Handle division by zero near odd multiples of PI/2
    } else {
        sin(x) / c
    }
}

pub fn ceil(x: f64) -> f64 {
    let i = x as i64 as f64;
    if i < x {
        i + 1.0
    } else {
        i
    }
}

pub fn floor(x: f64) -> f64 {
    let i = x as i64 as f64;
    if i > x {
        i - 1.0
    } else {
        i
    }
}

pub fn round(x: f64) -> f64 {
    if x.is_nan() {
        return x;
    }
    if x >= 0.0 {
        floor(x + 0.5)
    } else {
        ceil(x - 0.5)
    }
}

pub fn asin(x: f64) -> f64 {
    let clamped_x = if x < -1.0 { -1.0 } else if x > 1.0 { 1.0 } else { x };
    let abs_x = clamped_x.abs();

    // Polynomial coefficients for sqrt(1 - |x|) series approximation
    let p = 1.5707288
        - 0.2121144 * abs_x
        + 0.0742610 * abs_x * abs_x
        - 0.0187293 * abs_x * abs_x * abs_x;

    let result = FRAC_PI_2 - sqrt(1.0 - abs_x) * p;

    if clamped_x < 0.0 {
        -result
    } else {
        result
    }
}

pub fn acos(x: f64) -> f64 {
    FRAC_PI_2 - asin(x)
}

pub fn atan(x: f64) -> f64 {
    // If |x| > 1, use identity: atan(x) = sign(x)*(PI/2) - atan(1/x)
    if x > 1.0 {
        return (PI / 2.0) - atan(1.0 / x);
    } else if x < -1.0 {
        return (-PI / 2.0) - atan(1.0 / x);
    }

    let x_sq = x * x;
    let mut term = x;
    let mut sum = x;
    let mut sign = -1.0;

    for i in 1..=8 {
        let denom = (2 * i + 1) as f64;
        term *= x_sq;
        sum += sign * (term / denom);
        sign = -sign;
    }
    sum
}

pub fn atan2(y: f64, x: f64) -> f64 {
    if x > 0.0 {
        atan(y / x)
    } else if x < 0.0 && y >= 0.0 {
        atan(y / x) + PI
    } else if x < 0.0 && y < 0.0 {
        atan(y / x) - PI
    } else if x == 0.0 && y > 0.0 {
        PI / 2.0
    } else if x == 0.0 && y < 0.0 {
        -PI / 2.0
    } else {
        0.0 // undefined, return 0
    }
}

pub fn sinh(x: f64) -> f64 {
    (exp(x) - exp(-x)) / 2.0
}

pub fn cosh(x: f64) -> f64 {
    (exp(x) + exp(-x)) / 2.0
}

pub fn tanh(x: f64) -> f64 {
    let e_pos = exp(x);
    let e_neg = exp(-x);
    (e_pos - e_neg) / (e_pos + e_neg)
}

pub fn asinh(x: f64) -> f64 {
    ln(x + sqrt(x * x + 1.0))
}

pub fn acosh(x: f64) -> f64 {
    if x < 1.0 {
        return f64::NAN; // acosh is undefined for x < 1
    }
    ln(x + sqrt(x * x - 1.0))
}

pub fn atanh(x: f64) -> f64 {
    if x <= -1.0 || x >= 1.0 {
        return f64::NAN; // atanh is undefined for |x| >= 1
    }
    0.5 * ln((1.0 + x) / (1.0 - x))
}

pub fn hypot(x: f64, y: f64) -> f64 {
    sqrt(x * x + y * y)
}

pub fn exp(x: f64) -> f64 {
    // Handle exceptional cases
    if x.is_nan() { return f64::NAN; }
    if x > 709.782712893384f64 { return f64::INFINITY; } // Overflow limit for f64
    if x < -745.133219101941f64 { return 0.0; }          // Underflow limit for f64

    // e^x = 2^(x * log2(e))


    // Find the nearest integer to x * log2(e)
    let k = (x * LOG2_E + if x < 0.0 { -0.5 } else { 0.5 }) as i32;
    let k_f64 = k as f64;

    // Reduce the argument: x = k * ln(2) + r
    let r = x - k_f64 * LN2_HIGH - k_f64 * LN2_LOW;

    // Evaluate the Taylor/Padé approximation for e^r where |r| <= ln(2)/2
    // Polynomial coefficients for f64 accuracy
    let r2 = r * r;
    let c0 = 1.0 + r * 0.5 + r2 * (1.0 / 12.0);
    let c1 = r - r2 * (1.0 / 6.0);

    // Pade approximation: e^r approx (1 + r/2 + ...) / (1 - r/2 + ...)
    // For simplicity and speed, a high-degree Taylor polynomial can also be used:
    let exp_r = 1.0 + r +
        r2 * (0.5 +
            r * (1.0 / 6.0 +
                r * (1.0 / 24.0 +
                    r * (1.0 / 120.0 +
                        r * (1.0 / 720.0 +
                            r * (1.0 / 5040.0 +
                                r * (1.0 / 40320.0)))))));

    // Scale by 2^k using manual bit manipulation to avoid using std::f64::powi
    let bits = ((k + 1023) as u64) << 52;
    let scale = f64::from_bits(bits);

    exp_r * scale
}

pub fn frexp(x: f64) -> (f64, i32) {
    if x == 0.0 {
        return (0.0, 0);
    }
    let bits = x.to_bits();
    let exponent = ((bits >> 52) & 0x7FF) as i32 - 1022; // Adjust for bias
    let mantissa_bits = (bits & 0xFFFFFFFFFFFFF) | 0x10000000000000; // Restore the implicit leading 1
    let mantissa = f64::from_bits(mantissa_bits << 11); // Shift to get the mantissa in [1.0, 2.0)
    (mantissa / 2.0, exponent) // Return mantissa in [0.5, 1.0)
}

pub fn ln(x: f64) -> f64 {
    if x <= 0.0 {
        return f64::NAN; // ln is undefined for non-positive values
    }

    // Decompose x into mantissa and exponent: x = m * 2^e
    let (m, e) = frexp(x);

    // Use a polynomial approximation for ln(m) where m is in [0.5, 1)
    let y = m - 1.0;
    let y2 = y * y;

    // Coefficients for a polynomial approximation of ln(1 + y)
    let c1 = 1.0;
    let c2 = -0.5;
    let c3 = 1.0 / 3.0;
    let c4 = -1.0 / 4.0;
    let c5 = 1.0 / 5.0;

    let ln_m = y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * c5))));

    const LN_2: f64 = core::f64::consts::LN_2; // ln(2)

    // Combine with the exponent to get the final result
    ln_m + (e as f64) * LN_2
}

pub fn log10(x: f64) -> f64 {
    if x <= 0.0 {
        return f64::NAN; // log10 is undefined for non-positive values
    }
    ln(x) / core::f64::consts::LN_10
}

pub fn log2(x: f64) -> f64 {
    if x <= 0.0 {
        return f64::NAN; // log2 is undefined for non-positive values
    }
    ln(x) / core::f64::consts::LN_2
}

pub fn log(b: f64, x: f64) -> f64 {
    ln(x) / ln(b)
}

pub fn normalize_angle(mut rad: f64) -> f64 {
    while rad > PI {
        rad -= TAU;
    }
    while rad < -PI {
        rad += TAU;
    }
    rad
}

pub fn factorial(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return 1;
    }
    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }
    result
}

pub fn combinations(n: u64, r: u64) -> u64 {
    if r > n {
        return 0;
    }
    factorial(n) / (factorial(r) * factorial(n - r))
}

pub fn permutations(n: u64, r: u64) -> u64 {
    if r > n {
        return 0;
    }
    factorial(n) / factorial(n - r)
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a / gcd(a, b)) * b
}

pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

pub fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 2;
    }
    if n > 2 {
        factors.push(n);
    }
    factors
}

pub fn euler_totient(mut n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut result = n;
    let mut p = 2;
    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 {
                n /= p;
            }
            result -= result / p;
        }
        p += 1;
    }
    if n > 1 {
        result -= result / n;
    }
    result
}

pub fn collatz_sequence(mut n: u64) -> Vec<u64> {
    let mut sequence = Vec::new();
    while n != 1 {
        sequence.push(n);
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
    }
    sequence.push(1); // Add the last element
    sequence
}

pub fn collatz_steps(mut n: u64) -> u64 {
    let mut steps = 0;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        steps += 1;
    }
    steps
}

pub fn collatz_max(mut n: u64) -> u64 {
    let mut max_value = n;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        if n > max_value {
            max_value = n;
        }
    }
    max_value
}

pub fn collatz_sequence_with_steps(mut n: u64) -> Vec<(u64, u64)> {
    let mut sequence = Vec::new();
    let mut steps = 0;
    while n != 1 {
        sequence.push((n, steps));
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        steps += 1;
    }
    sequence.push((1, steps)); // Add the last element
    sequence
}

pub fn stdev(data: &[f64]) -> f64 {
    let n = data.len() as f64;
    if n == 0.0 {
        return 0.0;
    }
    let mean = data.iter().sum::<f64>() / n;
    let variance = data.iter().map(|&x| fpowi((x - mean), 2)).sum::<f64>() / n;
    sqrt(variance)
}

pub fn mean(data: &[f64]) -> f64 {
    let n = data.len() as f64;
    if n == 0.0 {
        return 0.0;
    }
    data.iter().sum::<f64>() / n
}

pub fn median(data: &mut [f64]) -> f64 {
    let n = data.len();
    if n == 0 {
        return 0.0;
    }
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if n % 2 == 0 {
        (data[n / 2 - 1] + data[n / 2]) / 2.0
    } else {
        data[n / 2]
    }
}

pub fn mode(data: &[f64]) -> Option<OrderedFloat<f64>> {
    use hashbrown::HashMap;
    let mut frequency = HashMap::new();
    for &value in data {
        *frequency.entry(OrderedFloat(value)).or_insert(0) += 1;
    }
    frequency.into_iter().max_by_key(|&(_, count)| count).map(|(value, _)| value)
}

pub fn range(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    max - min
}

pub fn variance(data: &[f64]) -> f64 {
    let n = data.len() as f64;
    if n == 0.0 {
        return 0.0;
    }
    let mean = mean(data);
    data.iter().map(|&x| fpowi((x - mean), 2)).sum::<f64>() / n
}


pub fn sum<F>(f: F, start: i64, end: i64) -> f64
where
    F: Fn(f64) -> f64,
{
    let mut sum = 0.0;
    for i in start..=end {
        sum += f(i as f64);
    }
    sum
}

pub fn sum2<F>(f: F, start_x: i64, end_x: i64, start_y: i64, end_y: i64) -> f64
where
    F: Fn(f64, f64) -> f64,
{
    let mut sum = 0.0;
    for x in start_x..=end_x {
        for y in start_y..=end_y {
            sum += f(x as f64, y as f64);
        }
    }
    sum
}

pub fn sum3<F>(
    f: F,
    start_x: i64,
    end_x: i64,
    start_y: i64,
    end_y: i64,
    start_z: i64,
    end_z: i64,
) -> f64
where
    F: Fn(f64, f64, f64) -> f64,
{
    let mut sum = 0.0;
    for x in start_x..=end_x {
        for y in start_y..=end_y {
            for z in start_z..=end_z {
                sum += f(x as f64, y as f64, z as f64);
            }
        }
    }
    sum
}

pub fn limit<F>(f: F, a: f64, h: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    // Average the function values just to the left and right of `a`
    let left = f(a - h);
    let right = f(a + h);

    (left + right) / 2.0
}

pub fn integral<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    let h = (b - a) / n as f64;
    let mut sum = 0.0;

    for i in 0..n {
        let x_i = a + i as f64 * h;
        sum += f(x_i);
    }

    sum * h
}

pub fn integral2<F>(f: F, a_x: f64, b_x: f64, a_y: f64, b_y: f64, n_x: usize, n_y: usize) -> f64
where
    F: Fn(f64, f64) -> f64,
{
    let h_x = (b_x - a_x) / n_x as f64;
    let h_y = (b_y - a_y) / n_y as f64;
    let mut sum = 0.0;

    for i in 0..n_x {
        let x_i = a_x + i as f64 * h_x;
        for j in 0..n_y {
            let y_j = a_y + j as f64 * h_y;
            sum += f(x_i, y_j);
        }
    }

    sum * h_x * h_y
}

pub fn integral3<F>(
    f: F,
    a_x: f64,
    b_x: f64,
    a_y: f64,
    b_y: f64,
    a_z: f64,
    b_z: f64,
    n_x: usize,
    n_y: usize,
    n_z: usize,
) -> f64
where
    F: Fn(f64, f64, f64) -> f64,
{
    let h_x = (b_x - a_x) / n_x as f64;
    let h_y = (b_y - a_y) / n_y as f64;
    let h_z = (b_z - a_z) / n_z as f64;
    let mut sum = 0.0;

    for i in 0..n_x {
        let x_i = a_x + i as f64 * h_x;
        for j in 0..n_y {
            let y_j = a_y + j as f64 * h_y;
            for k in 0..n_z {
                let z_k = a_z + k as f64 * h_z;
                sum += f(x_i, y_j, z_k);
            }
        }
    }

    sum * h_x * h_y * h_z
}

pub fn derivative<F>(f: F, x: f64, h: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    (f(x + h) - f(x - h)) / (2.0 * h)
}

/// y^2 = x^3 + ax + b where 4a^3 + 27b^2 neq 0
///
/// returns fn(x,y) for elliptic curve
pub fn elliptic_curve(x: f64, a: f64, b: f64) -> Box<dyn Fn(f64, f64)> {
    // Assert the smoothness/non-singularity condition
    assert_ne!(4.0 * fpowi(a, 3) + 27.0 * fpowi(b, 2), 0.0, "The curve is singular!");

    Box::new(move |x_val, y_val| {
        // Outer `x`, `a`, and `b` are safely moved into the closure
        let _residual = fpowi(y_val, 2) - (fpowi(x_val, 3) + a * x_val + b);
    })
}


/// Returns [P, Q, -R, R] where P + Q = R on the elliptic curve y^2 = x^3 + ax + b
pub fn elliptic_curve_add(
    p: (f64, f64),
    q: (f64, f64),
    a: f64,
    b: f64
) -> Option<[(f64, f64); 4]> {
    // 1. Verify the curve is non-singular
    let discriminant = 4.0 * fpowi(a, 3) + 27.0 * fpowi(b, 2);
    if discriminant == 0.0 {
        return None;
    }

    // 2. Compute the slope (lambda) based on whether P == Q (doubling) or P != Q (addition)
    let lambda = if p == q {
        // Point doubling: slope = (3*x^2 + a) / (2*y)
        if p.1 == 0.0 { return None; } // Tangent is vertical (point at infinity)
        (3.0 * fpowi(p.0, 2) + a) / (2.0 * p.1)
    } else {
        // Point addition: slope = (y2 - y1) / (x2 - x1)
        if p.0 == q.0 { return None; } // Vertical line (P + (-P) = Point at infinity)
        (q.1 - p.1) / (q.0 - p.0)
    };

    // 3. Compute R.x = lambda^2 - p.x - q.x
    let rx = fpowi(lambda, 2) - p.0 - q.0;

    // 4. Compute R.y = lambda * (p.x - R.x) - p.y
    let ry = lambda * (p.0 - rx) - p.1;

    let r = (rx, ry);
    let neg_r = (rx, -ry); // Inverse of R on an elliptic curve

    Some([p, q, neg_r, r])
}

/// Evaluates the curve at x and returns [R, -R] as coordinate pairs
pub fn elliptic_curve_lookup(x: f64, a: f64, b: f64) -> Option<[(f64, f64); 2]> {
    let discriminant = 4.0 * fpowi(a, 3) + 27.0 * fpowi(b, 2);
    if discriminant == 0.0 {
        return None;
    }

    let y_squared = fpowi(x, 3) + a * x + b;
    if y_squared < 0.0 {
        return None; // No real points at this x
    }

    let y = sqrt(y_squared);

    // Returns [R, -R]
    Some([(x, y), (x, -y)])
}


// Calculates the dot product of two 1D matrices (vectors) of any matching length.
/// Returns the result in the first element of a 4-element array, or None if lengths mismatch.
pub fn mm1d(mat1: &[f64], mat2: &[f64]) -> Option<[f64; 4]> {
    // 1D matrix multiplication (dot product) requires identical lengths
    if mat1.len() != mat2.len() {
        return None;
    }

    // Compute the dot product: sum of (mat1[i] * mat2[i])
    let dot_product: f64 = mat1
        .iter()
        .zip(mat2.iter())
        .map(|(&a, &b)| (a) * (b))
        .sum();

    // Return the result inside the required [f64; 4] structure
    Some([dot_product, 0.0, 0.0, 0.0])
}


/// Multiplies two 2D matrices represented as slices of slices.
/// Returns None if the dimensions are incompatible or if any matrix is empty/ragged.
pub fn mm2d(mat1: &[&[f64]], mat2: &[&[f64]]) -> Option<Vec<Vec<f64>>> {
    // Check for empty inputs
    if mat1.is_empty() || mat1[0].is_empty() || mat2.is_empty() || mat2[0].is_empty() {
        return None;
    }

    let rows_1 = mat1.len();
    let cols_1 = mat1[0].len();
    let rows_2 = mat2.len();
    let cols_2 = mat2[0].len();

    // Matrix multiplication constraint: Cols of A must equal Rows of B
    if cols_1 != rows_2 {
        return None;
    }

    // Ensure mat1 is not a "ragged" array (all rows must have the same length)
    if mat1.iter().any(|row| row.len() != cols_1) {
        return None;
    }

    // Ensure mat2 is not a "ragged" array
    if mat2.iter().any(|row| row.len() != cols_2) {
        return None;
    }

    // Initialize the result matrix with zeros (dimensions: rows_1 x cols_2)
    let mut result = vec![vec![0.0; cols_2]; rows_1];

    // Perform standard matrix multiplication (O(N^3))
    for i in 0..rows_1 {
        for j in 0..cols_2 {
            let mut sum = 0.0;
            for k in 0..cols_1 {
                sum += mat1[i][k] * mat2[k][j];
            }
            result[i][j] = sum;
        }
    }

    Some(result)
}

/// Performs batched 3D matrix multiplication.
/// Dimensions must be: (Batch, Rows_A, Cols_A) x (Batch, Cols_A, Cols_B) -> (Batch, Rows_A, Cols_B)
pub fn mm3d(mat1: &[&[&[f64]]], mat2: &[&[&[f64]]]) -> Option<Vec<Vec<Vec<f64>>>> {
    // 1. Ensure batches are not empty and lengths match
    if mat1.is_empty() || mat1.len() != mat2.len() {
        return None;
    }

    let batch_size = mat1.len();
    let mut result_batch = Vec::with_capacity(batch_size);

    // 2. Loop through each 2D matrix pair in the batch
    for b in 0..batch_size {
        let m1_2d = mat1[b];
        let m2_2d = mat2[b];

        // Ensure the 2D matrices in this batch slice are not empty
        if m1_2d.is_empty() || m1_2d[0].is_empty() || m2_2d.is_empty() || m2_2d[0].is_empty() {
            return None;
        }

        let rows_1 = m1_2d.len();
        let cols_1 = m1_2d[0].len();
        let rows_2 = m2_2d.len();
        let cols_2 = m2_2d[0].len();

        // Validate dimension alignment and protect against ragged structures
        if cols_1 != rows_2
            || m1_2d.iter().any(|row| row.len() != cols_1)
            || m2_2d.iter().any(|row| row.len() != cols_2)
        {
            return None;
        }

        // 3. Multiply the 2D matrices for this batch slice
        let mut result_2d = vec![vec![0.0; cols_2]; rows_1];
        for i in 0..rows_1 {
            for j in 0..cols_2 {
                let mut sum = 0.0;
                for k in 0..cols_1 {
                    sum += m1_2d[i][k] * m2_2d[k][j];
                }
                result_2d[i][j] = sum;
            }
        }

        result_batch.push(result_2d);
    }

    Some(result_batch)
}
