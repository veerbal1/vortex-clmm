use crate::{
    errors::VortexError,
    math::add_liquidity_delta,
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
