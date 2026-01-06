/// Check if a tick is valid for the given tick spacing.
/// Valid ticks are multiples of tick_spacing.
/// Example: tick_spacing=10 → valid ticks are ..., -20, -10, 0, 10, 20, ...
pub fn is_valid_tick(tick: i32, tick_spacing: u16) -> bool {
    tick % tick_spacing as i32 == 0
}

/// Find the start_tick_index of the TickArray containing the given tick.
///
/// Each TickArray holds 88 ticks. With tick_spacing, each array covers:
///   88 × tick_spacing tick indices
///
/// Examples (tick_spacing = 1):
///   tick 50   → start = 0    (array covers 0 to 87)
///   tick 100  → start = 88   (array covers 88 to 175)
///   tick -50  → start = -88  (array covers -88 to -1)
///   tick -100 → start = -176 (array covers -176 to -89)
///
/// Note: Rust integer division truncates toward zero, not toward negative infinity.
/// For negative ticks not on a boundary, we must subtract one array width to get
/// the correct (lower) start index.
pub fn get_start_tick_index(tick: i32, tick_spacing: u16) -> i32 {
    // Each TickArray covers this many tick indices
    let ticks_per_array = 88 * tick_spacing as i32;

    // Integer division gives us the array number, multiply back to get start
    let mut start = (tick / ticks_per_array) * ticks_per_array;

    // Fix for negative ticks: Rust division truncates toward zero, but we need
    // floor division (toward negative infinity). If tick is negative and not
    // exactly on a boundary, we're one array too high — subtract to fix.
    if tick < 0 && tick % ticks_per_array != 0 {
        start -= ticks_per_array;
    }

    start
}

