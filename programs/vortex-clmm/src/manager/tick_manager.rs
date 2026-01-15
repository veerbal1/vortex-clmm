use crate::{
    constants::NUM_REWARDS,
    errors::VortexError,
    math::add_liquidity_delta,
    state::{Tick, TickUpdate, WhirlpoolRewardInfo},
};

pub fn next_tick_modify_liquidity_update(
    tick: &Tick,
    tick_index: i32,
    tick_current_index: i32,
    fee_growth_global_a: u128,
    fee_growth_global_b: u128,
    reward_infos: &[WhirlpoolRewardInfo; NUM_REWARDS],
    liquidity_delta: i128,
    is_upper_tick: bool,
) -> Result<TickUpdate, VortexError> {
    if liquidity_delta == 0 {
        return Ok(TickUpdate {
            initialized: tick.initialized,
            liquidity_net: tick.liquidity_net,
            liquidity_gross: tick.liquidity_gross,
            fee_growth_outside_a: tick.fee_growth_outside_a,
            fee_growth_outside_b: tick.fee_growth_outside_b,
            reward_growths_outside: tick.reward_growths_outside,
        });
    }

    let new_liquidity_gross = add_liquidity_delta(tick.liquidity_gross, liquidity_delta)?;
    if new_liquidity_gross == 0 {
        return Ok(TickUpdate::default());
    }

    let (fee_growth_outside_a, fee_growth_outside_b, reward_growths_outside) =
        if tick.liquidity_gross == 0 {
            if tick_current_index >= tick_index {
                (
                    fee_growth_global_a,
                    fee_growth_global_b,
                    WhirlpoolRewardInfo::to_reward_growths(reward_infos),
                )
            } else {
                (0, 0, [0u128; NUM_REWARDS])
            }
        } else {
            (
                tick.fee_growth_outside_a,
                tick.fee_growth_outside_b,
                tick.reward_growths_outside,
            )
        };

    let liquidity_net = if is_upper_tick {
        tick.liquidity_net
            .checked_sub(liquidity_delta)
            .ok_or(VortexError::LiquidityNetUnderflow)?
    } else {
        tick.liquidity_net
            .checked_add(liquidity_delta)
            .ok_or(VortexError::LiquidityNetOverflow)?
    };

    Ok(TickUpdate {
        initialized: true,
        liquidity_net,
        liquidity_gross: new_liquidity_gross,
        fee_growth_outside_a,
        fee_growth_outside_b,
        reward_growths_outside,
    })
}
