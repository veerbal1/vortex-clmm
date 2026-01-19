pub const VOLATILITY_ACCUMULATOR_SCALE_FACTOR: u32 = 45000;

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
}