/// Find the next valid tick in a given direction.
///
/// Valid ticks are multiples of tick_spacing. During swaps, we need to find
/// the next boundary we'll cross.
///
/// ## Parameters
/// - `tick`: Current tick index
/// - `tick_spacing`: Pool's tick spacing (valid ticks are multiples of this)
/// - `zero_for_one`: Swap direction (true = a→b = price down = go LEFT)
///
/// ## The 4 Cases
///
/// | Case | Tick | Direction | We Need | Example (spacing=10) |
/// |------|------|-----------|---------|---------------------|
/// | 1    | +25  | LEFT      | Floor   | 25 → 20             |
/// | 2    | -25  | LEFT      | Floor   | -25 → -30           |
/// | 3    | +25  | RIGHT     | Ceiling | 25 → 30             |
/// | 4    | -25  | RIGHT     | Ceiling | -25 → -20           |
///
/// ## Why Negative Ticks Need Special Handling
///
/// Rust's `/` truncates toward ZERO, not toward -∞.
/// - `-25 / 10 = -2` (Rust) vs `-3` (floor)
///
/// For LEFT direction: we fix by subtracting spacing if negative & not on boundary.
/// For RIGHT direction: negative remainder makes `tick - remainder` work correctly.
pub fn get_next_valid_tick(tick: i32, tick_spacing: u16, zero_for_one: bool) -> i32 {
    let spacing = tick_spacing as i32;

    if zero_for_one {
        // Going LEFT (price down): round DOWN (floor toward -∞)
        //
        // Case 1 (positive): 25 / 10 = 2, 2 * 10 = 20 ✓
        // Case 2 (negative): -25 / 10 = -2 (wrong!), need -3
        //                    Fix: -20 - 10 = -30 ✓
        let mut result = (tick / spacing) * spacing;
        if tick < 0 && tick % spacing != 0 {
            result -= spacing;
        }
        result
    } else {
        // Going RIGHT (price up): round UP (ceiling toward +∞)
        let remainder = tick % spacing;

        if remainder == 0 {
            // Already on a valid tick boundary
            tick
        } else if tick > 0 {
            // Case 3 (positive): add what's missing to reach next multiple
            // 25 + (10 - 5) = 25 + 5 = 30 ✓
            tick + (spacing - remainder)
        } else {
            // Case 4 (negative): remainder is negative, so subtracting adds
            // -25 - (-5) = -25 + 5 = -20 ✓
            tick - remainder
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================
    // is_valid_tick tests
    // ========================

    #[test]
    fn test_valid_tick_multiples() {
        assert!(is_valid_tick(0, 10));
        assert!(is_valid_tick(10, 10));
        assert!(is_valid_tick(100, 10));
        assert!(is_valid_tick(-10, 10));
        assert!(is_valid_tick(-100, 10));
    }

    #[test]
    fn test_invalid_tick_non_multiples() {
        assert!(!is_valid_tick(5, 10));
        assert!(!is_valid_tick(15, 10));
        assert!(!is_valid_tick(-5, 10));
        assert!(!is_valid_tick(-15, 10));
    }

    // ========================
    // get_start_tick_index tests
    // ========================

    #[test]
    fn test_start_index_positive_ticks() {
        // tick_spacing = 1, ticks_per_array = 88
        assert_eq!(get_start_tick_index(0, 1), 0);
        assert_eq!(get_start_tick_index(50, 1), 0);
        assert_eq!(get_start_tick_index(87, 1), 0);
        assert_eq!(get_start_tick_index(88, 1), 88);
        assert_eq!(get_start_tick_index(100, 1), 88);
        assert_eq!(get_start_tick_index(175, 1), 88);
        assert_eq!(get_start_tick_index(176, 1), 176);
    }

    #[test]
    fn test_start_index_negative_ticks() {
        // tick_spacing = 1, ticks_per_array = 88
        assert_eq!(get_start_tick_index(-1, 1), -88);
        assert_eq!(get_start_tick_index(-50, 1), -88);
        assert_eq!(get_start_tick_index(-88, 1), -88); // exactly on boundary
        assert_eq!(get_start_tick_index(-89, 1), -176);
        assert_eq!(get_start_tick_index(-100, 1), -176);
        assert_eq!(get_start_tick_index(-176, 1), -176); // exactly on boundary
    }

    #[test]
    fn test_start_index_with_tick_spacing() {
        // tick_spacing = 10, ticks_per_array = 880
        assert_eq!(get_start_tick_index(0, 10), 0);
        assert_eq!(get_start_tick_index(500, 10), 0);
        assert_eq!(get_start_tick_index(879, 10), 0);
        assert_eq!(get_start_tick_index(880, 10), 880);
        assert_eq!(get_start_tick_index(-1, 10), -880);
        assert_eq!(get_start_tick_index(-500, 10), -880);
    }

    // ========================
    // get_next_valid_tick tests
    // ========================

    // Case 1: Positive tick, going LEFT (zero_for_one = true)
    #[test]
    fn test_next_valid_tick_case1_positive_left() {
        assert_eq!(get_next_valid_tick(25, 10, true), 20);
        assert_eq!(get_next_valid_tick(29, 10, true), 20);
        assert_eq!(get_next_valid_tick(20, 10, true), 20); // already on boundary
        assert_eq!(get_next_valid_tick(1, 10, true), 0);
    }

    // Case 2: Negative tick, going LEFT (zero_for_one = true)
    #[test]
    fn test_next_valid_tick_case2_negative_left() {
        assert_eq!(get_next_valid_tick(-25, 10, true), -30);
        assert_eq!(get_next_valid_tick(-21, 10, true), -30);
        assert_eq!(get_next_valid_tick(-20, 10, true), -20); // already on boundary
        assert_eq!(get_next_valid_tick(-1, 10, true), -10);
    }

    // Case 3: Positive tick, going RIGHT (zero_for_one = false)
    #[test]
    fn test_next_valid_tick_case3_positive_right() {
        assert_eq!(get_next_valid_tick(25, 10, false), 30);
        assert_eq!(get_next_valid_tick(21, 10, false), 30);
        assert_eq!(get_next_valid_tick(20, 10, false), 20); // already on boundary
        assert_eq!(get_next_valid_tick(1, 10, false), 10);
    }

    // Case 4: Negative tick, going RIGHT (zero_for_one = false)
    #[test]
    fn test_next_valid_tick_case4_negative_right() {
        assert_eq!(get_next_valid_tick(-25, 10, false), -20);
        assert_eq!(get_next_valid_tick(-29, 10, false), -20);
        assert_eq!(get_next_valid_tick(-20, 10, false), -20); // already on boundary
        assert_eq!(get_next_valid_tick(-1, 10, false), 0);
    }

    // Zero tick
    #[test]
    fn test_next_valid_tick_zero() {
        assert_eq!(get_next_valid_tick(0, 10, true), 0);
        assert_eq!(get_next_valid_tick(0, 10, false), 0);
    }
}
