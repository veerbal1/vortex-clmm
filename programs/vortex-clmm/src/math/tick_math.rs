use super::q64_64::U256;
use crate::constants::{MAX_TICK_INDEX, MIN_TICK_INDEX};
use crate::errors::VortexError;

pub const MAX_SQRT_PRICE_X64: u128 = 79226673515401279992447579055;
pub const MIN_SQRT_PRICE_X64: u128 = 4295048016;
// Constants for sqrt_price → tick conversion
const LOG_B_2_X32: i128 = 59543866431248;
const BIT_PRECISION: u32 = 14;
const LOG_B_P_ERR_MARGIN_LOWER_X64: i128 = 184467440737095516;
const LOG_B_P_ERR_MARGIN_UPPER_X64: i128 = 15793534762490258745;

/// Helper: multiply two u128s, shift right 96 bits (for Q96 math)
pub fn mul_shift_96(a: u128, b: u128) -> u128 {
    ((U256::from(a) * U256::from(b)) >> 96).as_u128()
}

/// Convert tick index to sqrt_price in Q64.64 format
/// Returns error if tick is outside valid bounds
pub fn get_sqrt_price_at_tick(tick: i32) -> Result<u128, VortexError> {
    if tick < MIN_TICK_INDEX || tick > MAX_TICK_INDEX {
        return Err(VortexError::InvalidTickIndex);
    }

    if tick >= 0 {
        Ok(get_sqrt_price_positive_tick(tick))
    } else {
        Ok(get_sqrt_price_negative_tick(tick))
    }
}

fn get_sqrt_price_positive_tick(tick: i32) -> u128 {
    let mut ratio = if tick & 1 != 0 {
        79232123823359799118286999567
    } else {
        79228162514264337593543950336
    };

    if tick & 2 != 0 {
        ratio = mul_shift_96(ratio, 79236085330515764027303304731);
    }

    if tick & 4 != 0 {
        ratio = mul_shift_96(ratio, 79244008939048815603706035061);
    }

    if tick & 8 != 0 {
        ratio = mul_shift_96(ratio, 79259858533276714757314932305);
    }

    if tick & 16 != 0 {
        ratio = mul_shift_96(ratio, 79291567232598584799939703904);
    }

    if tick & 32 != 0 {
        ratio = mul_shift_96(ratio, 79355022692464371645785046466);
    }

    if tick & 64 != 0 {
        ratio = mul_shift_96(ratio, 79482085999252804386437311141);
    }

    if tick & 128 != 0 {
        ratio = mul_shift_96(ratio, 79736823300114093921829183326);
    }

    if tick & 256 != 0 {
        ratio = mul_shift_96(ratio, 80248749790819932309965073892);
    }

    if tick & 512 != 0 {
        ratio = mul_shift_96(ratio, 81282483887344747381513967011);
    }

    if tick & 1024 != 0 {
        ratio = mul_shift_96(ratio, 83390072131320151908154831281);
    }

    if tick & 2048 != 0 {
        ratio = mul_shift_96(ratio, 87770609709833776024991924138);
    }

    if tick & 4096 != 0 {
        ratio = mul_shift_96(ratio, 97234110755111693312479820773);
    }

    if tick & 8192 != 0 {
        ratio = mul_shift_96(ratio, 119332217159966728226237229890);
    }

    if tick & 16384 != 0 {
        ratio = mul_shift_96(ratio, 179736315981702064433883588727);
    }

    if tick & 32768 != 0 {
        ratio = mul_shift_96(ratio, 407748233172238350107850275304);
    }

    if tick & 65536 != 0 {
        ratio = mul_shift_96(ratio, 2098478828474011932436660412517);
    }

    if tick & 131072 != 0 {
        ratio = mul_shift_96(ratio, 55581415166113811149459800483533);
    }

    if tick & 262144 != 0 {
        ratio = mul_shift_96(ratio, 38992368544603139932233054999993551);
    }

    ratio >> 32
}

