use super::tick::{Tick, NUM_REWARDS};
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
}
