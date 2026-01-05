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

### Ring 9: U256 Math Foundation
- [ ] Create `math/u256.rs`
- [ ] Implement U256 struct (4 x u64 limbs) OR use `uint` crate
- [ ] Implement `from_u128`, `to_u128`
- [ ] Implement basic add/sub
- [ ] Unit tests
- **Deliverable**: 256-bit integer support

### Ring 10: U256 Multiplication
- [ ] Implement `mul_u128(a: u128, b: u128) -> U256`
- [ ] Implement `mul_u256(a: U256, b: U256) -> U256` (with overflow check)
- [ ] Unit tests with max u128 values
- **Deliverable**: Overflow-safe multiplication

### Ring 11: U256 Division
- [ ] Implement `div_u256(a: U256, b: U256) -> U256`
- [ ] Implement `div_round_up` variant
- [ ] Implement `mod_u256`
- [ ] Unit tests
- **Deliverable**: 256-bit division

### Ring 12: U256 Shift Operations
- [ ] Implement `shl(value: U256, bits: u32) -> U256`
- [ ] Implement `shr(value: U256, bits: u32) -> U256`
- [ ] Unit tests
- **Deliverable**: Bit shifting for fixed-point math

### Ring 13: Tick Math - tick_to_sqrt_price
- [ ] Create `math/tick_math.rs`
- [ ] Implement `get_sqrt_price_at_tick(tick: i32) -> u128`
- [ ] Use the 1.0001^tick formula with lookup table optimization
- [ ] Handle negative ticks
- [ ] Validate tick is within bounds
- [ ] Unit tests: tick 0 = 2^64, known tick values
- **Deliverable**: Tick to sqrt_price conversion

### Ring 14: Tick Math - sqrt_price_to_tick
- [ ] Implement `get_tick_at_sqrt_price(sqrt_price: u128) -> i32`
- [ ] Binary search or logarithm approach
- [ ] Ensure round-down behavior
- [ ] Unit tests: roundtrip with tick_to_sqrt_price
- **Deliverable**: sqrt_price to tick conversion

### Ring 15: Tick Math - Utilities
- [ ] Implement `get_next_valid_tick(tick: i32, tick_spacing: u16, zero_for_one: bool) -> i32`
- [ ] Implement `is_valid_tick(tick: i32, tick_spacing: u16) -> bool`
- [ ] Implement `get_start_tick_index(tick: i32, tick_spacing: u16) -> i32`
- [ ] Unit tests
- **Deliverable**: Tick navigation helpers

### Ring 16: Liquidity Math - Add/Sub
- [ ] Create `math/liquidity_math.rs`
- [ ] Implement `add_liquidity(a: u128, b: u128) -> Result<u128>`
- [ ] Implement `sub_liquidity(a: u128, b: u128) -> Result<u128>`
- [ ] Overflow/underflow protection
- [ ] Unit tests
- **Deliverable**: Safe liquidity arithmetic

### Ring 17: Token Math - Amount from Liquidity (Token A)
- [ ] Create `math/token_math.rs`
- [ ] Implement `get_amount_a_delta(sqrt_price_lower: u128, sqrt_price_upper: u128, liquidity: u128, round_up: bool) -> u64`
- [ ] Formula: `L * (1/sqrt_lower - 1/sqrt_upper)`
- [ ] Use U256 for intermediate calculations
- [ ] Unit tests with known values
- **Deliverable**: Token A amount calculation

### Ring 18: Token Math - Amount from Liquidity (Token B)
- [ ] Implement `get_amount_b_delta(sqrt_price_lower: u128, sqrt_price_upper: u128, liquidity: u128, round_up: bool) -> u64`
- [ ] Formula: `L * (sqrt_upper - sqrt_lower)`
- [ ] Unit tests
- **Deliverable**: Token B amount calculation

### Ring 19: Token Math - Liquidity from Amounts
- [ ] Implement `get_liquidity_for_amounts(sqrt_price: u128, sqrt_price_lower: u128, sqrt_price_upper: u128, amount_a: u64, amount_b: u64) -> u128`
- [ ] Handle three cases: price below, in range, above
- [ ] Unit tests
- **Deliverable**: Liquidity calculation from token amounts

### Ring 20: Swap Math - Core Step
- [ ] Create `math/swap_math.rs`
- [ ] Implement `compute_swap_step(sqrt_price_current: u128, sqrt_price_target: u128, liquidity: u128, amount_remaining: u64, fee_rate: u16) -> SwapStepResult`
- [ ] Return: `sqrt_price_next`, `amount_in`, `amount_out`, `fee_amount`
- [ ] Handle both exact_input and exact_output
- [ ] Unit tests with known swap scenarios
- **Deliverable**: Single swap step calculation

---

## PHASE 2: STATE STRUCTURES (Rings 21-35)

> Define every account. Memory layout matters.

### Ring 21: WhirlpoolsConfig State
- [ ] Create `state/config.rs`
- [ ] Define `WhirlpoolsConfig` struct
- [ ] Fields: `fee_authority`, `collect_protocol_fees_authority`, `reward_emissions_super_authority`, `default_protocol_fee_rate`, `bump`
- [ ] Implement `DISCRIMINATOR`, `LEN`
- [ ] Add `seeds` for PDA
- **Deliverable**: Config account structure

### Ring 22: FeeTier State
- [ ] Create `state/fee_tier.rs`
- [ ] Define `FeeTier` struct
- [ ] Fields: `whirlpools_config`, `tick_spacing`, `default_fee_rate`, `bump`
- [ ] Implement `DISCRIMINATOR`, `LEN`
- [ ] Add `seeds` for PDA (config + tick_spacing)
- **Deliverable**: Fee tier account structure

### Ring 23: Tick State
- [ ] Create `state/tick.rs`
- [ ] Define `Tick` struct (NOT an account, embedded in TickArray)
- [ ] Fields: `initialized`, `liquidity_net` (i128), `liquidity_gross` (u128), `fee_growth_outside_a` (u128), `fee_growth_outside_b` (u128), `reward_growths_outside` ([u128; 3])
- [ ] Implement `default()`, `update()`, `cross()`
- **Deliverable**: Tick data structure

### Ring 24: TickArray State
- [ ] Create `state/tick_array.rs`
- [ ] Define `TickArray` struct
- [ ] Fields: `whirlpool`, `start_tick_index`, `ticks: [Tick; 88]`
- [ ] Implement `DISCRIMINATOR`, `LEN` (calculate exact size)
- [ ] Add `seeds` for PDA (whirlpool + start_tick_index)
- [ ] Implement `get_tick(tick_index) -> &Tick`
- [ ] Implement `get_tick_mut(tick_index) -> &mut Tick`
- **Deliverable**: Tick array account structure

### Ring 25: Whirlpool State - Core Fields
- [ ] Create `state/whirlpool.rs`
- [ ] Define `Whirlpool` struct - Part 1
- [ ] Fields: `whirlpools_config`, `bump`, `tick_spacing`, `tick_spacing_seed` (for PDA)
- [ ] Fields: `token_mint_a`, `token_mint_b`, `token_vault_a`, `token_vault_b`
- [ ] Fields: `fee_rate`, `protocol_fee_rate`
- **Deliverable**: Whirlpool core identity fields

### Ring 26: Whirlpool State - Price Fields
- [ ] Add to `Whirlpool`: `sqrt_price` (u128), `tick_current_index` (i32)
- [ ] Add: `liquidity` (u128) - active liquidity at current price
- [ ] Add: `fee_growth_global_a` (u128), `fee_growth_global_b` (u128)
- [ ] Add: `protocol_fee_owed_a` (u64), `protocol_fee_owed_b` (u64)
- **Deliverable**: Whirlpool price/fee state

