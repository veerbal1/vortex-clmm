use crate::errors::VortexError;

/// Safely adds two liquidity values.
///
/// # Errors
/// Returns `LiquidityOverflow` if the sum exceeds u128::MAX.
pub fn add_liquidity(current_liquidity: u128, new_liquidity: u128) -> Result<u128, VortexError> {
    current_liquidity
        .checked_add(new_liquidity)
        .ok_or(VortexError::LiquidityOverflow)
}

/// Safely subtracts two liquidity values.
///
/// # Errors
/// Returns `LiquidityUnderflow` if the result is less than 0.
pub fn sub_liquidity(current_liquidity: u128, new_liquidity: u128) -> Result<u128, VortexError> {
    current_liquidity
        .checked_sub(new_liquidity)
        .ok_or(VortexError::LiquidityUnderflow)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_liquidity() {
        assert_eq!(add_liquidity(1, 1).unwrap(), 2);
    }

    #[test]
    fn test_add_liquidity_overflow() {
        assert!(add_liquidity(u128::MAX, 1).is_err());
    }

    #[test]
    fn test_sub_liquidity() {
        assert_eq!(sub_liquidity(2, 1).unwrap(), 1);
    }

    #[test]
    fn test_sub_liquidity_underflow() {
        assert!(sub_liquidity(1, 2).is_err());
    }
}