fn get_sqrt_price_negative_tick(tick: i32) -> u128 {
    let abs_tick = tick.abs();

    // Start with reciprocal: 1/sqrt(1.0001^1) or 1.0 in Q64
    let mut ratio: u128 = if abs_tick & 1 != 0 {
        18445821805675392311 // 1/sqrt(1.0001^1) × 2^64
    } else {
        18446744073709551616 // 1.0 × 2^64 (exactly 2^64)
    };

    // Check each bit and multiply by reciprocal constant
    if abs_tick & 2 != 0 {
        ratio = (ratio * 18444899583751176498) >> 64;
    }
    if abs_tick & 4 != 0 {
        ratio = (ratio * 18443055278223354162) >> 64;
    }
    if abs_tick & 8 != 0 {
        ratio = (ratio * 18439367220385604838) >> 64;
    }
    if abs_tick & 16 != 0 {
        ratio = (ratio * 18431993317065449817) >> 64;
    }
    if abs_tick & 32 != 0 {
        ratio = (ratio * 18417254355718160513) >> 64;
    }
    if abs_tick & 64 != 0 {
        ratio = (ratio * 18387811781193591352) >> 64;
    }
    if abs_tick & 128 != 0 {
        ratio = (ratio * 18329067761203520168) >> 64;
    }
    if abs_tick & 256 != 0 {
        ratio = (ratio * 18212142134806087854) >> 64;
    }
    if abs_tick & 512 != 0 {
        ratio = (ratio * 17980523815641551639) >> 64;
    }
    if abs_tick & 1024 != 0 {
        ratio = (ratio * 17526086738831147013) >> 64;
    }
    if abs_tick & 2048 != 0 {
        ratio = (ratio * 16651378430235024244) >> 64;
    }
    if abs_tick & 4096 != 0 {
        ratio = (ratio * 15030750278693429944) >> 64;
    }
    if abs_tick & 8192 != 0 {
        ratio = (ratio * 12247334978882834399) >> 64;
    }
    if abs_tick & 16384 != 0 {
        ratio = (ratio * 8131365268884726200) >> 64;
    }
    if abs_tick & 32768 != 0 {
        ratio = (ratio * 3584323654723342297) >> 64;
    }
    if abs_tick & 65536 != 0 {
        ratio = (ratio * 696457651847595233) >> 64;
    }
    if abs_tick & 131072 != 0 {
        ratio = (ratio * 26294789957452057) >> 64;
    }
    if abs_tick & 262144 != 0 {
        ratio = (ratio * 37481735321082) >> 64;
    }

    ratio
}

/// Get Tick from Sqrt Price

