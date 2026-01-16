use crate::{errors::VortexError, math::q64_64::Q64_64, state::Whirlpool};

#[derive(Debug, Clone)]
pub struct SwapState {
    /// Amount remaining to be swapped (decreases as we go)
    pub amount_remaining: u64,

    /// Amount calculated (output for exact-in, input for exact-out)
    pub amount_calculated: u64,

    /// Current sqrt_price as we move through the swap
    pub sqrt_price: u128,

    /// Current tick index
    pub tick_index: i32,

    /// Active liquidity at current price
    pub liquidity: u128,

    /// Fee growth accumulated during this swap (token A or B depending on direction)
    pub fee_growth_global: u128,

    /// Protocol fees collected during this swap
    pub protocol_fee: u64,
}

pub struct SwapResult {
    /// Amount of input token consumed
    pub amount_in: u64,

    /// Amount of output token produced
    pub amount_out: u64,

    /// Final sqrt_price after swap
    pub next_sqrt_price: u128,

    /// Final tick index after swap
    pub next_tick_index: i32,

    /// Total fees collected (goes to LPs)
    pub total_fee: u64,

    /// Protocol's portion of fees
    pub protocol_fee: u64,
}

pub fn initialize_swap_state(
    whirlpool: &Whirlpool,
    amount: u64,
    sqrt_price_limit: u128,
    a_to_b: bool,
) -> SwapState {
    SwapState {
        amount_remaining: amount,
        amount_calculated: 0,
        sqrt_price: whirlpool.sqrt_price,
        tick_index: whirlpool.tick_current_index,
        liquidity: whirlpool.liquidity,
        fee_growth_global: 0,
        protocol_fee: 0,
    }
}
