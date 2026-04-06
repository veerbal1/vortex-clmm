# Adaptive Fee System — Deep Dive

## Source Files

| File | Purpose |
|------|---------|
| `z_cache/whirlpools/rust-sdk/core/src/math/adaptive_fee.rs` | Core fee calculation engine (`FeeRateManager` enum, `compute_adaptive_fee_rate()`, `update_reference()`, `update_volatility_accumulator()`) |
| `z_cache/whirlpools/rust-sdk/core/src/constants/adaptive_fee.rs` | Constants: `VOLATILITY_ACCUMULATOR_SCALE_FACTOR`, `REDUCTION_FACTOR_DENOMINATOR`, `ADAPTIVE_FEE_CONTROL_FACTOR_DENOMINATOR`, `MAX_REFERENCE_AGE`, `FEE_RATE_HARD_LIMIT` |
| `z_cache/whirlpools/programs/whirlpool/src/state/oracle.rs` | On-chain `AdaptiveFeeConstants`, `AdaptiveFeeVariables`, `Oracle` account structs + validation |
| `z_cache/whirlpools/programs/whirlpool/src/state/adaptive_fee_tier.rs` | `AdaptiveFeeTier` account — template config for creating adaptive-fee pools |
| `z_cache/whirlpools/programs/whirlpool/src/manager/fee_rate_manager.rs` | On-chain version of `FeeRateManager` (mirrors SDK version) |
| `z_cache/whirlpools/programs/whirlpool/src/instructions/adaptive_fee/` | All instruction handlers (initialize tier, initialize pool, set presets, set authorities, etc.) |

---

## What Problem Does It Solve?

Static fees are one-size-fits-all:

- **Calm markets**: 0.3% fee works fine, traders are happy
- **Volatile markets**: LPs get wrecked by arbitrageurs who extract value faster than fees compensate

Adaptive fees **automatically raise fees when the market is volatile** and **lower them when it's calm**. LPs earn more protection during chaos; traders get cheaper fees during peace.

---

## The Core Idea: Volatility = Tick Groups Crossed

The system doesn't use external oracles or complex price volatility math. It measures volatility by a simple proxy:

> **How many tick groups has the price crossed since the last reference point?**

More tick groups crossed = more price movement = more volatility = higher fees.

---

## Total Fee Composition

```
TOTAL_FEE = base_fee_rate (static) + adaptive_fee_rate (dynamic)
```

- `base_fee_rate` — stored in `Whirlpool.fee_rate` (u16, hundredths of a basis point)
- `adaptive_fee_rate` — computed dynamically from volatility
- Both capped at `FEE_RATE_HARD_LIMIT = 100,000` (= 10%)

Fee rate unit: **hundredths of a basis point**. So 100 = 0.01%, 10,000 = 1%, 100,000 = 10%.

---

## The 7 Configuration Parameters (AdaptiveFeeConstants)

Stored on `AdaptiveFeeTier` account. Copied into pool's `Oracle` at pool creation.

### 1. `filter_period` (u16, seconds)

Time window to detect high-frequency trading. If two swaps happen within this window, the reference **doesn't update** — fees stay elevated.

- Must be ≥ 1
- **Purpose**: Protects LPs from sandwich attacks and MEV by keeping fees high during rapid-fire trading

### 2. `decay_period` (u16, seconds)

After this much time with no activity, volatility reference resets to zero (fees drop fully).

- Must be ≥ 1 AND > `filter_period`
- **Purpose**: Ensures fees eventually return to baseline during calm periods

### 3. `reduction_factor` (u16, 0–9,999)

How much the volatility decays when elapsed time is between `filter_period` and `decay_period`.

- Divided by `REDUCTION_FACTOR_DENOMINATOR` (10,000)
- Example: 5,000 means 50% reduction — volatility reference becomes half of current accumulator
- Must be < 10,000

### 4. `adaptive_fee_control_factor` (u32, 0–99,999)

The "aggressiveness knob" — how steeply fees scale with volatility squared.

- Divided by `ADAPTIVE_FEE_CONTROL_FACTOR_DENOMINATOR` (100,000)
- Example: 1,000 means 0.01 multiplier
- Higher value = fees rise faster with less volatility
- Must be < 100,000

