use crate::{
    errors::VortexError,
    math::{add_liquidity_delta, q64_64::Q64_64, tick_math, token_math},
    state::{Position, PositionUpdate, Tick, TickUpdate, Whirlpool},
};

use super::{
    next_fee_growths_inside, next_position_modify_liquidity_update, next_reward_growths_inside,
    next_tick_modify_liquidity_update,
};

#[derive(Debug)]
pub struct ModifyLiquidityUpdate {
    pub whirlpool_liquidity: u128,
    pub tick_lower_update: TickUpdate,
    pub tick_upper_update: TickUpdate,
    pub position_update: PositionUpdate,
}

pub fn next_whirlpool_liquidity(
    whirlpool: &Whirlpool,
    tick_upper_index: i32,
    tick_lower_index: i32,
    liquidity_delta: i128,
) -> Result<u128, VortexError> {
    if tick_lower_index <= whirlpool.tick_current_index
        && whirlpool.tick_current_index < tick_upper_index
    {
        return Ok(add_liquidity_delta(whirlpool.liquidity, liquidity_delta)?);
    }
    Ok(whirlpool.liquidity)
}

pub fn calculate_modify_liquidity(
    whirlpool: &Whirlpool,
    position: &Position,
    tick_lower: &Tick,
    tick_upper: &Tick,
    tick_lower_index: i32,
    tick_upper_index: i32,
    liquidity_delta: i128,
) -> Result<ModifyLiquidityUpdate, VortexError> {
    if liquidity_delta == 0 && position.liquidity == 0 {
        return Err(VortexError::InsufficientLiquidity);
    }

    let whirlpool_liquidity = next_whirlpool_liquidity(
        whirlpool,
        tick_upper_index,
        tick_lower_index,
        liquidity_delta,
    )?;

    let tick_lower_update = next_tick_modify_liquidity_update(
        tick_lower,
        tick_lower_index,
        whirlpool.tick_current_index,
        whirlpool.fee_growth_global_a,
        whirlpool.fee_growth_global_b,
        &whirlpool.reward_infos,
        liquidity_delta,
        false, // is_upper = false
    )?;

    let tick_upper_update = next_tick_modify_liquidity_update(
        tick_upper,
        tick_upper_index,
        whirlpool.tick_current_index,
        whirlpool.fee_growth_global_a,
        whirlpool.fee_growth_global_b,
        &whirlpool.reward_infos,
        liquidity_delta,
        true, // is_upper = true
    )?;

    let (fee_growth_inside_a, fee_growth_inside_b) = next_fee_growths_inside(
        whirlpool.tick_current_index,
        tick_lower,
        tick_lower_index,
        tick_upper,
        tick_upper_index,
        whirlpool.fee_growth_global_a,
        whirlpool.fee_growth_global_b,
    );

    let reward_growths_inside = next_reward_growths_inside(
        whirlpool.tick_current_index,
        tick_lower,
        tick_lower_index,
        tick_upper,
        tick_upper_index,
        &whirlpool.reward_infos,
    );

    let position_update = next_position_modify_liquidity_update(
        position,
        liquidity_delta,
        fee_growth_inside_a,
        fee_growth_inside_b,
        &reward_growths_inside,
    )?;

    Ok(ModifyLiquidityUpdate {
        whirlpool_liquidity,
        tick_lower_update,
        tick_upper_update,
        position_update,
    })
}

/// Calculate how many tokens A and B are needed for a given liquidity delta.
///
/// The amounts depend on where the current price is relative to the position range:
/// - Price below range: only Token A needed
/// - Price inside range: both tokens needed  
/// - Price above range: only Token B needed
pub fn calculate_liquidity_token_deltas(
    current_tick_index: i32,
    sqrt_price: u128,
    position: &Position,
    liquidity_delta: i128,
) -> Result<(u64, u64), VortexError> {
    if liquidity_delta == 0 {
        return Err(VortexError::InsufficientLiquidity);
    }

    let liquidity = liquidity_delta.unsigned_abs();
    let round_up = liquidity_delta > 0; // round up when adding, down when removing

    // Get sqrt prices at tick boundaries
    let lower_sqrt_price = tick_math::get_sqrt_price_at_tick(position.tick_lower_index)?;
    let upper_sqrt_price = tick_math::get_sqrt_price_at_tick(position.tick_upper_index)?;

    // Convert to Q64_64 for token math
    let liquidity_q64 = Q64_64::from_encoded(liquidity);
    let lower_q64 = Q64_64::from_encoded(lower_sqrt_price);
    let upper_q64 = Q64_64::from_encoded(upper_sqrt_price);
    let current_q64 = Q64_64::from_encoded(sqrt_price);

    let (delta_a, delta_b) = if current_tick_index < position.tick_lower_index {
        // Price is BELOW range: only Token A needed
        let amount_a =
            token_math::get_amount_a_delta(lower_q64, upper_q64, liquidity_q64, round_up)?;
        (amount_a.to_u64(), 0u64)
    } else if current_tick_index < position.tick_upper_index {
        // Price is INSIDE range: both tokens needed
        let amount_a =
            token_math::get_amount_a_delta(current_q64, upper_q64, liquidity_q64, round_up)?;
        let amount_b =
            token_math::get_amount_b_delta(lower_q64, current_q64, liquidity_q64, round_up)?;
        (amount_a.to_u64(), amount_b.to_u64())
    } else {
        // Price is ABOVE range: only Token B needed
        let amount_b =
            token_math::get_amount_b_delta(lower_q64, upper_q64, liquidity_q64, round_up)?;
        (0u64, amount_b.to_u64())
    };

    Ok((delta_a, delta_b))
}