### Ring 27: Whirlpool State - Rewards
- [ ] Define `WhirlpoolRewardInfo` struct
- [ ] Fields: `mint`, `vault`, `authority`, `emissions_per_second_x64`, `growth_global_x64`
- [ ] Add to `Whirlpool`: `reward_infos: [WhirlpoolRewardInfo; 3]`
- [ ] Add: `reward_last_updated_timestamp`
- [ ] Calculate final `Whirlpool::LEN`
- **Deliverable**: Complete Whirlpool state

### Ring 28: Position State - Core
- [ ] Create `state/position.rs`
- [ ] Define `Position` struct
- [ ] Fields: `whirlpool`, `position_mint`, `liquidity` (u128)
- [ ] Fields: `tick_lower_index` (i32), `tick_upper_index` (i32)
- [ ] Implement `DISCRIMINATOR`, `LEN`
- **Deliverable**: Position identity fields

### Ring 29: Position State - Fees
- [ ] Add to `Position`: `fee_growth_checkpoint_a` (u128), `fee_growth_checkpoint_b` (u128)
- [ ] Add: `fee_owed_a` (u64), `fee_owed_b` (u64)
- **Deliverable**: Position fee tracking

### Ring 30: Position State - Rewards
- [ ] Define `PositionRewardInfo` struct
- [ ] Fields: `growth_inside_checkpoint` (u128), `amount_owed` (u64)
- [ ] Add to `Position`: `reward_infos: [PositionRewardInfo; 3]`
- [ ] Calculate final `Position::LEN`
- **Deliverable**: Complete Position state

### Ring 31: Position Bundle State
- [ ] Create `state/position_bundle.rs`
- [ ] Define `PositionBundle` struct
- [ ] Fields: `position_bundle_mint`, `positions_bitmap: [u8; 32]` (256 bits)
- [ ] Implement `DISCRIMINATOR`, `LEN`
- [ ] Implement `is_slot_occupied(index: u8) -> bool`
- [ ] Implement `set_slot_occupied(index: u8, occupied: bool)`
- **Deliverable**: Position bundle for batch management

### Ring 32: Oracle State
- [ ] Create `state/oracle.rs`
- [ ] Define `OracleObservation` struct
- [ ] Fields: `timestamp` (i64), `tick_cumulative` (i128), `initialized` (bool)
- [ ] Define observation array size (e.g., 100 observations)
- **Deliverable**: Oracle observation structure

### Ring 33: Adaptive Fee State
- [ ] Create `state/adaptive_fee.rs`
- [ ] Define `AdaptiveFeeTier` struct (LEN: 286 bytes like Orca)
- [ ] Fields: `whirlpools_config`, `fee_tier_index`, `tick_spacing`
- [ ] Fields: `initialize_pool_authority`, `delegated_fee_authority`
- [ ] Fields: `default_base_fee_rate`, `max_fee_rate`
- [ ] Fields: `filter_period`, `decay_period`, `reduction_factor`
- [ ] Fields: `adaptive_fee_control_factor`, `max_volatility_accumulator`
- [ ] Fields: `tick_group_size`, `major_swap_threshold_ticks`
- [ ] Add 128-byte reserve for future upgrades
- [ ] Implement `DISCRIMINATOR`, `LEN`
- **Deliverable**: Adaptive fee configuration

### Ring 34: Config Extension State
- [ ] Create `state/config_extension.rs`
- [ ] Define `WhirlpoolsConfigExtension` struct (LEN: 608 bytes)
- [ ] Fields: `whirlpools_config`, `config_extension_authority`, `token_badge_authority`
- [ ] Add 512-byte reserve for future features
- [ ] Implement `DISCRIMINATOR`, `LEN`
- [ ] Add `seeds` for PDA
- **Deliverable**: Config extension for Token-2022 features

### Ring 35: Token Badge State
- [ ] Create `state/token_badge.rs`
- [ ] Define `TokenBadge` struct (LEN: 200 bytes)
- [ ] Fields: `whirlpools_config`, `token_mint`
- [ ] Fields: `attribute_require_non_transferable_position` (bool)
- [ ] Add 127-byte reserve
- [ ] Implement `DISCRIMINATOR`, `LEN`
- [ ] Add `seeds` for PDA (config + token_mint)
- **Deliverable**: Token access control badge

### Ring 36: Lock Config State
- [ ] Create `state/lock_config.rs`
- [ ] Define `LockConfig` struct (LEN: 201 bytes)
- [ ] Fields: `position`, `position_owner`, `whirlpool`
- [ ] Fields: `locked_timestamp` (i64)
- [ ] Fields: `lock_type` (enum: Permanent)
- [ ] Add 128-byte reserve
- [ ] Implement `DISCRIMINATOR`, `LEN`
- **Deliverable**: Position lock configuration

### Ring 37: Dynamic Tick Array State
- [ ] Create `state/dynamic_tick_array.rs`
- [ ] Define `DynamicTickArray` struct (variable size)
- [ ] Fields: `whirlpool`, `start_tick_index`
- [ ] Fields: `initialized_tick_count`, `ticks: Vec<Tick>`
- [ ] Implement dynamic sizing logic
- [ ] Implement `get_tick()`, `get_tick_mut()`, `insert_tick()`
- **Deliverable**: Space-efficient tick storage

### Ring 38: Feature Flags
- [ ] Create `state/feature_flags.rs`
- [ ] Define `ConfigFeatureFlags` bitflags struct
- [ ] Flags: `TOKEN_BADGE` (bit 0)
- [ ] Reserve remaining bits for future features
- [ ] Add to `WhirlpoolsConfig`: `feature_flags` field
- **Deliverable**: Feature toggle system

### Ring 39: State Module Exports
- [ ] Create `state/mod.rs`
- [ ] Export all state structs
- [ ] Add state validation helpers
- **Deliverable**: Clean state module

### Ring 35: State Serialization Tests
- [ ] Write tests verifying `LEN` constants match actual serialized size
- [ ] Test PDA derivation for all accounts
- [ ] Test default values
- **Deliverable**: Verified state structures

---

## PHASE 3: MANAGER LAYER (Rings 36-50)

> Business logic. Managers orchestrate operations across state.

### Ring 36: Tick Manager - Initialization
- [ ] Create `manager/tick_manager.rs`
- [ ] Implement `initialize_tick(tick: &mut Tick)`
- [ ] Implement `deinitialize_tick(tick: &mut Tick) -> bool` (returns if actually deinitialized)
- [ ] Unit tests
- **Deliverable**: Tick lifecycle management

### Ring 37: Tick Manager - Update
- [ ] Implement `update_tick(tick: &mut Tick, liquidity_delta: i128, is_upper: bool) -> Result<bool>`
- [ ] Update `liquidity_net` (add if lower, subtract if upper)
- [ ] Update `liquidity_gross`
- [ ] Return whether tick should be initialized/deinitialized
- [ ] Unit tests
- **Deliverable**: Tick liquidity updates

### Ring 38: Tick Manager - Crossing
- [ ] Implement `cross_tick(tick: &mut Tick, fee_growth_global_a: u128, fee_growth_global_b: u128, reward_growths_global: &[u128; 3]) -> i128`
- [ ] Flip `fee_growth_outside` values
- [ ] Flip `reward_growths_outside` values
- [ ] Return `liquidity_net` for pool update
- [ ] Unit tests
- **Deliverable**: Tick crossing logic

### Ring 39: Tick Manager - Fee Growth
- [ ] Implement `get_fee_growth_inside(tick_lower: &Tick, tick_upper: &Tick, tick_current: i32, fee_growth_global_a: u128, fee_growth_global_b: u128) -> (u128, u128)`
- [ ] Handle three cases: current below, inside, above range
- [ ] Wrapping subtraction for accumulators
- [ ] Unit tests with various scenarios
- **Deliverable**: Fee growth calculation

