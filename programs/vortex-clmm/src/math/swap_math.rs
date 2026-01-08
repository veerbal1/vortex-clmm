use crate::errors::VortexError;
use crate::math::q64_64::{self, Q64_64};
use crate::math::token_math::{
    get_amount_a_delta, get_amount_b_delta, get_next_sqrt_price_from_a, get_next_sqrt_price_from_b,
};

/// Result of a single swap step
#[derive(Debug, Clone, Copy)]
pub struct SwapStepResult {
    pub sqrt_price_next: Q64_64,
    pub amount_in: Q64_64,
    pub amount_out: Q64_64,
    pub fee_amount: Q64_64,
}

/// Apply swap fee to amount (fee taken before swap)
/// fee_rate is in parts per million (3000 = 0.3%)
///
/// Formula: amount_after = amount × (1_000_000 - fee_rate) / 1_000_000
pub fn apply_swap_fee(amount: Q64_64, fee_rate: u32) -> Result<Q64_64, VortexError> {
    const FEE_DENOMINATOR: u128 = 1_000_000;

    let multiplier = FEE_DENOMINATOR
        .checked_sub(fee_rate as u128)
        .ok_or(VortexError::LiquidityOverflow)?;

    let result = q64_64::div(q64_64::mul(amount.inner(), multiplier), FEE_DENOMINATOR);

    Ok(Q64_64::from_encoded(result))
}

/// Reverse apply swap fee (calculate pre-fee amount from post-fee)
///
/// Formula: pre_fee = amount × 1_000_000 / (1_000_000 - fee_rate)
pub fn reverse_apply_swap_fee(amount: Q64_64, fee_rate: u32) -> Result<Q64_64, VortexError> {
    const FEE_DENOMINATOR: u128 = 1_000_000;

    let denominator = FEE_DENOMINATOR
        .checked_sub(fee_rate as u128)
        .ok_or(VortexError::LiquidityOverflow)?;

    // Round UP to ensure pool is protected
    let result = q64_64::div_round_up(q64_64::mul(amount.inner(), FEE_DENOMINATOR), denominator);

    Ok(Q64_64::from_encoded(result))
}

/// Calculate a single swap step
pub fn compute_swap_step(
    sqrt_price_current: Q64_64,
    sqrt_price_target: Q64_64,
    liquidity: Q64_64,
    amount_remaining: Q64_64,
    fee_rate: u16, // Fee in basis points (30 = 0.3%)
    a_to_b: bool,
) -> Result<SwapStepResult, VortexError> {
    // Step 1: Calculate fee
    // fee_amount = amount_remaining * fee_rate / 10000
    let fee_amount = q64_64::div(
        q64_64::mul(amount_remaining.inner(), fee_rate as u128),
        10000u128,
    );

    // Step 2: Amount after fee
    // amount_after_fee = amount_remaining - fee_amount
    let amount_after_fee = amount_remaining
        .inner()
        .checked_sub(fee_amount)
        .ok_or(VortexError::LiquidityOverflow)?;

    // Step 3: What's the max we can swap before hitting the boundary?
    // (uses get_amount_a_delta or get_amount_b_delta)
    let amount_after_fee = Q64_64::from_encoded(amount_after_fee);
    let max_swap_in_tick = if a_to_b {
        get_amount_a_delta(sqrt_price_target, sqrt_price_current, liquidity, true)?
    } else {
        get_amount_b_delta(sqrt_price_current, sqrt_price_target, liquidity, true)?
    };

    // Step 4: Do we reach the boundary or stop early?
    let reaches_boundary = amount_after_fee.inner() >= max_swap_in_tick.inner();

    // Step 5: Calculate sqrt_price_next
    let sqrt_price_next = if reaches_boundary {
        // We hit the boundary
        sqrt_price_target
    } else {
        // We stay within tick - calculate where we actually stop
        if a_to_b {
            get_next_sqrt_price_from_a(sqrt_price_current, liquidity, amount_after_fee, true)?
        } else {
            get_next_sqrt_price_from_b(sqrt_price_current, liquidity, amount_after_fee, true)?
        }
    };

    // Step 6: Calculate amount_in and amount_out
    let (amount_in, amount_out) = if reaches_boundary {
        // We hit boundary - use max_swap_in_tick as amount_in
        let amount_in = max_swap_in_tick;
        let amount_out = if a_to_b {
            // Selling A, getting B
            get_amount_b_delta(sqrt_price_next, sqrt_price_current, liquidity, false)?
        } else {
            // Selling B, getting A
            get_amount_a_delta(sqrt_price_current, sqrt_price_next, liquidity, false)?
        };
        (amount_in, amount_out)
    } else {
        // We stay within tick - use amount_after_fee as amount_in
        let amount_in = amount_after_fee;
        let amount_out = if a_to_b {
            get_amount_b_delta(sqrt_price_next, sqrt_price_current, liquidity, false)?
        } else {
            get_amount_a_delta(sqrt_price_current, sqrt_price_next, liquidity, false)?
        };
        (amount_in, amount_out)
    };

    // Step 7: Return result
    Ok(SwapStepResult {
        sqrt_price_next,
        amount_in,
        amount_out,
        fee_amount: Q64_64::from_encoded(fee_amount),
    })
}