/// Apply the calculated ModifyLiquidityUpdate to actual state.
pub fn sync_modify_liquidity_values(
    whirlpool: &mut Whirlpool,
    position: &mut Position,
    tick_lower: &mut Tick,
    tick_upper: &mut Tick,
    update: &ModifyLiquidityUpdate,
) {
    // Update whirlpool liquidity
    whirlpool.liquidity = update.whirlpool_liquidity;

    // Update ticks
    tick_lower.update(&update.tick_lower_update);
    tick_upper.update(&update.tick_upper_update);

    // Update position
    position.update(&update.position_update);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::NUM_REWARDS;
    use crate::state::{PositionRewardInfo, WhirlpoolRewardInfo};
    use anchor_lang::prelude::Pubkey;

    fn create_test_whirlpool(
        tick_current_index: i32,
        liquidity: u128,
        sqrt_price: u128,
    ) -> Whirlpool {
        Whirlpool {
            whirlpools_config: Pubkey::default(),
            whirlpool_bump: 0,
            tick_spacing: 10,
            fee_rate: 3000,
            protocol_fee_rate: 300,
            liquidity,
            sqrt_price,
            tick_current_index,
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

    fn create_test_position(
        tick_lower_index: i32,
        tick_upper_index: i32,
        liquidity: u128,
    ) -> Position {
        Position {
            whirlpool: Pubkey::default(),
            position_mint: Pubkey::default(),
            liquidity,
            tick_lower_index,
            tick_upper_index,
            fee_growth_checkpoint_a: 0,
            fee_owed_a: 0,
            fee_growth_checkpoint_b: 0,
            fee_owed_b: 0,
            reward_infos: [PositionRewardInfo::default(); NUM_REWARDS],
        }
    }

    // Test: Pool liquidity increases when position is in range
    #[test]
    fn test_whirlpool_liquidity_in_range() {
        let whirlpool = create_test_whirlpool(50, 1000, 1 << 64);

        // Position range: 0 to 100, current tick is 50 (in range)
        let result = next_whirlpool_liquidity(&whirlpool, 100, 0, 500).unwrap();

        assert_eq!(result, 1500); // 1000 + 500
    }

    // Test: Pool liquidity unchanged when position below range
    #[test]
    fn test_whirlpool_liquidity_below_range() {
        let whirlpool = create_test_whirlpool(50, 1000, 1 << 64);

        // Position range: 100 to 200, current tick is 50 (below range)
        let result = next_whirlpool_liquidity(&whirlpool, 200, 100, 500).unwrap();

        assert_eq!(result, 1000); // unchanged
    }

    // Test: Pool liquidity unchanged when position above range
    #[test]
    fn test_whirlpool_liquidity_above_range() {
        let whirlpool = create_test_whirlpool(250, 1000, 1 << 64);

        // Position range: 100 to 200, current tick is 250 (above range)
        let result = next_whirlpool_liquidity(&whirlpool, 200, 100, 500).unwrap();

        assert_eq!(result, 1000); // unchanged
    }

    // Test: Zero liquidity delta on zero liquidity position fails
    #[test]
    fn test_zero_delta_zero_liquidity_fails() {
        let whirlpool = create_test_whirlpool(50, 1000, 1 << 64);
        let position = create_test_position(0, 100, 0);
        let tick_lower = Tick::default();
        let tick_upper = Tick::default();

        let result = calculate_modify_liquidity(
            &whirlpool,
            &position,
            &tick_lower,
            &tick_upper,
            0,
            100,
            0, // zero delta
        );

        assert!(result.is_err());
    }

    // Test: Adding liquidity works
    #[test]
    fn test_add_liquidity() {
        let whirlpool = create_test_whirlpool(50, 1000, 1 << 64);
        let position = create_test_position(0, 100, 0);
        let tick_lower = Tick::default();
        let tick_upper = Tick::default();

        let result = calculate_modify_liquidity(
            &whirlpool,
            &position,
            &tick_lower,
            &tick_upper,
            0,
            100,
            500, // add 500 liquidity
        )
        .unwrap();

        assert_eq!(result.whirlpool_liquidity, 1500);
        assert_eq!(result.position_update.liquidity, 500);
        assert!(result.tick_lower_update.initialized);
        assert!(result.tick_upper_update.initialized);
    }
}
