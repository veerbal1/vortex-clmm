use uint::construct_uint;

construct_uint! {
    pub struct U256(4); // 4 * 64 bits = 256 bits
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
}