/// Convert sqrt_price (Q64.64) to tick index
/// This function has been copied from ORCA codebase, I just know it gives tick index for a given sqrt price
/// I have yet to understand the inner working of it. e.g log math etc.
pub fn get_tick_at_sqrt_price(sqrt_price_x64: u128) -> Result<i32, VortexError> {
    if sqrt_price_x64 < MIN_SQRT_PRICE_X64 || sqrt_price_x64 > MAX_SQRT_PRICE_X64 {
        return Err(VortexError::InvalidSqrtPrice);
    }

    // Step 1: Find MSB (most significant bit) position
    // This gives us the integer part of log₂
    let msb: u32 = 128 - sqrt_price_x64.leading_zeros() - 1;

    // Convert to Q32 format and adjust for Q64.64 input
    // Our input is Q64.64, so we subtract 64 to get the actual log₂ integer part
    let log2p_integer_x32: i128 = ((msb as i128) - 64) << 32;

    // Step 2: Calculate fractional part of log₂ using iterative squaring
    // Normalize r to be in range [1, 2) by shifting to position 63
    let mut r: u128 = if msb >= 64 {
        sqrt_price_x64 >> (msb - 63)
    } else {
        sqrt_price_x64 << (63 - msb)
    };

    // Iterate to find fractional bits
    // Start with bit = 0.5 in Q64.64 format
    let mut bit: i128 = 0x8000_0000_0000_0000i128;
    let mut precision = 0;
    let mut log2p_fraction_x64: i128 = 0;

    while bit > 0 && precision < BIT_PRECISION {
        // Square r (this doubles the log value, revealing the next bit)
        r = r.wrapping_mul(r);
        // Check if r >= 2 by looking at bit 127
        let is_r_more_than_two = r >> 127;
        // Shift right by 63 + (1 if r >= 2) to normalize back to [1, 2)
        r >>= 63 + is_r_more_than_two;
        // If r was >= 2, add this bit to our fraction
        log2p_fraction_x64 += bit * (is_r_more_than_two as i128);
        // Move to next bit
        bit >>= 1;
        precision += 1;
    }

    // Step 3: Combine integer and fractional parts
    let log2p_fraction_x32 = log2p_fraction_x64 >> 32;
    let log2p_x32 = log2p_integer_x32 + log2p_fraction_x32;

    // Step 4: Convert from log₂ to log₁.₀₀₀₁ using change of base
    // tick = log2(sqrt_price) / log2(1.0001^0.5) = log2(sqrt_price) * LOG_B_2_X32
    let logbp_x64 = log2p_x32 * LOG_B_2_X32;

    // Step 5: Calculate tick_low and tick_high with error margins
    // The iterative approximation may be slightly off, so we check both bounds
    let tick_low: i32 = ((logbp_x64 - LOG_B_P_ERR_MARGIN_LOWER_X64) >> 64) as i32;
    let tick_high: i32 = ((logbp_x64 + LOG_B_P_ERR_MARGIN_UPPER_X64) >> 64) as i32;

    // Step 6: Verify which tick is correct
    if tick_low == tick_high {
        Ok(tick_low)
    } else {
        // Check if tick_high gives a sqrt_price <= our input
        let sqrt_price_at_tick_high = get_sqrt_price_at_tick(tick_high)?;
        if sqrt_price_at_tick_high <= sqrt_price_x64 {
            Ok(tick_high)
        } else {
            Ok(tick_low)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick_0_returns_one() {
        // tick 0 = price 1.0 = sqrt(1.0) = 1.0 in Q64.64 = 2^64
        let result = get_sqrt_price_at_tick(0).unwrap();
        assert_eq!(result, 18446744073709551616); // 2^64
    }

    #[test]
    fn test_positive_tick_1() {
        let result = get_sqrt_price_at_tick(1).unwrap();
        // sqrt(1.0001) * 2^64 ≈ 18447666387855959850
        assert_eq!(result, 18447666387855959850);
    }

    #[test]
    fn test_negative_tick_1() {
        let result = get_sqrt_price_at_tick(-1).unwrap();
        // 1/sqrt(1.0001) * 2^64 ≈ 18445821805675392311
        assert_eq!(result, 18445821805675392311);
    }

    #[test]
    fn test_max_tick_returns_max_sqrt_price() {
        let result = get_sqrt_price_at_tick(MAX_TICK_INDEX).unwrap();
        assert_eq!(result, MAX_SQRT_PRICE_X64);
    }

    #[test]
    fn test_min_tick_returns_min_sqrt_price() {
        let result = get_sqrt_price_at_tick(MIN_TICK_INDEX).unwrap();
        assert_eq!(result, MIN_SQRT_PRICE_X64);
    }

    #[test]
    fn test_tick_out_of_bounds_high() {
        let result = get_sqrt_price_at_tick(MAX_TICK_INDEX + 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_tick_out_of_bounds_low() {
        let result = get_sqrt_price_at_tick(MIN_TICK_INDEX - 1);
        assert!(result.is_err());
    }

    // ========================
    // get_tick_at_sqrt_price tests
    // ========================

    #[test]
    fn test_tick_at_sqrt_price_at_one() {
        // sqrt_price = 2^64 (1.0 in Q64.64) should give tick 0
        let sqrt_price_x64: u128 = 18446744073709551616; // 2^64
        let tick = get_tick_at_sqrt_price(sqrt_price_x64).unwrap();
        assert_eq!(tick, 0);
    }

    #[test]
    fn test_tick_at_max_sqrt_price() {
        let tick = get_tick_at_sqrt_price(MAX_SQRT_PRICE_X64).unwrap();
        assert_eq!(tick, MAX_TICK_INDEX);
    }

    #[test]
    fn test_tick_at_min_sqrt_price() {
        let tick = get_tick_at_sqrt_price(MIN_SQRT_PRICE_X64).unwrap();
        assert_eq!(tick, MIN_TICK_INDEX);
    }

    #[test]
    fn test_tick_at_sqrt_price_out_of_bounds_low() {
        let result = get_tick_at_sqrt_price(MIN_SQRT_PRICE_X64 - 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_tick_at_sqrt_price_out_of_bounds_high() {
        let result = get_tick_at_sqrt_price(MAX_SQRT_PRICE_X64 + 1);
        assert!(result.is_err());
    }

    // ========================
    // Roundtrip tests (most important!)
    // ========================

    #[test]
    fn test_roundtrip_tick_0() {
        let original_tick = 0;
        let sqrt_price = get_sqrt_price_at_tick(original_tick).unwrap();
        let recovered_tick = get_tick_at_sqrt_price(sqrt_price).unwrap();
        assert_eq!(original_tick, recovered_tick);
    }

    #[test]
    fn test_roundtrip_tick_positive() {
        for tick in [1, 10, 100, 1000, 10000, 100000, MAX_TICK_INDEX] {
            let sqrt_price = get_sqrt_price_at_tick(tick).unwrap();
            let recovered_tick = get_tick_at_sqrt_price(sqrt_price).unwrap();
            assert_eq!(tick, recovered_tick, "Roundtrip failed for tick {}", tick);
        }
    }

    #[test]
    fn test_roundtrip_tick_negative() {
        for tick in [-1, -10, -100, -1000, -10000, -100000, MIN_TICK_INDEX] {
            let sqrt_price = get_sqrt_price_at_tick(tick).unwrap();
            let recovered_tick = get_tick_at_sqrt_price(sqrt_price).unwrap();
            assert_eq!(tick, recovered_tick, "Roundtrip failed for tick {}", tick);
        }
    }

    #[test]
    fn test_sqrt_price_floor_behavior() {
        // When sqrt_price is between tick N and tick N+1,
        // get_tick_at_sqrt_price should return N (floor behavior)
        let tick = 100;
        let sqrt_price_at_tick = get_sqrt_price_at_tick(tick).unwrap();
        let sqrt_price_at_tick_plus_1 = get_sqrt_price_at_tick(tick + 1).unwrap();

        // sqrt_price slightly above tick 100 should still return 100
        let sqrt_price_between = sqrt_price_at_tick + 1;
        let result = get_tick_at_sqrt_price(sqrt_price_between).unwrap();
        assert_eq!(result, tick);

        // sqrt_price just below tick 101 should return 100
        let sqrt_price_just_below = sqrt_price_at_tick_plus_1 - 1;
        let result2 = get_tick_at_sqrt_price(sqrt_price_just_below).unwrap();
        assert_eq!(result2, tick);
    }
}
