use crate::{
    constants::NUM_REWARDS,
    errors::VortexError,
    math::{add_liquidity_delta, q64_64::Q64_64, swap_math, tick_math},
    state::{Tick, Whirlpool, WhirlpoolRewardInfo},
};

use super::next_tick_cross_update;

#[derive(Debug, Clone)]
pub struct SwapState {
    /// Amount remaining to be swapped (decreases as we go)
    pub amount_remaining: u64,

    /// Amount calculated (output for exact-in, input for exact-out)
    pub amount_calculated: u64,

    /// Current sqrt_price as we move through the swap
    pub sqrt_price: u128,

    /// Current tick index
    pub tick_index: i32,

    /// Active liquidity at current price
    pub liquidity: u128,

    /// Fee growth accumulated during this swap (token A or B depending on direction)
    pub fee_growth_global: u128,

    /// Protocol fees collected during this swap
    pub protocol_fee: u64,
}

pub struct SwapResult {
    /// Amount of input token consumed
    pub amount_in: u64,

    /// Amount of output token produced
    pub amount_out: u64,

    /// Final sqrt_price after swap
    pub next_sqrt_price: u128,

    /// Final tick index after swap
    pub next_tick_index: i32,

    /// Total fees collected (goes to LPs)
    pub total_fee: u64,

    /// Protocol's portion of fees
    pub protocol_fee: u64,
}

#[derive(Debug)]
pub struct SwapStepResult {
    /// Amount consumed in this step
    pub amount_in: u64,

    /// Amount produced in this step  
    pub amount_out: u64,

    /// Fee collected in this step
    pub fee_amount: u64,

    /// New sqrt_price after this step
    pub next_sqrt_price: u128,

    /// New tick index after this step
    pub next_tick_index: i32,

    /// Did we reach the target tick boundary?
    pub reached_tick_boundary: bool,
}

pub fn initialize_swap_state(
    whirlpool: &Whirlpool,
    amount: u64,
    sqrt_price_limit: u128,
    a_to_b: bool,
) -> SwapState {
    SwapState {
        amount_remaining: amount,
        amount_calculated: 0,
        sqrt_price: whirlpool.sqrt_price,
        tick_index: whirlpool.tick_current_index,
        liquidity: whirlpool.liquidity,
        fee_growth_global: 0,
        protocol_fee: 0,
    }
}

/// Execute a single swap step within the current tick range.
/// Returns what happened in this step and whether we hit a tick boundary.
pub fn execute_swap_step(
    state: &SwapState,
    next_tick_index: i32, // The next initialized tick we're moving toward
    fee_rate: u16,
    protocol_fee_rate: u16,
    a_to_b: bool,
) -> Result<SwapStepResult, VortexError> {
    // 1. Get sqrt_price at target tick
    let target_sqrt_price = tick_math::get_sqrt_price_at_tick(next_tick_index)?;

    // 2. Convert to Q64_64 types for swap_math
    let current_sqrt_price_q64 = Q64_64::from_encoded(state.sqrt_price);
    let target_sqrt_price_q64 = Q64_64::from_encoded(target_sqrt_price);
    let liquidity_q64 = Q64_64::from_encoded(state.liquidity);
    let amount_remaining_q64 = Q64_64::from_raw(state.amount_remaining);

    // 3. Call compute_swap_step from swap_math
    let step_result = swap_math::compute_swap_step(
        current_sqrt_price_q64,
        target_sqrt_price_q64,
        liquidity_q64,
        amount_remaining_q64,
        fee_rate,
        a_to_b,
    )?;

    // 4. Determine if we reached the tick boundary
    let reached_tick_boundary = step_result.sqrt_price_next.inner() == target_sqrt_price;

    // 5. Calculate new tick index
    let next_tick = if reached_tick_boundary {
        if a_to_b {
            next_tick_index - 1 // Moving left, land just below the tick
        } else {
            next_tick_index // Moving right, land at the tick
        }
    } else {
        tick_math::get_tick_at_sqrt_price(step_result.sqrt_price_next.inner())?
    };

    // 6. Calculate protocol fee portion
    let fee_amount_u64 = step_result.fee_amount.to_u64();
    let protocol_fee = ((fee_amount_u64 as u128) * (protocol_fee_rate as u128) / 10000) as u64;

    Ok(SwapStepResult {
        amount_in: step_result.amount_in.to_u64(),
        amount_out: step_result.amount_out.to_u64(),
        fee_amount: fee_amount_u64,
        next_sqrt_price: step_result.sqrt_price_next.inner(),
        next_tick_index: next_tick,
        reached_tick_boundary,
    })
}

/// Handle crossing a tick boundary during a swap.
/// This flips the fee/reward growth outside values and updates liquidity.
///
/// Returns the tick update to apply and the new liquidity after crossing.
pub fn handle_tick_crossing(
    state: &mut SwapState,
    tick: &mut Tick,
    fee_growth_global_a: u128,
    fee_growth_global_b: u128,
    reward_infos: &[WhirlpoolRewardInfo; NUM_REWARDS],
    a_to_b: bool,
) -> Result<(), VortexError> {
    // 1. Get the tick cross update (flips fee/reward growths)
    let tick_update =
        next_tick_cross_update(tick, fee_growth_global_a, fee_growth_global_b, reward_infos);

    // 2. Apply the update to the tick
    tick.update(&tick_update);

    // 3. Update liquidity in swap state
    // When moving left (a_to_b):  crossing a tick means we're EXITING positions that end here
    //                             and ENTERING positions that start here
    //                             liquidity_net is positive at lower ticks (where positions start)
    //                             so we SUBTRACT liquidity_net
    // When moving right (b_to_a): we're doing the opposite, so we ADD liquidity_net
    let liquidity_delta = if a_to_b {
        -tick.liquidity_net // Subtract when moving left
    } else {
        tick.liquidity_net // Add when moving right
    };

    state.liquidity = add_liquidity_delta(state.liquidity, liquidity_delta)?;

    Ok(())
}

