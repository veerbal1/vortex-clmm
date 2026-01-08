use anchor_lang::prelude::*;

use crate::{constants::MAX_FEE_RATE, errors::VortexError};

#[account]
#[derive(InitSpace)]
pub struct FeeTier {
    pub whirlpools_config: Pubkey,
    pub tick_spacing: u16,
    pub default_fee_rate: u16,
}

impl FeeTier {
    pub const LEN: usize = 8 + Self::INIT_SPACE;

    pub fn initialize(
        &mut self,
        whirlpools_config: Pubkey,
        tick_spacing: u16,
        default_fee_rate: u16,
    ) -> Result<()> {
        require!(
            default_fee_rate <= MAX_FEE_RATE,
            VortexError::InvalidFeeRate
        );
        require!(tick_spacing > 0, VortexError::InvalidTickSpacing);
        self.whirlpools_config = whirlpools_config;
        self.tick_spacing = tick_spacing;
        self.default_fee_rate = default_fee_rate;
        Ok(())
    }

    pub fn update_default_fee_rate(&mut self, default_fee_rate: u16) -> Result<()> {
        require!(
            default_fee_rate <= MAX_FEE_RATE,
            VortexError::InvalidFeeRate
        );
        self.default_fee_rate = default_fee_rate;
        Ok(())
    }
}