### Ring 40: Tick Manager - Reward Growth
- [ ] Implement `get_reward_growth_inside(tick_lower: &Tick, tick_upper: &Tick, tick_current: i32, reward_growths_global: &[u128; 3]) -> [u128; 3]`
- [ ] Same logic as fees, for each reward
- [ ] Unit tests
- **Deliverable**: Reward growth calculation

### Ring 41: Position Manager - Fee Calculation
- [ ] Create `manager/position_manager.rs`
- [ ] Implement `calculate_fee_owed(position: &Position, fee_growth_inside_a: u128, fee_growth_inside_b: u128) -> (u64, u64)`
- [ ] Formula: `(growth_inside - checkpoint) * liquidity`
- [ ] Handle accumulator wrapping
- [ ] Unit tests
- **Deliverable**: Position fee calculation

### Ring 42: Position Manager - Fee Update
- [ ] Implement `update_position_fees(position: &mut Position, fee_growth_inside_a: u128, fee_growth_inside_b: u128)`
- [ ] Calculate owed amounts
- [ ] Add to `fee_owed_a/b`
- [ ] Update checkpoints
- [ ] Unit tests
- **Deliverable**: Position fee updates

### Ring 43: Position Manager - Reward Calculation
- [ ] Implement `calculate_reward_owed(position: &Position, reward_index: usize, reward_growth_inside: u128) -> u64`
- [ ] Same pattern as fees
- [ ] Unit tests
- **Deliverable**: Position reward calculation

### Ring 44: Position Manager - Reward Update
- [ ] Implement `update_position_rewards(position: &mut Position, reward_growths_inside: &[u128; 3])`
- [ ] Update all three reward checkpoints and owed amounts
- [ ] Unit tests
- **Deliverable**: Position reward updates

### Ring 45: Liquidity Manager - Add
- [ ] Create `manager/liquidity_manager.rs`
- [ ] Implement `add_liquidity(whirlpool: &mut Whirlpool, position: &mut Position, tick_lower: &mut Tick, tick_upper: &mut Tick, liquidity_delta: u128) -> Result<(u64, u64)>`
- [ ] Update position liquidity
- [ ] Update tick liquidity_net/gross
- [ ] Update pool liquidity if in range
- [ ] Return required token amounts
- [ ] Unit tests
- **Deliverable**: Add liquidity orchestration

### Ring 46: Liquidity Manager - Remove
- [ ] Implement `remove_liquidity(whirlpool: &mut Whirlpool, position: &mut Position, tick_lower: &mut Tick, tick_upper: &mut Tick, liquidity_delta: u128) -> Result<(u64, u64)>`
- [ ] Reverse of add
- [ ] Return withdrawn token amounts
- [ ] Unit tests
- **Deliverable**: Remove liquidity orchestration

### Ring 47: Swap Manager - Core Loop Setup
- [ ] Create `manager/swap_manager.rs`
- [ ] Define `SwapState` struct: `amount_remaining`, `amount_calculated`, `sqrt_price`, `tick`, `liquidity`, `fee_growth_global`, `protocol_fee`
- [ ] Define `SwapResult` struct
- [ ] Implement `initialize_swap_state(whirlpool: &Whirlpool, amount: u64, sqrt_price_limit: u128, a_to_b: bool) -> SwapState`
- **Deliverable**: Swap state management

### Ring 48: Swap Manager - Step Execution
- [ ] Implement `execute_swap_step(state: &mut SwapState, tick_array: &TickArray, a_to_b: bool) -> Result<bool>`
- [ ] Find next initialized tick
- [ ] Calculate swap step using swap_math
- [ ] Update state
- [ ] Return whether swap is complete
- [ ] Unit tests
- **Deliverable**: Single swap step execution

### Ring 49: Swap Manager - Tick Crossing
- [ ] Implement `handle_tick_crossing(state: &mut SwapState, tick: &mut Tick, ...) -> Result<()>`
- [ ] Cross tick and update liquidity
- [ ] Update fee growth outside
- [ ] Unit tests
- **Deliverable**: Tick crossing during swap

### Ring 50: Swap Manager - Full Swap
- [ ] Implement `execute_swap(whirlpool: &mut Whirlpool, tick_arrays: &mut [TickArray], amount: u64, sqrt_price_limit: u128, a_to_b: bool, ...) -> Result<SwapResult>`
- [ ] Loop through swap steps
- [ ] Handle tick array transitions
- [ ] Update pool state
- [ ] Return final result
- [ ] Integration tests
- **Deliverable**: Complete swap execution

---

## PHASE 4: CORE INSTRUCTIONS (Rings 51-75)

> The instruction layer. What users actually call.

### Ring 51: Initialize Config Instruction
- [ ] Create `instructions/initialize_config.rs`
- [ ] Define `InitializeConfig` accounts struct
- [ ] Implement instruction handler
- [ ] Validate authorities
- [ ] Create PDA
- [ ] Unit test
- **Deliverable**: Config initialization

### Ring 52: Initialize Fee Tier Instruction
- [ ] Create `instructions/initialize_fee_tier.rs`
- [ ] Define accounts: config, fee_tier, funder, system_program
- [ ] Validate tick_spacing (must be > 0, reasonable max)
- [ ] Validate fee_rate
- [ ] Create PDA with config + tick_spacing seeds
- [ ] Unit test
- **Deliverable**: Fee tier initialization

### Ring 53: Initialize Pool Instruction
- [ ] Create `instructions/initialize_pool.rs`
- [ ] Define accounts: config, fee_tier, whirlpool, token_mint_a, token_mint_b, token_vault_a, token_vault_b, funder
- [ ] Validate token ordering (mint_a < mint_b lexicographically)
- [ ] Validate initial_sqrt_price within bounds
- [ ] Create token vaults (ATAs)
- [ ] Initialize pool state
- [ ] Unit test
- **Deliverable**: Pool initialization

### Ring 54: Initialize Tick Array Instruction
- [ ] Create `instructions/initialize_tick_array.rs`
- [ ] Define accounts: whirlpool, tick_array, funder
- [ ] Validate start_tick_index alignment
- [ ] Create PDA with whirlpool + start_tick_index seeds
- [ ] Initialize all 88 ticks to default
- [ ] Unit test
- **Deliverable**: Tick array initialization

### Ring 55: Open Position Instruction (Setup)
- [ ] Create `instructions/open_position.rs`
- [ ] Define accounts: whirlpool, position, position_mint, position_token_account, owner, funder
- [ ] Validate tick_lower < tick_upper
- [ ] Validate ticks aligned to tick_spacing
- [ ] Validate ticks within bounds
- **Deliverable**: Position validation

### Ring 56: Open Position Instruction (Execution)
- [ ] Create position NFT mint
- [ ] Create position token account
- [ ] Mint 1 NFT to owner
- [ ] Initialize position state (liquidity = 0)
- [ ] Set fee/reward checkpoints to current values
- [ ] Unit test
- **Deliverable**: Position creation

### Ring 57: Increase Liquidity Instruction (Setup)
- [ ] Create `instructions/increase_liquidity.rs`
- [ ] Define accounts: whirlpool, position, position_authority, tick_array_lower, tick_array_upper, token_vault_a, token_vault_b, token_owner_account_a, token_owner_account_b
- [ ] Validate position ownership
- [ ] Validate tick arrays contain position bounds
- **Deliverable**: Increase liquidity validation

### Ring 58: Increase Liquidity Instruction (Execution)
- [ ] Calculate required token amounts
- [ ] Transfer tokens from user to vaults
- [ ] Update position liquidity
- [ ] Update tick states
- [ ] Update pool liquidity if in range
- [ ] Update fee/reward checkpoints
- [ ] Unit test
- **Deliverable**: Liquidity addition

