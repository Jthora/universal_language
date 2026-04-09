# Phase 2 — Variable Binding

**Status:** ✅ COMPLETE (April 7, 2026)  
**Risk:** Medium (requires design decision with downstream consequences)  
**New operations:** +1 (`bind`)  
**D2 cases affected:** 3.1, 3.4, 3.5, and downstream improvements to 1.5, 10.1  
**Expected case conversions:** 3–5 (⚠️ → ✅)  
**Dependencies:** None, but benefits from Phase 1A/1B being settled first  
**Blocks:** Phase 4 (Montague mapping needs binding mechanism to map lambda abstraction)

---

## Problem Statement

The D2 completeness challenge found that **every case in Category 3 (Quantifier Scope)** scored ⚠️ rather than ✅. The common root cause: Σ_UL has no formal mechanism for **variable binding with scope ordering.**

### The Core Issue

Consider "Every student read some book" (Case 3.1), which has two readings:

- **Reading 1 (∀ > ∃):** For every student, there exists a (possibly different) book they read.
- **Reading 2 (∃ > ∀):** There exists one particular book that every student read.

The difference is **scope ordering** — which quantifier binds first. In predicate logic, this is handled by variable binding:

```
Reading 1: ∀x. Student(x) → ∃y. Book(y) ∧ Read(x, y)
Reading 2: ∃y. Book(y) ∧ ∀x. Student(x) → Read(x, y)
```

UL's `quantify(m, e) → a` is **monadic** — it quantifies one entity at a time and produces an assertion. There is no formal mechanism to:

1. **Bind** an entity to a variable that appears in multiple predications
2. **Order** the scope of multiple quantifiers relative to each other
3. **Co-refer** — ensure that the "student" in "reads" is the same entity as the "student" in "loves" (Case 3.4)

The D2 analysis noted: "the algebra lacks a formal mechanism for variable binding — `quantify` produces assertions, not bound variables." The visual system CAN represent scope via nesting depth, but the algebraic decomposition doesn't capture this.

---

## Design Options Analysis

### Option A: Lambda-Style Binding

**Mechanism:** Add an abstraction operation that creates functions:

```
lambda : e → a → (e → a)    # abstract over an entity in an assertion
apply  : (e → a) × e → a    # apply the function to a specific entity
```

**Geometric grounding:** A hollow mark (○) within a frame represents an unfilled slot. Filling it (●) with a specific entity produces a complete assertion.

**Pros:**
- Clean algebra; Montague-compatible (direct map to his type system)
- Well-understood theory (simply-typed lambda calculus)

**Cons:**
- Introduces a **higher-order type** (e → a) that doesn't fit the 4-sort system
- Would require either a 5th sort or a type-constructor mechanism
- Significantly increases algebraic complexity

**Verdict:** Powerful but too heavy. Violates UL's minimalist design philosophy.

### Option B: Scope-as-Nesting (Visual Convention Only)

**Mechanism:** No new operation. Scope ordering = frame nesting depth. Inner frame = narrower scope.

```
Reading 1: [ ∀student [ ∃book [ student reads book ] ] ]
Reading 2: [ ∃book [ ∀student [ student reads book ] ] ]
```

**Geometric grounding:** Already present in the spatial system. Nesting depth is visually unambiguous.

**Pros:**
- No new operations, no new sorts
- Visually natural and immediately readable

**Cons:**
- No **algebraic** representation — the scope information is spatial, not operational
- A serialization/parser cannot recover scope order from the operation sequence
- Co-reference across frames is implicit (same mark shape) not formal (bound variable)

**Verdict:** Necessary as a visual convention but insufficient as a formal mechanism. The algebra needs to capture what the visual system shows.

### Option C: Labeled Slots (RECOMMENDED)

**Mechanism:** Add a single new operation:

```
bind : e × a → a
```

**Semantics:** Given a **labeled-slot entity** e_x ∈ Gₑ (a hollow mark with label x) and an assertion a containing occurrences of e_x, `bind(e_x, a)` produces an assertion in which:

1. The slot e_x is **closed** (hollow mark becomes filled at the point of binding)
2. All occurrences of e_x within a are **co-referential** (they must be the same entity when the assertion is evaluated)
3. The **scope** of the binding extends to the frame boundary of a

**Geometric realization:**

```
BEFORE bind:        AFTER bind:
┌─────────────┐     ┌─────────────┐
│ ○_x ──→ ●   │     │ ●_x ──→ ●   │
│             │  →  │    ↑ bound  │
│ ○_x ──→ ●   │     │ ●_x ──→ ●   │
└─────────────┘     └─────────────┘
```

