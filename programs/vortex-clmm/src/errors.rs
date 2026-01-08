use anchor_lang::prelude::*;

#[error_code]
pub enum VortexError {
    #[msg("Tick index is out of bounds")]
    InvalidTickIndex,

    #[msg("Sqrt price is out of bounds")]
    InvalidSqrtPrice,

    #[msg("Liquidity overflow")]
    LiquidityOverflow,

    #[msg("Liquidity underflow")]
    LiquidityUnderflow,

    #[msg("Token amount overflow")]
    TokenAmountOverflow,

    #[msg("Invalid tick spacing")]
    InvalidTickSpacing,

    #[msg("Tick array not found")]
    TickArrayNotFound,

    #[msg("Position not found")]
    PositionNotFound,

    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,

    #[msg("Invalid fee rate")]
    InvalidFeeRate,

    #[msg("Invalid protocol fee rate")]
    InvalidProtocolFeeRate,

    #[msg("Tick array index out of bounds")]
    TickArrayIndexOutOfBounds,
}
