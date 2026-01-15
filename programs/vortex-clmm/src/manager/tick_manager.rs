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

pub fn next_tick_cross_update(
    tick: &Tick,
    fee_growth_global_a: u128,
    fee_growth_global_b: u128,
    reward_infos: &[WhirlpoolRewardInfo; NUM_REWARDS],
) -> TickUpdate {
    let mut update = TickUpdate {
        initialized: tick.initialized,
        liquidity_net: tick.liquidity_net,
        liquidity_gross: tick.liquidity_gross,
        fee_growth_outside_a: tick.fee_growth_outside_a,
        fee_growth_outside_b: tick.fee_growth_outside_b,
        reward_growths_outside: tick.reward_growths_outside,
    };

    update.fee_growth_outside_a = fee_growth_global_a.wrapping_sub(tick.fee_growth_outside_a);
    update.fee_growth_outside_b = fee_growth_global_b.wrapping_sub(tick.fee_growth_outside_b);

    for i in 0..NUM_REWARDS {
        update.reward_growths_outside[i] = reward_infos[i]
            .growth_global_x64
            .wrapping_sub(tick.reward_growths_outside[i]);
    }

    update
}

pub fn next_fee_growths_inside(
    tick_current_index: i32,
    tick_lower: &Tick,
    tick_lower_index: i32,
    tick_upper: &Tick,
    tick_upper_index: i32,
    fee_growth_global_a: u128,
    fee_growth_global_b: u128,
) -> (u128, u128) {
    // ============================================
    // STEP 1: Calculate fee growth BELOW the range
    // ============================================
    let (fee_growth_below_a, fee_growth_below_b) = if tick_current_index < tick_lower_index {
        // Price is BELOW lower tick → flip
        (
            fee_growth_global_a.wrapping_sub(tick_lower.fee_growth_outside_a),
            fee_growth_global_b.wrapping_sub(tick_lower.fee_growth_outside_b),
        )
    } else {
        (
            tick_lower.fee_growth_outside_a,
            tick_lower.fee_growth_outside_b,
        )
    };

    let (fee_growth_above_a, fee_growth_above_b) = if tick_current_index < tick_upper_index {
        // Price is BELOW upper tick → use directly
        (
            tick_upper.fee_growth_outside_a,
            tick_upper.fee_growth_outside_b,
        )
    } else {
        // Price is AT or ABOVE upper tick → flip
        (
            fee_growth_global_a.wrapping_sub(tick_upper.fee_growth_outside_a),
            fee_growth_global_b.wrapping_sub(tick_upper.fee_growth_outside_b),
        )
    };

    // ============================================
    // STEP 3: Calculate fee growth INSIDE the range
    // inside = global - below - above
    // ============================================
    (
        fee_growth_global_a
            .wrapping_sub(fee_growth_below_a)
            .wrapping_sub(fee_growth_above_a),
        fee_growth_global_b
            .wrapping_sub(fee_growth_below_b)
            .wrapping_sub(fee_growth_above_b),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_reward_infos(growth_global: u128) -> [WhirlpoolRewardInfo; NUM_REWARDS] {
        [
            WhirlpoolRewardInfo {
                mint: Pubkey::new_unique(),
                vault: Pubkey::default(),
                authority: Pubkey::default(),
                emissions_per_second_x64: 1,
                growth_global_x64: growth_global,
            },
            WhirlpoolRewardInfo {
                mint: Pubkey::new_unique(),
                vault: Pubkey::default(),
                authority: Pubkey::default(),
                emissions_per_second_x64: 1,
                growth_global_x64: growth_global,
            },
            WhirlpoolRewardInfo {
                mint: Pubkey::new_unique(),
                vault: Pubkey::default(),
                authority: Pubkey::default(),
                emissions_per_second_x64: 1,
                growth_global_x64: growth_global,
            },
        ]
    }

    // Test 1: No-op when delta is zero
    #[test]
    fn test_zero_delta_returns_unchanged() {
        let tick = Tick {
            initialized: true,
            liquidity_net: 1000,
            liquidity_gross: 2000,
            fee_growth_outside_a: 100,
            fee_growth_outside_b: 200,
            reward_growths_outside: [10, 20, 30],
        };
        let reward_infos = create_reward_infos(500);

        let update =
            next_tick_modify_liquidity_update(&tick, 100, 50, 1000, 2000, &reward_infos, 0, false)
                .unwrap();

        assert_eq!(update.initialized, true);
        assert_eq!(update.liquidity_net, 1000);
        assert_eq!(update.liquidity_gross, 2000);
    }

    // Test 2: Deinitialize when liquidity becomes zero
    #[test]
    fn test_deinitialize_when_gross_becomes_zero() {
        let tick = Tick {
            initialized: true,
            liquidity_net: 1000,
            liquidity_gross: 1000,
            ..Default::default()
        };
        let reward_infos = create_reward_infos(0);

        let update =
            next_tick_modify_liquidity_update(&tick, 100, 50, 0, 0, &reward_infos, -1000, false)
                .unwrap();

        assert_eq!(update, TickUpdate::default());
    }

    // Test 3: Initialize tick when price >= tick_index (growths set to global)
    #[test]
    fn test_initialize_tick_price_above() {
        let tick = Tick::default(); // liquidity_gross = 0
        let reward_infos = create_reward_infos(500);

        let update = next_tick_modify_liquidity_update(
            &tick,
            100,  // tick_index
            200,  // tick_current_index (price ABOVE tick)
            1000, // fee_growth_global_a
            2000, // fee_growth_global_b
            &reward_infos,
            5000,  // liquidity_delta
            false, // is_upper_tick
        )
        .unwrap();

        assert_eq!(update.initialized, true);
        assert_eq!(update.liquidity_net, 5000);
        assert_eq!(update.liquidity_gross, 5000);
        assert_eq!(update.fee_growth_outside_a, 1000); // set to global
        assert_eq!(update.fee_growth_outside_b, 2000); // set to global
        assert_eq!(update.reward_growths_outside, [500, 500, 500]);
    }

    // Test 4: Initialize tick when price < tick_index (growths set to zero)
    #[test]
    fn test_initialize_tick_price_below() {
        let tick = Tick::default();
        let reward_infos = create_reward_infos(500);

        let update = next_tick_modify_liquidity_update(
            &tick,
            100, // tick_index
            50,  // tick_current_index (price BELOW tick)
            1000,
            2000,
            &reward_infos,
            5000,
            false,
        )
        .unwrap();

        assert_eq!(update.fee_growth_outside_a, 0);
        assert_eq!(update.fee_growth_outside_b, 0);
        assert_eq!(update.reward_growths_outside, [0, 0, 0]);
    }

    // Test 5: Upper tick subtracts from liquidity_net
    #[test]
    fn test_upper_tick_subtracts_liquidity_net() {
        let tick = Tick {
            initialized: true,
            liquidity_net: 10000,
            liquidity_gross: 10000,
            ..Default::default()
        };
        let reward_infos = create_reward_infos(0);

        let update = next_tick_modify_liquidity_update(
            &tick,
            100,
            50,
            0,
            0,
            &reward_infos,
            3000,
            true, // is_upper_tick = true
        )
        .unwrap();

        assert_eq!(update.liquidity_net, 7000); // 10000 - 3000
        assert_eq!(update.liquidity_gross, 13000); // 10000 + 3000
    }

    // Test 6: Lower tick adds to liquidity_net
    #[test]
    fn test_lower_tick_adds_liquidity_net() {
        let tick = Tick {
            initialized: true,
            liquidity_net: 10000,
            liquidity_gross: 10000,
            ..Default::default()
        };
        let reward_infos = create_reward_infos(0);

        let update = next_tick_modify_liquidity_update(
            &tick,
            100,
            50,
            0,
            0,
            &reward_infos,
            3000,
            false, // is_upper_tick = false
        )
        .unwrap();

        assert_eq!(update.liquidity_net, 13000); // 10000 + 3000
        assert_eq!(update.liquidity_gross, 13000);
    }

    // Test: Tick crossing flips fee growth outside values
    #[test]
    fn test_tick_cross_flips_fee_growth() {
        let tick = Tick {
            initialized: true,
            liquidity_net: 5000,
            liquidity_gross: 5000,
            fee_growth_outside_a: 300,
            fee_growth_outside_b: 400,
            reward_growths_outside: [100, 200, 300],
        };
        let reward_infos = create_reward_infos(1000);
        let update = next_tick_cross_update(
            &tick,
            1000, // fee_growth_global_a
            1000, // fee_growth_global_b
            &reward_infos,
        );
        // Flip formula: new_outside = global - old_outside
        assert_eq!(update.fee_growth_outside_a, 700); // 1000 - 300
        assert_eq!(update.fee_growth_outside_b, 600); // 1000 - 400
        assert_eq!(update.reward_growths_outside, [900, 800, 700]); // 1000 - [100, 200, 300]

        // These should remain unchanged
        assert_eq!(update.initialized, true);
        assert_eq!(update.liquidity_net, 5000);
        assert_eq!(update.liquidity_gross, 5000);
    }

    // Test: Fee growth inside calculation - price in range
    #[test]
    fn test_fee_growth_inside_price_in_range() {
        let tick_lower = Tick {
            initialized: true,
            fee_growth_outside_a: 200,
            fee_growth_outside_b: 100,
            ..Default::default()
        };
        let tick_upper = Tick {
            initialized: true,
            fee_growth_outside_a: 300,
            fee_growth_outside_b: 150,
            ..Default::default()
        };
        // Price is inside the range: tick_lower(100) < current(150) < tick_upper(200)
        let (inside_a, inside_b) = next_fee_growths_inside(
            150, // tick_current_index (inside range)
            &tick_lower,
            100, // tick_lower_index
            &tick_upper,
            200,  // tick_upper_index
            1000, // fee_growth_global_a
            500,  // fee_growth_global_b
        );
        // inside = global - below - above
        // For A: 1000 - 200 (below, use directly) - 300 (above, use directly) = 500
        // For B: 500 - 100 - 150 = 250
        assert_eq!(inside_a, 500);
        assert_eq!(inside_b, 250);
    }
}
