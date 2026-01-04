pub const MIN_TICK_INDEX: i32 = -443636;

pub const MAX_TICK_INDEX: i32 = 443636;

pub const TICK_ARRAY_SIZE: usize = 88;

/// Maximum fee rate (100% = 1,000,000)
pub const MAX_FEE_RATE: u16 = 10000; // 1%

/// Fee rate denominator (basis points: 1,000,000 = 100%)
pub const FEE_RATE_DENOMINATOR: u32 = 1_000_000;

/// Maximum protocol fee rate (25% of trading fee)
pub const MAX_PROTOCOL_FEE_RATE: u16 = 2500;

/// Number of reward tokens per pool
pub const NUM_REWARDS: usize = 3;

/// Q64.64 scale factor (2^64)
pub const Q64_RESOLUTION: u8 = 64;

/// Minimum sqrt price (at MIN_TICK)
/// price = 1.0001^(-443636) ≈ 0.0000000000000000000000000000000000000029
/// sqrt(price) ≈ 0.000000000000000000054
/// ---
/// Multiply by 2^64 to get Q64.64 format:
// 0.000000000000000000054 × 2^64 ≈ 4295048016
pub const MIN_SQRT_PRICE: u128 = 4295048016;

/// Maximum sqrt price (at MAX_TICK)
pub const MAX_SQRT_PRICE: u128 = 79226673515401279992447579055;
