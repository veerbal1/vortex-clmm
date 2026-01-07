use std::cmp::min;

use crate::{
    errors::VortexError,
    math::q64_64::{self, Q64_64},
};

pub fn get_amount_a_delta(
    sqrt_price_lower: Q64_64,
    sqrt_price_upper: Q64_64,
    liquidity: Q64_64,
    round_up: bool,
) -> Result<Q64_64, VortexError> {
    // Token A:  Δx = L × (√P_upper - √P_lower) / (√P_lower × √P_upper)
    let price_diff = sqrt_price_upper
        .inner()
        .checked_sub(sqrt_price_lower.inner())
        .ok_or(VortexError::LiquidityOverflow)?;

    let numerator = q64_64::mul(liquidity.inner(), price_diff);
    let denominator = q64_64::mul(sqrt_price_lower.inner(), sqrt_price_upper.inner());

    let result = if round_up {
        q64_64::div_round_up(numerator, denominator)
    } else {
        q64_64::div(numerator, denominator)
    };
    Ok(Q64_64::from_encoded(result))
}

pub fn get_amount_b_delta(
    sqrt_price_lower: Q64_64,
    sqrt_price_upper: Q64_64,
    liquidity: Q64_64,
    round_up: bool,
) -> Result<Q64_64, VortexError> {
    // Token B:  Δy = L × (√P_upper - √P_lower)
    let price_diff = sqrt_price_upper
        .inner()
        .checked_sub(sqrt_price_lower.inner())
        .ok_or(VortexError::LiquidityOverflow)?;

    let result = if round_up {
        q64_64::mul_round_up(liquidity.inner(), price_diff)
    } else {
        q64_64::mul(liquidity.inner(), price_diff)
    };

    Ok(Q64_64::from_encoded(result))
}

pub fn get_liquidity_for_amounts(
    sqrt_price_current: Q64_64,
    sqrt_price_lower: Q64_64,
    sqrt_price_upper: Q64_64,
    amount_a: Q64_64,
    amount_b: Q64_64,
) -> Result<Q64_64, VortexError> {
    // L = amount_a × (sqrt_lower × sqrt_upper) / (sqrt_upper - sqrt_lower)
    let l1 = {
        let num = q64_64::mul(sqrt_price_lower.inner(), sqrt_price_upper.inner());
        let den = sqrt_price_upper
            .inner()
            .checked_sub(sqrt_price_lower.inner())
            .ok_or(VortexError::LiquidityOverflow)?;
        let base = q64_64::mul(amount_a.inner(), num);
        let result = q64_64::div(base, den);
        result
    };

    let l2 = {
        // L =  amount_b / (sqrt_upper - sqrt_lower)
        let num = amount_b.inner();
        let den = sqrt_price_upper
            .inner()
            .checked_sub(sqrt_price_lower.inner())
            .ok_or(VortexError::LiquidityOverflow)?;
        let result = q64_64::div(num, den);
        result
    };

    // With this:
    let result = if sqrt_price_current <= sqrt_price_lower {
        l1 // Price below: only A
    } else if sqrt_price_current >= sqrt_price_upper {
        l2 // Price above: only B
    } else {
        min(l1, l2) // Price in range: take min
    };

    Ok(Q64_64::from_encoded(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_amount_a_delta_basic() {
        // Use known sqrt_price values
        // tick 0 = 1:1 price, sqrt_price = 2^64
        let sqrt_price_lower = Q64_64::from_raw(1u64);
        let sqrt_price_upper = Q64_64::from_raw(2u64);
        let liquidity = Q64_64::from_raw(1000u64);
        let result =
            get_amount_a_delta(sqrt_price_lower, sqrt_price_upper, liquidity, false).unwrap();

        println!("Result get_amount_a_delta: {}", result.inner());

        assert!(result.inner() > 0);
    }

    #[test]
    fn test_get_amount_b_delta_basic() {
        let sqrt_price_lower = Q64_64::from_raw(1u64);
        let sqrt_price_upper = Q64_64::from_raw(2u64);
        let liquidity = Q64_64::from_raw(1000u64);

        let result =
            get_amount_b_delta(sqrt_price_lower, sqrt_price_upper, liquidity, false).unwrap();
        println!("Result get_amount_b_delta: {}", result.inner());

        assert!(result.inner() > 0);
    }

    #[test]
    fn test_get_liquidity_for_amounts_price_in_range() {
        // Price IN range → uses min of both
        let sqrt_price_lower = Q64_64::from_raw(4u64); // 1.0
        let sqrt_price_upper = Q64_64::from_raw(8u64); // 2.0
        let sqrt_price_current = Q64_64::from_raw(5u64); // 5 (in range)

        let result = get_liquidity_for_amounts(
            sqrt_price_current,
            sqrt_price_lower,
            sqrt_price_upper,
            Q64_64::from_raw(500u64),
            Q64_64::from_raw(1000u64),
        )
        .unwrap();

        println!("Liquidity (in range): {}", result.inner());
        assert!(result.inner() > 0);
    }

    #[test]
    fn test_get_liquidity_for_amounts_price_below_range() {
        // Price IN range → uses min of both
        let sqrt_price_lower = Q64_64::from_raw(4u64); // 4.0
        let sqrt_price_upper = Q64_64::from_raw(8u64); // 8.0
        let sqrt_price_current = Q64_64::from_raw(1u64); // 1 (Below range)

        let result = get_liquidity_for_amounts(
            sqrt_price_current,
            sqrt_price_lower,
            sqrt_price_upper,
            Q64_64::from_raw(500u64),
            Q64_64::from_raw(1000u64),
        )
        .unwrap();

        println!("Liquidity (below range): {}", result.inner());
        assert!(result.inner() > 0);
    }

    #[test]
    fn test_get_liquidity_for_amounts_price_above_range() {
        // Price IN range → uses min of both
        let sqrt_price_lower = Q64_64::from_raw(4u64); // 4.0
        let sqrt_price_upper = Q64_64::from_raw(8u64); // 8.0
        let sqrt_price_current = Q64_64::from_raw(9u64); // 9 (Above range)

        let result = get_liquidity_for_amounts(
            sqrt_price_current,
            sqrt_price_lower,
            sqrt_price_upper,
            Q64_64::from_raw(500u64),
            Q64_64::from_raw(1000u64),
        )
        .unwrap();

        println!("Liquidity (above range): {}", result.inner());
        assert!(result.inner() > 0);
    }

    #[test]
    fn test_roundtrip() {
        let lower = Q64_64::from_raw(4u64); // 4.0
        let upper = Q64_64::from_raw(8u64); // 8.0
        let current = Q64_64::from_raw(5u64);

        // Start with known liquidity
        let original_l = Q64_64::from_raw(1000u64);

        // Get token amounts FROM liquidity
        let amount_a = get_amount_a_delta(lower, upper, original_l, false).unwrap();
        let amount_b = get_amount_b_delta(lower, upper, original_l, false).unwrap();

        // Get liquidity back FROM token amounts
        let recovered_l =
            get_liquidity_for_amounts(current, lower, upper, amount_a, amount_b).unwrap();

        // Should be close to original (may lose some precision)
        println!(
            "Original: {}, Recovered: {}",
            original_l.inner(),
            recovered_l.inner()
        );
        assert_eq!(recovered_l.inner(), original_l.inner());
    }
}
