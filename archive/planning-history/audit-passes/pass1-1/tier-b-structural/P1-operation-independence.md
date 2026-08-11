# P1 — Operation Independence Analysis

**Finding:** F2 (De Morgan redundancy) + B4 suspects  
**Dependency:** P0 (negation must be boundary inversion, not reflection)  
**Date:** April 2026

---

## 1. Result Summary

| # | Operation | Signature | Independent? | Proof Method |
|---|-----------|-----------|:---:|---|
| 1 | `predicate` | e × r × e → a | ✅ | Unique sort-transition (§2.1) |
| 2 | `modify_entity` | m × e → e | ✅ | Unique sort-transition (§2.2) |
| 3 | `modify_relation` | m × r → r | ✅ | Unique sort-transition (§2.3) |
| 4 | `negate` | a → a | ✅ | Semantic irreducibility (§2.4) |
| 5 | `conjoin` | a × a → a | ❌ DERIVABLE | De Morgan from {negate, disjoin} (§3.1) |
| 6 | `disjoin` | a × a → a | ✅ | Required for propositional completeness (§2.5) |
| 7 | `embed` | a → e | ✅ | Unique sort-transition (§2.6) |
| 8 | `abstract` | e → m | ✅ | Unique sort-transition (§2.7) |
| 9 | `compose` | r × r → r | ✅ | Unique sort-transition (§2.8) |
| 10 | `invert` | r → r | ✅ | Parametric ≠ spatial (§2.9) |
| 11 | `quantify` | m × e → a | ✅ | Unique sort-transition (§2.10) |
| 12 | `bind` | e × a → a | ✅ | Unique sort-transition + co-reference gap (§2.11) |
| 13 | `modify_assertion` | m × a → a | ✅ | Unique sort-transition + frame-decoration gap (§2.12) |

**Verdict: 12 independent operations, 1 derivable (conjoin). Total: 13 operations.**

The previous claim of "10 independent operations" is updated to: "Σ_UL⁺ has 12 independent operations. `conjoin` is included for readability but is derivable from `{negate, disjoin}` via De Morgan's law."

---

## 2. Independence Proofs

### Proof Strategy

We use two complementary methods:

**Method 1 — Unique Sort-Transition.** If operation ω is the ONLY operation whose input-output sort signature matches f: S₁ × ... × Sₙ → S_out, then no combination of other operations can reproduce ω (compositions cannot create a sort-transition that none of the components provides).

**Method 2 — Model Separation.** To prove ω independent of a specific other operation ω', construct a Σ_UL-algebra where ω' behaves differently from ω on some input, demonstrating they are not interchangeable.

### Sort-Transition Map

The 13 operations span the following sort-pathways:

```
ENTITY-PRODUCING:
  m × e → e    modify_entity     (only m × e → e path)
  a → e        embed             (only a → e path)

RELATION-PRODUCING:
  m × r → r    modify_relation   (only m × r → r path)
  r × r → r    compose           (only r × r → r path)
  r → r        invert            (shares with modify_relation? — NO; see §2.9)

MODIFIER-PRODUCING:
  e → m        abstract          (only e → m path)

ASSERTION-PRODUCING:
  e × r × e → a    predicate         (only path using e × r × e)
  a → a             negate            (shares with conjoin/disjoin composition)
  a × a → a         conjoin           (De Morgan-redundant with disjoin)
  a × a → a         disjoin           (keep — see §2.5)
  m × e → a         quantify          (only m × e → a path)
  e × a → a         bind              (only e × a → a path — see §2.11)
  m × a → a         modify_assertion  (only m × a → a path — see §2.12)
```

Operations with **unique sort signatures** (no other operation has the same input-output sort types) are independent by Method 1 alone. Operations sharing sort signatures require Method 2.

---

### 2.1 predicate: e × r × e → a — INDEPENDENT

**Proof (Method 1).** `predicate` is the only operation with input sorts (e, r, e) and output sort a. No other operation takes two entities and a relation as input. No composition of other operations can create this specific sort-combination pathway, because:
- The only other assertion-producing operations are negate (a → a), conjoin/disjoin (a × a → a), and quantify (m × e → a)
- None of these accepts a relation as input
- To "consume" a relation, one would need an operation with r in its input sorts: only modify_relation, compose, and invert — but all three produce r, not a

Therefore no composition of other operations can take (e, r, e) and produce a. ∎

### 2.2 modify_entity: m × e → e — INDEPENDENT

