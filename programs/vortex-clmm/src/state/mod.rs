pub mod config;
pub mod fee_tier;
pub mod oracle;
pub mod position;
pub mod tick;
pub mod tick_array;
pub mod whirlpool;

pub use config::*;
pub use fee_tier::*;
pub use oracle::*;
pub use position::*;
pub use tick::*;
pub use tick_array::*;
pub use whirlpool::*;

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::*;
    use anchor_lang::Space;

    #[test]
    fn test_tick_size() {
        assert_eq!(Tick::LEN, 113);
    }

    #[test]
    fn test_whirlpools_config_size() {
        assert_eq!(WhirlpoolsConfig::LEN, 108);
    }

    #[test]
    fn test_fee_tier_size() {
        assert_eq!(FeeTier::LEN, 44);
    }

    #[test]
    fn test_position_size() {
        assert_eq!(Position::LEN, 216);
    }

    #[test]
    fn test_whirlpool_size() {
        let expected = 8 + Whirlpool::INIT_SPACE;
        assert_eq!(Whirlpool::LEN, expected);
    }

    #[test]
    fn test_tick_array_size() {
        let expected = 8 + 32 + 4 + (88 * 113);
        assert_eq!(TickArray::LEN, expected);
    }

    #[test]
    fn test_oracle_size() {
        let expected = 8 + Oracle::INIT_SPACE;
        assert_eq!(Oracle::LEN, expected);
    }

    #[test]
    fn test_adaptive_fee_constants_size() {
        assert_eq!(AdaptiveFeeConstants::LEN, 34);
    }

    #[test]
    fn test_adaptive_fee_variables_size() {
        assert_eq!(AdaptiveFeeVariables::LEN, 44);
    }

    // Default value testing
    #[test]
    fn test_tick_default() {
        let tick = Tick::default();
        assert!(!tick.initialized);
        assert_eq!(tick.liquidity_net, 0);
        assert_eq!(tick.liquidity_gross, 0);
        assert_eq!(tick.fee_growth_outside_a, 0);
        assert_eq!(tick.fee_growth_outside_b, 0);
        assert_eq!(tick.reward_growths_outside, [0; 3]);
    }

    #[test]
    fn test_whirlpool_reward_info_default() {
        let info = WhirlpoolRewardInfo::default();
        assert_eq!(info.mint, Pubkey::default());
        assert_eq!(info.vault, Pubkey::default());
        assert_eq!(info.authority, Pubkey::default());
        assert_eq!(info.emissions_per_second_x64, 0);
        assert_eq!(info.growth_global_x64, 0);
    }
}
