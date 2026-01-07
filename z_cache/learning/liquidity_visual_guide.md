# L VISUAL GUIDE: Formulas & Diagrams

**Companion to liquidity_deep_dive.md**

---

## THE TWO MASTER FORMULAS

### Formula 1: Token B Amount (The Simple One)
```
┌─────────────────────────────────────┐
│  Δy = L × (√P_upper - √P_lower)    │
└─────────────────────────────────────┘

Where:
  Δy         = Amount of token B needed
  L          = Liquidity constant
  √P_upper   = Square root of upper price bound
  √P_lower   = Square root of lower price bound
```

**Visual interpretation:**
```
  Token B
  Amount
    ↑
    │     ╱
    │    ╱  ← Slope = L
    │   ╱
    │  ╱
    │ ╱
    └──────────→ √Price
      √P_a  √P_b

The amount of token B increases LINEARLY with √P
The slope of this line is L
```

---

### Formula 2: Token A Amount (The Reciprocal One)
```
┌─────────────────────────────────────────────┐
│  Δx = L × (1/√P_lower - 1/√P_upper)        │
└─────────────────────────────────────────────┘

Alternative form:
┌──────────────────────────────────────────────────────┐
│  Δx = L × (√P_upper - √P_lower)                     │
│          ───────────────────────                     │
│          √P_lower × √P_upper                         │
└──────────────────────────────────────────────────────┘

Where:
  Δx         = Amount of token A needed
  L          = Liquidity constant
  √P_lower   = Square root of lower price bound
  √P_upper   = Square root of upper price bound
```

**Visual interpretation:**
```
  Token A
  Amount
    ↑
    │╲
    │ ╲
    │  ╲  ← Curve (hyperbolic)
    │   ╲
    │    ╲___
    └──────────→ √Price
      √P_a  √P_b

The amount of token A decreases as √P increases
This is a hyperbolic relationship (1/√P)
```

---

## COMPLETE POSITION VISUALIZATION

### Scenario: Position with L=1000, Range [√P=10, √P=12]

```
                    Current Price
                         ↓
Price:    √P=9   √P=10   √P=11   √P=12   √P=13
          ─────────┼───────┼───────┼─────────
                   │       │       │
                   └───────┴───────┘
                   Your Range [10,12]


Case 1: Price at √P=9 (BELOW range)
┌──────────────────────────────────────┐
│  Position Holds:                     │
│  Token A: 16.67 ████████████████     │
│  Token B: 0                          │
│                                      │
│  Status: OUT OF RANGE (no fees)     │
└──────────────────────────────────────┘

Calculation:
  Δx = 1000 × (1/10 - 1/12) = 1000 × 0.0167 = 16.67
  Δy = 0 (all token B was sold already)


Case 2: Price at √P=11 (IN range)
┌──────────────────────────────────────┐
│  Position Holds:                     │
│  Token A: 7.58 ████████              │
│  Token B: 1000 ████████████████████  │
│                                      │
│  Status: ACTIVE (earning fees!)     │
└──────────────────────────────────────┘

Calculation:
  x = 1000 × (1/11 - 1/12) = 1000 × 0.0076 = 7.58
  y = 1000 × (11 - 10) = 1000


Case 3: Price at √P=13 (ABOVE range)
┌──────────────────────────────────────┐
│  Position Holds:                     │
│  Token A: 0                          │
│  Token B: 2000 ████████████████████  │
│                                      │
│  Status: OUT OF RANGE (no fees)     │
└──────────────────────────────────────┘

Calculation:
  Δx = 0 (all token A was bought)
  Δy = 1000 × (12 - 10) = 2000
```

---

## UNDERSTANDING THE RANGE BOUNDARIES

### What √P_lower and √P_upper Mean

```
         √P_lower = 10              √P_upper = 12
              ↓                           ↓
    ──────────●═══════════════════════════●──────────
              │                           │
              │    Your Liquidity        │
              │    Exists Here           │
              └───────────────────────────┘

Token Composition Changes Across Range:

At √P=10:     [████████████ 100% A, 0% B]
At √P=10.5:   [██████████ 80% A, 20% B]
At √P=11:     [████ 40% A, 60% B]
At √P=11.5:   [██ 20% A, 80% B]
At √P=12:     [0% A, 100% B ████████████]

As price increases →
  Token A decreases (being sold)
  Token B increases (being bought)
```

