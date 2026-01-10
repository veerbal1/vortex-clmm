use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default, Debug, PartialEq, InitSpace)]
pub struct AdaptiveFeeConstants {
    pub filter_period: u16,

    pub decay_period: u16,

    pub reduction_factor: u16,

    pub adaptive_fee_control_factor: u32,

    pub max_volatility_accumulator: u32,

    pub tick_group_size: u16,

    pub major_swap_threshold_ticks: u16,

    pub reserved: [u8; 16],
}

impl AdaptiveFeeConstants {
    pub const LEN: usize = AdaptiveFeeConstants::INIT_SPACE;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default, Debug, PartialEq, InitSpace)]
pub struct AdaptiveFeeVariables {
    pub last_reference_update_timestamp: u64,

    pub last_major_swap_timestamp: u64,

    pub volatility_reference: u32,

    pub tick_group_index_reference: i32,

    pub volatility_accumulator: u32,

    pub reserved: [u8; 16],
}

impl AdaptiveFeeVariables {
    pub const LEN: usize = AdaptiveFeeVariables::INIT_SPACE;
}

#[account]
#[derive(InitSpace)]
pub struct Oracle {
    pub whirlpool: Pubkey,
    pub trade_enable_timestamp: u64,
    pub adaptive_fee_constants: AdaptiveFeeConstants,
    pub adaptive_fee_variables: AdaptiveFeeVariables,
    // Reserved for future use
    pub reserved: [u8; 128],
}

impl Oracle {
    pub const LEN: usize = 8 + Oracle::INIT_SPACE;
}