### Ring 59: Decrease Liquidity Instruction
- [ ] Create `instructions/decrease_liquidity.rs`
- [ ] Same account structure as increase
- [ ] Validate sufficient liquidity in position
- [ ] Calculate withdrawn token amounts
- [ ] Transfer tokens from vaults to user
- [ ] Update all states (reverse of increase)
- [ ] Unit test
- **Deliverable**: Liquidity removal

### Ring 60: Update Fees and Rewards Instruction
- [ ] Create `instructions/update_fees_and_rewards.rs`
- [ ] Define accounts: whirlpool, position, tick_array_lower, tick_array_upper
- [ ] Calculate fee growth inside range
- [ ] Calculate reward growth inside range
- [ ] Update position owed amounts
- [ ] Update checkpoints
- [ ] Unit test
- **Deliverable**: Fee/reward calculation

### Ring 61: Collect Fees Instruction
- [ ] Create `instructions/collect_fees.rs`
- [ ] Define accounts: whirlpool, position, position_authority, token_vault_a, token_vault_b, token_owner_account_a, token_owner_account_b
- [ ] Validate ownership
- [ ] Transfer owed fees from vaults
- [ ] Reset fee_owed to 0
- [ ] Unit test
- **Deliverable**: Fee collection

### Ring 62: Collect Reward Instruction
- [ ] Create `instructions/collect_reward.rs`
- [ ] Define accounts: whirlpool, position, position_authority, reward_vault, token_owner_account, reward_index
- [ ] Validate reward index (0-2)
- [ ] Transfer owed reward
- [ ] Reset reward amount_owed to 0
- [ ] Unit test
- **Deliverable**: Reward collection

### Ring 63: Close Position Instruction
- [ ] Create `instructions/close_position.rs`
- [ ] Validate liquidity = 0
- [ ] Validate fee_owed_a = 0, fee_owed_b = 0
- [ ] Validate all reward_owed = 0
- [ ] Burn position NFT
- [ ] Close position account (return rent)
- [ ] Close position mint account
- [ ] Unit test
- **Deliverable**: Position closure

### Ring 64: Swap Instruction (Setup)
- [ ] Create `instructions/swap.rs`
- [ ] Define accounts: whirlpool, token_vault_a, token_vault_b, token_owner_account_a, token_owner_account_b, tick_array_0, tick_array_1, tick_array_2, oracle (optional)
- [ ] Define params: amount, other_amount_threshold, sqrt_price_limit, amount_specified_is_input, a_to_b
- [ ] Validate sqrt_price_limit direction
- **Deliverable**: Swap validation

### Ring 65: Swap Instruction (Execution)
- [ ] Execute swap using swap_manager
- [ ] Update pool sqrt_price, tick_current_index, liquidity
- [ ] Update fee_growth_global
- [ ] Accumulate protocol fees
- [ ] Transfer tokens
- [ ] Validate slippage (other_amount_threshold)
- [ ] Unit test
- **Deliverable**: Core swap execution

### Ring 66: Two Hop Swap Instruction
- [ ] Create `instructions/two_hop_swap.rs`
- [ ] Define accounts for two pools
- [ ] Execute first swap
- [ ] Execute second swap with first swap's output
- [ ] Atomic (both succeed or both fail)
- [ ] Unit test
- **Deliverable**: Multi-hop routing

### Ring 67: Collect Protocol Fees Instruction
- [ ] Create `instructions/collect_protocol_fees.rs`
- [ ] Validate caller is collect_protocol_fees_authority
- [ ] Transfer protocol_fee_owed_a/b to authority
- [ ] Reset owed amounts
- [ ] Unit test
- **Deliverable**: Protocol fee collection

### Ring 68: Set Fee Rate Instruction
- [ ] Create `instructions/set_fee_rate.rs`
- [ ] Validate caller is fee_authority
- [ ] Validate new rate within bounds
- [ ] Update pool fee_rate
- [ ] Unit test
- **Deliverable**: Fee adjustment

### Ring 69: Set Protocol Fee Rate Instruction
- [ ] Create `instructions/set_protocol_fee_rate.rs`
- [ ] Validate caller is fee_authority
- [ ] Validate rate within MAX_PROTOCOL_FEE_RATE
- [ ] Update pool protocol_fee_rate
- [ ] Unit test
- **Deliverable**: Protocol fee adjustment

### Ring 70: Set Reward Emissions Instruction
- [ ] Create `instructions/set_reward_emissions.rs`
- [ ] Validate caller is reward_authority
- [ ] Validate reward index
- [ ] Update emissions_per_second_x64
- [ ] Update reward_last_updated_timestamp
- [ ] Checkpoint current growth
- [ ] Unit test
- **Deliverable**: Reward emission control

### Ring 71: Initialize Reward Instruction
- [ ] Create `instructions/initialize_reward.rs`
- [ ] Validate reward slot not already initialized
- [ ] Create reward vault
- [ ] Set reward mint and authority
- [ ] Unit test
- **Deliverable**: Reward program setup

### Ring 72: Set Reward Authority Instruction
- [ ] Create `instructions/set_reward_authority.rs`
- [ ] Validate caller is current authority
- [ ] Transfer authority to new address
- [ ] Unit test
- **Deliverable**: Reward authority transfer

### Ring 73: Set Fee Authority Instruction
- [ ] Create `instructions/set_fee_authority.rs`
- [ ] Validate caller is current fee_authority
- [ ] Transfer to new address
- [ ] Unit test
- **Deliverable**: Fee authority transfer

### Ring 74: Set Collect Protocol Fees Authority
- [ ] Create `instructions/set_collect_protocol_fees_authority.rs`
- [ ] Same pattern
- [ ] Unit test
- **Deliverable**: Protocol fees authority transfer

### Ring 75: Instruction Module Exports
- [ ] Create `instructions/mod.rs`
- [ ] Export all instructions
- [ ] Create `lib.rs` program entrypoint with all handlers
- [ ] Verify `anchor build` succeeds
- **Deliverable**: Complete instruction module

---

## PHASE 5: ADVANCED FEATURES (Rings 76-90)

### Ring 76: Position Bundle - Open Bundle
- [ ] Create `instructions/open_position_bundle.rs`
- [ ] Create bundle NFT
- [ ] Initialize bitmap to all zeros
- [ ] Unit test
- **Deliverable**: Bundle creation

### Ring 77: Position Bundle - Open Bundled Position
- [ ] Create `instructions/open_bundled_position.rs`
- [ ] Find first empty slot in bitmap
- [ ] Create position linked to bundle
- [ ] Set slot occupied
- [ ] Unit test
- **Deliverable**: Bundled position creation

### Ring 78: Position Bundle - Close Bundled Position
- [ ] Create `instructions/close_bundled_position.rs`
- [ ] Validate position empty
- [ ] Clear bitmap slot
- [ ] Close position account
- [ ] Unit test
- **Deliverable**: Bundled position closure

### Ring 79: Position Bundle - Close Bundle
- [ ] Create `instructions/close_position_bundle.rs`
- [ ] Validate all slots empty (bitmap = 0)
- [ ] Burn bundle NFT
- [ ] Return rent
- [ ] Unit test
- **Deliverable**: Bundle closure

### Ring 80: Oracle - Initialize
- [ ] Create `instructions/initialize_oracle.rs`
- [ ] Allocate observation array
- [ ] Initialize first observation with current tick
- [ ] Unit test
- **Deliverable**: Oracle initialization

### Ring 81: Oracle - Update Logic
- [ ] Create `manager/oracle_manager.rs`
- [ ] Implement `update_oracle(oracle: &mut Oracle, tick: i32, timestamp: i64)`
- [ ] Calculate tick_cumulative delta
- [ ] Write to circular buffer
- [ ] Update index
- [ ] Unit test
- **Deliverable**: Oracle update logic

