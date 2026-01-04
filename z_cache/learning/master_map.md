# MASTER MAP: Whirlpools CLMM Codebase

## Source
- **Type**: GitHub Repository (Orca Whirlpools)
- **Link**: Local clone at `/Users/veerbalsingh/Documents/work/defi/clmm/whirlpools`
- **Key Files**:
  - `programs/whirlpool/src/lib.rs` - Main program entry point
  - `programs/whirlpool/src/state/` - All data structures
  - `programs/whirlpool/src/instructions/` - All program instructions
  - `programs/whirlpool/src/math/` - Mathematical operations
  - `programs/whirlpool/src/manager/` - Business logic managers

---

## The Bucket — All Concepts to Cover

### Foundation Layer (What is this thing?)
1. What is trading / exchanging tokens
2. What is a DEX (Decentralized Exchange)
3. What is an AMM (Automated Market Maker)
4. How traditional order books work
5. Why AMMs replaced order books on-chain

### CPMM Basics (The "old" way)
6. What is a liquidity pool (the simple version)
7. What is a liquidity provider (LP)
8. The x * y = k formula (Constant Product)
9. What slippage means
10. Problem: Capital inefficiency in CPMM

### CLMM Core Concept (The "new" way)
11. What is Concentrated Liquidity
12. Why concentrate liquidity (capital efficiency)
13. Price ranges - the core idea
14. Comparison: CPMM vs CLMM capital efficiency

### Price Representation
15. What is "price" in a pool
16. Why we use sqrt(price) instead of price
17. What Q64.64 fixed-point means (simple version)
18. sqrt_price in the code

### The Tick System
19. What is a tick (single discrete price point)
20. Why prices are discrete, not continuous
21. tick_spacing - trading off granularity vs gas
22. Tick index and how it maps to price
23. MIN_TICK_INDEX and MAX_TICK_INDEX boundaries
24. The tick math formula: price = 1.0001^tick_index

### Tick Data Structure
25. The `Tick` struct in code
26. `initialized` flag - what it means
27. `liquidity_net` - liquidity change at this tick
28. `liquidity_gross` - total liquidity referencing this tick
29. Why we track both net and gross

### Tick Arrays (Storage Optimization)
30. Why we can't store every tick on-chain
31. What is a TickArray (container for 88 ticks)
32. `TICK_ARRAY_SIZE` constant (88)
33. `start_tick_index` of a tick array
34. How to find which TickArray holds a tick
35. FixedTickArray vs DynamicTickArray

### Positions (Your liquidity stake)
36. What is a Position
37. Position as an NFT (ownership token)
38. `tick_lower_index` and `tick_upper_index`
39. Position `liquidity` field
40. When your position is "in range" vs "out of range"
41. Position fee tracking: `fee_growth_checkpoint_a/b`
42. Position fee collection: `fee_owed_a/b`

### The Whirlpool (Pool State)
43. What the `Whirlpool` struct represents
44. `token_mint_a` and `token_mint_b` (the pair)
45. `token_vault_a` and `token_vault_b` (where tokens live)
46. `sqrt_price` - current pool price
47. `tick_current_index` - current tick
48. `liquidity` - active liquidity in current tick range