Where ○_x is a hollow labeled mark (open slot) and ●_x is a filled labeled mark (bound variable).

**Scope ordering via nesting:**

```
Reading 1 (∀x > ∃y):
┌─────────────────────────────────┐
│ bind(x,                         │
│   ┌─────────────────────────┐   │
│   │ bind(y,                 │   │
│   │   pred(x, r_read, y)   │   │
│   │ )                       │   │
│   └─────────────────────────┘   │
│ )                               │
└─────────────────────────────────┘
  x quantified universally (outer frame)
  y quantified existentially (inner frame)
```

**Pros:**
- No higher-order types (bind takes e × a → a, all existing sorts)
- Explicit co-reference (labeled marks)
- Scope = frame nesting (consistent with visual system)
- No variable capture by construction (each frame is a scope boundary; labels within a frame are bound to that frame)

**Cons:**
- Requires label management (unique labels within a construction)
- Adds infrastructure (labeled slots as a distinguished subset of Gₑ)

**Verdict:** Best balance of formal precision, geometric naturalness, and minimal disruption.

### Option D: De Bruijn Indices

**Mechanism:** Like Option C but replaces labels with positional indices (binding depth count).

**Verdict:** Technically clean but geometrically unnatural. Indices are a string-language convention; UL is spatial. Rejected.

---

## Recommended Design: Option C (Labeled Slots)

### Formal Specification

#### New Distinguished Subset of Gₑ

Define the set of **slot entities:**

```
Gₑ_slot = { (○, {(center, x)}) | x ∈ Labels }
```

where ○ is the open circle (hollow mark) and x is a label from a countably infinite set Labels = {x₁, x₂, x₃, ...}. Slot entities are a distinguished proper subset of Gₑ — they ARE entities (compact subsets with decorations) but with the special property of being "unfilled."

**Filled counterpart:** When bound, the slot entity becomes:

```
(●, {(center, x)})
```

A solid (filled) mark with the same label. This is also an element of Gₑ.

#### New Operation: bind

```
bind : e × a → a
```

**Definition.** Given a slot entity e_x = (○, {(center, x)}) ∈ Gₑ_slot and an assertion a = (F, C, σ) ∈ Gₐ:

```
bind(e_x, a) = (F, C[○_x ↦ ●_x], σ)
```

where C[○_x ↦ ●_x] replaces every occurrence of the hollow mark ○ labeled x within C with the filled mark ● labeled x. The frame and assertional sign are unchanged.

**What this does:**
1. Declares that all occurrences of ○_x in a are the **same** entity (co-reference)
2. Establishes that the scope of this co-reference is the frame F (scope boundary)
3. Closes the slot — subsequent constructions outside F cannot bind to x

#### Scope Ordering

When multiple bind operations are nested, scope order is determined by nesting depth:

```
bind(e_x, bind(e_y, a))
```

Here x scopes over y: x is bound at the outer frame, y at the inner frame. The entity substituted for x is determined first, then y.

This is equivalent to Montague's scope ordering via lambda abstraction nesting, but without introducing function types.

### Verification Requirements

**Closure:** The output of bind is an assertion (same frame, content with filled marks, same sign). ✓

**Totality:** For any slot entity e_x and any assertion a:
- If a contains occurrences of ○_x: all are replaced with ●_x. ✓
- If a contains NO occurrences of ○_x: the result is a itself (vacuous binding). ✓

**Determinism:** Substitution of a labeled mark is a deterministic operation. ✓

**Independence proof (sketch):**
bind : e × a → a is the ONLY operation that:
1. Takes e × a → a (no other operation has this signature), AND
2. Establishes co-reference between entity occurrences within an assertion

The sort signature e × a → a is unique among the current 11 operations (none take an entity and an assertion to produce an assertion). Therefore bind cannot be derived from any combination of existing operations.

**Full independence proof needed:** Exhibit a model where all 11 existing operations are interpretable but bind is not. The standard technique: construct a Σ_UL-algebra where entities within assertions are independent (no co-reference mechanism). The existing operations preserve independence; bind would violate it.

### Interaction with Quantify

With bind in place, multi-variable quantification decomposes naturally:

**"Every student read some book" (Reading 1: ∀ > ∃):**

