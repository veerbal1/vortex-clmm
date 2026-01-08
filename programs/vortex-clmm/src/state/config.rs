use anchor_lang::prelude::*;

use crate::{constants::MAX_PROTOCOL_FEE_RATE, errors::VortexError};

#[account]
#[derive(InitSpace)]
pub struct WhirlpoolsConfig {
    pub fee_authority: Pubkey,
    pub collect_protocol_fees_authority: Pubkey,
    pub reward_emissions_super_authority: Pubkey,
    pub default_protocol_fee_rate: u16,
    pub feature_flags: u16,
}

impl WhirlpoolsConfig {
    pub const LEN: usize = 8 + Self::INIT_SPACE;

    pub fn initialize(
        &mut self,
        fee_authority: Pubkey,
        collect_protocol_fees_authority: Pubkey,
        reward_emissions_super_authority: Pubkey,
        default_protocol_fee_rate: u16,
    ) -> Result<()> {
        require!(
            default_protocol_fee_rate <= MAX_PROTOCOL_FEE_RATE,
            VortexError::InvalidProtocolFeeRate
        );

        self.fee_authority = fee_authority;
        self.collect_protocol_fees_authority = collect_protocol_fees_authority;
        self.reward_emissions_super_authority = reward_emissions_super_authority;
        self.default_protocol_fee_rate = default_protocol_fee_rate;

        Ok(())
    }
}
