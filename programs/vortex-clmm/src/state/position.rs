use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Position {
    pub whirlpool: Pubkey,
    pub position_mint: Pubkey,
    pub liquidity: u128,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,

    pub fee_growth_checkpoint_a: u128,
    pub fee_owed_a: u64,
    pub fee_growth_checkpoint_b: u128,
    pub fee_owed_b: u64,

    pub reward_infos: [PositionRewardInfo; 3],
}

impl Position {
    pub const LEN: usize = 8 + Position::INIT_SPACE;
}

#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize, InitSpace)]
pub struct PositionRewardInfo {
    pub growth_inside_checkpoint: u128, // Q64.64
    pub amount_owed: u64,
}