/// Update swap state after executing a swap step.
/// This is called after each step to accumulate results.
pub fn update_swap_state(
    state: &mut SwapState,
    step_result: &SwapStepResult,
    protocol_fee: u64,
    amount_specified_is_input: bool,
) {
    // Update amounts based on swap direction
    if amount_specified_is_input {
        // Exact input: we're consuming amount_in from remaining
        state.amount_remaining = state
            .amount_remaining
            .saturating_sub(step_result.amount_in + step_result.fee_amount);
        state.amount_calculated = state
            .amount_calculated
            .saturating_add(step_result.amount_out);
    } else {
        // Exact output: we're producing amount_out, accumulating amount_in
        state.amount_remaining = state
            .amount_remaining
            .saturating_sub(step_result.amount_out);
        state.amount_calculated = state
            .amount_calculated
            .saturating_add(step_result.amount_in + step_result.fee_amount);
    }

    // Update price and tick
    state.sqrt_price = step_result.next_sqrt_price;
    state.tick_index = step_result.next_tick_index;

    // Accumulate fees
    state.protocol_fee = state.protocol_fee.saturating_add(protocol_fee);
}

/// Finalize swap state into a SwapResult.
/// Called after the swap loop completes.
pub fn finalize_swap(
    state: &SwapState,
    initial_amount: u64,
    total_fee: u64,
    amount_specified_is_input: bool,
) -> SwapResult {
    let (amount_in, amount_out) = if amount_specified_is_input {
        (
            initial_amount.saturating_sub(state.amount_remaining),
            state.amount_calculated,
        )
    } else {
        (
            state.amount_calculated,
            initial_amount.saturating_sub(state.amount_remaining),
        )
    };

    SwapResult {
        amount_in,
        amount_out,
        next_sqrt_price: state.sqrt_price,
        next_tick_index: state.tick_index,
        total_fee,
        protocol_fee: state.protocol_fee,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_whirlpool() -> Whirlpool {
        Whirlpool {
            whirlpools_config: Pubkey::default(),
            whirlpool_bump: 0,
            tick_spacing: 10,
            fee_rate: 3000,
            protocol_fee_rate: 300,
            liquidity: 1_000_000,
            sqrt_price: 1 << 64, // 1.0 in Q64.64
            tick_current_index: 0,
            protocol_fee_owed_a: 0,
            protocol_fee_owed_b: 0,
            token_mint_a: Pubkey::default(),
            token_vault_a: Pubkey::default(),
            fee_growth_global_a: 0,
            token_mint_b: Pubkey::default(),
            token_vault_b: Pubkey::default(),
            fee_growth_global_b: 0,
            reward_last_updated_timestamp: 0,
            reward_infos: [WhirlpoolRewardInfo::default(); NUM_REWARDS],
        }
    }

    #[test]
    fn test_initialize_swap_state() {
        let whirlpool = create_test_whirlpool();
        let state = initialize_swap_state(&whirlpool, 1000, 0, true);

        assert_eq!(state.amount_remaining, 1000);
        assert_eq!(state.amount_calculated, 0);
        assert_eq!(state.sqrt_price, whirlpool.sqrt_price);
        assert_eq!(state.tick_index, whirlpool.tick_current_index);
        assert_eq!(state.liquidity, whirlpool.liquidity);
    }

    #[test]
    fn test_update_swap_state_exact_input() {
        let whirlpool = create_test_whirlpool();
        let mut state = initialize_swap_state(&whirlpool, 1000, 0, true);

        let step_result = SwapStepResult {
            amount_in: 100,
            amount_out: 95,
            fee_amount: 3,
            next_sqrt_price: (1 << 64) - 1000,
            next_tick_index: -1,
            reached_tick_boundary: false,
        };

        update_swap_state(&mut state, &step_result, 1, true);

        // amount_remaining = 1000 - 100 - 3 = 897
        assert_eq!(state.amount_remaining, 897);
        // amount_calculated = 0 + 95 = 95
        assert_eq!(state.amount_calculated, 95);
        assert_eq!(state.sqrt_price, step_result.next_sqrt_price);
        assert_eq!(state.tick_index, -1);
        assert_eq!(state.protocol_fee, 1);
    }

    #[test]
    fn test_finalize_swap() {
        let state = SwapState {
            amount_remaining: 0,
            amount_calculated: 950,
            sqrt_price: 1 << 63,
            tick_index: -100,
            liquidity: 1_000_000,
            fee_growth_global: 0,
            protocol_fee: 10,
        };

        let result = finalize_swap(&state, 1000, 30, true);

        assert_eq!(result.amount_in, 1000); // consumed all
        assert_eq!(result.amount_out, 950);
        assert_eq!(result.next_sqrt_price, 1 << 63);
        assert_eq!(result.next_tick_index, -100);
        assert_eq!(result.total_fee, 30);
        assert_eq!(result.protocol_fee, 10);
    }
}
