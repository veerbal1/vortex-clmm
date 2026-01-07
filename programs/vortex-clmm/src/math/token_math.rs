use crate::{
    errors::VortexError,
    math::q64_64::{self},
};

pub fn get_amount_a_delta(
    sqrt_price_lower: u128,
    sqrt_price_upper: u128,
    liquidity: u128,
    round_up: bool,
) -> Result<u64, VortexError> {
    // Token A:  Δx = L × (√P_upper - √P_lower) / (√P_lower × √P_upper)
    let price_diff = sqrt_price_upper
        .checked_sub(sqrt_price_lower)
        .ok_or(VortexError::LiquidityOverflow)?;

    let numerator = q64_64::mul(liquidity, price_diff);
    let denominator = q64_64::mul(sqrt_price_lower, sqrt_price_upper);

    let result = if round_up {
        q64_64::div_round_up(numerator, denominator)
    } else {
        q64_64::div(numerator, denominator)
    };
    Ok(q64_64::to_u64(result))
}

pub fn get_amount_b_delta(
    sqrt_price_lower: u128,
    sqrt_price_upper: u128,
    liquidity: u128,
    round_up: bool,
) -> Result<u64, VortexError> {
    // Token B:  Δy = L × (√P_upper - √P_lower)
    let price_diff = sqrt_price_upper
        .checked_sub(sqrt_price_lower)
        .ok_or(VortexError::LiquidityOverflow)?;

    let result = if round_up {
        q64_64::mul_round_up(liquidity, price_diff)
    } else {
        q64_64::mul(liquidity, price_diff)
    };

    Ok(q64_64::to_u64(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_amount_a_delta_basic() {
        // Use known sqrt_price values
        // tick 0 = 1:1 price, sqrt_price = 2^64
        let sqrt_price_lower = 1u128 << 64;
        let sqrt_price_upper = 2u128 << 64;
        let liquidity = 1000u128 << 64;
        let result =
            get_amount_a_delta(sqrt_price_lower, sqrt_price_upper, liquidity, false).unwrap();

        println!("Result get_amount_a_delta: {}", result);

        assert!(result > 0);
    }

    #[test]
    fn test_get_amount_b_delta_basic() {
        let sqrt_price_lower = 1u128 << 64;
        let sqrt_price_upper = 2u128 << 64;
        let liquidity = 1000u128 << 64;

        let result =
            get_amount_b_delta(sqrt_price_lower, sqrt_price_upper, liquidity, false).unwrap();
        println!("Result get_amount_b_delta: {}", result);

        assert!(result > 0);
    }
}
