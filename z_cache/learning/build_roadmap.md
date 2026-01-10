# BUILD ROADMAP: Vortex CLMM Protocol

> From zero to production-grade Concentrated Liquidity Market Maker
> Target: Portfolio piece for $200k+ Protocol Engineer role
> Reference: Orca Whirlpools (62 instructions, 17 account types)

---

## BUILD PHILOSOPHY

Each ring = ONE brush stroke. Thin layers. No jumping.

- **Ring completes when**: Code compiles, tests pass, feature works
- **Reference allowed**: Orca Whirlpools for patterns (not copy-paste)
- **Quality bar**: Production-ready from Ring 1

---

## ORCA FEATURE PARITY CHECKLIST

### Instructions (62 total in Orca)
- [ ] V1 Core Instructions (31)
- [ ] V1 Extended Instructions (7)
- [ ] Adaptive Fee Instructions (7)
- [ ] Token-2022 V2 Instructions (11)
- [ ] Token Badge Instructions (5)
- [ ] Migration/Utility Instructions (1)

### State Accounts (17 total)
- [ ] WhirlpoolsConfig
- [ ] WhirlpoolsConfigExtension
- [ ] Whirlpool
- [ ] FeeTier
- [ ] AdaptiveFeeTier
- [ ] TickArray (Fixed)
- [ ] DynamicTickArray
- [ ] Tick (embedded)
- [ ] Position
- [ ] PositionBundle
- [ ] LockConfig
- [ ] TokenBadge
- [ ] Oracle (embedded in pool)

### Key Features
- [ ] SPL Token support
- [ ] Token-2022 support (transfer hooks, fees, metadata)
- [ ] Metaplex metadata for positions
- [ ] Position bundling (256 positions per bundle)
- [ ] Position locking (permanent locks)
- [ ] Adaptive/dynamic fees
- [ ] TWAP Oracle
- [ ] Token badge system (access control)
- [ ] Feature flags system
- [ ] Multi-hop swaps

---

## PHASE 0: PROJECT FOUNDATION (Rings 1-5) ✅ COMPLETE

### Ring 1: Workspace Setup ✅
- [x] Create Anchor workspace: `vortex-clmm`
- [x] Configure Cargo.toml with proper dependencies
- [x] Set up program ID placeholder
- [x] Create folder structure: `/programs/vortex/src/{state,instructions,math,manager,errors,util}`
- [x] Verify `anchor build` works (empty program)
- **Deliverable**: Empty Anchor program that compiles ✅

### Ring 2: Error Foundation ✅
- [x] Create `errors.rs` with `VortexError` enum
- [x] Add first 10 core errors: `InvalidTickIndex`, `InvalidSqrtPrice`, `LiquidityOverflow`, `LiquidityUnderflow`, `TokenAmountOverflow`, `InvalidTickSpacing`, `TickArrayNotFound`, `PositionNotFound`, `InsufficientLiquidity`, `InvalidFeeRate`
- [x] Export from lib.rs
- **Deliverable**: Error types ready for use ✅

### Ring 3: Constants Foundation ✅
- [x] Create `constants.rs`
- [x] Define: `MIN_TICK_INDEX = -443636`, `MAX_TICK_INDEX = 443636`
- [x] Define: `TICK_ARRAY_SIZE = 88`
- [x] Define: `MAX_FEE_RATE = 10000` (100% in basis points)
- [x] Define: `FEE_RATE_DENOMINATOR = 1_000_000`
- [x] Define: `MAX_PROTOCOL_FEE_RATE = 2500` (25%)
- [x] Define: `NUM_REWARDS = 3`
- [x] Define: `MAX_SQRT_PRICE`, `MIN_SQRT_PRICE` (Q64.64 bounds)
- **Deliverable**: All protocol constants defined ✅

### Ring 4: Basic Type Aliases ✅
- [x] Create `types.rs`
- [x] Define `SqrtPrice = u128` (Q64.64)
- [x] Define `Liquidity = u128`
- [x] Define `TickIndex = i32`
- [x] Define `FeeRate = u16`
- [x] Define `Timestamp = i64`
- **Deliverable**: Type safety for core values ✅

### Ring 5: Test Infrastructure ✅
- [x] Set up `/tests/` folder
- [x] Create test utilities module
- [x] Create basic integration test file
- [x] Verify `anchor test` runs (no tests yet, just setup)
- **Deliverable**: Testing infrastructure ready ✅

---

## PHASE 1: MATH LIBRARY (Rings 6-20)

> The foundation. Every calculation depends on this. Must be bulletproof.

### Ring 6: Q64.64 Fixed-Point Basics ✅
- [x] Create `math/q64_64.rs`
- [x] Define `Q64_64_RESOLUTION = 64`
- [x] Implement `from_u64(value: u64) -> u128` (left shift 64)
- [x] Implement `to_u64(value: u128) -> u64` (right shift 64, with rounding option)
- [x] Unit tests for conversion
- **Deliverable**: Basic fixed-point conversion ✅

### Ring 7: Q64.64 Multiplication ✅
- [x] Implement `mul(a: u128, b: u128) -> u128` using U256 intermediate
- [x] Handle overflow protection
- [x] Implement `mul_round_up` variant
- [x] Unit tests with edge cases (near max values)
- **Deliverable**: Safe fixed-point multiplication ✅

### Ring 8: Q64.64 Division ✅
- [x] Implement `div(a: u128, b: u128) -> u128` using U256 intermediate
- [x] Handle division by zero
- [x] Implement `div_round_up` variant
- [x] Unit tests with edge cases
- **Deliverable**: Safe fixed-point division ✅

### Rings 9-12: U256 Math ✅ (SKIPPED - using `uint` crate)
- [x] U256 struct provided by `uint` crate via `construct_uint!`
- [x] Conversions: `U256::from()`, `.as_u128()`
- [x] Arithmetic: `*`, `/`, `+`, `-` operators
- [x] Shifts: `<<`, `>>` operators
- **Deliverable**: 256-bit support via external crate ✅

### Ring 13: Tick Math - tick_to_sqrt_price ✅
- [x] Create `math/tick_math.rs`
- [x] Implement `get_sqrt_price_at_tick(tick: i32) -> u128`
- [x] Use the 1.0001^tick formula with lookup table optimization
- [x] Handle negative ticks (reciprocal approach)
- [x] Validate tick is within bounds
- [x] Unit tests: tick 0 = 2^64, known tick values
- **Deliverable**: Tick to sqrt_price conversion ✅

### Ring 14: Tick Math - sqrt_price_to_tick ✅
- [x] Implement `get_tick_at_sqrt_price(sqrt_price: u128) -> i32`
- [x] Log₂ approximation via MSB + iterative squaring
- [x] Ensure round-down (floor) behavior with verification step
- [x] Unit tests: roundtrip with tick_to_sqrt_price (16 tests pass)
- **Deliverable**: sqrt_price to tick conversion ✅
- ⚠️ **Note**: Code copied from Orca Whirlpools — logarithm algorithm was complex and time-consuming; will study inner workings later

### Ring 15: Tick Math - Utilities ✅
- [x] Implement `get_next_valid_tick(tick: i32, tick_spacing: u16, zero_for_one: bool) -> i32`
- [x] Implement `is_valid_tick(tick: i32, tick_spacing: u16) -> bool`
- [x] Implement `get_start_tick_index(tick: i32, tick_spacing: u16) -> i32`
- [x] Unit tests (10 tests pass)
- **Deliverable**: Tick navigation helpers ✅

### Ring 16: Liquidity Math - Add/Sub ✅
- [x] Create `math/liquidity_math.rs`
- [x] Implement `add_liquidity(a: u128, b: u128) -> Result<u128>`
- [x] Implement `sub_liquidity(a: u128, b: u128) -> Result<u128>`
- [x] Overflow/underflow protection
- [x] Unit tests (4 tests pass)
- **Deliverable**: Safe liquidity arithmetic ✅

### Ring 17: Token Math - Amount from Liquidity (Token A) ✅
- [x] Create `math/token_math.rs`
- [x] Implement `get_amount_a_delta(sqrt_price_lower: u128, sqrt_price_upper: u128, liquidity: u128, round_up: bool) -> u64`
- [x] Formula: `L * (sqrt_upper - sqrt_lower) / (sqrt_lower * sqrt_upper)`
- [x] Uses q64_64 utilities for precision
- [x] Unit tests with known values
- **Deliverable**: Token A amount calculation ✅

### Ring 18: Token Math - Amount from Liquidity (Token B) ✅
- [x] Implement `get_amount_b_delta(sqrt_price_lower: u128, sqrt_price_upper: u128, liquidity: u128, round_up: bool) -> u64`
- [x] Formula: `L * (sqrt_upper - sqrt_lower)`
- [x] Unit tests
- **Deliverable**: Token B amount calculation ✅