**Proof (Method 1).** The only operation with sort signature m × e → e. The only other entity-producing operation is embed: a → e. No composition involving embed can accept (m, e) as input and return e, because embed consumes an assertion, not a modifier-entity pair. ∎

### 2.3 modify_relation: m × r → r — INDEPENDENT

**Proof (Method 1).** The only operation with sort signature m × r → r. The other relation-producing operations (compose: r × r → r, invert: r → r) do not accept a modifier as input. No composition route from (m, r) to r avoids modify_relation. ∎

### 2.4 negate: a → a — INDEPENDENT

**Proof.** After the P0 resolution, negate performs boundary inversion: it flips the assertional sign σ ∈ {⊕, ⊖} of an assertion (F, C, σ) without altering F or C.

The only other a → a operations are compositions of conjoin/disjoin applied to an assertion with itself (but these cannot flip σ — they only combine existing signs). Specifically:
- `disjoin(a, a)` produces assertion with sign σ ∨ σ = σ (unchanged)
- `conjoin(a, a)` produces assertion with sign σ ∧ σ = σ (unchanged)
- No composition of disjoin, conjoin, predicate, embed, etc. can flip an assertional sign from ⊕ to ⊖

The sign-flip is a unique semantic capability not reproducible from other operations. ∎

### 2.5 disjoin: a × a → a — INDEPENDENT

**Proof.** `disjoin` and `conjoin` share the sort signature a × a → a. By De Morgan, each is derivable from {negate, the other}. So at least ONE of them is needed; the other is redundant.

We must keep at least one binary assertion combiner. We choose to keep `disjoin` (and derive `conjoin`) because:
1. The choice is symmetric — either could be kept
2. The B4 planning document identifies `conjoin` as derivable
3. Disjunction (adjacent frames) has a slightly simpler geometric realization than conjunction (overlapping frames), since adjacency requires no spatial overlap management

`disjoin` is independent of the remaining 9 operations (excluding `conjoin`) because none of them can combine two assertions into a single assertion. The only a → a operation among the remaining set is `negate`, which is unary. ∎

### 2.6 embed: a → e — INDEPENDENT

**Proof (Method 1).** The only operation with sort signature a → e. No other operation converts assertions to entities. ∎

### 2.7 abstract: e → m — INDEPENDENT

**Proof (Method 1).** The only operation with sort signature e → m. No other operation converts entities to modifiers. ∎

### 2.8 compose: r × r → r — INDEPENDENT

**Proof (Method 1).** The only operation with sort signature r × r → r. The other relation-producing operations are modify_relation (m × r → r) and invert (r → r). Neither takes two relations as input. No composition route from (r, r) to r avoids compose:
- `modify_relation(abstract(embed(predicate(e₁, r₁, e₂))), r₂)` — this applies a modifier derived from r₁ to r₂, but the modifier imposes r₁'s shape on r₂ rather than concatenating the paths. The result is a spatially transformed version of r₂, not the sequential composition r₁ · r₂.
- There is no path-concatenation capability in any other operation. ∎

### 2.9 invert: r → r — INDEPENDENT

**Proof (Method 2 — Model Separation from modify_relation).**

Both `invert` and `modify_relation` produce r from r-involving inputs. The question: does there exist a modifier m ∈ Gₘ such that `modify_relation(m, r) = invert(r)` for ALL r?

Recall the definitions:
- `modify_relation(m, r) = (T ∘ γ, d')` where T is a fixed invertible transformation of ℝ², and d' = −d iff T is orientation-reversing.
- `invert(r) = (γ⁻¹, −d)` where γ⁻¹(t) = γ(1−t) is the reversed path.

For these to agree, we need a SINGLE transformation T such that for ALL paths γ:

```
T(γ(t)) = γ(1−t)    for all t ∈ [0,1]
```

**Counterexample:** Let γ₁(t) = (t, 0) (line from origin to (1,0)) and γ₂(t) = (0, t) (line from origin to (0,1)).

- For γ₁: T(t, 0) = (1−t, 0) for all t. At t=0: T(0,0) = (1,0). At t=1: T(1,0) = (0,0).
- For γ₂: T(0, t) = (0, 1−t) for all t. At t=0: T(0,0) = (0,1). At t=1: T(0,1) = (0,0).

From γ₁: T(0,0) = (1,0). From γ₂: T(0,0) = (0,1). Contradiction — T cannot map the origin to two different points.