### Ring 82: Oracle - Read TWAP
- [ ] Implement `get_twap(oracle: &Oracle, seconds_ago: u32) -> i32`
- [ ] Find relevant observations
- [ ] Calculate time-weighted average tick
- [ ] Handle edge cases (not enough history)
- [ ] Unit test
- **Deliverable**: TWAP calculation

### Ring 83: Oracle - Integration with Swap
- [ ] Modify swap instruction to update oracle
- [ ] Update after price change
- [ ] Integration test
- **Deliverable**: Oracle in swap flow

### Ring 84: Adaptive Fee - Initialize
- [ ] Create `instructions/initialize_adaptive_fee_tier.rs`
- [ ] Set all parameters
- [ ] Link to fee_tier
- [ ] Unit test
- **Deliverable**: Adaptive fee setup

### Ring 85: Adaptive Fee - Volatility Update
- [ ] Create `manager/adaptive_fee_manager.rs`
- [ ] Implement `update_volatility(state: &mut AdaptiveFeeTier, ticks_crossed: u32, timestamp: i64)`
- [ ] Accumulate volatility
- [ ] Apply decay
- [ ] Unit test
- **Deliverable**: Volatility tracking

### Ring 86: Adaptive Fee - Current Fee Calculation
- [ ] Implement `get_current_fee_rate(state: &AdaptiveFeeTier) -> u16`
- [ ] Apply formula: base + (accumulator * factor)
- [ ] Cap at max_fee_rate
- [ ] Unit test
- **Deliverable**: Dynamic fee calculation

### Ring 87: Adaptive Fee - Integration with Swap
- [ ] Modify swap to use adaptive fee when configured
- [ ] Update volatility accumulator after swap
- [ ] Integration test
- **Deliverable**: Adaptive fees in swap

### Ring 88: Position Lock - Lock Position
- [ ] Create `instructions/lock_position.rs`
- [ ] Set lock_release_time on position
- [ ] Add lock_release_time field to Position struct
- [ ] Unit test
- **Deliverable**: Position locking

### Ring 89: Position Lock - Validation in Decrease/Close
- [ ] Modify decrease_liquidity to check lock
- [ ] Modify close_position to check lock
- [ ] Allow only if current_time >= lock_release_time
- [ ] Integration test
- **Deliverable**: Lock enforcement

### Ring 90: Reset Position Range
- [ ] Create `instructions/reset_position_range.rs`
- [ ] Validate position unlocked
- [ ] Validate liquidity = 0
- [ ] Allow changing tick_lower/tick_upper
- [ ] Reset checkpoints
- [ ] Unit test
- **Deliverable**: Position range reset

### Ring 91: Transfer Locked Position
- [ ] Create `instructions/transfer_locked_position.rs`
- [ ] Allow transfer of locked positions
- [ ] Update position_owner in LockConfig
- [ ] Validate lock is active
- [ ] Unit test
- **Deliverable**: Locked position transfers

---

## PHASE 6: TOKEN-2022 SUPPORT (Rings 92-110)

> Full Token Extensions support - critical for modern tokens

### Ring 92: Token-2022 Utility Module
- [ ] Create `util/token_2022.rs`
- [ ] Implement `is_token_2022(mint: &AccountInfo) -> bool`
- [ ] Implement `get_token_program_id(mint: &AccountInfo) -> Pubkey`
- [ ] Handle transfer hooks detection
- [ ] Handle transfer fees detection
- **Deliverable**: Token-2022 detection utilities

### Ring 93: Remaining Accounts Handler
- [ ] Create `util/remaining_accounts.rs`
- [ ] Define `RemainingAccountsInfo` struct
- [ ] Implement parsing of remaining accounts for transfer hooks
- [ ] Handle supplemental tick arrays
- [ ] Unit tests
- **Deliverable**: Dynamic account handling

### Ring 94: Transfer with Hook Support
- [ ] Create `util/transfer.rs`
- [ ] Implement `transfer_from_owner_to_vault()` with hook support
- [ ] Implement `transfer_from_vault_to_owner()` with hook support
- [ ] Handle both SPL Token and Token-2022
- [ ] Handle transfer fees calculation
- [ ] Unit tests
- **Deliverable**: Universal transfer utility

### Ring 95: Initialize Pool V2
- [ ] Create `instructions/v2/initialize_pool_v2.rs`
- [ ] Support Token-2022 token mints
- [ ] Handle token extensions detection
- [ ] Create vaults with correct token program
- [ ] Unit test
- **Deliverable**: Pool init with Token-2022

### Ring 96: Increase Liquidity V2
- [ ] Create `instructions/v2/increase_liquidity_v2.rs`
- [ ] Support remaining accounts for transfer hooks
- [ ] Handle transfer fees on deposit
- [ ] Calculate correct amounts after fees
- [ ] Unit test
- **Deliverable**: Add liquidity with Token-2022

### Ring 97: Decrease Liquidity V2
- [ ] Create `instructions/v2/decrease_liquidity_v2.rs`
- [ ] Support remaining accounts for transfer hooks
- [ ] Handle transfer fees on withdrawal
- [ ] Unit test
- **Deliverable**: Remove liquidity with Token-2022

### Ring 98: Swap V2
- [ ] Create `instructions/v2/swap_v2.rs`
- [ ] Support Token-2022 for both tokens
- [ ] Handle transfer hooks on both sides
- [ ] Handle transfer fees
- [ ] Remaining accounts for hooks
- [ ] Unit test
- **Deliverable**: Swap with Token-2022

### Ring 99: Two Hop Swap V2
- [ ] Create `instructions/v2/two_hop_swap_v2.rs`
- [ ] Handle three tokens (A, intermediate, B)
- [ ] Support Token-2022 for all three
- [ ] Proper remaining accounts handling
- [ ] Unit test
- **Deliverable**: Multi-hop with Token-2022

### Ring 100: Collect Fees V2
- [ ] Create `instructions/v2/collect_fees_v2.rs`
- [ ] Support Token-2022 vaults
- [ ] Handle transfer fees on collection
- [ ] Unit test
- **Deliverable**: Fee collection with Token-2022

### Ring 101: Collect Protocol Fees V2
- [ ] Create `instructions/v2/collect_protocol_fees_v2.rs`
- [ ] Support Token-2022 vaults
- [ ] Unit test
- **Deliverable**: Protocol fees with Token-2022

### Ring 102: Collect Reward V2
- [ ] Create `instructions/v2/collect_reward_v2.rs`
- [ ] Support Token-2022 reward tokens
- [ ] Handle transfer hooks
- [ ] Unit test
- **Deliverable**: Reward collection with Token-2022

### Ring 103: Initialize Reward V2
- [ ] Create `instructions/v2/initialize_reward_v2.rs`
- [ ] Support Token-2022 reward mints
- [ ] Create vault with correct program
- [ ] Unit test
- **Deliverable**: Reward setup with Token-2022

### Ring 104: Set Reward Emissions V2
- [ ] Create `instructions/v2/set_reward_emissions_v2.rs`
- [ ] Handle Token-2022 reward tokens
- [ ] Unit test
- **Deliverable**: Emissions with Token-2022

### Ring 105: V2 Module Exports
- [ ] Create `instructions/v2/mod.rs`
- [ ] Export all V2 instructions
- [ ] Add to main program
- [ ] Integration tests
- **Deliverable**: Complete V2 instruction set

---

## PHASE 7: TOKEN BADGE SYSTEM (Rings 106-115)

> Access control for tokens - restrict which tokens can be used

### Ring 106: Initialize Config Extension
- [ ] Create `instructions/initialize_config_extension.rs`
- [ ] Create ConfigExtension PDA
- [ ] Set initial authorities
- [ ] Unit test
- **Deliverable**: Config extension initialization