```
e_x = slot entity "x" (student)
e_y = slot entity "y" (book)

pred(e_x, r_read, e_y) → a₁  "x reads y"
bind(e_y, a₁) → a₂  "for a specific y, x reads y" (y bound, x still open)
quant(m_∃, emb(a₂)) → a₃  "there exists a y such that x reads y"
bind(e_x, a₃) → a₄  "for a specific x, ∃y. x reads y" (x bound)
quant(m_∀, emb(a₄)) → a₅  "for all x, ∃y. x reads y"
```

**Reading 2 (∃ > ∀):** Reverse the binding order:

```
bind(e_x, a₁) → a₂'  (x bound first)
quant(m_∀, emb(a₂')) → a₃'  "for all x, x reads y" (y still free)
bind(e_y, a₃') → a₄'  (y bound at outer scope)
quant(m_∃, emb(a₄')) → a₅'  "∃y. ∀x. x reads y"
```

The scope ambiguity is resolved by **which bind is applied first** (inner vs. outer), exactly mirroring the frame-nesting convention.

---

## Impact Assessment on D2 Cases

| Case | Current Verdict | After Phase 2 | Reason |
|---|---|---|---|
| 3.1 "Every student read some book" | ⚠️ | ✅ | Scope ambiguity resolvable via bind ordering |
| 3.2 "Most dogs chase some cat" | ❌ (→✅ after 1B) | ✅ | Graduated quantification (Phase 1B) + bind for multi-variable |
| 3.3 "Exactly three students passed" | ⚠️ | ⚠️ | Bind helps co-reference but cardinality still O(n) |
| 3.4 "Everyone who reads loves some author" | ⚠️ | ✅ | Restricted quantification via embed + explicit co-reference via bind |
| 3.5 "No one loves no one" | ⚠️ | ✅ | Double negation + nested bind cleanly decomposes |
| 1.5 "No student passed the exam" | ⚠️ | ✅ | Quantifier-negation interaction now formal via bind + negate |

**Net conversions:** 4 cases ⚠️→✅ (3.1, 3.4, 3.5, 1.5).

---

## Files to Modify

| File | Change |
|---|---|
| `foundations/formal-foundations.md` | §1.5 — add `bind` operation + independence argument; update operation count to 13 (12 independent + 1 derived) |
| `foundations/formal-operations.md` | New §1.12 (or renumber) — full definition of bind with closure/totality/determinism/injectivity; define Gₑ_slot |
| `ul-core/grammar/formal-grammar.md` | Add C12 construction rule — slot entities and binding |
| `ul-core/writing-system/writers-companion.md` | Add examples with slot notation |
| `ul-core/symbology/symbol-map.md` | Add hollow marks to atomic mark inventory |
| `experiments/D2-completeness-challenge.md` | Re-score Cases 3.1, 3.4, 3.5, 1.5; update summary |
| `ul-core/CRITIQUE.md` | Document the extension and its rationale |
| `docs/planning/audits/improvements/pass1-1/tier-b-structural/P1-operation-independence.md` | Add bind independence proof |

---

## Open Questions

### Q1: Does bind break the Unique Grounding Theorem?

The UGT claims G is initial in Lang(Σ_UL). Adding bind extends Σ_UL to Σ_UL⁺. The theorem needs to be re-verified for Σ_UL⁺. Expectation: the geometric grounding of bind (hollow → filled marks) is structurally forced by the same role-property argument. **Needs formal verification.**

### Q2: Is labeled-slot notation universal across writing systems?

The hollow-vs-filled distinction (○ vs ●) is visually clear in the Latin/geometric writing system. Is it equally clear in all visual contexts? For machine parsing, labels are unambiguous. For hand-writing, hollow marks are standard mathematical notation for "undetermined."

### Q3: Should quantify be redefined in terms of bind?

With bind available, `quantify(m, e) → a` could be decomposed as:
1. Create a slot entity for e
2. Apply predication involving the slot
3. bind the slot
4. Apply the quantifier modifier to the frame

This would make `quantify` derivable from `{bind, predicate, modify_entity}`. If so, the independent operation count stays at 12 (replacing quantify's independence with bind's).

**This needs formal analysis.** If quantify becomes derivable, the operation set becomes: **12 operations (11 independent including bind + 1 derived conjoin)**, with quantify as a convenient shorthand rather than a primitive. This would be an elegant result.

### Q4: Does the binding mechanism handle donkey sentences?

"Every farmer who owns a donkey beats it." The pronoun "it" co-refers with "a donkey" across a quantifier boundary. This is a notorious problem in formal semantics (Discourse Representation Theory was invented specifically for it).

With bind: "it" = ○_donkey, bound to the same slot as "a donkey." The binding scope must extend across the conditional. **Needs a worked example to verify.**