---

## LIQUIDITY CONCENTRATION COMPARISON

### Same Capital, Different Concentration

#### Scenario: You have 10,000 tokens to deploy

**Option 1: Wide Range (LOW concentration)**
```
Range: [√P=5, √P=15]
Width: 10 units

       5              10              15
       ├──────────────┼──────────────┤
       │░░░░░░░░░░░░░░░░░░░░░░░░░░░░│  ← L = 1000
       └──────────────────────────────┘

Liquidity Density: 1000 / 10 = 100 per unit
Fee Share at √P=10: 5% (competing with many LPs)
```

**Option 2: Narrow Range (HIGH concentration)**
```
Range: [√P=9, √P=11]
Width: 2 units

            9      10      11
            ├──────┼──────┤
            │██████│██████│  ← L = 5000
            └──────────────┘

Liquidity Density: 5000 / 2 = 2500 per unit
Fee Share at √P=10: 25% (concentrated power!)
```

**Key Insight:**
Same capital, but Option 2 has:
- 5x higher L
- 25x higher density
- 5x more fees per trade (when in range)
- Higher risk (easier to exit range)

---

## MULTIPLE POSITIONS = LAYERED LIQUIDITY

### Three LPs at Different Ranges

```
Price Range:  8    9    10    11    12    13    14
             ─────┼────┼────┼────┼────┼────┼────

LP1 (L=1000) ════════════════════════════════  [8,14]
LP2 (L=500)       ══════════════════           [9,13]
LP3 (L=2000)           ══════════              [10,12]

Active L at each price:
  √P=8:   L=1000  (only LP1)
  √P=9:   L=1500  (LP1+LP2)
  √P=10:  L=3500  (LP1+LP2+LP3)  ← Most liquid!
  √P=11:  L=3500  (LP1+LP2+LP3)
  √P=12:  L=1500  (LP1+LP2)
  √P=13:  L=1000  (only LP1)
  √P=14:  L=0     (no one)

This creates a "liquidity hill" centered at [10,12]
```

---

## TICK BOUNDARIES & LIQUIDITY_NET

### How liquidity_net Tracks L Changes

```
Tick Index:  90   100   110   120   130
            ─┼─────┼─────┼─────┼─────┼─

Position A (L=1000):  [100, 120]
Position B (L=500):   [110, 130]

Liquidity_net at each tick:
  Tick 90:  net=0
  Tick 100: net=+1000  ← Position A enters
  Tick 110: net=+500   ← Position B enters
  Tick 120: net=-1000  ← Position A exits
  Tick 130: net=-500   ← Position B exits

Walking through ticks (left to right):
  Start:    L = 0
  @ 100:    L = 0 + 1000 = 1000
  @ 110:    L = 1000 + 500 = 1500
  @ 120:    L = 1500 - 1000 = 500
  @ 130:    L = 500 - 500 = 0
  End:      L = 0
```

**Key Insight:**
You can compute active L at any price by summing liquidity_net from left to right!

---

## SWAP MECHANICS WITH L

### Example Swap: Buy 1000 token B

```
Initial State:
  √P = 10.0
  L = 5000
  Pool holds: 500 token A, 50,000 token B

Trader deposits: 1000 token B
Question: How much does price move?

From: Δy = L × Δ√P
      1000 = 5000 × Δ√P
      Δ√P = 1000 / 5000
      Δ√P = 0.2

New price: √P = 10.0 + 0.2 = 10.2

Trader receives:
  Δx = L × (1/√P_old - 1/√P_new)
     = 5000 × (1/10.0 - 1/10.2)
     = 5000 × (0.1 - 0.098)
     = 5000 × 0.002
     = 10 token A

Final State:
  √P = 10.2
  L = 5000 (unchanged)
  Pool holds: 490 token A, 51,000 token B
```

**Visual:**
```
  √P
   ↑
10.2│         ●  ← End position
    │        ╱
10.0│   ●───╱
    │      ↖ Price moved by Δ√P=0.2
    │
    └────────→ Time
       Swap
```

---

## CALCULATING L FROM TOKEN AMOUNTS

### Scenario: You want to provide liquidity

