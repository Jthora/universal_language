# Critical Errors — F1 and F2

**Date:** April 7, 2026  
**Severity:** 🔴 CRITICAL (F1), 🟠 SIGNIFICANT (F2)  
**Impact:** These findings affect the correctness of the formal algebra, not just its presentation.

---

## F1: Negation-as-Reflection Implements Converse, Not Negation

### The Claim

`formal-operations.md` §1.4 defines `negate(a)` as:

```
negate(a) = (F, ρ_c(C))
```

where ρ_c is reflection through the vertical axis at the centroid. The frame F is preserved; the content C is reflected.

### The Evidence

`syntax-dictionary.md` §3.4 gives the worked example:

```
ORIGINAL:                      NEGATED:
┌───────────────┐              ┌───────────────┐
│  △ ──→ ○      │    REFLECT   │      ○ ←── △  │
└───────────────┘     ══►      └───────────────┘
"fundamental acts               "fundamental does NOT
 on universal"                   act on universal"
```

### The Problem

The output `[○ ←── △]` reads as "universal is acted upon by fundamental" — which is the **relational converse**, not the **logical negation**.

| Operation | Input | Output | Semantic |
|-----------|-------|--------|----------|
| Negation | "A acts on B" | "A does NOT act on B" | Denial of the relationship |
| Converse | "A acts on B" | "B is acted upon by A" | Role swap, same truth value |

These have entirely different truth conditions:
- Negation changes truth value: if "A acts on B" is true, then "A does NOT act on B" is false
- Converse preserves truth value: if "A acts on B" is true, then "B is acted upon by A" is also true

Reflection through a vertical axis mechanically swaps left/right positions and reverses arrow directions. This produces subject-object swap + direction reversal = converse.

### Downstream Impact

1. **Propositional completeness:** The argument that `{negate, conjoin, disjoin}` is functionally complete (§3.4 of `formal-operations.md`) depends on `negate` implementing logical negation. If it implements converse, the Boolean logic subsystem is broken.

2. **De Morgan derivations:** `conjoin(a,b) = negate(disjoin(negate(a), negate(b)))` — if `negate` is actually converse, this derivation doesn't produce logical AND.

3. **Contradiction:** The axiom `conjoin(a, negate(a)) = ⊥` (contradiction) fails for converse, because `conjoin("A→B", "B→A")` is NOT contradictory — it's just mutual relation.

4. **Every use of `negate`** throughout the system needs re-examination.

### Self-Awareness in Existing Docs

`pass1/phase-2-hard-problems/2.3-semantic-assignment-uniqueness.md` partially flags this:

> "Geometric candidates satisfying N1 [involution]: Reflection through any line, rotation by π, point inversion. All are involutions. Which also satisfy N2+N3? Reflection through a line: the intersection of a region and its reflection can be empty (if the region doesn't straddle the line) — not always ⊥. So N2 may fail for some inputs."

But this analysis treats negation as "one of several valid geometric choices." The actual problem is deeper: **no form of reflection produces logical negation**. Reflection is inherently a *spatial rearrangement* operation, not a *truth-value-flipping* operation.

### Possible Resolutions

| Option | Description | Implication |
|--------|-------------|-------------|
| **A: Geometric complement** | Define negate(a) = (F, F \ C) — the content becomes everything in the frame that ISN'T C | Preserves "all meaning is geometric" but requires defining "complement within a frame" precisely |
| **B: Boundary inversion** | Define negate(a) = (F̄, C) where F̄ marks the frame as "denied" (dashed, crossed, inverted fill) | Adds a visual marker; the content is unchanged; only the frame's assertional status flips |
| **C: Algebraic axiom** | Accept that negation cannot be geometrized; define it axiomatically as an operation satisfying N1+N2+N3 | Honest, but creates the first non-geometric operation in UL — challenging the core axiom |

**Recommendation:** Option B (boundary inversion) is the least disruptive. The frame itself carries assertional status — solid boundary = asserted, dashed/crossed boundary = denied. Content stays identical. Double-denial (dashing a dashed frame) restores solid = involution holds. Conjunction of a and negate(a) produces overlapping frames where one is solid and one is dashed — reading as "both claimed and denied" = ⊥.

### Status

**Status:** 🔴 UNRESOLVED — requires design decision before any other propositional logic work proceeds.

---

## F2: Conjoin/Disjoin Redundancy (De Morgan)

### The Claim

`formal-foundations.md` §1.5 and `formal-operations.md` §3.1 claim the 11 operations are minimal: "removing any sort or operation makes the system unable to express some class of finite relationships."

### The Evidence

By De Morgan's laws:

$$\text{conjoin}(a_1, a_2) = \text{negate}(\text{disjoin}(\text{negate}(a_1), \text{negate}(a_2)))$$

$$\text{disjoin}(a_1, a_2) = \text{negate}(\text{conjoin}(\text{negate}(a_1), \text{negate}(a_2)))$$

Therefore: given `negate`, either `conjoin` or `disjoin` is derivable from the other. **At most 10 operations are needed, not 11.**

### Downstream Impact

1. **Minimality claim is false** as stated. Either weaken to "each operation covers a semantically natural class" (true and defensible) or reduce to 10 operations.

2. **This interacts with F1:** If `negate` is redefined (per F1 resolution), the De Morgan derivation needs to be re-verified with the corrected negation.

### Self-Awareness in Existing Docs

`pass1/phase-2-hard-problems/2.2-algebraic-independence.md` already lists conjoin/disjoin as "Suspect 1" and notes the De Morgan derivation explicitly. This is flagged but unresolved.

### Possible Resolutions

| Option | Description |
|--------|-------------|
| **A: Keep both, fix claim** | Retain both conjoin and disjoin for expressiveness/naturalness but label the set as "complete but not independent" — similar to how {AND, OR, NOT} is a standard but non-minimal basis in Boolean algebra |
| **B: Drop one** | Remove disjoin (less commonly needed) and derive it from conjoin + negate. Reduces to 10 operations. |
| **C: Replace both** | Replace conjoin + disjoin with a single NAND or NOR operation. Produces a truly minimal set. Loses human readability. |

**Recommendation:** Option A. Natural languages universally have both "and" and "or." The minimality claim should be weakened: "The 11 operations are a *complete and natural* generating set; the *minimal independent* set contains 10 operations (conjoin or disjoin is redundant via De Morgan)."

### Status

**Status:** 🟠 CONFIRMED — concrete redundancy, awaiting resolution decision.

---

## Dependencies

```
F1 (Negation bug) ──blocks──→ F2 resolution (De Morgan relies on negate)
F1 (Negation bug) ──blocks──→ All propositional logic claims
F2 (De Morgan)    ──blocks──→ Minimality claim correction
```
