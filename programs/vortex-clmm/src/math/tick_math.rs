use super::q64_64::U256;
use crate::errors::VortexError;

pub const MAX_SQRT_PRICE_X64: u128 = 79226673515401279992447579055;
pub const MIN_SQRT_PRICE_X64: u128 = 4295048016;

/// Helper: multiply two u128s, shift right 96 bits (for Q96 math)
pub fn mul_shift_96(a: u128, b: u128) -> u128 {
    ((U256::from(a) * U256::from(b)) >> 96).as_u128()
}

pub fn get_sqrt_price_at_tick(tick: i32) -> Result<u128, VortexError> {
    // You'll implement this
    todo!()
}