### Ring 107: Set Config Extension Authority
- [ ] Create `instructions/set_config_extension_authority.rs`
- [ ] Validate current authority
- [ ] Transfer to new authority
- [ ] Unit test
- **Deliverable**: Extension authority transfer

### Ring 108: Set Token Badge Authority
- [ ] Create `instructions/set_token_badge_authority.rs`
- [ ] Validate current authority
- [ ] Transfer badge authority
- [ ] Unit test
- **Deliverable**: Badge authority transfer

### Ring 109: Initialize Token Badge
- [ ] Create `instructions/initialize_token_badge.rs`
- [ ] Create TokenBadge PDA for token mint
- [ ] Set initial attributes (non-transferable position)
- [ ] Unit test
- **Deliverable**: Token badge creation

### Ring 110: Delete Token Badge
- [ ] Create `instructions/delete_token_badge.rs`
- [ ] Validate authority
- [ ] Close badge account, return rent
- [ ] Unit test
- **Deliverable**: Token badge removal

### Ring 111: Set Token Badge Attribute
- [ ] Create `instructions/set_token_badge_attribute.rs`
- [ ] Update `require_non_transferable_position`
- [ ] Future: add more attributes
- [ ] Unit test
- **Deliverable**: Badge attribute updates

### Ring 112: Token Badge Validation in Pool Init
- [ ] Modify `initialize_pool` to check token badges
- [ ] Check feature flag is enabled
- [ ] Enforce badge requirements
- [ ] Integration test
- **Deliverable**: Badge enforcement in pools

### Ring 113: Token Badge Validation in Position Open
- [ ] Modify `open_position` to check badge
- [ ] Enforce non-transferable if required
- [ ] Open position with token extensions
- [ ] Integration test
- **Deliverable**: Badge enforcement in positions

### Ring 114: Set Config Feature Flag
- [ ] Create `instructions/set_config_feature_flag.rs`
- [ ] Enable/disable TOKEN_BADGE feature
- [ ] Validate authority
- [ ] Unit test
- **Deliverable**: Feature flag control

### Ring 115: Token Badge Integration Tests
- [ ] Test full badge lifecycle
- [ ] Test badge with pool creation
- [ ] Test non-transferable positions
- [ ] Test feature flag gating
- **Deliverable**: Badge system fully tested

---

## PHASE 8: METADATA SUPPORT (Rings 116-122)

> Metaplex metadata for positions - human-readable NFTs

### Ring 116: Metaplex Integration Setup
- [ ] Add `anchor-spl` metadata feature
- [ ] Create `util/metadata.rs`
- [ ] Define metadata constants (name, symbol, URI patterns)
- **Deliverable**: Metadata utilities

### Ring 117: Open Position With Metadata
- [ ] Create `instructions/open_position_with_metadata.rs`
- [ ] Create position mint
- [ ] Create Metaplex metadata account
- [ ] Set name: "Vortex Position"
- [ ] Set collection if applicable
- [ ] Unit test
- **Deliverable**: Position with Metaplex metadata

### Ring 118: Initialize Position Bundle With Metadata
- [ ] Create `instructions/initialize_position_bundle_with_metadata.rs`
- [ ] Create bundle mint
- [ ] Create Metaplex metadata
- [ ] Unit test
- **Deliverable**: Bundle with metadata

### Ring 119: Open Position With Token Extensions
- [ ] Create `instructions/open_position_with_token_extensions.rs`
- [ ] Use Token-2022 for position mint
- [ ] Add metadata extension (not Metaplex)
- [ ] Handle non-transferable extension if badge requires
- [ ] Unit test
- **Deliverable**: Position with Token-2022 metadata

### Ring 120: Close Position With Token Extensions
- [ ] Create `instructions/close_position_with_token_extensions.rs`
- [ ] Handle Token-2022 position NFT
- [ ] Burn with correct program
- [ ] Close mint account
- [ ] Unit test
- **Deliverable**: Token-2022 position closure

### Ring 121: Position Bumps Struct
- [ ] Create `OpenPositionBumps` struct
- [ ] Fields: `position_bump`, `metadata_bump` (optional)
- [ ] Use in position creation
- **Deliverable**: Clean bump handling

### Ring 122: Metadata Integration Tests
- [ ] Test position with Metaplex metadata
- [ ] Test position with Token-2022 metadata
- [ ] Test bundle with metadata
- [ ] Verify metadata content
- **Deliverable**: Metadata fully tested

---

## PHASE 9: ADAPTIVE FEE INSTRUCTIONS (Rings 123-132)

> Dynamic fees that respond to market volatility

### Ring 123: Initialize Adaptive Fee Tier
- [ ] Create `instructions/initialize_adaptive_fee_tier.rs`
- [ ] Set all volatility parameters
- [ ] Link to base fee tier
- [ ] Set authorities
- [ ] Unit test
- **Deliverable**: Adaptive tier creation

### Ring 124: Set Default Base Fee Rate
- [ ] Create `instructions/set_default_base_fee_rate.rs`
- [ ] Update AdaptiveFeeTier base rate
- [ ] Validate bounds
- [ ] Unit test
- **Deliverable**: Base fee updates

### Ring 125: Set Delegated Fee Authority
- [ ] Create `instructions/set_delegated_fee_authority.rs`
- [ ] Allow delegation of fee setting
- [ ] Validate current authority
- [ ] Unit test
- **Deliverable**: Fee delegation

### Ring 126: Set Initialize Pool Authority
- [ ] Create `instructions/set_initialize_pool_authority.rs`
- [ ] Control who can create pools with this tier
- [ ] Unit test
- **Deliverable**: Pool creation control

### Ring 127: Set Preset Adaptive Fee Constants
- [ ] Create `instructions/set_preset_adaptive_fee_constants.rs`
- [ ] Update: filter_period, decay_period, reduction_factor
- [ ] Update: adaptive_fee_control_factor, max_volatility_accumulator
- [ ] Update: tick_group_size, major_swap_threshold_ticks
- [ ] Validate all bounds
- [ ] Unit test
- **Deliverable**: Volatility parameter updates

### Ring 128: Initialize Pool With Adaptive Fee
- [ ] Create `instructions/initialize_pool_with_adaptive_fee.rs`
- [ ] Use AdaptiveFeeTier instead of FeeTier
- [ ] Initialize volatility accumulator to 0
- [ ] Unit test
- **Deliverable**: Adaptive fee pool creation

### Ring 129: Set Fee Rate By Delegated Authority
- [ ] Create `instructions/set_fee_rate_by_delegated_fee_authority.rs`
- [ ] Allow delegated authority to set fees
- [ ] Validate delegation
- [ ] Unit test
- **Deliverable**: Delegated fee setting

### Ring 130: Adaptive Fee Manager
- [ ] Create `manager/adaptive_fee_manager.rs`
- [ ] Implement volatility accumulation
- [ ] Implement decay calculation
- [ ] Implement fee calculation from accumulator
- [ ] Constants: VOLATILITY_ACCUMULATOR_SCALE_FACTOR = 10000
- [ ] Unit tests
- **Deliverable**: Adaptive fee logic

### Ring 131: Integrate Adaptive Fees in Swap
- [ ] Modify swap to detect adaptive fee pools
- [ ] Calculate current fee from accumulator
- [ ] Update volatility after swap
- [ ] Track ticks crossed
- [ ] Integration test
- **Deliverable**: Adaptive fees in swaps

### Ring 132: Adaptive Fee Integration Tests
- [ ] Test fee increases with volatility
- [ ] Test fee decay over time
- [ ] Test major swap threshold
- [ ] Test bounds enforcement
- **Deliverable**: Adaptive fees fully tested

---

## PHASE 10: ADDITIONAL AUTHORITY INSTRUCTIONS (Rings 133-140)