### Fee Mechanics
49. What is `fee_rate` (trading fee)
50. What is `protocol_fee_rate` (Orca's cut)
51. `fee_growth_global_a/b` - accumulated fees per liquidity
52. How fees are distributed to LPs

### Configuration Layer
53. What is `WhirlpoolsConfig`
54. `fee_authority` and `collect_protocol_fees_authority`
55. `reward_emissions_super_authority`
56. What is a `FeeTier`

### Reward System
57. What are reward emissions (LP incentives)
58. `WhirlpoolRewardInfo` structure
59. Reward vaults and mints
60. `growth_global_x64` for rewards

### Core Instructions - Pool Lifecycle
61. `initialize_config` instruction
62. `initialize_fee_tier` instruction
63. `initialize_pool` instruction
64. `initialize_tick_array` instruction

### Core Instructions - Positions
65. `open_position` instruction
66. `increase_liquidity` instruction
67. `decrease_liquidity` instruction
68. `close_position` instruction

### Core Instructions - Trading
69. `swap` instruction overview
70. Swap direction: a_to_b vs b_to_a
71. How swap moves through ticks
72. sqrt_price_limit in swaps
73. `two_hop_swap` for routing

### Core Instructions - Collecting
74. `collect_fees` instruction
75. `update_fees_and_rewards` instruction
76. `collect_reward` instruction
77. `collect_protocol_fees` instruction

### Math Layer
78. `tick_math.rs` - tick ↔ price conversions
79. `swap_math.rs` - swap calculations
80. `liquidity_math.rs` - liquidity calculations
81. `token_math.rs` - token amount calculations
82. `u256_math.rs` - big number operations

### Manager Layer (Business Logic)
83. `swap_manager.rs` - orchestrates swaps
84. `liquidity_manager.rs` - handles liquidity changes
85. `position_manager.rs` - position updates
86. `tick_manager.rs` - tick crossing logic
87. `fee_rate_manager.rs` - fee calculations

### Advanced Topics — Position Bundles
88. What is a Position Bundle (NFT container for multiple positions)
89. Why bundles exist (gas savings, portfolio management)
90. Bundle index system (256 slots)
91. Open/close bundled positions

### Advanced Topics — Dynamic Tick Arrays
92. Fixed vs Dynamic tick arrays
93. Why variable-length arrays (storage optimization)
94. When to use each type

### Advanced Topics — Adaptive Fee System (Volatility-Based)
95. What is adaptive fee (fees that change with market conditions)
96. Volatility accumulator concept
97. Decay period and reduction factor
98. Major swap threshold detection
99. Tick group size for volatility measurement
100. `AdaptiveFeeTier` struct deep dive

### Advanced Topics — Oracle (TWAP)
101. What is an on-chain oracle
102. TWAP (Time-Weighted Average Price) concept
103. Oracle observations array
104. How swaps update oracle state
105. Reading historical prices from oracle

### Advanced Topics — Position Locks
106. Lock types (preventing liquidity changes)
107. Locked position transfers
108. Reset position range (after unlock)

### Advanced Topics — Bit Math Utilities
109. Why bit manipulation in CLMM
110. Most significant bit (MSB) operations
111. Tick bitmap patterns

---

## Concept Dependencies

- Concepts 6-10 require: 1-5
- Concepts 11-14 require: 6-10
- Concepts 15-18 require: 11-14
- Concepts 19-24 require: 15-18
- Concepts 25-29 require: 19-24
- Concepts 30-35 require: 25-29
- Concepts 36-42 require: 19-24, 30-35
- Concepts 43-48 require: 18, 24, 35, 42
- Concepts 49-52 require: 43-48
- Concepts 53-56 require: 49-52
- Concepts 57-60 require: 43-48
- Concepts 61-64 require: 53-56
- Concepts 65-68 require: 36-42, 61-64
- Concepts 69-73 require: 43-48, 61-64
- Concepts 74-77 require: 49-52, 57-60
- Concepts 78-82 require: 69-73
- Concepts 83-87 require: 78-82
- Concepts 88-91 require: 36-42 (Position Bundles need Position knowledge)
- Concepts 92-94 require: 30-35 (Dynamic Tick Arrays need Tick Array knowledge)
- Concepts 95-100 require: 49-52, 83-87 (Adaptive Fees need Fee System + Managers)
- Concepts 101-105 require: 43-48, 69-73 (Oracle needs Pool State + Swap knowledge)
- Concepts 106-108 require: 36-42 (Position Locks need Position knowledge)
- Concepts 109-111 require: 78-82 (Bit Math is part of Math layer)

---

## Ring Plan

### Ring 1: Absolute Basics (12-year-old level)
**Concepts: 1-5**
- What is trading
- What is a DEX
- Why computers do the trading (AMM)
- No jargon yet

### Ring 2: The Pool Idea
**Concepts: 6-10**
- Pool = two piles of tokens
- Anyone can add to the piles (LP)
- The magic balancing formula
- Problem: most tokens sit unused

### Ring 3: Concentrated Liquidity Core
**Concepts: 11-14**
- Focus your tokens in a range
- More trades, more fees (for you)
- The efficiency comparison

### Ring 4: Price in Code
**Concepts: 15-18**
- Price = ratio of token amounts
- Why sqrt(price)
- Fixed-point numbers (simple)

### Ring 5: Ticks Introduction
**Concepts: 19-24**
- Price as steps on a ladder
- Why steps instead of smooth
- tick_spacing trade-off
- The 1.0001 formula

### Ring 6: Tick Data Structure
**Concepts: 25-29**
- Looking at actual Tick struct
- What each field means
- net vs gross liquidity

### Ring 7: Tick Arrays
**Concepts: 30-35**
- Storage problem
- 88 ticks in a container
- Finding the right array

### Ring 8: Positions
**Concepts: 36-42**
- Your liquidity stake
- NFT representation
- Tick boundaries
- In range vs out of range

### Ring 9: The Pool State
**Concepts: 43-48**
- The Whirlpool struct
- Token mints and vaults
- Current price state

### Ring 10: Fee System
**Concepts: 49-52**
- Trading fees
- Protocol fees
- Fee distribution

### Ring 11: Configuration
**Concepts: 53-56**
- Config accounts
- Authorities
- Fee tiers

### Ring 12: Rewards
**Concepts: 57-60**
- LP incentives
- Reward tracking

### Ring 13: Pool Lifecycle Instructions
**Concepts: 61-64**
- Creating config
- Creating fee tier
- Creating pool
- Creating tick arrays

### Ring 14: Position Instructions
**Concepts: 65-68**
- Opening positions
- Adding liquidity
- Removing liquidity
- Closing positions

### Ring 15: Swap Mechanics
**Concepts: 69-73**
- How swaps work
- Price limits
- Multi-hop swaps

### Ring 16: Collection Instructions
**Concepts: 74-77**
- Collecting fees
- Updating rewards
- Protocol fee collection

### Ring 17: Math Deep Dive
**Concepts: 78-82**
- Tick math
- Swap math
- Token calculations

### Ring 18: Manager Logic
**Concepts: 83-87**
- Business logic layer
- How managers orchestrate operations

### Ring 19: Position Bundles
**Concepts: 88-91**
- NFT container for multiple positions
- Gas-efficient batch management
- Bundle index system (256 slots)

### Ring 20: Dynamic Tick Arrays
**Concepts: 92-94**
- Fixed vs variable-length storage
- Storage optimization patterns

### Ring 21: Adaptive Fee System
**Concepts: 95-100**
- Volatility-based dynamic fees
- Market condition detection
- Fee adjustment mechanics
- **Career highlight: cutting-edge DeFi feature**

### Ring 22: Oracle / TWAP
**Concepts: 101-105**
- On-chain price history
- Time-weighted average pricing
- **Career highlight: every DeFi protocol needs this**

### Ring 23: Position Locks & Bit Math
**Concepts: 106-111**
- Position locking mechanics
- Bit manipulation for efficiency

---

## Vocabulary Restrictions Per Ring

### Ring 1
- NO: "smart contract", "account", "liquidity", "AMM", "protocol"
- USE: "program", "storage", "tokens", "computer trader", "trading system"

### Ring 2
- CAN NOW USE: "liquidity", "pool", "LP", "formula"
- Still NO: "tick", "sqrt", "invariant"

### Ring 3
- CAN NOW USE: "concentrated", "range", "capital efficiency"
- Still NO: "tick", "sqrt_price", "Q64.64"

### Ring 4
- CAN NOW USE: "sqrt_price", "fixed-point"
- Still NO: "tick", "tick_index"

### Ring 5
- CAN NOW USE: "tick", "tick_index", "tick_spacing"
- Still NO: "TickArray", "liquidity_net"

### Ring 6
- CAN NOW USE: "liquidity_net", "liquidity_gross", "fee_growth_outside"

### Ring 7
- CAN NOW USE: "TickArray", "start_tick_index", "TICK_ARRAY_SIZE"

### Ring 8
- CAN NOW USE: "Position", "tick_lower_index", "tick_upper_index", "fee_owed"

### Ring 9
- CAN NOW USE: "Whirlpool struct", "token_vault", "tick_current_index"

### Ring 10+
- All technical terms are unlocked progressively
