use super::tick::Tick;
use crate::errors::VortexError;
use anchor_lang::prelude::*;

pub const TICK_ARRAY_SIZE: usize = 88;

#[account(zero_copy(unsafe))]
#[repr(C, packed)]
#[derive(InitSpace)]
pub struct TickArray {
    pub whirlpool: Pubkey,
    pub start_tick_index: i32,
    pub ticks: [Tick; TICK_ARRAY_SIZE],
}

impl TickArray {
    pub const LEN: usize = 8 + TickArray::INIT_SPACE;

    /// Get the tick at a specific index (0-87)
    pub fn get_tick(&self, index: usize) -> Result<&Tick> {
        require!(
            index < TICK_ARRAY_SIZE,
            VortexError::TickArrayIndexOutOfBounds,
        );
        Ok(&self.ticks[index])
    }

    /// Get mutable tick at a specific index (0-87)  
    pub fn get_tick_mut(&mut self, index: usize) -> Result<&mut Tick> {
        require!(
            index < TICK_ARRAY_SIZE,
            VortexError::TickArrayIndexOutOfBounds,
        );
        Ok(&mut self.ticks[index])
    }

    /// Convert tick_index to array index
    pub fn tick_index_to_array_index(
        tick_index: i32,
        start_tick_index: i32,
        tick_spacing: u16,
    ) -> Result<usize> {
        // Calculate: (tick_index - start_tick_index) / tick_spacing
        // Check bounds
        let array_index = (tick_index - start_tick_index) / tick_spacing as i32;
        require!(
            array_index >= 0 && array_index < TICK_ARRAY_SIZE as i32,
            VortexError::TickArrayIndexOutOfBounds
        );
        Ok(array_index as usize)
    }
}
