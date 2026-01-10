use anchor_lang::prelude::*;

use crate::constants::NUM_REWARDS;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default, Debug, PartialEq, InitSpace)]
pub struct WhirlpoolRewardInfo {
    pub mint: Pubkey,                   // Reward token mint
    pub vault: Pubkey,                  // Reward vault
    pub authority: Pubkey,              // Who can set emissions
    pub emissions_per_second_x64: u128, // Q64.64 emission rate
    pub growth_global_x64: u128,        // Q64.64 accumulator
}

impl WhirlpoolRewardInfo {
    pub fn to_reward_growths(
        reward_infos: &[WhirlpoolRewardInfo; NUM_REWARDS],
    ) -> [u128; NUM_REWARDS] {
        let mut reward_growths = [0u128; NUM_REWARDS];
        for i in 0..NUM_REWARDS {
            reward_growths[i] = reward_infos[i].growth_global_x64;
        }
        reward_growths
    }
}

#[account]
#[derive(InitSpace)]
pub struct Whirlpool {
    pub whirlpools_config: Pubkey,
    pub whirlpool_bump: u8,

    pub tick_spacing: u16,

    pub fee_rate: u16,
    pub protocol_fee_rate: u16,

    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,

    pub token_vault_a: Pubkey,
    pub token_vault_b: Pubkey,

    pub sqrt_price: u128,        // Q64.64 current price
    pub tick_current_index: i32, // Current tick where price is

    // Active liquidity at current price
    pub liquidity: u128,

    // Fee growth accumulators (Q64.64)
    pub fee_growth_global_a: u128, // Total fees earned per unit liquidity (Token A)
    pub fee_growth_global_b: u128, // Total fees earned per unit liquidity (Token B)

    // Uncollected protocol fees
    pub protocol_fee_owed_a: u64,
    pub protocol_fee_owed_b: u64,

    // Reward tracking
    pub reward_last_updated_timestamp: u64,
    pub reward_infos: [WhirlpoolRewardInfo; NUM_REWARDS],
}

impl Whirlpool {
    pub const LEN: usize = 8 + Whirlpool::INIT_SPACE;
}
