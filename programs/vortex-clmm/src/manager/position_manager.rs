use crate::{
    constants::NUM_REWARDS,
    errors::VortexError,
    math::{add_liquidity_delta, q64_64},
    state::{Position, PositionRewardInfo, PositionUpdate},
};

pub fn next_position_modify_liquidity_update(
    position: &Position,
    liquidity_delta: i128,
    fee_growth_inside_a: u128,
    fee_growth_inside_b: u128,
    reward_growths_inside: &[u128; NUM_REWARDS],
) -> Result<PositionUpdate, VortexError> {
    let mut update = PositionUpdate::default();

    let growth_delta_a = fee_growth_inside_a.wrapping_sub(position.fee_growth_checkpoint_a);
    let fee_delta_a = q64_64::mul(position.liquidity, growth_delta_a) as u64;

    let growth_delta_b = fee_growth_inside_b.wrapping_sub(position.fee_growth_checkpoint_b);
    let fee_delta_b = q64_64::mul(position.liquidity, growth_delta_b) as u64;

    update.fee_growth_checkpoint_a = fee_growth_inside_a;
    update.fee_growth_checkpoint_b = fee_growth_inside_b;
    update.fee_owed_a = position.fee_owed_a.wrapping_add(fee_delta_a);
    update.fee_owed_b = position.fee_owed_b.wrapping_add(fee_delta_b);

    for i in 0..NUM_REWARDS {
        let reward_growth_inside = reward_growths_inside[i];
        let curr_reward_info = position.reward_infos[i];

        // Calculate delta
        let reward_growth_delta =
            reward_growth_inside.wrapping_sub(curr_reward_info.growth_inside_checkpoint);
        let amount_owed_delta = q64_64::mul(position.liquidity, reward_growth_delta) as u64;

        // Set on update
        update.reward_infos[i] = PositionRewardInfo {
            growth_inside_checkpoint: reward_growth_inside,
            amount_owed: curr_reward_info.amount_owed.wrapping_add(amount_owed_delta),
        };
    }

    update.liquidity = add_liquidity_delta(position.liquidity, liquidity_delta)?;
    Ok(update)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::q64_64::Q64_RESOLUTION;

    /// Helper to build a test position
    fn build_test_position(
        liquidity: u128,
        fee_growth_checkpoint_a: u128,
        fee_growth_checkpoint_b: u128,
        fee_owed_a: u64,
        fee_owed_b: u64,
        reward_infos: [PositionRewardInfo; NUM_REWARDS],
    ) -> Position {
        Position {
            whirlpool: anchor_lang::prelude::Pubkey::default(),
            position_mint: anchor_lang::prelude::Pubkey::default(),
            liquidity,
            tick_lower_index: -100,
            tick_upper_index: 100,
            fee_growth_checkpoint_a,
            fee_growth_checkpoint_b,
            fee_owed_a,
            fee_owed_b,
            reward_infos,
        }
    }

    #[test]
    fn test_add_liquidity_with_fee_growth() {
        // Position with 10,000 liquidity, checkpointed at 100 fee growth
        let position = build_test_position(
            10_000,                // liquidity
            100 << Q64_RESOLUTION, // fee_growth_checkpoint_a = 100
            100 << Q64_RESOLUTION, // fee_growth_checkpoint_b = 100
            0,                     // fee_owed_a
            0,                     // fee_owed_b
            [PositionRewardInfo::default(); NUM_REWARDS],
        );

        // Current fee growth is 120 for A, 250 for B
        // Fee earned = (120 - 100) * 10,000 = 200,000 for A
        // Fee earned = (250 - 100) * 10,000 = 1,500,000 for B
        let update = next_position_modify_liquidity_update(
            &position,
            5_000,                 // adding 5,000 liquidity
            120 << Q64_RESOLUTION, // fee_growth_inside_a
            250 << Q64_RESOLUTION, // fee_growth_inside_b
            &[0, 0, 0],
        )
        .unwrap();

        assert_eq!(update.liquidity, 15_000);
        assert_eq!(update.fee_growth_checkpoint_a, 120 << Q64_RESOLUTION);
        assert_eq!(update.fee_growth_checkpoint_b, 250 << Q64_RESOLUTION);
        assert_eq!(update.fee_owed_a, 200_000);
        assert_eq!(update.fee_owed_b, 1_500_000);
    }

    #[test]
    fn test_remove_liquidity_with_existing_fee_owed() {
        let position = build_test_position(
            10_000,
            100 << Q64_RESOLUTION,
            100 << Q64_RESOLUTION,
            50,  // existing fee_owed_a
            100, // existing fee_owed_b
            [PositionRewardInfo::default(); NUM_REWARDS],
        );

        let update = next_position_modify_liquidity_update(
            &position,
            -5_000, // removing 5,000 liquidity
            120 << Q64_RESOLUTION,
            250 << Q64_RESOLUTION,
            &[0, 0, 0],
        )
        .unwrap();

        assert_eq!(update.liquidity, 5_000);
        // New fees ADDED to existing owed amounts
        assert_eq!(update.fee_owed_a, 50 + 200_000);
        assert_eq!(update.fee_owed_b, 100 + 1_500_000);
    }

    #[test]
    fn test_reward_growth_calculation() {
        let position = build_test_position(
            2_500,
            0,
            0,
            0,
            0,
            [
                PositionRewardInfo {
                    growth_inside_checkpoint: 100 << Q64_RESOLUTION,
                    amount_owed: 50,
                },
                PositionRewardInfo {
                    growth_inside_checkpoint: 250 << Q64_RESOLUTION,
                    amount_owed: 100,
                },
                PositionRewardInfo {
                    growth_inside_checkpoint: 10 << Q64_RESOLUTION,
                    amount_owed: 0,
                },
            ],
        );

        // R0: (200 - 100) * 2500 = 250,000 + 50 = 250,050
        // R1: (500 - 250) * 2500 = 625,000 + 100 = 625,100
        // R2: (1000 - 10) * 2500 = 2,475,000 + 0 = 2,475,000
        let update = next_position_modify_liquidity_update(
            &position,
            0, // no liquidity change
            0,
            0,
            &[
                200 << Q64_RESOLUTION,
                500 << Q64_RESOLUTION,
                1000 << Q64_RESOLUTION,
            ],
        )
        .unwrap();

        assert_eq!(update.reward_infos[0].amount_owed, 250_050);
        assert_eq!(update.reward_infos[1].amount_owed, 625_100);
        assert_eq!(update.reward_infos[2].amount_owed, 2_475_000);
    }

    #[test]
    #[should_panic(expected = "LiquidityUnderflow")]
    fn test_liquidity_underflow() {
        let position = build_test_position(
            100,
            0,
            0,
            0,
            0,
            [PositionRewardInfo::default(); NUM_REWARDS],
        );

        // Try to remove more liquidity than exists
        next_position_modify_liquidity_update(&position, -200, 0, 0, &[0, 0, 0]).unwrap();
    }

    #[test]
    fn test_zero_liquidity_earns_no_fees() {
        // Position with 0 liquidity earns no fees
        let position =
            build_test_position(0, 0, 0, 0, 0, [PositionRewardInfo::default(); NUM_REWARDS]);

        let update = next_position_modify_liquidity_update(
            &position,
            1000,                   // adding liquidity
            1000 << Q64_RESOLUTION, // big fee growth
            1000 << Q64_RESOLUTION,
            &[1000 << Q64_RESOLUTION, 0, 0],
        )
        .unwrap();

        // No fees earned because liquidity was 0 during the growth
        assert_eq!(update.fee_owed_a, 0);
        assert_eq!(update.fee_owed_b, 0);
        assert_eq!(update.reward_infos[0].amount_owed, 0);
        // But checkpoints ARE updated
        assert_eq!(update.fee_growth_checkpoint_a, 1000 << Q64_RESOLUTION);
    }
}
