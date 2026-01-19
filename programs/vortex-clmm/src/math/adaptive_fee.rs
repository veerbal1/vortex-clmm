pub const VOLATILITY_ACCUMULATOR_SCALE_FACTOR: u32 = 45000;
pub const REDUCTION_FACTOR_DENOMINATOR: u32 = 10_000;
pub const MAX_REFERENCE_AGE: u64 = 86_400; // 24 hours

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReferenceUpdate {
    pub tick_group_index_reference: i32,
    pub volatility_reference: u32,
    pub last_reference_update_timestamp: u64,
}

pub fn get_tick_group_index(tick_index: i32, tick_group_size: u16) -> i32 {
    if tick_index < 0 && tick_index % tick_group_size as i32 != 0 {
        return (tick_index / tick_group_size as i32) - 1;
    }

    tick_index / tick_group_size as i32
}

pub fn update_volatility_accumulator(
    volatility_reference: u32,
    tick_group_index_reference: i32,
    current_tick_group_index: i32,
    max_volatility_accumulator: u32,
) -> u32 {
    let index_delta = (current_tick_group_index - tick_group_index_reference).unsigned_abs();

    let raw_accumulator = (volatility_reference as u64)
        + index_delta as u64 * VOLATILITY_ACCUMULATOR_SCALE_FACTOR as u64;

    std::cmp::min(raw_accumulator, max_volatility_accumulator as u64) as u32
}

pub fn update_reference(
    // Inputs describing "NOW"
    current_tick_group_index: i32,
    current_timestamp: u64,

    // Inputs describing "OLD STATE"
    last_reference_update_timestamp: u64,
    tick_group_index_reference: i32,
    volatility_reference: u32,

    // Inputs describing "INTENSITY"
    volatility_accumulator: u32,

    // Configuration
    filter_period: u16,
    decay_period: u16,
    reduction_factor: u16,
) -> ReferenceUpdate {
    // 1. Safety Check: If data is ancient (24h+), hard reset
    let reference_age = current_timestamp.saturating_sub(last_reference_update_timestamp);
    if reference_age > MAX_REFERENCE_AGE {
        return ReferenceUpdate {
            tick_group_index_reference: current_tick_group_index,
            volatility_reference: 0,
            last_reference_update_timestamp: current_timestamp,
        };
    };

    // 2. Calculate time passed
    let elapsed = current_timestamp.saturating_sub(last_reference_update_timestamp);

    if elapsed < filter_period as u64 {
        return ReferenceUpdate {
            tick_group_index_reference: tick_group_index_reference,
            volatility_reference: volatility_reference,
            last_reference_update_timestamp: last_reference_update_timestamp,
        };
    } else if elapsed < decay_period as u64 {
        let new_reference = (volatility_accumulator as u64) * (reduction_factor as u64)
            / (REDUCTION_FACTOR_DENOMINATOR as u64);

        return ReferenceUpdate {
            tick_group_index_reference: current_tick_group_index,
            volatility_reference: new_reference as u32,
            last_reference_update_timestamp: current_timestamp,
        };
    } else {
        return ReferenceUpdate {
            tick_group_index_reference: current_tick_group_index,
            volatility_reference: 0,
            last_reference_update_timestamp: current_timestamp,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tick_group_index() {
        assert_eq!(get_tick_group_index(0, 64), 0);
        assert_eq!(get_tick_group_index(63, 64), 0);
        assert_eq!(get_tick_group_index(64, 64), 1);
        assert_eq!(get_tick_group_index(-1, 64), -1);
        assert_eq!(get_tick_group_index(-64, 64), -1);
        assert_eq!(get_tick_group_index(-65, 64), -2);
        assert_eq!(get_tick_group_index(-129, 64), -3);
    }

    #[test]
    fn test_update_volatility_accumulator() {
        let max_cap = 1_000_000;
        // Scenario A: No move
        // 0 + |10 - 10| * 45000 = 0
        assert_eq!(update_volatility_accumulator(0, 10, 10, max_cap), 0);

        // Scenario B: Medium move (5 groups)
        // 0 + |15 - 10| * 45000 = 225,000
        assert_eq!(update_volatility_accumulator(0, 10, 15, max_cap), 225_000);

        // Scenario C: Stacked move (Continuing from B)
        // 225,000 + |20 - 15| * 45000 = 450,000
        assert_eq!(
            update_volatility_accumulator(225_000, 15, 20, max_cap),
            450_000
        );

        // Scenario D: Hitting the Cap
        // If we calculate 2,000,000 but cap is 1,000,000, we should get 1,000,000
        assert_eq!(update_volatility_accumulator(0, 0, 100, max_cap), max_cap);
    }

    #[test]
    fn test_noise_zone_keeps_state() {
        let filter = 10;
        let last_ts = 1000;
        let last_ref_tick = 50; // Group 50
        let last_ref_vol = 100_000;

        let current_tick = 55; // We moved to Group 55
        let current_ts = 1005; // 5 seconds later

        let res = update_reference(
            current_tick,
            current_ts,
            last_ts,
            last_ref_tick,
            last_ref_vol,
            200_000, // Accumulated volatility is high
            filter,
            600,
            5000,
        );

        assert_eq!(res.tick_group_index_reference, last_ref_tick as i32);
        assert_eq!(res.volatility_reference, last_ref_vol as u32);
        assert_eq!(res.last_reference_update_timestamp, last_ts);
    }

    #[test]
    fn test_filter_over_zone_keeps_state() {
        let filter = 10;
        let last_ts = 1000;
        let last_ref_tick = 50; // Group 50
        let last_ref_vol = 100_000;

        let current_tick = 55; // We moved to Group 55
        let current_ts = 1015; // 15 seconds later

        let res = update_reference(
            current_tick,
            current_ts,
            last_ts,
            last_ref_tick,
            last_ref_vol,
            300_000, // Accumulated volatility is high
            filter,
            600,
            5000,
        );

        assert_eq!(res.tick_group_index_reference, current_tick as i32);
        assert_eq!(res.volatility_reference, 150_000 as u32);
        assert_eq!(res.last_reference_update_timestamp, current_ts);
    }

    #[test]
    fn test_decay_over_zone_keeps_state() {
        let filter = 10;
        let last_ts = 1000;
        let last_ref_tick = 50; // Group 50
        let last_ref_vol = 100_000;

        let current_tick = 55; // We moved to Group 55
        let current_ts = 3000; // 15 seconds later

        let res = update_reference(
            current_tick,
            current_ts,
            last_ts,
            last_ref_tick,
            last_ref_vol,
            300_000, // Accumulated volatility is high
            filter,
            600,
            5000,
        );

        assert_eq!(res.tick_group_index_reference, current_tick as i32);
        assert_eq!(res.volatility_reference, 0 as u32);
        assert_eq!(res.last_reference_update_timestamp, current_ts);
    }
}