### 5. `max_volatility_accumulator` (u32)

Hard cap on the volatility measurement. Prevents unbounded fee increases.

- Constraint: `max_volatility_accumulator × tick_group_size` must fit in u32

### 6. `tick_group_size` (u16)

Groups adjacent ticks together for volatility measurement.

- Must be > 0, must divide `tick_spacing` evenly, must be ≤ `tick_spacing`
- Tick group index = `floor(tick_index / tick_group_size)`
- Larger groups = less sensitivity to small price movements

### 7. `major_swap_threshold_ticks` (u16)

If a single swap moves price by this many ticks or more, it's classified as a "major swap."

- Must be > 0, must be ≤ `TICK_ARRAY_SIZE × tick_spacing`
- Resets the decay timer, preventing premature fee reduction after big moves

---

## The 5 Runtime Variables (AdaptiveFeeVariables)

Stored in the `Oracle` account. Updated on every swap.

### 1. `volatility_reference` (u32)

Decayed baseline volatility — the "memory" of past volatility.

- Starts at 0
- Updated during `update_reference()` based on decay logic

### 2. `volatility_accumulator` (u32)

Current volatility measurement. This is what drives the fee calculation.

- Formula: `reference + |current_tick_group - reference_tick_group| × 10,000`
- Capped at `max_volatility_accumulator`

### 3. `tick_group_index_reference` (i32)

The tick group where the reference was last set. Distance from this point drives the accumulator.

### 4. `last_reference_update_timestamp` (u64)

When the reference was last recalculated. Used for time-based decay logic.

### 5. `last_major_swap_timestamp` (u64)

When a large price movement last happened. Affects the decay timer.

---

## The Adaptive Fee Calculation Formula

```rust
// From: rust-sdk/core/src/math/adaptive_fee.rs:363-384

crossed = volatility_accumulator × tick_group_size

squared = crossed²      // u64

adaptive_fee = ceil(
    adaptive_fee_control_factor × squared
    /
    (ADAPTIVE_FEE_CONTROL_FACTOR_DENOMINATOR × VOLATILITY_ACCUMULATOR_SCALE_FACTOR²)
)
// = ceil(control_factor × crossed² / (100,000 × 100,000,000))

adaptive_fee = min(adaptive_fee, 100,000)   // hard cap at 10%

total_fee = min(base_fee + adaptive_fee, 100,000)
```

**Key insight**: Fees scale with the **SQUARE** of volatility. This is deliberate:
- Small price movements → barely any fee increase
- Large price movements → aggressive fee ramp-up
- It's a **quadratic response curve**

### Worked Example

Suppose:
- `volatility_accumulator` = 30,000 (3 tick groups crossed × 10,000 scale)
- `tick_group_size` = 8
- `adaptive_fee_control_factor` = 1,000

```
crossed = 30,000 × 8 = 240,000
squared = 240,000² = 57,600,000,000
fee = ceil(1,000 × 57,600,000,000 / (100,000 × 100,000,000))
    = ceil(57,600,000,000,000 / 10,000,000,000,000)
    = ceil(5.76)
    = 6     (hundredths of a bps = 0.0006%)
```

With higher volatility (e.g. 20 tick groups → accumulator = 200,000):
```
crossed = 200,000 × 8 = 1,600,000
squared = 1,600,000² = 2,560,000,000,000
fee = ceil(1,000 × 2,560,000,000,000 / 10,000,000,000,000)
    = ceil(256)
    = 256   (hundredths of a bps = 0.0256%)
```

The quadratic scaling: 6.67× more tick groups → 42× higher fee.

---

## The Reference Update Logic (The Brain of the System)

Called at the start of every swap via `update_reference()` (line 403 in adaptive_fee.rs).