**Given:**
- Range: [√P=10, √P=12]
- You have: 100 token A, 2000 token B
- Current price: √P=11 (in range)

**Step 1: Calculate L from token A**
```
L_A = Δx / (1/√P_current - 1/√P_upper)
    = 100 / (1/11 - 1/12)
    = 100 / 0.00758
    = 13,193
```

**Step 2: Calculate L from token B**
```
L_B = Δy / (√P_current - √P_lower)
    = 2000 / (11 - 10)
    = 2000 / 1
    = 2000
```

**Step 3: Take minimum**
```
L = min(13,193, 2000) = 2000

Why? Because token B runs out first!
```

**Step 4: Calculate actual amounts used**
```
Using L=2000:

Token A needed:
  x = 2000 × (1/11 - 1/12) = 15.15

Token B needed:
  y = 2000 × (11 - 10) = 2000

Result:
  Uses: 15.15 token A, 2000 token B
  Leftover: 84.85 token A, 0 token B
```

---

## POSITION LIFECYCLE DIAGRAM

### From Opening to Closing

```
Step 1: OPEN POSITION
┌─────────────────────────┐
│ Create NFT              │
│ Set range [√P_a, √P_b] │
│ L = 0 (empty)           │
└─────────────────────────┘
         ↓

Step 2: ADD LIQUIDITY (increase_liquidity)
┌─────────────────────────┐
│ Deposit tokens          │
│ Calculate L from amounts│
│ Update tick boundaries  │
│ L = 1000 (active)       │
└─────────────────────────┘
         ↓

Step 3: EARN FEES (while in range)
┌─────────────────────────┐
│ Traders swap            │
│ Fees accumulate         │
│ Your share ∝ L / L_total│
└─────────────────────────┘
         ↓

Step 4: COLLECT FEES
┌─────────────────────────┐
│ Claim accumulated fees  │
│ L unchanged             │
└─────────────────────────┘
         ↓

Step 5: REMOVE LIQUIDITY (decrease_liquidity)
┌─────────────────────────┐
│ Withdraw tokens         │
│ Update tick boundaries  │
│ L = 0 (empty)           │
└─────────────────────────┘
         ↓

Step 6: CLOSE POSITION
┌─────────────────────────┐
│ Burn NFT                │
│ Reclaim rent            │
└─────────────────────────┘
```

---

## THE CONSTANT PRODUCT CURVE VISUALIZATION

### Full CPMM vs CLMM Segment

```
Token B
   ↑
   │     Full CPMM curve (x × y = L²)
   │    ╱
   │   ╱
   │  ╱
   │ ╱
   │╱________________
   └─────────────────→ Token A


Your CLMM position is just a PIECE of this curve:

Token B
   │
1000│     ●────────●   ← Your position
   │    ╱          ╲    [√P=10, √P=12]
   │   ╱            ╲
   │  ╱              ╲
   │ ╱                ╲
   └────────────────────→ Token A
      ↑              ↑
   At √P=10      At √P=12

Outside your range, you DON'T provide liquidity
Inside your range, you behave like a regular CPMM
```

---

## FORMULA CHEAT SHEET

### Core Formulas (MEMORIZE THESE)

```
┌────────────────────────────────────────────────────┐
│ TOKEN AMOUNTS FROM L:                             │
├────────────────────────────────────────────────────┤
│                                                    │
│  Δy = L × (√P_upper - √P_lower)                   │
│                                                    │
│  Δx = L × (1/√P_lower - 1/√P_upper)               │
│                                                    │
│     = L × (√P_upper - √P_lower)                   │
│           ─────────────────────                   │
│           √P_lower × √P_upper                     │
└────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────┐
│ L FROM TOKEN AMOUNTS:                             │
├────────────────────────────────────────────────────┤
│                                                    │
│  L = Δy / (√P_upper - √P_lower)                   │
│                                                    │
│  L = Δx × √P_lower × √P_upper                     │
│      ─────────────────────────                    │
│      √P_upper - √P_lower                          │
└────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────┐
│ CURRENT POSITION AMOUNTS (at price √P_c):        │
├────────────────────────────────────────────────────┤
│                                                    │
│  If √P_c < √P_lower:                              │
│    x = L × (1/√P_lower - 1/√P_upper)              │
│    y = 0                                           │
│                                                    │
│  If √P_lower ≤ √P_c ≤ √P_upper:                   │
│    x = L × (1/√P_c - 1/√P_upper)                  │
│    y = L × (√P_c - √P_lower)                      │
│                                                    │
│  If √P_c > √P_upper:                              │
│    x = 0                                           │
│    y = L × (√P_upper - √P_lower)                  │
└────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────┐
│ PRICE CHANGES:                                    │
├────────────────────────────────────────────────────┤
│                                                    │
│  Δ√P = Δy / L                                     │
│                                                    │
│  Δ√P = -Δx / (L / (√P_old × √P_new))             │
└────────────────────────────────────────────────────┘
```

