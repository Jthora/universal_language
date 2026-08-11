# Phase 3 — Independence Proofs Consolidation

**Status:** ✅ COMPLETE  
**Risk:** Medium (may change the canonical independence count)  
**New operations:** 0  
**Dependencies:** None (can run in parallel with Phase 2)  
**Blocks:** Phase 5 (summary needs final independence count)

**Result:** Both `bind` and `modify_assertion` proven fully independent → **12 independent + 1 derived (conjoin)**

---

## Problem Statement

The operation independence proofs are currently split across three documents:

| Document | What It Covers |
|----------|---------------|
| `docs/planning/audits/improvements/pass1-1/tier-b-structural/P1-operation-independence.md` | Original 10 independent + 1 derived (conjoin). Full proofs for invert, quantify; sort-signature arguments for the other 7. |
| `foundations/formal-operations.md` §1.12 | `bind` independence **sketch** — unique e × a → a signature + co-reference mechanism argument |
| `foundations/formal-operations.md` §1.13 | `modify_assertion` independence **sketch** — unique m × a → a signature + frame-decoration argument |

The sketches need to be expanded to full proofs and consolidated into P1-operation-independence.md.

---

## The Independence Question

### Current Claimed Count

`formal-foundations.md` says: "13 operations (12 independent + 1 derived conjoin + 1 near-derived modify_assertion)"

But the Phase 3 planning doc from Pass 1.2 described modify_assertion as potentially independent (unique m × a → a signature, no other operation touches ∂F).

### What Needs Resolution

1. **Is `bind` independent?** Argument sketch: bind(e, a) → a has unique sort signature e × a → a. No other operation takes (entity, assertion) and produces assertion. The co-reference mechanism (slot filling) is not derivable from any combination of other operations. **Likely: YES, independent.**

2. **Is `modify_assertion` independent?** Argument sketch: modify_assertion(m, a) → a has sort signature m × a → a. But consider: could `modify_assertion(m, a)` be derived from `embed(a) → e`, then `modify_entity(m, e) → e'`, then some way to "un-embed" back to assertion? There is no `un-embed` operation. So modify_assertion is not derivable from {embed, modify_entity}. But is it "near-derived" in some weaker sense? **Needs formal proof.**

3. **Final count:** If both bind and modify_assertion are independent:
   - 12 independent + 1 derived (conjoin) = 13 total
   - This would change the minimality claim from "11 independent" to "12 independent"

### Proof Strategy

For each new operation, prove independence by showing:

**Existence argument:** There exists a Σ_UL⁺-algebra where the operation is non-trivial (produces distinct outputs) — so it's not the identity.

**Non-derivability argument:** There is no finite composition of other operations that replicates the target operation's behavior on ALL inputs. Typically shown by:
- Sort-signature uniqueness (no composition path through other operations produces the same input→output type transition)
- Semantic gap (the operation achieves something — e.g., co-reference, frame decoration — that no other operation affects)
- Counterexample (construct a specific input where all candidate derivations fail)

---

## Deliverables

- [x] Expand bind independence sketch to full proof in P1-operation-independence.md (§2.11)
- [x] Expand modify_assertion independence sketch to full proof in P1-operation-independence.md (§2.12)
- [x] Determine final independence count: **12 independent** (bind AND modify_assertion both proven independent)
- [x] Update `formal-foundations.md` minimality claim → "12 independent + 1 derived"
- [x] Update `AGENTS.md`, `llms.txt` metadata → "12 independent + 1 derived"
- [x] Update P1-operation-independence.md summary table with all 13 operations
- [x] Update formal-operations.md sketches → reference full proofs

---

## Relationship to Notation

The independence count affects how UL is described:

| If 12 independent | If 12 independent |
|-------------------|-------------------|
| "13 operations (12 independent + 1 derived + 1 near-derived)" | "13 operations (12 independent + 1 derived)" |
| modify_assertion is a convenience operation | modify_assertion is a genuine primitive |
| Minimality: 12-dimensional operation space | Minimality: 13-dimensional operation space |

Either result is defensible. The question is whether modify_assertion(m, a) adds genuine algebraic dimension or is a structured shorthand for something expressible (if awkwardly) via other operations.