### Ring 133: Set Reward Authority By Super Authority
- [ ] Create `instructions/set_reward_authority_by_super_authority.rs`
- [ ] Allow super authority to override reward authority
- [ ] Validate super authority
- [ ] Unit test
- **Deliverable**: Super authority override

### Ring 134: Set Reward Emissions Super Authority
- [ ] Create `instructions/set_reward_emissions_super_authority.rs`
- [ ] Transfer super authority
- [ ] Validate current super authority
- [ ] Unit test
- **Deliverable**: Super authority transfer

### Ring 135: Set Default Fee Rate (Fee Tier)
- [ ] Create `instructions/set_default_fee_rate.rs`
- [ ] Update fee tier's default rate
- [ ] Validate fee authority
- [ ] Unit test
- **Deliverable**: Fee tier default update

### Ring 136: Set Default Protocol Fee Rate
- [ ] Create `instructions/set_default_protocol_fee_rate.rs`
- [ ] Update config's default protocol fee
- [ ] Validate authority
- [ ] Unit test
- **Deliverable**: Default protocol fee update

### Ring 137: Dynamic Tick Array Initialize
- [ ] Create `instructions/initialize_dynamic_tick_array.rs`
- [ ] Support idempotent flag (don't fail if exists)
- [ ] Variable size allocation
- [ ] Unit test
- **Deliverable**: Dynamic tick array creation

### Ring 138: Migration Instruction (Repurpose Reward Authority Space)
- [ ] Create `instructions/migrate_repurpose_reward_authority_space.rs`
- [ ] One-time migration helper
- [ ] Handle legacy account format
- [ ] Unit test
- **Deliverable**: Migration support

### Ring 139: IDL Include (For Complete IDL)
- [ ] Create `instructions/idl_include.rs`
- [ ] Include all types in IDL
- [ ] Ensure complete type coverage
- **Deliverable**: Complete IDL generation

### Ring 140: Authority Instruction Module
- [ ] Create `instructions/authority/mod.rs`
- [ ] Organize all authority instructions
- [ ] Clean exports
- **Deliverable**: Authority module complete

---

## PHASE 11: SECURITY HARDENING (Rings 141-152)

### Ring 141: Overflow Protection Audit
- [ ] Audit all arithmetic operations
- [ ] Ensure checked_add/checked_sub/checked_mul used
- [ ] Add overflow tests
- **Deliverable**: Overflow safety verified

### Ring 142: Access Control Audit
- [ ] Audit all authority checks
- [ ] Verify signer requirements
- [ ] Test unauthorized access attempts
- **Deliverable**: Access control verified

### Ring 143: Account Validation
- [ ] Verify all account constraints
- [ ] Check PDA derivations
- [ ] Ensure proper owner checks
- [ ] Test with malicious accounts
- **Deliverable**: Account safety verified

### Ring 144: Price Manipulation Protection
- [ ] Verify sqrt_price_limit enforced
- [ ] Test sandwich attack scenarios
- [ ] Document slippage recommendations
- **Deliverable**: Price manipulation mitigated

### Ring 145: Reentrancy Protection
- [ ] Audit for reentrancy vectors
- [ ] Ensure state updates before external calls
- [ ] Use checks-effects-interactions pattern
- **Deliverable**: Reentrancy safe

### Ring 146: Integer Precision Audit
- [ ] Verify rounding direction (favor protocol)
- [ ] Test edge cases at min/max values
- [ ] Ensure no precision loss in critical paths
- **Deliverable**: Precision verified

### Ring 147: Token Account Validation
- [ ] Verify token program checks
- [ ] Validate mint matches expected
- [ ] Check for token extensions compatibility
- **Deliverable**: Token safety verified

### Ring 148: Signer Validation
- [ ] Audit all is_signer checks
- [ ] Ensure proper authority hierarchy
- [ ] Test with incorrect signers
- **Deliverable**: Signer safety verified

### Ring 149: Rent Exemption
- [ ] Verify all accounts rent-exempt
- [ ] Test account closure returns rent
- [ ] Check minimum lamports requirements
- **Deliverable**: Rent handling verified

### Ring 150: CPI Safety Audit
- [ ] Verify all CPI targets are correct programs
- [ ] Check for CPI privilege escalation
- [ ] Validate account ownership after CPI
- **Deliverable**: CPI safety verified

### Ring 151: Token-2022 Security Audit
- [ ] Audit transfer hook handling
- [ ] Verify transfer fee calculations
- [ ] Test with malicious extensions
- **Deliverable**: Token-2022 safety verified

### Ring 152: Security Test Suite
- [ ] Create comprehensive security test file
- [ ] Test all edge cases
- [ ] Test all attack vectors
- [ ] Document security model
- **Deliverable**: Security test coverage

---

## PHASE 12: OPTIMIZATION (Rings 153-162)

### Ring 153: Compute Unit Profiling
- [ ] Profile each instruction's CU usage
- [ ] Identify hotspots
- [ ] Document baseline metrics
- **Deliverable**: CU baseline established

### Ring 154: Math Optimization
- [ ] Optimize hot paths in math modules
- [ ] Use lookup tables where beneficial
- [ ] Benchmark improvements
- **Deliverable**: Math optimized

### Ring 155: Account Size Optimization
- [ ] Review all struct sizes
- [ ] Pack fields efficiently
- [ ] Minimize rent costs
- **Deliverable**: Account sizes minimized

### Ring 156: Swap Loop Optimization
- [ ] Optimize tick crossing loop
- [ ] Minimize redundant calculations
- [ ] Cache values where possible
- **Deliverable**: Swap optimized

### Ring 157: Serialization Optimization
- [ ] Use zero-copy where beneficial
- [ ] Minimize borsh overhead
- [ ] Benchmark improvements
- **Deliverable**: Serialization optimized

### Ring 158: Memory Optimization
- [ ] Audit stack usage
- [ ] Avoid unnecessary allocations
- [ ] Use references where possible
- **Deliverable**: Memory usage optimized

### Ring 159: Cross-Program Invocation Optimization
- [ ] Minimize CPIs
- [ ] Batch token transfers where possible
- [ ] Optimize invoke_signed calls
- **Deliverable**: CPIs optimized

### Ring 160: Bit Math Optimization
- [ ] Implement MSB (most significant bit) fast path
- [ ] Optimize tick bitmap operations
- [ ] Use bitwise operations for bundle bitmap
- **Deliverable**: Bit operations optimized

### Ring 161: Sparse Swap Optimization
- [ ] Optimize for sparse tick arrays
- [ ] Skip empty regions efficiently
- [ ] Reduce iterations on low-liquidity pools
- **Deliverable**: Sparse swap optimized

### Ring 162: Final Performance Benchmark
- [ ] Full benchmark suite
- [ ] Compare to Orca Whirlpools
- [ ] Document performance characteristics
- **Deliverable**: Performance documented

---

## PHASE 13: COMPREHENSIVE TESTING (Rings 163-178)

### Ring 163: Unit Test Coverage - Math
- [ ] 100% coverage on math modules
- [ ] Edge case tests
- [ ] Property-based tests for invariants
- **Deliverable**: Math fully tested

### Ring 164: Unit Test Coverage - Managers
- [ ] Full coverage on manager modules
- [ ] State transition tests
- [ ] Error condition tests
- **Deliverable**: Managers fully tested

### Ring 165: Integration Tests - Happy Paths
- [ ] Full LP journey test
- [ ] Swap test with multiple tick crossings
- [ ] Multi-hop swap test
- **Deliverable**: Happy paths tested

### Ring 166: Integration Tests - Edge Cases
- [ ] Min/max tick positions
- [ ] Zero liquidity scenarios
- [ ] Price limit hit scenarios
- **Deliverable**: Edge cases tested

### Ring 167: Integration Tests - Rewards
- [ ] Reward emission test
- [ ] Multiple reward programs
- [ ] Reward collection timing
- **Deliverable**: Rewards tested

### Ring 168: Integration Tests - Bundles
- [ ] Full bundle lifecycle
- [ ] Multiple positions in bundle (256 max)
- [ ] Bundle transfer
- **Deliverable**: Bundles tested

### Ring 169: Integration Tests - Oracle
- [ ] Oracle update on swaps
- [ ] TWAP calculation accuracy
- [ ] Circular buffer behavior
- **Deliverable**: Oracle tested

### Ring 170: Integration Tests - Adaptive Fees
- [ ] Volatility accumulation
- [ ] Decay behavior
- [ ] Fee bounds
- **Deliverable**: Adaptive fees tested

### Ring 171: Integration Tests - Token-2022
- [ ] Pool with Token-2022 tokens
- [ ] Transfer hooks execution
- [ ] Transfer fees handling
- **Deliverable**: Token-2022 tested

### Ring 172: Integration Tests - Token Badge
- [ ] Badge creation and deletion
- [ ] Non-transferable positions
- [ ] Feature flag gating
- **Deliverable**: Token badge tested

### Ring 173: Integration Tests - Position Locks
- [ ] Lock creation
- [ ] Locked position transfers
- [ ] Unlock and range reset
- **Deliverable**: Position locks tested

### Ring 174: Integration Tests - Metadata
- [ ] Metaplex metadata positions
- [ ] Token-2022 metadata positions
- [ ] Bundle metadata
- **Deliverable**: Metadata tested

### Ring 175: Fuzz Testing Setup
- [ ] Set up Trident or similar framework
- [ ] Define fuzz targets
- [ ] Configure coverage
- **Deliverable**: Fuzzing infrastructure

### Ring 176: Fuzz Testing - Core Operations
- [ ] Fuzz swap amounts and directions
- [ ] Fuzz liquidity operations
- [ ] Fuzz tick indices
- **Deliverable**: Core fuzzing complete

### Ring 177: Fuzz Testing - Edge Cases
- [ ] Fuzz near boundary values
- [ ] Fuzz with adversarial inputs
- [ ] Run extended campaigns
- **Deliverable**: Edge case fuzzing complete

### Ring 178: Test Documentation
- [ ] Document test coverage (target >95%)
- [ ] Document test methodology
- [ ] Create test matrix
- **Deliverable**: Testing documented

---

## PHASE 14: PRODUCTION READY (Rings 179-200)

### Ring 179: Mainnet Configuration
- [ ] Create mainnet config templates
- [ ] Define production fee tiers
- [ ] Set authority addresses
- **Deliverable**: Mainnet config ready

### Ring 180: Upgrade Authority Strategy
- [ ] Document upgrade process
- [ ] Define upgrade authority multisig
- [ ] Plan for future upgrades
- [ ] Consider timelock mechanisms
- **Deliverable**: Upgrade strategy documented

### Ring 181: SDK Foundation
- [ ] Create TypeScript SDK project
- [ ] Set up build tooling (tsup/rollup)
- [ ] Define core types matching on-chain
- [ ] Implement account fetchers
- **Deliverable**: SDK foundation

### Ring 182: SDK - Account Deserialization
- [ ] Implement Whirlpool account parsing
- [ ] Implement Position account parsing
- [ ] Implement TickArray parsing
- [ ] Handle all account types
- **Deliverable**: Account parsing complete

### Ring 183: SDK - Core Instructions
- [ ] Implement pool initialization
- [ ] Implement position open/close
- [ ] Implement liquidity add/remove
- [ ] Implement swap
- **Deliverable**: Core instruction builders

### Ring 184: SDK - V2 Instructions
- [ ] Implement all V2 (Token-2022) instructions
- [ ] Handle remaining accounts
- [ ] Handle transfer hooks
- **Deliverable**: V2 instruction builders

### Ring 185: SDK - Math Library
- [ ] Port tick math to TypeScript
- [ ] Port token math
- [ ] Port swap math
- [ ] Use BigInt for precision
- **Deliverable**: SDK math complete

### Ring 186: SDK - Quote Functions
- [ ] Implement swap quote
- [ ] Implement liquidity quote
- [ ] Implement fee estimation
- **Deliverable**: Quote functions

### Ring 187: SDK - Position Helpers
- [ ] Implement position value calculation
- [ ] Implement fee accrued calculation
- [ ] Implement reward accrued calculation
- **Deliverable**: Position helpers

### Ring 188: SDK - Testing
- [ ] Unit tests for all SDK functions
- [ ] Integration tests against localnet
- [ ] E2E tests for common flows
- **Deliverable**: SDK fully tested

### Ring 189: Documentation - Architecture
- [ ] System architecture overview
- [ ] Account relationships diagram
- [ ] Instruction flow diagrams
- [ ] State machine documentation
- **Deliverable**: Architecture docs

### Ring 190: Documentation - API Reference
- [ ] Document all instructions
- [ ] Document all account structures
- [ ] Document all error codes
- [ ] Generate from IDL
- **Deliverable**: API reference

### Ring 191: Documentation - Integration Guide
- [ ] Quick start guide
- [ ] Common use cases
- [ ] Best practices
- [ ] Troubleshooting guide
- **Deliverable**: Integration guide

### Ring 192: Documentation - Security Model
- [ ] Document authority hierarchy
- [ ] Document access control
- [ ] Document trust assumptions
- [ ] Security best practices
- **Deliverable**: Security documentation

### Ring 193: Deployment - Localnet
- [ ] Create localnet setup script
- [ ] Deploy all programs
- [ ] Initialize test config
- [ ] Create demo pools
- **Deliverable**: Localnet deployment

### Ring 194: Deployment - Devnet
- [ ] Deploy to devnet
- [ ] Initialize production config
- [ ] Create test pools
- [ ] Verify all instructions
- **Deliverable**: Devnet deployment

### Ring 195: Deployment - Verification
- [ ] Verify program on-chain matches source
- [ ] Anchor verify
- [ ] Publish verified IDL
- **Deliverable**: Verified deployment

### Ring 196: Monitoring Setup
- [ ] Set up pool analytics
- [ ] Transaction monitoring
- [ ] Error alerting
- **Deliverable**: Monitoring infrastructure

### Ring 197: Error Codes Documentation
- [ ] Document all 67+ error codes
- [ ] Create error handling guide
- [ ] Map errors to solutions
- **Deliverable**: Error documentation

### Ring 198: Performance Documentation
- [ ] Document CU usage per instruction
- [ ] Document account sizes
- [ ] Compare with Orca benchmarks
- **Deliverable**: Performance docs

### Ring 199: Final Code Review
- [ ] Full codebase review
- [ ] Remove dead code
- [ ] Final formatting pass
- [ ] License headers
- **Deliverable**: Production-ready code

### Ring 200: Launch Checklist
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
- [ ] All 200 rings completed
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
| V1 Core | 31 | Rings 51-75 |
| V1 Extended | 7 | Rings 76-91 |
| Token-2022 V2 | 11 | Rings 92-105 |
| Token Badge | 5 | Rings 106-115 |
| Metadata | 4 | Rings 116-122 |
| Adaptive Fee | 7 | Rings 123-132 |
| Additional Authority | 8 | Rings 133-140 |
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
| DynamicTickArray | Variable | 37 |
| Position | 216 bytes | 28-30 |
| PositionBundle | 136 bytes | 31 |
| LockConfig | 201 bytes | 36 |
| TokenBadge | 200 bytes | 35 |

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

*This roadmap covers 200 rings across 14 phases. Each ring is a single brush stroke. Complete them all, and you'll have a production-grade CLMM that rivals Orca Whirlpools.*
