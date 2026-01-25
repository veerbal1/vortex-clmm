# L FORMULA QUICK REFERENCE CARD

**Keep this open while coding!**

---

## THE TWO MASTER FORMULAS

```
┌──────────────────────────────────────────┐
│  Δy = L × (√P_upper - √P_lower)         │  ← LINEAR
└──────────────────────────────────────────┘

┌──────────────────────────────────────────┐
│  Δx = L × (1/√P_lower - 1/√P_upper)     │  ← RECIPROCAL
└──────────────────────────────────────────┘
```

---

## QUICK LOOKUP TABLE

| Want to calculate... | Formula | Notes |
|---------------------|---------|-------|
| **Token B amount from L** | `Δy = L × (√P_b - √P_a)` | Simple linear |
| **Token A amount from L** | `Δx = L × (1/√P_a - 1/√P_b)` | Reciprocal |
| **L from token B** | `L = Δy / (√P_b - √P_a)` | Inverse of above |
| **L from token A** | `L = Δx × (√P_a × √P_b) / (√P_b - √P_a)` | Inverse of above |
| **Price change from trade** | `Δ√P = Δy / L` | For token B trades |
| **Token A at current price** | `x = L × (1/√P_c - 1/√P_b)` | If in range |
| **Token B at current price** | `y = L × (√P_c - √P_a)` | If in range |

---

## POSITION STATE FORMULAS

### Below Range (√P < √P_a)
```rust
x = L × (1/√P_a - 1/√P_b)  // Full amount
y = 0                       // All sold
```

### In Range (√P_a ≤ √P ≤ √P_b)
```rust
x = L × (1/√P - 1/√P_b)    // Partial
y = L × (√P - √P_a)         // Partial
```

### Above Range (√P > √P_b)
```rust
x = 0                       // All sold
y = L × (√P_b - √P_a)       // Full amount
```

---

## CODE TEMPLATES

### Calculate Token Amounts from L
```rust
fn get_amount_a_delta(
    sqrt_price_lower: u128,
    sqrt_price_upper: u128,
    liquidity: u128,
    round_up: bool,
) -> u64 {
    // Δx = L × (√P_upper - √P_lower) / (√P_lower × √P_upper)
    let sqrt_price_diff = sqrt_price_upper - sqrt_price_lower;
    let numerator = (liquidity as u256) * (sqrt_price_diff as u256);
    let denominator = (sqrt_price_lower as u256) * (sqrt_price_upper as u256);

    let amount = if round_up {
        (numerator + denominator - 1) / denominator
    } else {
        numerator / denominator
    };

    amount as u64
}

fn get_amount_b_delta(
    sqrt_price_lower: u128,
    sqrt_price_upper: u128,
    liquidity: u128,
    round_up: bool,
) -> u64 {
    // Δy = L × (√P_upper - √P_lower)
    let sqrt_price_diff = sqrt_price_upper - sqrt_price_lower;

    let amount = if round_up {
        q64_64::mul_round_up(liquidity, sqrt_price_diff) >> 64
    } else {
        (liquidity * sqrt_price_diff) >> 64
    };

    amount as u64
}
```

### Calculate L from Token Amounts
```rust
fn get_liquidity_from_amount_a(
    sqrt_price_lower: u128,
    sqrt_price_upper: u128,
    amount: u64,
) -> u128 {
    // L = Δx × (√P_lower × √P_upper) / (√P_upper - √P_lower)
    let sqrt_price_diff = sqrt_price_upper - sqrt_price_lower;
    let numerator = (amount as u256)
        * (sqrt_price_lower as u256)
        * (sqrt_price_upper as u256);
    let denominator = sqrt_price_diff as u256;

    (numerator / denominator) as u128
}

fn get_liquidity_from_amount_b(
    sqrt_price_lower: u128,
    sqrt_price_upper: u128,
    amount: u64,
) -> u128 {
    // L = Δy / (√P_upper - √P_lower)
    let sqrt_price_diff = sqrt_price_upper - sqrt_price_lower;
    ((amount as u128) << 64) / sqrt_price_diff
}
```

### Calculate L from Both Amounts (at current price)
```rust
fn get_liquidity_from_amounts(
    sqrt_price_current: u128,
    sqrt_price_lower: u128,
    sqrt_price_upper: u128,
    amount_a: u64,
    amount_b: u64,
) -> u128 {
    if sqrt_price_current <= sqrt_price_lower {
        // Below range: only use amount_a
        get_liquidity_from_amount_a(
            sqrt_price_lower,
            sqrt_price_upper,
            amount_a,
        )
    } else if sqrt_price_current >= sqrt_price_upper {
        // Above range: only use amount_b
        get_liquidity_from_amount_b(
            sqrt_price_lower,
            sqrt_price_upper,
            amount_b,
        )
    } else {
        // In range: use both, take minimum
        let liquidity_a = get_liquidity_from_amount_a(
            sqrt_price_current,
            sqrt_price_upper,
            amount_a,
        );
        let liquidity_b = get_liquidity_from_amount_b(
            sqrt_price_lower,
            sqrt_price_current,
            amount_b,
        );

        std::cmp::min(liquidity_a, liquidity_b)
    }
}
```

---