```
age     = now - last_reference_update_timestamp
elapsed = now - max(last_reference_update_timestamp, last_major_swap_timestamp)

CASE 1: age > MAX_REFERENCE_AGE (3,600 seconds = 1 hour)
    → FULL RESET
    → volatility_reference = 0
    → tick_group_index_reference = current tick group
    → update timestamp
    Rationale: Safety valve. Fees must eventually return to baseline.

CASE 2: elapsed < filter_period
    → NO CHANGE
    → Keep current volatility_reference unchanged
    Rationale: High-frequency trading detected. Keep fees elevated
    to protect LPs from sandwich attacks.

CASE 3: filter_period ≤ elapsed < decay_period
    → PARTIAL DECAY
    → volatility_reference = accumulator × reduction_factor / 10,000
    → tick_group_index_reference = current tick group
    → update timestamp
    Rationale: Normal trading cadence. Reduce fees partially.

CASE 4: elapsed ≥ decay_period
    → FULL DECAY
    → volatility_reference = 0
    → tick_group_index_reference = current tick group
    → update timestamp
    Rationale: Long period of inactivity. Drop fees completely.
```

### Why `max(last_reference_update, last_major_swap)` for elapsed?

This is subtle and important. The elapsed time is measured from whichever happened most recently — the last reference update OR the last major swap. So if a huge swap just happened, even if the reference was updated a while ago, the system treats it as "recent activity" and won't decay the fees prematurely.

---

## The Volatility Accumulator Update

Called during every swap step as price crosses tick groups (line 388 in adaptive_fee.rs):

```rust
index_delta = |tick_group_index_reference - current_tick_group_index|

volatility_accumulator = min(
    volatility_reference + index_delta × VOLATILITY_ACCUMULATOR_SCALE_FACTOR,
    max_volatility_accumulator
)
```

The accumulator measures: **"How far has price moved from the reference point?"**

- Each tick group crossed adds 10,000 to the value (the scale factor prevents premature decay to 0 in integer math)
- It starts from `volatility_reference` (the decayed baseline), not from zero
- It's capped at `max_volatility_accumulator`

---

## Major Swap Detection

After a swap completes, the system checks if it was a "major swap" (line 447 in adaptive_fee.rs):

```rust
is_major = larger_sqrt_price >= smaller_sqrt_price × sqrt(1.0001^threshold_ticks)
```

This checks if the price moved by at least `major_swap_threshold_ticks` ticks. If yes, `last_major_swap_timestamp` is updated, which affects the decay timer (see reference update logic above).

**Purpose**: Prevents the fee from decaying too quickly after a large price move. Without this, an attacker could:
1. Make a huge swap (moving price far)
2. Wait just past the filter_period
3. Get cheap fees on the return swap despite high volatility

The major swap timestamp keeps fees elevated after significant price movements.

---

## The Skip Optimization (Core Tick Group Range)

A performance optimization in `FeeRateManager::new()` (line 65 in adaptive_fee.rs).

The system pre-computes: *"At what distance from the reference will the accumulator hit its max?"*

```
max_delta = ceil(
    (max_volatility_accumulator - volatility_reference) / VOLATILITY_ACCUMULATOR_SCALE_FACTOR
)

core_range = [
    tick_group_index_reference - max_delta,
    tick_group_index_reference + max_delta
]
```

**Outside this range**, the accumulator is guaranteed to be maxed out → the fee is always at maximum → no need to recalculate per tick group. The swap loop can **skip** the step-by-step fee calculation entirely.

This is exposed via `get_bounded_sqrt_price_target()` (line 268), which returns:
- A bounded sqrt_price and `skip = false` if within core range (must calculate step by step)
- A bounded sqrt_price and `skip = true` if outside core range (can skip ahead)

When `skip = true`, `advance_tick_group_after_skip()` is called instead of `advance_tick_group()`, which jumps directly to the correct tick group index.

---

## Anti-DoS Protection

### The Attack

An attacker could try to keep fees artificially high by repeatedly making "major swaps," which resets `last_major_swap_timestamp` and prevents decay.

### The Defense

`MAX_REFERENCE_AGE = 3,600 seconds (1 hour)` acts as a hard safety valve:

> If the reference hasn't been updated in over an hour, it forcibly resets volatility_reference to zero regardless of major swap activity.

This guarantees fees eventually return to baseline even under sustained attack.

---

## Account Architecture

```
WhirlpoolsConfig
  ├── FeeTier (static fees — simple: tick_spacing + default_fee_rate)
  │     └── Whirlpool (static fee pool)
  │
  └── AdaptiveFeeTier (dynamic fees — 7 params + authorities)
        └── Whirlpool (adaptive fee pool, fee_rate = base fee)
              └── Oracle (AdaptiveFeeConstants + AdaptiveFeeVariables)
```