Therefore no single transformation T ∈ Gₘ satisfies T ∘ γ = γ⁻¹ for all γ. The operation `invert` (path reversal) is fundamentally PARAMETRIC — it reverses the traversal order — while `modify_relation` is SPATIAL — it moves the path through space. No spatial transformation can reverse the parameterization of all paths simultaneously. ∎

### 2.10 quantify: m × e → a — INDEPENDENT

**Proof (Method 1).** The only operation with sort signature m × e → a. The only other assertion-producing operation that takes an entity as input is `predicate: e × r × e → a`, which also requires a relation and a second entity. No composition of operations can reduce m × e → a to a path through predicate, because:
- To use `predicate`, we'd need a relation r and a second entity e₂
- These cannot be manufactured from (m, e) alone: the only m → ... path is through modify_entity or modify_relation (neither produces r or e from m alone)
- `abstract` goes e → m (wrong direction), `embed` goes a → e (no assertion available yet)

There is no sort-valid composition chain from (m, e) to a that avoids `quantify`. ∎

### 2.11 bind: e × a → a — INDEPENDENT

**Proof (Method 1 + Method 2).**

**Step 1 — Unique sort-transition.** `bind` has sort signature e × a → a. No other operation in Σ_UL⁺ takes (entity, assertion) as input:
- The assertion-producing operations are `predicate` (e × r × e → a), `negate` (a → a), `conjoin/disjoin` (a × a → a), `quantify` (m × e → a), and `modify_assertion` (m × a → a)
- None accepts the specific pair (entity, assertion) as input
- No composition of other operations can produce the signature e × a → a because:
  - To consume an entity alongside an assertion, we'd need an operation that accepts both; none exists outside bind
  - `predicate` requires (e, r, e), not (e, a); `embed` goes a → e (consumes assertion, produces entity — wrong direction); `modify_entity` takes (m, e) not (e, a)

Therefore no sort-valid composition chain from (e, a) to a exists without bind.

**Step 2 — Semantic gap (co-reference).** Even if some sort-valid workaround existed, bind performs a semantically unique function: it establishes **co-reference** — the declaration that multiple entity-positions in an assertion refer to the same entity. No other operation relates entity-occurrences within an assertion to each other:
- `predicate` connects entities via a relation (structural connection), but does not identify them
- `conjoin/disjoin` combine whole assertions, not entities within them
- `modify_entity` transforms a single entity, not a relationship between entity-positions

**Step 3 — Model separation.** Construct a Σ_UL⁺-algebra A_no_coref where the carrier set for assertions consists of (frame, content, sign) triples with NO co-reference tracking — all entity-positions are independent. In this algebra:
- All operations 1–11 are interpretable (predicate creates frames, negate flips signs, etc.)
- `bind(e_x, a)` is NOT interpretable: there is no mechanism to "identify" entity-positions
- `modify_assertion` is interpretable (it transforms ∂F without needing co-reference)

Therefore bind is not derivable from {predicate, modify_entity, modify_relation, negate, disjoin, embed, abstract, compose, invert, quantify, modify_assertion}. ∎

### 2.12 modify_assertion: m × a → a — INDEPENDENT

**Proof (Method 1 + Method 2).**

**Step 1 — Unique sort-transition.** `modify_assertion` has sort signature m × a → a. No other operation in Σ_UL⁺ takes (modifier, assertion) as input:
- `modify_entity` takes (m, e), not (m, a)
- `modify_relation` takes (m, r), not (m, a)
- `quantify` takes (m, e), not (m, a)
- `bind` takes (e, a), not (m, a)
- No other operation accepts a modifier alongside an assertion

Therefore no sort-valid composition from (m, a) to a exists without modify_assertion.

**Step 2 — Non-derivability via embed/modify_entity.** The most plausible derivation attempt would be:
1. `embed(a) → e` (convert assertion to entity)
2. `modify_entity(m, e) → e'` (apply modifier to the entity)
3. ??? (convert back to assertion)

But there is no inverse of `embed`. The only a → e operation is `embed` itself (one-way). Once an assertion is embedded as an entity, it cannot be "unembedded" back to an assertion with a modified frame. Moreover:
- `modify_entity` transforms the entity's geometric properties (shape, size, position), which affects the embedded assertion's content representation — it does NOT selectively transform the frame boundary (∂F) while preserving content (C) and sign (σ)
- What we need is a transformation that touches ∂F only. No combination of embed, modify_entity, and any other operations achieves this.