### Ring 19: Token Math - Liquidity from Amounts ✅
- [x] Implement `get_liquidity_for_amounts(sqrt_price: u128, sqrt_price_lower: u128, sqrt_price_upper: u128, amount_a: u64, amount_b: u64) -> u128`
- [x] Handle three cases: price below, in range, above
- [x] Unit tests (6 tests including roundtrip)
- **Deliverable**: Liquidity calculation from token amounts ✅

### Type-Safe Q64_64 Refactoring ✅
- [x] Create `Q64_64` newtype struct with `from_raw()`, `from_encoded()`, `inner()`, `to_u64()`
- [x] Add `checked_mul()` and `checked_div()` with overflow protection
- [x] Refactor all token_math functions to accept/return `Q64_64`
- [x] Update all unit tests
- **Deliverable**: Compile-time type safety (improvement over Orca's plain u128 approach) ✅

### Ring 20: Swap Math - Core Step
- [x] Create `math/swap_math.rs`
- [x] Define `SwapStepResult` struct with Q64_64 types
- [x] **Ring 20a**: `get_next_sqrt_price_from_a()` in `token_math.rs` ✅
  - Input: sqrt_price, liquidity, amount_a, add (bool)
  - Formula: `new_price = price × L / (L + amount × price)`
  - Note: `add=true` when adding A (price down), `add=false` when removing A (price up)
- [x] **Ring 20b**: `get_next_sqrt_price_from_b()` in `token_math.rs` ✅
  - Input: sqrt_price, liquidity, amount_b, add (bool)
  - Formula: `new_price = price ± (amount / L)`
- [x] **Ring 20c**: Fee helper functions in `swap_math.rs` ✅
  - `apply_swap_fee(amount, fee_rate)` → amount after fee
  - `reverse_apply_swap_fee(amount, fee_rate)` → pre-fee amount
- [x] **Ring 20d**: Complete `compute_swap_step()` function ✅
  - Step 1: Calculate fee (using 20c)
  - Step 2: Amount after fee
  - Step 3: Calculate max swap in tick (using get_amount_delta)
  - Step 4: Check if reaches boundary
  - Step 5: Calculate sqrt_price_next (using 20a/20b)
  - Step 6: Calculate amount_in, amount_out
  - Step 7: Return SwapStepResult
- [x] **Ring 20e**: Unit tests with known swap scenarios ✅
  - 6 tests: fee helpers + compute_swap_step (a_to_b, b_to_a, within_tick)
- **Deliverable**: Single swap step calculation ✅

---

## PHASE 2: STATE STRUCTURES (Rings 21-39)

> Define every account. Memory layout matters.

### Ring 21: WhirlpoolsConfig State ✅
- [x] Create `state/config.rs`
- [x] Define `WhirlpoolsConfig` struct
- [x] Fields: `fee_authority`, `collect_protocol_fees_authority`, `reward_emissions_super_authority`, `default_protocol_fee_rate`, `feature_flags`
- [x] Implement `LEN` using `#[derive(InitSpace)]`
- [x] Add `initialize()` and update methods
- **Deliverable**: Config account structure ✅ — Completed Jan 8, 2026

### Ring 22: FeeTier State ✅
- [x] Create `state/fee_tier.rs`
- [x] Define `FeeTier` struct
- [x] Fields: `whirlpools_config`, `tick_spacing`, `default_fee_rate`
- [x] Implement `LEN` using `#[derive(InitSpace)]`
- [x] Add `initialize()` with tick_spacing > 0 validation and `update_default_fee_rate()`
- **Deliverable**: Fee tier account structure ✅ — Completed Jan 8, 2026

### Ring 23: Tick State ✅
- [x] Added `Tick` struct to `state/tick.rs`
- [x] Define `Tick` struct with `#[repr(C, packed)]` (NOT an account, embedded in TickArray)
- [x] Fields: `initialized`, `liquidity_net` (i128), `liquidity_gross` (u128), `fee_growth_outside_a` (u128), `fee_growth_outside_b` (u128), `reward_growths_outside` ([u128; 3])
- [x] Added `NUM_REWARDS = 3` constant, LEN = 113 bytes
- **Deliverable**: Tick data structure ✅ — Completed Jan 8, 2026

### Ring 24: TickArray State ✅
- [x] Create `state/tick_array.rs`
- [x] Define `TickArray` struct with `#[account(zero_copy(unsafe))]`
- [x] Fields: `whirlpool`, `start_tick_index`, `ticks: [Tick; 88]`
- [x] `TICK_ARRAY_SIZE = 88` constant, LEN using `InitSpace`
- [x] Implement `get_tick(index)` with bounds check
- [x] Implement `get_tick_mut(index)` with bounds check
- [x] Implement `tick_index_to_array_index()` with full bounds validation
- **Deliverable**: Tick array account structure ✅ — Completed Jan 8, 2026

### Ring 25: Whirlpool State - Core Fields ✅
- [x] Create `state/whirlpool.rs`
- [x] Define `Whirlpool` struct - Part 1
- [x] Fields: `whirlpools_config`, `bump`, `tick_spacing`
- [x] Fields: `token_mint_a`, `token_mint_b`, `token_vault_a`, `token_vault_b`
- [x] Fields: `fee_rate`, `protocol_fee_rate`
- **Deliverable**: Whirlpool core identity fields ✅ — Completed Jan 8, 2026

### Ring 26: Whirlpool State - Price Fields ✅
- [x] Add to `Whirlpool`: `sqrt_price` (u128), `tick_current_index` (i32)
- [x] Add: `liquidity` (u128) - active liquidity at current price
- [x] Add: `fee_growth_global_a` (u128), `fee_growth_global_b` (u128)
- [x] Add: `protocol_fee_owed_a` (u64), `protocol_fee_owed_b` (u64)
- **Deliverable**: Whirlpool price/fee state ✅ — Completed Jan 8, 2026

### Ring 27: Whirlpool State - Rewards ✅
- [x] Define `WhirlpoolRewardInfo` struct
- [x] Fields: `mint`, `vault`, `authority`, `emissions_per_second_x64`, `growth_global_x64`
- [x] Add to `Whirlpool`: `reward_infos: [WhirlpoolRewardInfo; 3]`
- [x] Add: `reward_last_updated_timestamp`
- [x] Calculate final `Whirlpool::LEN`
- **Deliverable**: Complete Whirlpool state ✅ — Completed Jan 8, 2026

### Ring 28: Position State - Core ✅
- [x] Create `state/position.rs`
- [x] Define `Position` struct
- [x] Fields: `whirlpool`, `position_mint`, `liquidity` (u128)
- [x] Fields: `tick_lower_index` (i32), `tick_upper_index` (i32)
- [x] Implement `DISCRIMINATOR`, `LEN`
- **Deliverable**: Position identity fields ✅ — Completed Jan 10, 2026

### Ring 29: Position State - Fees ✅
- [x] Add to `Position`: `fee_growth_checkpoint_a` (u128), `fee_growth_checkpoint_b` (u128)
- [x] Add: `fee_owed_a` (u64), `fee_owed_b` (u64)
- **Deliverable**: Position fee tracking ✅ — Completed Jan 10, 2026

### Ring 30: Position State - Rewards ✅
- [x] Define `PositionRewardInfo` struct
- [x] Fields: `growth_inside_checkpoint` (u128), `amount_owed` (u64)
- [x] Add to `Position`: `reward_infos: [PositionRewardInfo; 3]`
- [x] Calculate final `Position::LEN`
- **Deliverable**: Complete Position state ✅ — Completed Jan 10, 2026

### Ring 31: Position Bundle State — SKIPPED
- [~] Skipped — Advanced feature, most users use regular positions
- **Deliverable**: Position bundle for batch management (deferred)

### Ring 32: Oracle State (with Adaptive Fees) ✅ — ALIGNED WITH ORCA
> Orca's Oracle is NOT traditional TWAP — it's for Adaptive Fees that protect LPs during volatility

- [x] Create `state/oracle.rs`
- [x] Define `AdaptiveFeeConstants` struct (embedded in Oracle)
  - Fields: `filter_period`, `decay_period`, `reduction_factor`
  - Fields: `adaptive_fee_control_factor`, `max_volatility_accumulator`
  - Fields: `tick_group_size`, `major_swap_threshold_ticks`
  - Add 16-byte reserve
- [x] Define `AdaptiveFeeVariables` struct (embedded in Oracle)
  - Fields: `last_reference_update_timestamp`, `last_major_swap_timestamp`
  - Fields: `volatility_reference`, `tick_group_index_reference`, `volatility_accumulator`
  - Add 16-byte reserve
- [x] Define `Oracle` account struct
  - Fields: `whirlpool`, `trade_enable_timestamp`
  - Fields: `adaptive_fee_constants`, `adaptive_fee_variables`
  - Add 128-byte reserve
- [x] Implement `LEN`, validation methods
- **Deliverable**: Oracle with adaptive fee support ✅ — Completed Jan 10, 2026

### Ring 33: Config Extension State — SKIPPED (for future)
- [~] Skipped — Token-2022 advanced feature
- **Deliverable**: Config extension for Token-2022 features (deferred)

### Ring 34: Token Badge State — SKIPPED (for future)
- [~] Skipped — Access control feature, not core CLMM
- **Deliverable**: Token access control badge (deferred)

### Ring 35: Lock Config State — SKIPPED (for future)
- [~] Skipped — Position locking, advanced feature
- **Deliverable**: Position lock configuration (deferred)

### Ring 36: Dynamic Tick Array State — SKIPPED (for future)
- [~] Skipped — Optimization, fixed tick arrays work for MVP
- **Deliverable**: Space-efficient tick storage (deferred)

### Ring 37: Feature Flags — SKIPPED (for future)
- [~] Skipped — Can add when needed for Token Badge
- **Deliverable**: Feature toggle system (deferred)

### Ring 38: State Module Exports ✅
- [x] Create `state/mod.rs`
- [x] Export all state structs with `pub use module::*`
- [x] Centralized `NUM_REWARDS` constant in `constants.rs`
- **Deliverable**: Clean state module ✅ — Completed Jan 10, 2026

### Ring 39: State Serialization Tests ✅
- [x] 11 tests verifying `LEN` constants match expected sizes
- [x] Tests for: Tick, WhirlpoolsConfig, FeeTier, Position, Whirlpool, TickArray, Oracle
- [x] Default value tests for Tick and WhirlpoolRewardInfo
- **Deliverable**: Verified state structures ✅ — Completed Jan 10, 2026

---

## PHASE 3: MANAGER LAYER (Rings 40-54)

> Business logic. Managers orchestrate operations across state.

### Ring 40: Tick Manager - Initialization
- [ ] Create `manager/tick_manager.rs`
- [ ] Implement `initialize_tick(tick: &mut Tick)`
- [ ] Implement `deinitialize_tick(tick: &mut Tick) -> bool` (returns if actually deinitialized)
- [ ] Unit tests
- **Deliverable**: Tick lifecycle management

### Ring 41: Tick Manager - Update
- [ ] Implement `update_tick(tick: &mut Tick, liquidity_delta: i128, is_upper: bool) -> Result<bool>`
- [ ] Update `liquidity_net` (add if lower, subtract if upper)
- [ ] Update `liquidity_gross`
- [ ] Return whether tick should be initialized/deinitialized
- [ ] Unit tests
- **Deliverable**: Tick liquidity updates

### Ring 42: Tick Manager - Crossing
- [ ] Implement `cross_tick(tick: &mut Tick, fee_growth_global_a: u128, fee_growth_global_b: u128, reward_growths_global: &[u128; 3]) -> i128`
- [ ] Flip `fee_growth_outside` values
- [ ] Flip `reward_growths_outside` values
- [ ] Return `liquidity_net` for pool update
- [ ] Unit tests
- **Deliverable**: Tick crossing logic

### Ring 43: Tick Manager - Fee Growth
- [ ] Implement `get_fee_growth_inside(tick_lower: &Tick, tick_upper: &Tick, tick_current: i32, fee_growth_global_a: u128, fee_growth_global_b: u128) -> (u128, u128)`
- [ ] Handle three cases: current below, inside, above range
- [ ] Wrapping subtraction for accumulators
- [ ] Unit tests with various scenarios
- **Deliverable**: Fee growth calculation

### Ring 44: Tick Manager - Reward Growth
- [ ] Implement `get_reward_growth_inside(tick_lower: &Tick, tick_upper: &Tick, tick_current: i32, reward_growths_global: &[u128; 3]) -> [u128; 3]`
- [ ] Same logic as fees, for each reward
- [ ] Unit tests
- **Deliverable**: Reward growth calculation

### Ring 45: Position Manager - Fee Calculation
- [ ] Create `manager/position_manager.rs`
- [ ] Implement `calculate_fee_owed(position: &Position, fee_growth_inside_a: u128, fee_growth_inside_b: u128) -> (u64, u64)`
- [ ] Formula: `(growth_inside - checkpoint) * liquidity`
- [ ] Handle accumulator wrapping
- [ ] Unit tests
- **Deliverable**: Position fee calculation

### Ring 46: Position Manager - Fee Update
- [ ] Implement `update_position_fees(position: &mut Position, fee_growth_inside_a: u128, fee_growth_inside_b: u128)`
- [ ] Calculate owed amounts
- [ ] Add to `fee_owed_a/b`
- [ ] Update checkpoints
- [ ] Unit tests
- **Deliverable**: Position fee updates

### Ring 47: Position Manager - Reward Calculation
- [ ] Implement `calculate_reward_owed(position: &Position, reward_index: usize, reward_growth_inside: u128) -> u64`
- [ ] Same pattern as fees
- [ ] Unit tests
- **Deliverable**: Position reward calculation

### Ring 48: Position Manager - Reward Update
- [ ] Implement `update_position_rewards(position: &mut Position, reward_growths_inside: &[u128; 3])`
- [ ] Update all three reward checkpoints and owed amounts
- [ ] Unit tests
- **Deliverable**: Position reward updates

### Ring 49: Liquidity Manager - Add
- [ ] Create `manager/liquidity_manager.rs`
- [ ] Implement `add_liquidity(whirlpool: &mut Whirlpool, position: &mut Position, tick_lower: &mut Tick, tick_upper: &mut Tick, liquidity_delta: u128) -> Result<(u64, u64)>`
- [ ] Update position liquidity
- [ ] Update tick liquidity_net/gross
- [ ] Update pool liquidity if in range
- [ ] Return required token amounts
- [ ] Unit tests
- **Deliverable**: Add liquidity orchestration

### Ring 50: Liquidity Manager - Remove
- [ ] Implement `remove_liquidity(whirlpool: &mut Whirlpool, position: &mut Position, tick_lower: &mut Tick, tick_upper: &mut Tick, liquidity_delta: u128) -> Result<(u64, u64)>`
- [ ] Reverse of add
- [ ] Return withdrawn token amounts
- [ ] Unit tests
- **Deliverable**: Remove liquidity orchestration

### Ring 51: Swap Manager - Core Loop Setup
- [ ] Create `manager/swap_manager.rs`
- [ ] Define `SwapState` struct: `amount_remaining`, `amount_calculated`, `sqrt_price`, `tick`, `liquidity`, `fee_growth_global`, `protocol_fee`
- [ ] Define `SwapResult` struct
- [ ] Implement `initialize_swap_state(whirlpool: &Whirlpool, amount: u64, sqrt_price_limit: u128, a_to_b: bool) -> SwapState`
- **Deliverable**: Swap state management

### Ring 52: Swap Manager - Step Execution
- [ ] Implement `execute_swap_step(state: &mut SwapState, tick_array: &TickArray, a_to_b: bool) -> Result<bool>`
- [ ] Find next initialized tick
- [ ] Calculate swap step using swap_math
- [ ] Update state
- [ ] Return whether swap is complete
- [ ] Unit tests
- **Deliverable**: Single swap step execution

### Ring 53: Swap Manager - Tick Crossing
- [ ] Implement `handle_tick_crossing(state: &mut SwapState, tick: &mut Tick, ...) -> Result<()>`
- [ ] Cross tick and update liquidity
- [ ] Update fee growth outside
- [ ] Unit tests
- **Deliverable**: Tick crossing during swap

### Ring 54: Swap Manager - Full Swap
- [ ] Implement `execute_swap(whirlpool: &mut Whirlpool, tick_arrays: &mut [TickArray], amount: u64, sqrt_price_limit: u128, a_to_b: bool, ...) -> Result<SwapResult>`
- [ ] Loop through swap steps
- [ ] Handle tick array transitions
- [ ] Update pool state
- [ ] Return final result
- [ ] Integration tests
- **Deliverable**: Complete swap execution

---

## PHASE 4: CORE INSTRUCTIONS (Rings 55-79)

> The instruction layer. What users actually call.

### Ring 55: Initialize Config Instruction
- [ ] Create `instructions/initialize_config.rs`
- [ ] Define `InitializeConfig` accounts struct
- [ ] Implement instruction handler
- [ ] Validate authorities
- [ ] Create PDA
- [ ] Unit test
- **Deliverable**: Config initialization

### Ring 56: Initialize Fee Tier Instruction
- [ ] Create `instructions/initialize_fee_tier.rs`
- [ ] Define accounts: config, fee_tier, funder, system_program
- [ ] Validate tick_spacing (must be > 0, reasonable max)
- [ ] Validate fee_rate
- [ ] Create PDA with config + tick_spacing seeds
- [ ] Unit test
- **Deliverable**: Fee tier initialization

### Ring 57: Initialize Pool Instruction
- [ ] Create `instructions/initialize_pool.rs`
- [ ] Define accounts: config, fee_tier, whirlpool, token_mint_a, token_mint_b, token_vault_a, token_vault_b, funder
- [ ] Validate token ordering (mint_a < mint_b lexicographically)
- [ ] Validate initial_sqrt_price within bounds
- [ ] Create token vaults (ATAs)
- [ ] Initialize pool state
- [ ] Unit test
- **Deliverable**: Pool initialization

### Ring 58: Initialize Tick Array Instruction
- [ ] Create `instructions/initialize_tick_array.rs`
- [ ] Define accounts: whirlpool, tick_array, funder
- [ ] Validate start_tick_index alignment
- [ ] Create PDA with whirlpool + start_tick_index seeds
- [ ] Initialize all 88 ticks to default
- [ ] Unit test
- **Deliverable**: Tick array initialization

### Ring 59: Open Position Instruction (Setup)
- [ ] Create `instructions/open_position.rs`
- [ ] Define accounts: whirlpool, position, position_mint, position_token_account, owner, funder
- [ ] Validate tick_lower < tick_upper
- [ ] Validate ticks aligned to tick_spacing
- [ ] Validate ticks within bounds
- **Deliverable**: Position validation

### Ring 60: Open Position Instruction (Execution)
- [ ] Create position NFT mint
- [ ] Create position token account
- [ ] Mint 1 NFT to owner
- [ ] Initialize position state (liquidity = 0)
- [ ] Set fee/reward checkpoints to current values
- [ ] Unit test
- **Deliverable**: Position creation

### Ring 61: Increase Liquidity Instruction (Setup)
- [ ] Create `instructions/increase_liquidity.rs`
- [ ] Define accounts: whirlpool, position, position_authority, tick_array_lower, tick_array_upper, token_vault_a, token_vault_b, token_owner_account_a, token_owner_account_b
- [ ] Validate position ownership
- [ ] Validate tick arrays contain position bounds
- **Deliverable**: Increase liquidity validation

### Ring 62: Increase Liquidity Instruction (Execution)
- [ ] Calculate required token amounts
- [ ] Transfer tokens from user to vaults
- [ ] Update position liquidity
- [ ] Update tick states
- [ ] Update pool liquidity if in range
- [ ] Update fee/reward checkpoints
- [ ] Unit test
- **Deliverable**: Liquidity addition

### Ring 63: Decrease Liquidity Instruction
- [ ] Create `instructions/decrease_liquidity.rs`
- [ ] Same account structure as increase
- [ ] Validate sufficient liquidity in position
- [ ] Calculate withdrawn token amounts
- [ ] Transfer tokens from vaults to user
- [ ] Update all states (reverse of increase)
- [ ] Unit test
- **Deliverable**: Liquidity removal

### Ring 64: Update Fees and Rewards Instruction
- [ ] Create `instructions/update_fees_and_rewards.rs`
- [ ] Define accounts: whirlpool, position, tick_array_lower, tick_array_upper
- [ ] Calculate fee growth inside range
- [ ] Calculate reward growth inside range
- [ ] Update position owed amounts
- [ ] Update checkpoints
- [ ] Unit test
- **Deliverable**: Fee/reward calculation

### Ring 65: Collect Fees Instruction
- [ ] Create `instructions/collect_fees.rs`
- [ ] Define accounts: whirlpool, position, position_authority, token_vault_a, token_vault_b, token_owner_account_a, token_owner_account_b
- [ ] Validate ownership
- [ ] Transfer owed fees from vaults
- [ ] Reset fee_owed to 0
- [ ] Unit test
- **Deliverable**: Fee collection

### Ring 66: Collect Reward Instruction
- [ ] Create `instructions/collect_reward.rs`
- [ ] Define accounts: whirlpool, position, position_authority, reward_vault, token_owner_account, reward_index
- [ ] Validate reward index (0-2)
- [ ] Transfer owed reward
- [ ] Reset reward amount_owed to 0
- [ ] Unit test
- **Deliverable**: Reward collection

### Ring 67: Close Position Instruction
- [ ] Create `instructions/close_position.rs`
- [ ] Validate liquidity = 0
- [ ] Validate fee_owed_a = 0, fee_owed_b = 0
- [ ] Validate all reward_owed = 0
- [ ] Burn position NFT
- [ ] Close position account (return rent)
- [ ] Close position mint account
- [ ] Unit test
- **Deliverable**: Position closure

### Ring 68: Swap Instruction (Setup)
- [ ] Create `instructions/swap.rs`
- [ ] Define accounts: whirlpool, token_vault_a, token_vault_b, token_owner_account_a, token_owner_account_b, tick_array_0, tick_array_1, tick_array_2, oracle (optional)
- [ ] Define params: amount, other_amount_threshold, sqrt_price_limit, amount_specified_is_input, a_to_b
- [ ] Validate sqrt_price_limit direction
- **Deliverable**: Swap validation

### Ring 69: Swap Instruction (Execution)
- [ ] Execute swap using swap_manager
- [ ] Update pool sqrt_price, tick_current_index, liquidity
- [ ] Update fee_growth_global
- [ ] Accumulate protocol fees
- [ ] Transfer tokens
- [ ] Validate slippage (other_amount_threshold)
- [ ] Unit test
- **Deliverable**: Core swap execution

### Ring 70: Two Hop Swap Instruction
- [ ] Create `instructions/two_hop_swap.rs`
- [ ] Define accounts for two pools
- [ ] Execute first swap
- [ ] Execute second swap with first swap's output
- [ ] Atomic (both succeed or both fail)
- [ ] Unit test
- **Deliverable**: Multi-hop routing

### Ring 71: Collect Protocol Fees Instruction
- [ ] Create `instructions/collect_protocol_fees.rs`
- [ ] Validate caller is collect_protocol_fees_authority
- [ ] Transfer protocol_fee_owed_a/b to authority
- [ ] Reset owed amounts
- [ ] Unit test
- **Deliverable**: Protocol fee collection

### Ring 72: Set Fee Rate Instruction
- [ ] Create `instructions/set_fee_rate.rs`
- [ ] Validate caller is fee_authority
- [ ] Validate new rate within bounds
- [ ] Update pool fee_rate
- [ ] Unit test
- **Deliverable**: Fee adjustment

### Ring 73: Set Protocol Fee Rate Instruction
- [ ] Create `instructions/set_protocol_fee_rate.rs`
- [ ] Validate caller is fee_authority
- [ ] Validate rate within MAX_PROTOCOL_FEE_RATE
- [ ] Update pool protocol_fee_rate
- [ ] Unit test
- **Deliverable**: Protocol fee adjustment

### Ring 74: Set Reward Emissions Instruction
- [ ] Create `instructions/set_reward_emissions.rs`
- [ ] Validate caller is reward_authority
- [ ] Validate reward index
- [ ] Update emissions_per_second_x64
- [ ] Update reward_last_updated_timestamp
- [ ] Checkpoint current growth
- [ ] Unit test
- **Deliverable**: Reward emission control

### Ring 75: Initialize Reward Instruction
- [ ] Create `instructions/initialize_reward.rs`
- [ ] Validate reward slot not already initialized
- [ ] Create reward vault
- [ ] Set reward mint and authority
- [ ] Unit test
- **Deliverable**: Reward program setup

### Ring 76: Set Reward Authority Instruction
- [ ] Create `instructions/set_reward_authority.rs`
- [ ] Validate caller is current authority
- [ ] Transfer authority to new address
- [ ] Unit test
- **Deliverable**: Reward authority transfer

### Ring 77: Set Fee Authority Instruction
- [ ] Create `instructions/set_fee_authority.rs`
- [ ] Validate caller is current fee_authority
- [ ] Transfer to new address
- [ ] Unit test
- **Deliverable**: Fee authority transfer

### Ring 78: Set Collect Protocol Fees Authority
- [ ] Create `instructions/set_collect_protocol_fees_authority.rs`
- [ ] Same pattern
- [ ] Unit test
- **Deliverable**: Protocol fees authority transfer

### Ring 79: Instruction Module Exports
- [ ] Create `instructions/mod.rs`
- [ ] Export all instructions
- [ ] Create `lib.rs` program entrypoint with all handlers
- [ ] Verify `anchor build` succeeds
- **Deliverable**: Complete instruction module

---

## PHASE 5: ADVANCED FEATURES (Rings 80-95)

### Ring 80: Position Bundle - Open Bundle
- [ ] Create `instructions/open_position_bundle.rs`
- [ ] Create bundle NFT
- [ ] Initialize bitmap to all zeros
- [ ] Unit test
- **Deliverable**: Bundle creation

### Ring 81: Position Bundle - Open Bundled Position
- [ ] Create `instructions/open_bundled_position.rs`
- [ ] Find first empty slot in bitmap
- [ ] Create position linked to bundle
- [ ] Set slot occupied
- [ ] Unit test
- **Deliverable**: Bundled position creation

### Ring 82: Position Bundle - Close Bundled Position
- [ ] Create `instructions/close_bundled_position.rs`
- [ ] Validate position empty
- [ ] Clear bitmap slot
- [ ] Close position account
- [ ] Unit test
- **Deliverable**: Bundled position closure

### Ring 83: Position Bundle - Close Bundle
- [ ] Create `instructions/close_position_bundle.rs`
- [ ] Validate all slots empty (bitmap = 0)
- [ ] Burn bundle NFT
- [ ] Return rent
- [ ] Unit test
- **Deliverable**: Bundle closure

### Ring 84: Oracle - Initialize
- [ ] Create `instructions/initialize_oracle.rs`
- [ ] Allocate observation array
- [ ] Initialize first observation with current tick
- [ ] Unit test
- **Deliverable**: Oracle initialization

### Ring 85: Oracle - Update Logic
- [ ] Create `manager/oracle_manager.rs`
- [ ] Implement `update_oracle(oracle: &mut Oracle, tick: i32, timestamp: i64)`
- [ ] Calculate tick_cumulative delta
- [ ] Write to circular buffer
- [ ] Update index
- [ ] Unit test
- **Deliverable**: Oracle update logic

### Ring 86: Oracle - Read TWAP
- [ ] Implement `get_twap(oracle: &Oracle, seconds_ago: u32) -> i32`
- [ ] Find relevant observations
- [ ] Calculate time-weighted average tick
- [ ] Handle edge cases (not enough history)
- [ ] Unit test
- **Deliverable**: TWAP calculation

### Ring 87: Oracle - Integration with Swap
- [ ] Modify swap instruction to update oracle
- [ ] Update after price change
- [ ] Integration test
- **Deliverable**: Oracle in swap flow

### Ring 88: Adaptive Fee - Initialize
- [ ] Create `instructions/initialize_adaptive_fee_tier.rs`
- [ ] Set all parameters
- [ ] Link to fee_tier
- [ ] Unit test
- **Deliverable**: Adaptive fee setup

### Ring 89: Adaptive Fee - Volatility Update
- [ ] Create `manager/adaptive_fee_manager.rs`
- [ ] Implement `update_volatility(state: &mut AdaptiveFeeTier, ticks_crossed: u32, timestamp: i64)`
- [ ] Accumulate volatility
- [ ] Apply decay
- [ ] Unit test
- **Deliverable**: Volatility tracking

### Ring 90: Adaptive Fee - Current Fee Calculation
- [ ] Implement `get_current_fee_rate(state: &AdaptiveFeeTier) -> u16`
- [ ] Apply formula: base + (accumulator * factor)
- [ ] Cap at max_fee_rate
- [ ] Unit test
- **Deliverable**: Dynamic fee calculation

### Ring 91: Adaptive Fee - Integration with Swap
- [ ] Modify swap to use adaptive fee when configured
- [ ] Update volatility accumulator after swap
- [ ] Integration test
- **Deliverable**: Adaptive fees in swap

### Ring 92: Position Lock - Lock Position
- [ ] Create `instructions/lock_position.rs`
- [ ] Set lock_release_time on position
- [ ] Add lock_release_time field to Position struct
- [ ] Unit test
- **Deliverable**: Position locking

### Ring 93: Position Lock - Validation in Decrease/Close
- [ ] Modify decrease_liquidity to check lock
- [ ] Modify close_position to check lock
- [ ] Allow only if current_time >= lock_release_time
- [ ] Integration test
- **Deliverable**: Lock enforcement

### Ring 94: Reset Position Range
- [ ] Create `instructions/reset_position_range.rs`
- [ ] Validate position unlocked
- [ ] Validate liquidity = 0
- [ ] Allow changing tick_lower/tick_upper
- [ ] Reset checkpoints
- [ ] Unit test
- **Deliverable**: Position range reset

### Ring 95: Transfer Locked Position
- [ ] Create `instructions/transfer_locked_position.rs`
- [ ] Allow transfer of locked positions
- [ ] Update position_owner in LockConfig
- [ ] Validate lock is active
- [ ] Unit test
- **Deliverable**: Locked position transfers

---

## PHASE 6: TOKEN-2022 SUPPORT (Rings 96-109)

> Full Token Extensions support - critical for modern tokens

### Ring 96: Token-2022 Utility Module
- [ ] Create `util/token_2022.rs`
- [ ] Implement `is_token_2022(mint: &AccountInfo) -> bool`
- [ ] Implement `get_token_program_id(mint: &AccountInfo) -> Pubkey`
- [ ] Handle transfer hooks detection
- [ ] Handle transfer fees detection
- **Deliverable**: Token-2022 detection utilities

### Ring 97: Remaining Accounts Handler
- [ ] Create `util/remaining_accounts.rs`
- [ ] Define `RemainingAccountsInfo` struct
- [ ] Implement parsing of remaining accounts for transfer hooks
- [ ] Handle supplemental tick arrays
- [ ] Unit tests
- **Deliverable**: Dynamic account handling

### Ring 98: Transfer with Hook Support
- [ ] Create `util/transfer.rs`
- [ ] Implement `transfer_from_owner_to_vault()` with hook support
- [ ] Implement `transfer_from_vault_to_owner()` with hook support
- [ ] Handle both SPL Token and Token-2022
- [ ] Handle transfer fees calculation
- [ ] Unit tests
- **Deliverable**: Universal transfer utility

### Ring 99: Initialize Pool V2
- [ ] Create `instructions/v2/initialize_pool_v2.rs`
- [ ] Support Token-2022 token mints
- [ ] Handle token extensions detection
- [ ] Create vaults with correct token program
- [ ] Unit test
- **Deliverable**: Pool init with Token-2022

### Ring 100: Increase Liquidity V2
- [ ] Create `instructions/v2/increase_liquidity_v2.rs`
- [ ] Support remaining accounts for transfer hooks
- [ ] Handle transfer fees on deposit
- [ ] Calculate correct amounts after fees
- [ ] Unit test
- **Deliverable**: Add liquidity with Token-2022

### Ring 101: Decrease Liquidity V2
- [ ] Create `instructions/v2/decrease_liquidity_v2.rs`
- [ ] Support remaining accounts for transfer hooks
- [ ] Handle transfer fees on withdrawal
- [ ] Unit test
- **Deliverable**: Remove liquidity with Token-2022

### Ring 102: Swap V2
- [ ] Create `instructions/v2/swap_v2.rs`
- [ ] Support Token-2022 for both tokens
- [ ] Handle transfer hooks on both sides
- [ ] Handle transfer fees
- [ ] Remaining accounts for hooks
- [ ] Unit test
- **Deliverable**: Swap with Token-2022

### Ring 103: Two Hop Swap V2
- [ ] Create `instructions/v2/two_hop_swap_v2.rs`
- [ ] Handle three tokens (A, intermediate, B)
- [ ] Support Token-2022 for all three
- [ ] Proper remaining accounts handling
- [ ] Unit test
- **Deliverable**: Multi-hop with Token-2022

### Ring 104: Collect Fees V2
- [ ] Create `instructions/v2/collect_fees_v2.rs`
- [ ] Support Token-2022 vaults
- [ ] Handle transfer fees on collection
- [ ] Unit test
- **Deliverable**: Fee collection with Token-2022

### Ring 105: Collect Protocol Fees V2
- [ ] Create `instructions/v2/collect_protocol_fees_v2.rs`
- [ ] Support Token-2022 vaults
- [ ] Unit test
- **Deliverable**: Protocol fees with Token-2022

### Ring 106: Collect Reward V2
- [ ] Create `instructions/v2/collect_reward_v2.rs`
- [ ] Support Token-2022 reward tokens
- [ ] Handle transfer hooks
- [ ] Unit test
- **Deliverable**: Reward collection with Token-2022

### Ring 107: Initialize Reward V2
- [ ] Create `instructions/v2/initialize_reward_v2.rs`
- [ ] Support Token-2022 reward mints
- [ ] Create vault with correct program
- [ ] Unit test
- **Deliverable**: Reward setup with Token-2022

### Ring 108: Set Reward Emissions V2
- [ ] Create `instructions/v2/set_reward_emissions_v2.rs`
- [ ] Handle Token-2022 reward tokens
- [ ] Unit test
- **Deliverable**: Emissions with Token-2022

### Ring 109: V2 Module Exports
- [ ] Create `instructions/v2/mod.rs`
- [ ] Export all V2 instructions
- [ ] Add to main program
- [ ] Integration tests
- **Deliverable**: Complete V2 instruction set

---

## PHASE 7: TOKEN BADGE SYSTEM (Rings 110-119)

> Access control for tokens - restrict which tokens can be used

### Ring 110: Initialize Config Extension
- [ ] Create `instructions/initialize_config_extension.rs`
- [ ] Create ConfigExtension PDA
- [ ] Set initial authorities
- [ ] Unit test
- **Deliverable**: Config extension initialization

### Ring 111: Set Config Extension Authority
- [ ] Create `instructions/set_config_extension_authority.rs`
- [ ] Validate current authority
- [ ] Transfer to new authority
- [ ] Unit test
- **Deliverable**: Extension authority transfer

### Ring 112: Set Token Badge Authority
- [ ] Create `instructions/set_token_badge_authority.rs`
- [ ] Validate current authority
- [ ] Transfer badge authority
- [ ] Unit test
- **Deliverable**: Badge authority transfer

### Ring 113: Initialize Token Badge
- [ ] Create `instructions/initialize_token_badge.rs`
- [ ] Create TokenBadge PDA for token mint
- [ ] Set initial attributes (non-transferable position)
- [ ] Unit test
- **Deliverable**: Token badge creation

### Ring 114: Delete Token Badge
- [ ] Create `instructions/delete_token_badge.rs`
- [ ] Validate authority
- [ ] Close badge account, return rent
- [ ] Unit test
- **Deliverable**: Token badge removal

### Ring 115: Set Token Badge Attribute
- [ ] Create `instructions/set_token_badge_attribute.rs`
- [ ] Update `require_non_transferable_position`
- [ ] Future: add more attributes
- [ ] Unit test
- **Deliverable**: Badge attribute updates

### Ring 116: Token Badge Validation in Pool Init
- [ ] Modify `initialize_pool` to check token badges
- [ ] Check feature flag is enabled
- [ ] Enforce badge requirements
- [ ] Integration test
- **Deliverable**: Badge enforcement in pools

### Ring 117: Token Badge Validation in Position Open
- [ ] Modify `open_position` to check badge
- [ ] Enforce non-transferable if required
- [ ] Open position with token extensions
- [ ] Integration test
- **Deliverable**: Badge enforcement in positions

### Ring 118: Set Config Feature Flag
- [ ] Create `instructions/set_config_feature_flag.rs`
- [ ] Enable/disable TOKEN_BADGE feature
- [ ] Validate authority
- [ ] Unit test
- **Deliverable**: Feature flag control

### Ring 119: Token Badge Integration Tests
- [ ] Test full badge lifecycle
- [ ] Test badge with pool creation
- [ ] Test non-transferable positions
- [ ] Test feature flag gating
- **Deliverable**: Badge system fully tested

---

## PHASE 8: METADATA SUPPORT (Rings 120-126)

> Metaplex metadata for positions - human-readable NFTs

### Ring 120: Metaplex Integration Setup
- [ ] Add `anchor-spl` metadata feature
- [ ] Create `util/metadata.rs`
- [ ] Define metadata constants (name, symbol, URI patterns)
- **Deliverable**: Metadata utilities

### Ring 121: Open Position With Metadata
- [ ] Create `instructions/open_position_with_metadata.rs`
- [ ] Create position mint
- [ ] Create Metaplex metadata account
- [ ] Set name: "Vortex Position"
- [ ] Set collection if applicable
- [ ] Unit test
- **Deliverable**: Position with Metaplex metadata

### Ring 122: Initialize Position Bundle With Metadata
- [ ] Create `instructions/initialize_position_bundle_with_metadata.rs`
- [ ] Create bundle mint
- [ ] Create Metaplex metadata
- [ ] Unit test
- **Deliverable**: Bundle with metadata

### Ring 123: Open Position With Token Extensions
- [ ] Create `instructions/open_position_with_token_extensions.rs`
- [ ] Use Token-2022 for position mint
- [ ] Add metadata extension (not Metaplex)
- [ ] Handle non-transferable extension if badge requires
- [ ] Unit test
- **Deliverable**: Position with Token-2022 metadata

### Ring 124: Close Position With Token Extensions
- [ ] Create `instructions/close_position_with_token_extensions.rs`
- [ ] Handle Token-2022 position NFT
- [ ] Burn with correct program
- [ ] Close mint account
- [ ] Unit test
- **Deliverable**: Token-2022 position closure

### Ring 125: Position Bumps Struct
- [ ] Create `OpenPositionBumps` struct
- [ ] Fields: `position_bump`, `metadata_bump` (optional)
- [ ] Use in position creation
- **Deliverable**: Clean bump handling

### Ring 126: Metadata Integration Tests
- [ ] Test position with Metaplex metadata
- [ ] Test position with Token-2022 metadata
- [ ] Test bundle with metadata
- [ ] Verify metadata content
- **Deliverable**: Metadata fully tested

---

## PHASE 9: ADAPTIVE FEE INSTRUCTIONS (Rings 127-136)

> Dynamic fees that respond to market volatility

### Ring 127: Initialize Adaptive Fee Tier
- [ ] Create `instructions/initialize_adaptive_fee_tier.rs`
- [ ] Set all volatility parameters
- [ ] Link to base fee tier
- [ ] Set authorities
- [ ] Unit test
- **Deliverable**: Adaptive tier creation

### Ring 128: Set Default Base Fee Rate
- [ ] Create `instructions/set_default_base_fee_rate.rs`
- [ ] Update AdaptiveFeeTier base rate
- [ ] Validate bounds
- [ ] Unit test
- **Deliverable**: Base fee updates

### Ring 129: Set Delegated Fee Authority
- [ ] Create `instructions/set_delegated_fee_authority.rs`
- [ ] Allow delegation of fee setting
- [ ] Validate current authority
- [ ] Unit test
- **Deliverable**: Fee delegation

### Ring 130: Set Initialize Pool Authority
- [ ] Create `instructions/set_initialize_pool_authority.rs`
- [ ] Control who can create pools with this tier
- [ ] Unit test
- **Deliverable**: Pool creation control

### Ring 131: Set Preset Adaptive Fee Constants
- [ ] Create `instructions/set_preset_adaptive_fee_constants.rs`
- [ ] Update: filter_period, decay_period, reduction_factor
- [ ] Update: adaptive_fee_control_factor, max_volatility_accumulator
- [ ] Update: tick_group_size, major_swap_threshold_ticks
- [ ] Validate all bounds
- [ ] Unit test
- **Deliverable**: Volatility parameter updates

### Ring 132: Initialize Pool With Adaptive Fee
- [ ] Create `instructions/initialize_pool_with_adaptive_fee.rs`
- [ ] Use AdaptiveFeeTier instead of FeeTier
- [ ] Initialize volatility accumulator to 0
- [ ] Unit test
- **Deliverable**: Adaptive fee pool creation

### Ring 133: Set Fee Rate By Delegated Authority
- [ ] Create `instructions/set_fee_rate_by_delegated_fee_authority.rs`
- [ ] Allow delegated authority to set fees
- [ ] Validate delegation
- [ ] Unit test
- **Deliverable**: Delegated fee setting

### Ring 134: Adaptive Fee Manager
- [ ] Create `manager/adaptive_fee_manager.rs`
- [ ] Implement volatility accumulation
- [ ] Implement decay calculation
- [ ] Implement fee calculation from accumulator
- [ ] Constants: VOLATILITY_ACCUMULATOR_SCALE_FACTOR = 10000
- [ ] Unit tests
- **Deliverable**: Adaptive fee logic

### Ring 135: Integrate Adaptive Fees in Swap
- [ ] Modify swap to detect adaptive fee pools
- [ ] Calculate current fee from accumulator
- [ ] Update volatility after swap
- [ ] Track ticks crossed
- [ ] Integration test
- **Deliverable**: Adaptive fees in swaps

### Ring 136: Adaptive Fee Integration Tests
- [ ] Test fee increases with volatility
- [ ] Test fee decay over time
- [ ] Test major swap threshold
- [ ] Test bounds enforcement
- **Deliverable**: Adaptive fees fully tested

---

## PHASE 10: ADDITIONAL AUTHORITY INSTRUCTIONS (Rings 137-144)

### Ring 137: Set Reward Authority By Super Authority
- [ ] Create `instructions/set_reward_authority_by_super_authority.rs`
- [ ] Allow super authority to override reward authority
- [ ] Validate super authority
- [ ] Unit test
- **Deliverable**: Super authority override

### Ring 138: Set Reward Emissions Super Authority
- [ ] Create `instructions/set_reward_emissions_super_authority.rs`
- [ ] Transfer super authority
- [ ] Validate current super authority
- [ ] Unit test
- **Deliverable**: Super authority transfer

### Ring 139: Set Default Fee Rate (Fee Tier)
- [ ] Create `instructions/set_default_fee_rate.rs`
- [ ] Update fee tier's default rate
- [ ] Validate fee authority
- [ ] Unit test
- **Deliverable**: Fee tier default update

### Ring 140: Set Default Protocol Fee Rate
- [ ] Create `instructions/set_default_protocol_fee_rate.rs`
- [ ] Update config's default protocol fee
- [ ] Validate authority
- [ ] Unit test
- **Deliverable**: Default protocol fee update

### Ring 141: Dynamic Tick Array Initialize
- [ ] Create `instructions/initialize_dynamic_tick_array.rs`
- [ ] Support idempotent flag (don't fail if exists)
- [ ] Variable size allocation
- [ ] Unit test
- **Deliverable**: Dynamic tick array creation

### Ring 142: Migration Instruction (Repurpose Reward Authority Space)
- [ ] Create `instructions/migrate_repurpose_reward_authority_space.rs`
- [ ] One-time migration helper
- [ ] Handle legacy account format
- [ ] Unit test
- **Deliverable**: Migration support

### Ring 143: IDL Include (For Complete IDL)
- [ ] Create `instructions/idl_include.rs`
- [ ] Include all types in IDL
- [ ] Ensure complete type coverage
- **Deliverable**: Complete IDL generation

### Ring 144: Authority Instruction Module
- [ ] Create `instructions/authority/mod.rs`
- [ ] Organize all authority instructions
- [ ] Clean exports
- **Deliverable**: Authority module complete

---

## PHASE 11: SECURITY HARDENING (Rings 145-156)

### Ring 145: Overflow Protection Audit
- [ ] Audit all arithmetic operations
- [ ] Ensure checked_add/checked_sub/checked_mul used
- [ ] Add overflow tests
- **Deliverable**: Overflow safety verified

### Ring 146: Access Control Audit
- [ ] Audit all authority checks
- [ ] Verify signer requirements
- [ ] Test unauthorized access attempts
- **Deliverable**: Access control verified

### Ring 147: Account Validation
- [ ] Verify all account constraints
- [ ] Check PDA derivations
- [ ] Ensure proper owner checks
- [ ] Test with malicious accounts
- **Deliverable**: Account safety verified

### Ring 148: Price Manipulation Protection
- [ ] Verify sqrt_price_limit enforced
- [ ] Test sandwich attack scenarios
- [ ] Document slippage recommendations
- **Deliverable**: Price manipulation mitigated

### Ring 149: Reentrancy Protection
- [ ] Audit for reentrancy vectors
- [ ] Ensure state updates before external calls
- [ ] Use checks-effects-interactions pattern
- **Deliverable**: Reentrancy safe

### Ring 150: Integer Precision Audit
- [ ] Verify rounding direction (favor protocol)
- [ ] Test edge cases at min/max values
- [ ] Ensure no precision loss in critical paths
- **Deliverable**: Precision verified

### Ring 151: Token Account Validation
- [ ] Verify token program checks
- [ ] Validate mint matches expected
- [ ] Check for token extensions compatibility
- **Deliverable**: Token safety verified

### Ring 152: Signer Validation
- [ ] Audit all is_signer checks
- [ ] Ensure proper authority hierarchy
- [ ] Test with incorrect signers
- **Deliverable**: Signer safety verified

### Ring 153: Rent Exemption
- [ ] Verify all accounts rent-exempt
- [ ] Test account closure returns rent
- [ ] Check minimum lamports requirements
- **Deliverable**: Rent handling verified

### Ring 154: CPI Safety Audit
- [ ] Verify all CPI targets are correct programs
- [ ] Check for CPI privilege escalation
- [ ] Validate account ownership after CPI
- **Deliverable**: CPI safety verified

### Ring 155: Token-2022 Security Audit
- [ ] Audit transfer hook handling
- [ ] Verify transfer fee calculations
- [ ] Test with malicious extensions
- **Deliverable**: Token-2022 safety verified

### Ring 156: Security Test Suite
- [ ] Create comprehensive security test file
- [ ] Test all edge cases
- [ ] Test all attack vectors
- [ ] Document security model
- **Deliverable**: Security test coverage

---

## PHASE 12: OPTIMIZATION (Rings 157-166)

### Ring 157: Compute Unit Profiling
- [ ] Profile each instruction's CU usage
- [ ] Identify hotspots
- [ ] Document baseline metrics
- **Deliverable**: CU baseline established

### Ring 158: Math Optimization
- [ ] Optimize hot paths in math modules
- [ ] Use lookup tables where beneficial
- [ ] Benchmark improvements
- **Deliverable**: Math optimized

### Ring 159: Account Size Optimization
- [ ] Review all struct sizes
- [ ] Pack fields efficiently
- [ ] Minimize rent costs
- **Deliverable**: Account sizes minimized

### Ring 160: Swap Loop Optimization
- [ ] Optimize tick crossing loop
- [ ] Minimize redundant calculations
- [ ] Cache values where possible
- **Deliverable**: Swap optimized

### Ring 161: Serialization Optimization
- [ ] Use zero-copy where beneficial
- [ ] Minimize borsh overhead
- [ ] Benchmark improvements
- **Deliverable**: Serialization optimized

### Ring 162: Memory Optimization
- [ ] Audit stack usage
- [ ] Avoid unnecessary allocations
- [ ] Use references where possible
- **Deliverable**: Memory usage optimized

### Ring 163: Cross-Program Invocation Optimization
- [ ] Minimize CPIs
- [ ] Batch token transfers where possible
- [ ] Optimize invoke_signed calls
- **Deliverable**: CPIs optimized

### Ring 164: Bit Math Optimization
- [ ] Implement MSB (most significant bit) fast path
- [ ] Optimize tick bitmap operations
- [ ] Use bitwise operations for bundle bitmap
- **Deliverable**: Bit operations optimized

### Ring 165: Sparse Swap Optimization
- [ ] Optimize for sparse tick arrays
- [ ] Skip empty regions efficiently
- [ ] Reduce iterations on low-liquidity pools
- **Deliverable**: Sparse swap optimized

### Ring 166: Final Performance Benchmark
- [ ] Full benchmark suite
- [ ] Compare to Orca Whirlpools
- [ ] Document performance characteristics
- **Deliverable**: Performance documented

---

## PHASE 13: COMPREHENSIVE TESTING (Rings 167-182)

### Ring 167: Unit Test Coverage - Math
- [ ] 100% coverage on math modules
- [ ] Edge case tests
- [ ] Property-based tests for invariants
- **Deliverable**: Math fully tested

### Ring 168: Unit Test Coverage - Managers
- [ ] Full coverage on manager modules
- [ ] State transition tests
- [ ] Error condition tests
- **Deliverable**: Managers fully tested

### Ring 169: Integration Tests - Happy Paths
- [ ] Full LP journey test
- [ ] Swap test with multiple tick crossings
- [ ] Multi-hop swap test
- **Deliverable**: Happy paths tested

### Ring 170: Integration Tests - Edge Cases
- [ ] Min/max tick positions
- [ ] Zero liquidity scenarios
- [ ] Price limit hit scenarios
- **Deliverable**: Edge cases tested

### Ring 171: Integration Tests - Rewards
- [ ] Reward emission test
- [ ] Multiple reward programs
- [ ] Reward collection timing
- **Deliverable**: Rewards tested

### Ring 172: Integration Tests - Bundles
- [ ] Full bundle lifecycle
- [ ] Multiple positions in bundle (256 max)
- [ ] Bundle transfer
- **Deliverable**: Bundles tested

### Ring 173: Integration Tests - Oracle
- [ ] Oracle update on swaps
- [ ] TWAP calculation accuracy
- [ ] Circular buffer behavior
- **Deliverable**: Oracle tested

### Ring 174: Integration Tests - Adaptive Fees
- [ ] Volatility accumulation
- [ ] Decay behavior
- [ ] Fee bounds
- **Deliverable**: Adaptive fees tested

### Ring 175: Integration Tests - Token-2022
- [ ] Pool with Token-2022 tokens
- [ ] Transfer hooks execution
- [ ] Transfer fees handling
- **Deliverable**: Token-2022 tested

### Ring 176: Integration Tests - Token Badge
- [ ] Badge creation and deletion
- [ ] Non-transferable positions
- [ ] Feature flag gating
- **Deliverable**: Token badge tested

### Ring 177: Integration Tests - Position Locks
- [ ] Lock creation
- [ ] Locked position transfers
- [ ] Unlock and range reset
- **Deliverable**: Position locks tested

### Ring 178: Integration Tests - Metadata
- [ ] Metaplex metadata positions
- [ ] Token-2022 metadata positions
- [ ] Bundle metadata
- **Deliverable**: Metadata tested

### Ring 179: Fuzz Testing Setup
- [ ] Set up Trident or similar framework
- [ ] Define fuzz targets
- [ ] Configure coverage
- **Deliverable**: Fuzzing infrastructure

### Ring 180: Fuzz Testing - Core Operations
- [ ] Fuzz swap amounts and directions
- [ ] Fuzz liquidity operations
- [ ] Fuzz tick indices
- **Deliverable**: Core fuzzing complete

### Ring 181: Fuzz Testing - Edge Cases
- [ ] Fuzz near boundary values
- [ ] Fuzz with adversarial inputs
- [ ] Run extended campaigns
- **Deliverable**: Edge case fuzzing complete

### Ring 182: Test Documentation
- [ ] Document test coverage (target >95%)
- [ ] Document test methodology
- [ ] Create test matrix
- **Deliverable**: Testing documented

---

## PHASE 14: PRODUCTION READY (Rings 183-204)

### Ring 183: Mainnet Configuration
- [ ] Create mainnet config templates
- [ ] Define production fee tiers
- [ ] Set authority addresses
- **Deliverable**: Mainnet config ready

### Ring 184: Upgrade Authority Strategy
- [ ] Document upgrade process
- [ ] Define upgrade authority multisig
- [ ] Plan for future upgrades
- [ ] Consider timelock mechanisms
- **Deliverable**: Upgrade strategy documented

### Ring 185: SDK Foundation
- [ ] Create TypeScript SDK project
- [ ] Set up build tooling (tsup/rollup)
- [ ] Define core types matching on-chain
- [ ] Implement account fetchers
- **Deliverable**: SDK foundation

### Ring 186: SDK - Account Deserialization
- [ ] Implement Whirlpool account parsing
- [ ] Implement Position account parsing
- [ ] Implement TickArray parsing
- [ ] Handle all account types
- **Deliverable**: Account parsing complete

### Ring 187: SDK - Core Instructions
- [ ] Implement pool initialization
- [ ] Implement position open/close
- [ ] Implement liquidity add/remove
- [ ] Implement swap
- **Deliverable**: Core instruction builders

### Ring 188: SDK - V2 Instructions
- [ ] Implement all V2 (Token-2022) instructions
- [ ] Handle remaining accounts
- [ ] Handle transfer hooks
- **Deliverable**: V2 instruction builders

### Ring 189: SDK - Math Library
- [ ] Port tick math to TypeScript
- [ ] Port token math
- [ ] Port swap math
- [ ] Use BigInt for precision
- **Deliverable**: SDK math complete

### Ring 190: SDK - Quote Functions
- [ ] Implement swap quote
- [ ] Implement liquidity quote
- [ ] Implement fee estimation
- **Deliverable**: Quote functions

### Ring 191: SDK - Position Helpers
- [ ] Implement position value calculation
- [ ] Implement fee accrued calculation
- [ ] Implement reward accrued calculation
- **Deliverable**: Position helpers

### Ring 192: SDK - Testing
- [ ] Unit tests for all SDK functions
- [ ] Integration tests against localnet
- [ ] E2E tests for common flows
- **Deliverable**: SDK fully tested

### Ring 193: Documentation - Architecture
- [ ] System architecture overview
- [ ] Account relationships diagram
- [ ] Instruction flow diagrams
- [ ] State machine documentation
- **Deliverable**: Architecture docs

### Ring 194: Documentation - API Reference
- [ ] Document all instructions
- [ ] Document all account structures
- [ ] Document all error codes
- [ ] Generate from IDL
- **Deliverable**: API reference

### Ring 195: Documentation - Integration Guide
- [ ] Quick start guide
- [ ] Common use cases
- [ ] Best practices
- [ ] Troubleshooting guide
- **Deliverable**: Integration guide

### Ring 196: Documentation - Security Model
- [ ] Document authority hierarchy
- [ ] Document access control
- [ ] Document trust assumptions
- [ ] Security best practices
- **Deliverable**: Security documentation

### Ring 197: Deployment - Localnet
- [ ] Create localnet setup script
- [ ] Deploy all programs
- [ ] Initialize test config
- [ ] Create demo pools
- **Deliverable**: Localnet deployment

### Ring 198: Deployment - Devnet
- [ ] Deploy to devnet
- [ ] Initialize production config
- [ ] Create test pools
- [ ] Verify all instructions
- **Deliverable**: Devnet deployment

### Ring 199: Deployment - Verification
- [ ] Verify program on-chain matches source
- [ ] Anchor verify
- [ ] Publish verified IDL
- **Deliverable**: Verified deployment

### Ring 200: Monitoring Setup
- [ ] Set up pool analytics
- [ ] Transaction monitoring
- [ ] Error alerting
- **Deliverable**: Monitoring infrastructure

### Ring 201: Error Codes Documentation
- [ ] Document all 67+ error codes
- [ ] Create error handling guide
- [ ] Map errors to solutions
- **Deliverable**: Error documentation

### Ring 202: Performance Documentation
- [ ] Document CU usage per instruction
- [ ] Document account sizes
- [ ] Compare with Orca benchmarks
- **Deliverable**: Performance docs

### Ring 203: Final Code Review
- [ ] Full codebase review
- [ ] Remove dead code
- [ ] Final formatting pass
- [ ] License headers
- **Deliverable**: Production-ready code

### Ring 204: Launch Checklist
- [ ] All tests passing
- [ ] Documentation complete
- [ ] SDK published to npm
- [ ] Program deployed to mainnet
- [ ] Demo UI ready
- [ ] Announcement ready
- **Deliverable**: LAUNCH READY 🚀

---

## COMPLETION CRITERIA

For $200k+ Protocol Engineer Portfolio:

### Must Have (Non-negotiable)
- [ ] All 204 rings completed
- [ ] Full test coverage (>95%)
- [ ] Security audit-ready code
- [ ] All 62+ instructions matching Orca feature parity
- [ ] Token-2022 full support
- [ ] TypeScript SDK published

### Production Quality
- [ ] Clean git history with clear commits
- [ ] Deployed to devnet with working pools
- [ ] Performance comparable to Orca
- [ ] Zero known bugs
- [ ] Comprehensive error handling

### Documentation Quality
- [ ] Architecture documentation
- [ ] API reference (auto-generated)
- [ ] Integration guide with examples
- [ ] Security model documented

### Portfolio Presentation
- [ ] README with clear value proposition
- [ ] Demo video or screenshots
- [ ] Technical blog post explaining design decisions
- [ ] Comparison with Orca (what's same, what's different)

---

## INSTRUCTION COUNT SUMMARY

| Category | Count | Status |
|----------|-------|--------|
| V1 Core | 31 | Rings 55-79 |
| V1 Extended | 7 | Rings 80-95 |
| Token-2022 V2 | 11 | Rings 96-109 |
| Token Badge | 5 | Rings 110-119 |
| Metadata | 4 | Rings 120-126 |
| Adaptive Fee | 7 | Rings 127-136 |
| Additional Authority | 8 | Rings 137-144 |
| **Total** | **73** | *Exceeds Orca's 62* |

---

## ACCOUNT COUNT SUMMARY

| Account | Size | Ring |
|---------|------|------|
| WhirlpoolsConfig | 100 bytes | 21 |
| WhirlpoolsConfigExtension | 608 bytes | 34 |
| Whirlpool | 653 bytes | 25-27 |
| FeeTier | 44 bytes | 22 |
| AdaptiveFeeTier | 286 bytes | 33 |
| TickArray | 10,040 bytes | 24 |
| DynamicTickArray | Variable | 36 |
| Position | 216 bytes | 28-30 |
| PositionBundle | 136 bytes | 31 |
| LockConfig | 201 bytes | 35 |
| TokenBadge | 200 bytes | 34 |

---

## REFERENCE MATERIALS

- **Orca Whirlpools**: Primary reference (62 instructions, production-proven)
- **Uniswap V3 Whitepaper**: Mathematical foundations
- **Solana Cookbook**: Solana best practices
- **Anchor Book**: Anchor patterns
- **SPL Token-2022 Docs**: Token extensions

---

## KEY DEPENDENCIES

```toml
[dependencies]
anchor-lang = "0.32.1"
anchor-spl = { version = "0.32.1", features = ["metadata"] }
spl-transfer-hook-interface = "0.9.0"
solana-program = "2.2.1"
uint = "0.9.5"
bytemuck = "1.22.0"
bitflags = "2.8.0"
```

---

*Build in public. Document decisions. Ship quality.*

*This roadmap covers 204 rings across 14 phases. Each ring is a single brush stroke. Complete them all, and you'll have a production-grade CLMM that rivals Orca Whirlpools.*
