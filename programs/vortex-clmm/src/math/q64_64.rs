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
}
