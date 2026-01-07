use std::ops::{Add, Sub};

use uint::construct_uint;

use crate::errors::VortexError;

construct_uint! {
    pub struct U256(4); // 4 * 64 bits = 256 bits
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Q64_64(u128);

impl Q64_64 {
    /// Create from raw u64 (encodes to Q64.64)
    pub fn from_raw(raw: u64) -> Self {
        Q64_64((raw as u128) << 64)
    }

    /// Create from already-encoded u128 (wraps without shifting)
    pub fn from_encoded(encoded: u128) -> Self {
        Q64_64(encoded)
    }

    /// Get inner u128 value
    pub fn inner(self) -> u128 {
        self.0
    }

    /// Convert back to u64 (truncates fractional part)
    pub fn to_u64(self) -> u64 {
        (self.0 >> 64) as u64
    }
}

impl Add for Q64_64 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Q64_64(self.0 + other.0)
    }
}

impl Sub for Q64_64 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Q64_64(self.0 - other.0)
    }
}

impl Q64_64 {
    /// Multiply two Q64.64 values with overflow protection
    pub fn checked_mul(self, other: Self) -> Result<Self, VortexError> {
        let a = U256::from(self.0);
        let b = U256::from(other.0);
        let result = (a * b) >> 64;

        // Check if result fits in u128
        if result > U256::from(u128::MAX) {
            return Err(VortexError::LiquidityOverflow);
        }
        Ok(Q64_64(result.as_u128()))
    }

    /// Divide two Q64.64 values with zero-division protection
    pub fn checked_div(self, other: Self) -> Result<Self, VortexError> {
        if other.0 == 0 {
            return Err(VortexError::LiquidityUnderflow); // or create DivisionByZero error
        }
        let a = U256::from(self.0) << 64;
        let b = U256::from(other.0);
        Ok(Q64_64((a / b).as_u128()))
    }
}

/// Q64.64 fixed-point resolution (64 fractional bits)
pub const Q64_RESOLUTION: u8 = 64;

/// Convert u64 to Q64.64 format (left shift by 64)
/// << means left shift, left shift means multiply
/// e.g
/// 5 << 1 => 5 * 2^1 = 10
/// 5 << 2 => 5 * 2^2 = 20
/// 5 << 3 => 5 * 2^3 = 40
pub fn from_u64(value: u64) -> u128 {
    (value as u128) << Q64_RESOLUTION
}

/// Convert Q64.64 to u64 (right shift by 64, truncates)
/// >> means right shift, it means divide
/// e.g
/// 40 >> 1 => 40 / 2^1 = 20
/// 40 >> 2 => 40 / 2^2 = 10
pub fn to_u64(value: u128) -> u64 {
    (value >> Q64_RESOLUTION) as u64
}

/// Convert Q64.64 to u64 with rounding up
pub fn to_u64_round_up(value: u128) -> u64 {
    // (a + b - 1) / b
    let fraction_mask = (1u128 << Q64_RESOLUTION) - 1;
    let val = (value + fraction_mask) >> Q64_RESOLUTION;
    val as u64
}

pub fn mul(a: u128, b: u128) -> u128 {
    let a_256 = U256::from(a);
    let b_256 = U256::from(b);
    let product = a_256 * b_256;

    let result = product >> 64;

    result.as_u128()
}

pub fn mul_round_up(a: u128, b: u128) -> u128 {
    let a_256 = U256::from(a);
    let b_256 = U256::from(b);
    let product = a_256 * b_256;

    let rounding = U256::from((1u128 << 64) - 1);
    let result = (product + rounding) >> 64;

    result.as_u128()
}

/// Divide two Q64.64 numbers (round down)
pub fn div(a: u128, b: u128) -> u128 {
    let a_256 = U256::from(a) << 64;
    let b_256 = U256::from(b);
    (a_256 / b_256).as_u128()
}

/// Divide two Q64.64 numbers (round up)
pub fn div_round_up(a: u128, b: u128) -> u128 {
    let a_256 = U256::from(a) << 64;
    let b_256 = U256::from(b);
    let rounding = b_256 - U256::from(1u128);
    ((a_256 + rounding) / b_256).as_u128()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u64() {
        assert_eq!(from_u64(1), 1u128 << 64);
        assert_eq!(from_u64(0), 0);
    }

    #[test]
    fn test_to_u64() {
        assert_eq!(to_u64(1u128 << 64), 1);
        assert_eq!(to_u64((1u128 << 64) + (1u128 << 63)), 1); // 1.5 truncates to 1
    }

    #[test]
    fn test_to_u64_round_up() {
        assert_eq!(to_u64_round_up(1u128 << 64), 1);
        assert_eq!(to_u64_round_up((1u128 << 64) + 1), 2); // any fraction rounds up
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul(2 << 64, 2 << 64), 4 << 64);
        assert_ne!(mul(2 << 64, 2 << 64), 5 << 64);

        assert_eq!(mul((1 << 64) + (1 << 63), 3 << 64), (4 << 64) + (1 << 63));
    }

    #[test]
    fn test_mul_round_up() {
        // For exact results, mul and mul_round_up are the same
        let one_point_five = (1u128 << 64) + (1u128 << 63);
        let three = 3u128 << 64;

        let result = mul_round_up(one_point_five, three);
        let expected = (4u128 << 64) + (1u128 << 63); // 4.5 in Q64.64
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mul_round_up_difference() {
        // This shows when mul_round_up actually differs from mul
        // Use values that produce sub-fractional bits
        let a = (1u128 << 64) + 1; // 1 + epsilon
        let b = (1u128 << 64) + 1; // 1 + epsilon

        let floor = mul(a, b);
        let ceil = mul_round_up(a, b);

        // ceil should be >= floor
        assert!(ceil >= floor);
        // They should differ by at most 1
        assert!(ceil - floor <= 1);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(4 << 64, 2 << 64), 2 << 64);
        assert_eq!(div(6 << 64, 2 << 64), 3 << 64);
        assert_ne!(div(6 << 64, 2 << 64), 6 << 64);

        assert_eq!(div(9 << 64, 2 << 64), (4 << 64) + (1 << 63));
        assert_ne!(div(9 << 64, 2 << 64), (4 << 64) + (1 << 64));
    }

    #[test]
    fn test_div_round_up() {
        assert_eq!(div_round_up(6 << 64, 2 << 64), 3 << 64);
        assert_eq!(
            div_round_up(9 << 64, 2 << 64),
            (4u128 << 64) + (1u128 << 63)
        );

        let a = 7u128 << 64;
        let b = 3u128 << 64;

        let floor = div(a, b);
        let ceil = div_round_up(a, b);
        assert!(ceil >= floor);
    }

    #[test]
    fn test_div_round_up_difference() {
        let a = (1u128 << 64) + 1; // 1 + epsilon
        let b = 3u128 << 64; // 3

        let floor = div(a, b);
        let ceil = div_round_up(a, b);

        // ceil should be >= floor
        assert!(ceil >= floor);
    }
}