**Step 3 — Semantic gap (frame decoration).** `modify_assertion` is the only operation that transforms the **boundary properties** of an assertion's frame (∂F) without altering the content (C) or the truth status (σ). The frame decoration encodes epistemic stance — evidentiality, emphasis, hedging — which is a dimension of meaning orthogonal to both content and truth:
- `negate` changes σ (truth status) but not ∂F or C
- `predicate/embed/abstract/quantify/bind` construct or restructure C but don't touch ∂F
- `modify_entity/modify_relation` act on elements within C, not on the frame boundary

**Step 4 — Model separation.** Construct a Σ_UL⁺-algebra A_uniform_frames where all assertion frames have identical boundaries (unit circles, solid stroke). In this algebra:
- All operations 1–12 are interpretable (predicate creates frames, negate flips signs, bind establishes co-reference, etc.)
- `modify_assertion(m, a)` is NOT interpretable when m ≠ identity: there is no way to produce a non-solid frame boundary
- Therefore modify_assertion is non-trivial in the standard algebra but trivial in A_uniform_frames

This demonstrates that modify_assertion adds algebraic structure not present in {operations 1–12}. ∎

---

## 3. The conjoin Redundancy

### 3.1 Derivation

```
conjoin(a, b) = negate(disjoin(negate(a), negate(b)))
```

**Verification (De Morgan):**
- negate(a) = (F_a, C_a, −σ_a)
- negate(b) = (F_b, C_b, −σ_b)
- disjoin(negate(a), negate(b)) = adjacent framing of the two negated assertions
- negate(disjoin(...)) = flip the sign of the disjunction

For σ_a = σ_b = ⊕ (both asserted):
- negate(a) has sign ⊖, negate(b) has sign ⊖
- disjoin has sign ⊖ ∨ ⊖ = ⊖
- negate(disjoin) flips to ⊕

This matches conjoin(a, b) with sign ⊕ ∧ ⊕ = ⊕. ✓

The derivation holds for all sign combinations by De Morgan's laws:
- −(−σ₁ ∨ −σ₂) = σ₁ ∧ σ₂ ✓

### 3.2 Why Keep conjoin Anyway?

Despite being derivable, `conjoin` SHOULD remain in the operation set for practical reasons:

1. **Readability:** `conjoin(a, b)` is clearer than `negate(disjoin(negate(a), negate(b)))`
2. **Geometric naturalness:** Overlapping frames (conjunction) is a distinct and intuitive visual pattern
3. **Precedent:** Standard algebraic signatures routinely include derived operations for usability (e.g., subtraction in ring theory, though derivable from addition + negation)

The correction is not to REMOVE conjoin but to RELABEL the claim: "11 operations (10 independent + 1 derived)" instead of "11 independent operations."

---

## 4. Could disjoin Be Derived Instead?

By the symmetric De Morgan law:
```
disjoin(a, b) = negate(conjoin(negate(a), negate(b)))
```

So the choice of which to keep is SYMMETRIC. Either {negate, disjoin} or {negate, conjoin} generates both. We keep `disjoin` as the primitive for consistency with the existing B4 analysis. The choice is a convention, not a mathematical necessity.

---

## 5. Impact on Formal Claims

### 5.1 What Changes
- `formal-foundations.md`: "13 operations (12 independent, 1 derived)"
- `formal-operations.md` §3: Completeness argument references 13 operations with derivability noted
- `CRITIQUE.md` §1.3: Operation count error marked RESOLVED with full history
- All writing system docs: Updated to 13 operations (Phase 1 + Phase 2 of Pass 1.4)

### 5.2 What Doesn't Change
- The operation SET remains {14}. No operation is removed.
- The geometric realizations are unchanged.
- The completeness argument is STRENGTHENED: De Morgan derivability is an internal consistency check.
- The writing system is unaffected — conjoin (overlapping frames) remains a valid construction.

---

## 6. Remaining Open Question: Is the Set COMPLETE?

Independence tells us all 13 are needed. But are they SUFFICIENT? This is the harder question (B4 §3.7). The known gap:

- **No a → r path.** There is no operation that extracts a relation from an assertion. If you have "the fact that dogs bark," there's no operation to recover "bark" as a free-standing relation. This gap (see P4, B3) means certain linguistic operations (denominalization → verbalization chain) may be inexpressible.

This is a separate issue from independence and is tracked under P4.