## COMMON OPERATIONS

### Adding Liquidity
```rust
// 1. Calculate L from user's token amounts
let liquidity = get_liquidity_from_amounts(
    pool.sqrt_price,
    tick_lower_sqrt_price,
    tick_upper_sqrt_price,
    amount_a,
    amount_b,
);

// 2. Calculate actual amounts to deposit (might be less)
let amount_a_needed = get_amount_a_delta(
    tick_lower_sqrt_price,
    tick_upper_sqrt_price,
    liquidity,
    true, // round up for deposits
);
let amount_b_needed = get_amount_b_delta(
    tick_lower_sqrt_price,
    tick_upper_sqrt_price,
    liquidity,
    true, // round up for deposits
);

// 3. Update position
position.liquidity += liquidity;

// 4. Update ticks
tick_lower.liquidity_net += liquidity as i128;
tick_upper.liquidity_net -= liquidity as i128;

// 5. Update pool (if in range)
if in_range {
    pool.liquidity += liquidity;
}
```

### Removing Liquidity
```rust
// 1. Calculate amounts to withdraw
let amount_a = get_amount_a_delta(
    tick_lower_sqrt_price,
    tick_upper_sqrt_price,
    liquidity_to_remove,
    false, // round down for withdrawals
);
let amount_b = get_amount_b_delta(
    tick_lower_sqrt_price,
    tick_upper_sqrt_price,
    liquidity_to_remove,
    false, // round down for withdrawals
);

// 2. Update position
position.liquidity -= liquidity_to_remove;

// 3. Update ticks
tick_lower.liquidity_net -= liquidity_to_remove as i128;
tick_upper.liquidity_net += liquidity_to_remove as i128;

// 4. Update pool (if in range)
if in_range {
    pool.liquidity -= liquidity_to_remove;
}
```

### Price Impact of Swap
```rust
// For token B → token A swap
let sqrt_price_change = (amount_b << 64) / pool.liquidity;
let new_sqrt_price = pool.sqrt_price + sqrt_price_change;

// For token A → token B swap (more complex)
let amount_a_shifted = (amount_a as u256) << 64;
let product = (pool.sqrt_price as u256) * (pool.liquidity as u256);
let sqrt_price_change = amount_a_shifted / product;
let new_sqrt_price = pool.sqrt_price - sqrt_price_change;
```

---

## CRITICAL GOTCHAS

### 1. Rounding Direction
```
DEPOSITS:    round_up = true   (protocol benefits)
WITHDRAWALS: round_up = false  (protocol benefits)
```

### 2. Q64.64 Bit Shifting
```rust
// Converting to Q64.64
let q64_value = (value as u128) << 64;

// Converting from Q64.64
let normal_value = (q64_value >> 64) as u64;

// Rounding up from Q64.64
let rounded_up = ((q64_value + ((1 << 64) - 1)) >> 64) as u64;
```

### 3. U256 for Overflow Prevention
```rust
// Always use U256 for (u128 × u128)
let product = (a as u256) * (b as u256);
let result = (product / (c as u256)) as u128;
```

### 4. Liquidity Net is Signed
```rust
// liquidity_net is i128 (can be negative!)
tick.liquidity_net += liquidity_delta as i128;  // entering
tick.liquidity_net -= liquidity_delta as i128;  // exiting
```

### 5. In-Range Check
```rust
let in_range =
    pool.tick_current_index >= tick_lower_index &&
    pool.tick_current_index < tick_upper_index;
// Note: tick_upper is EXCLUSIVE
```

---

## DEBUG CHECKLIST

When your calculations seem wrong, check:

- [ ] Are you using the correct √P values (not regular P)?
- [ ] Is the rounding direction correct (up for deposits, down for withdrawals)?
- [ ] Did you use U256 for multiplication that might overflow?
- [ ] Are you bit-shifting correctly for Q64.64 conversions?
- [ ] Is liquidity_net signed (i128) where needed?
- [ ] Are you checking in_range correctly?
- [ ] Did you update both ticks AND pool liquidity?
- [ ] Are you handling the three cases (below/in/above range)?

---

## MENTAL MODELS

### L is...
- The "slope" of the price vs token amount relationship
- The "thickness" of your liquidity segment
- The "depth" that prevents price slippage
- The constant that connects price changes to token changes

### √P is...
- Not the price! It's the square root of price
- Used because it makes the math linear and symmetric
- Related to geometric mean of token amounts
- The reason token B formula is so simple

### Range [√P_a, √P_b] is...
- Where your position "exists"
- Where you earn fees
- Where your capital is concentrated
- The bounds of your chosen price exposure

---

## FINAL WISDOM

### The Universal Truth:
```
Everything in CLMM comes down to:
  1. What is L?
  2. What is the range [√P_a, √P_b]?
  3. What is the current price √P?

Master these three, and everything else is just arithmetic.
```

### The Two Formulas:
```
Δy = L × (√P_upper - √P_lower)     [LINEAR - easy]
Δx = L × (1/√P_lower - 1/√P_upper)  [RECIPROCAL - slightly harder]

These two formulas are the KEY to the entire protocol.
Memorize them. Understand them. Live them.
```

---

**Pin this reference card to your screen while building!** 📌