### Authority Structure

- **`fee_authority`** (on WhirlpoolsConfig) — Creates/manages tiers, sets presets
- **`initialize_pool_authority`** (on AdaptiveFeeTier) — Who can create pools with this tier. If set to `Pubkey::default()`, it's permissionless
- **`delegated_fee_authority`** (on AdaptiveFeeTier) — Who can set individual pool base fee rates

---

## Instruction Handlers

### Adaptive Fee Specific (`instructions/adaptive_fee/`)

| Instruction | What It Does |
|-------------|-------------|
| `initialize_adaptive_fee_tier` | Creates new AdaptiveFeeTier with all 7 params + authorities |
| `initialize_pool_with_adaptive_fee` | Creates a Whirlpool using an AdaptiveFeeTier (sets up Oracle account) |
| `set_preset_adaptive_fee_constants` | Updates all 7 adaptive fee params on an existing tier |
| `set_default_base_fee_rate` | Updates the static base fee component for adaptive pools |
| `set_delegated_fee_authority` | Sets who can update fees on individual pools |
| `set_fee_rate_by_delegated_fee_authority` | Allows delegated authority to set a pool's base fee_rate |
| `set_initialize_pool_authority` | Sets who can create pools with this tier |

### General Fee Instructions

| Instruction | What It Does |
|-------------|-------------|
| `set_fee_rate` | Sets fee on any pool (via fee_authority) |
| `set_default_fee_rate` | Sets default fee on a static FeeTier |
| `set_protocol_fee_rate` | Sets the protocol's cut of trading fees |
| `collect_fees` | LPs collect accumulated trading fees |
| `collect_protocol_fees` | Protocol collects its fee share |
| `update_fees_and_rewards` | Recalculates pending fees/rewards for a position |

---

## Validation Constraints

From `AdaptiveFeeConstants::validate_constants()` (oracle.rs:59):

1. `filter_period` ≥ 1
2. `decay_period` ≥ 1 AND > `filter_period`
3. `reduction_factor` < 10,000
4. `adaptive_fee_control_factor` < 100,000
5. `max_volatility_accumulator × tick_group_size` ≤ u32::MAX (overflow prevention)
6. `tick_group_size` > 0, ≤ `tick_spacing`, and `tick_spacing % tick_group_size == 0`
7. `major_swap_threshold_ticks` > 0 and ≤ `TICK_ARRAY_SIZE × tick_spacing`

---

## FeeRateManager Lifecycle During a Swap

```
1. INIT (FeeRateManager::new)
   ├── Call update_reference() — time-based decay logic
   ├── Pre-compute core tick group range bounds (skip optimization)
   └── Return Adaptive or Static variant

2. SWAP LOOP (per tick-group boundary crossed)
   ├── get_bounded_sqrt_price_target() — should we skip?
   ├── update_volatility_accumulator() — recalculate accumulator
   ├── get_total_fee_rate() — compute base + adaptive fee
   └── advance_tick_group() or advance_tick_group_after_skip()

3. POST-SWAP
   └── update_major_swap_timestamp() — was this a major swap?
```

---

## Key Design Insights

1. **Dual system**: A pool is either static-fee or adaptive-fee, determined at creation by which tier initialized it

2. **Volatility proxy**: Not actual price volatility — just tick group crossing count. Simple, deterministic, on-chain friendly

3. **Quadratic response**: `fee ∝ volatility²` means small moves barely affect fees, but large moves ramp aggressively

4. **Time decay**: Three time zones — filter (keep fees high), decay (reduce partially), expired (reset fully)

5. **Anti-manipulation**: `MAX_REFERENCE_AGE` (1 hour) guarantees fees reset even under attack

6. **Scale factor (10,000)**: Prevents integer math from decaying small volatility values to zero prematurely. A single tick group crossed = 10,000, so a 50% reduction factor gives 5,000 (still meaningful)

7. **Skip optimization**: Pre-computes the "core range" where fees aren't maxed out, allowing the swap loop to skip step-by-step calculation outside that range