---

## RELATIONSHIP DIAGRAM

### How Everything Connects

```
                    ┌──────────┐
                    │    L     │
                    │(Liquidity│
                    │Constant) │
                    └─────┬────┘
                          │
            ┌─────────────┼─────────────┐
            ↓             ↓             ↓
      ┌─────────┐   ┌─────────┐   ┌─────────┐
      │ Token A │   │ Token B │   │ Price   │
      │ Amount  │   │ Amount  │   │ Impact  │
      │  (Δx)   │   │  (Δy)   │   │ (Δ√P)   │
      └────┬────┘   └────┬────┘   └────┬────┘
           │             │             │
           └─────────────┼─────────────┘
                         ↓
                   ┌──────────┐
                   │  Range   │
                   │[√P_a,√P_b]│
                   └──────────┘

Formulas connect:
  L + Range → Token Amounts (Δx, Δy)
  Token Amounts + Range → L
  L + Token Amount → Price Impact (Δ√P)
  Price Impact + L → Token Amount
```

---

## COMMON CALCULATION PATTERNS

### Pattern 1: "How much liquidity can I add?"
```
Given: Token amounts (x, y), Range [√P_a, √P_b]
Want: L

Step 1: Calculate L from each token
  L_x = x × √P_a × √P_b / (√P_b - √P_a)
  L_y = y / (√P_b - √P_a)

Step 2: Take minimum
  L = min(L_x, L_y)

Step 3: Calculate actual amounts used
  x_used = L × (1/√P_a - 1/√P_b)
  y_used = L × (√P_b - √P_a)
```

### Pattern 2: "How much will this trade move the price?"
```
Given: Trade amount (Δy), Current L, Current √P
Want: New √P

Step 1: Calculate price change
  Δ√P = Δy / L

Step 2: Calculate new price
  √P_new = √P_old + Δ√P

Step 3: Check if crossed tick boundary
  If crossed: update L, repeat with remaining amount
```

### Pattern 3: "What are my current holdings?"
```
Given: Position (L, [√P_a, √P_b]), Current price √P_c
Want: Current (x, y)

Step 1: Check if in range
  If √P_c < √P_a: x = full, y = 0
  If √P_c > √P_b: x = 0, y = full
  Else: mixed

Step 2: Calculate amounts
  x = L × (1/√P_c - 1/√P_b)  [or full if below]
  y = L × (√P_c - √P_a)      [or full if above]
```

---

## SUCCESS CHECKLIST

### You've mastered L when you can:

**Visual Understanding:**
- [ ] Draw the constant product curve
- [ ] Sketch a concentrated position on the curve
- [ ] Visualize how token composition changes across range
- [ ] Draw multiple overlapping positions

**Formula Application:**
- [ ] Calculate Δx and Δy given L and range
- [ ] Calculate L given token amounts and range
- [ ] Calculate current holdings at any price
- [ ] Calculate price impact of a trade

**Conceptual Mastery:**
- [ ] Explain why √P instead of P
- [ ] Explain the trade-off of range width
- [ ] Explain how liquidity_net works
- [ ] Explain fee distribution across overlapping positions

**Implementation Ready:**
- [ ] Can write increase_liquidity logic
- [ ] Can write decrease_liquidity logic
- [ ] Can write swap price update logic
- [ ] Can write tick crossing logic

---

## YOU ARE READY! 🚀

With this visual guide + the deep dive document, you have:
- ✅ Complete mathematical foundation
- ✅ Visual intuition for all concepts
- ✅ Formula mastery
- ✅ Implementation patterns

**Now go build your CLMM!** 🔨
