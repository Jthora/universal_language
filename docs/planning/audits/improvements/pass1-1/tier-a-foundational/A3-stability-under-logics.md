# A3 — Do the 4 Sorts Survive Non-Classical Logics?

**Tier:** A (Foundational)  
**Targets:** `foundations/formal-foundations.md` §1.5 (sort definitions), §6.3 (open issues: non-classical negation)  
**Dependencies:** B1 (negation) — must be resolved before this analysis is complete  
**Effort:** 3–6 months of formal analysis

---

## The Assumption

Σ_UL's 4 sorts (Entity, Relation, Modifier, Assertion) and its propositional operations (negate, conjoin, disjoin) presuppose **classical logic**:

- Assertions have definite truth values (bivalence)
- `negate(negate(a)) = a` (double negation elimination)
- `disjoin(a, negate(a))` is a tautology (excluded middle)
- `conjoin(a, negate(a)) = ⊥` (non-contradiction)

Natural language violates ALL of these:

| Classical Property | Natural Language Violation | Example |
|---|---|---|
| Bivalence | Vague predicates have no definite truth value | "This heap has many grains" — how many is "many"? |
| Double negation elimination | "Not unhappy" ≠ "happy" in pragmatic usage | Litotes conveys hedging, not equivalence |
| Excluded middle | Some statements are genuinely indeterminate | "The present king of France is bald" — neither true nor false? |
| Non-contradiction | Paraconsistent reasoning is common | "Quantum particles are both here and not-here" (superposition metaphor) |

If UL claims universality for natural language, it must handle these cases.

## The Questions

### Q3a: Do the 4 sorts survive intuitionistic logic?

In intuitionistic logic, excluded middle (`a ∨ ¬a`) is not assumed, and double negation elimination fails. `negate(negate(a)) = a` is NOT a theorem.

**Impact on UL:**
- `negate` can still be an operation, but it's no longer an involution. `negate(negate(a))` is weaker than `a` — it means "it is impossible that a is impossible" (double negation), which is not the same as "a is true."
- The Assertion sort still makes sense (bounded claims still exist) but its logical structure is weaker.
- The embedding theorem is unaffected (it's about algebraic structure, not logic).
- The uniqueness theorem is unaffected (it's about geometric primitives, not logical properties).

**Verdict:** 4 sorts probably survive. The propositional operations need retuning, especially `negate`.

### Q3b: Do the 4 sorts survive paraconsistent logic?

In paraconsistent logic, `conjoin(a, negate(a)) ≠ ⊥` — contradictions don't cause explosive collapse. Some things can be "both true and false."

**Impact on UL:**
- The Assertion sort needs to accommodate true contradictions. Instead of partitioning meaning-space into {true, false}, assertions would have a 4-valued status: {true only, false only, both, neither}.
- `conjoin(a, negate(a))` produces a "both" assertion, not the bottom element ⊥.
- The geometric model needs a way to represent "both asserted and denied" — e.g., overlapping solid and dashed frames (from F1 resolution Option B) could represent this naturally.

**Verdict:** 4 sorts survive — but the Assertion sort's internal structure becomes richer (4-valued vs. 2-valued).

### Q3c: Do the 4 sorts survive fuzzy logic?

In fuzzy logic, assertions have truth *degrees* in [0, 1] rather than truth values in {0, 1}.

**Impact on UL:**
- The Assertion sort becomes a [0,1]-valued sort rather than a {true, false}-valued sort.
- `negate(a)` = 1 − truth(a) (fuzzy complement)
- `conjoin(a, b)` = min(truth(a), truth(b)) (fuzzy AND)
- `disjoin(a, b)` = max(truth(a), truth(b)) (fuzzy OR)

**Critical question:** Does the Assertion sort merge with the Modifier sort? In fuzzy logic, the truth degree of an assertion IS a continuous parameter in [0, 1]. Modifiers are already continuous parameters (angles in [0°, 360°)). Are these the same thing?

If truth degree IS a modifier, then fuzzy assertions are just "modified assertions" and the 4 sorts reduce to 3 — Entity, Relation, Modifier (with Assertion as a special case of a modified Entity).

**Verdict:** 4 sorts may REDUCE to 3 under fuzzy logic. This needs careful analysis.

### Q3d: Do the 4 sorts survive modal logic?

Modal logic adds operators: □ (necessarily) and ◇ (possibly). In UL, `formal-foundations.md` §6.3 flags "systematic quantification" as open work. Modality is related but distinct from quantification.

**Impact on UL:**
- Modal operators could be Modifiers: □ = `modify_entity(necessity_modifier, embed(a))` and ◇ = `modify_entity(possibility_modifier, embed(a))`
- This works: embed the assertion as an entity, modify it with a necessity/possibility quality
- The writing system already has conventions for modality: bold lines (necessity) and dashed lines (possibility) from `symbol-map.md` §VII

**Verdict:** 4 sorts survive modal logic. Modality enters through the Modifier sort, which already handles continuous scaling of qualities.

### Q3e: Summary: Which Logics Break the 4-Sort Decomposition?

| Logic | Excluded Middle | Non-Contradiction | Bivalence | 4 Sorts Status |
|-------|----------------|-------------------|-----------|----------------|
| Classical | ✅ | ✅ | ✅ | ✅ UNCHANGED |
| Intuitionistic | ❌ | ✅ | ❌ | ✅ Survive (negate weakened) |
| Paraconsistent | ✅ | ❌ | ❌ | ✅ Survive (assertion enriched to 4-valued) |
| Fuzzy | N/A | N/A | ❌ | ⚠️ MAY REDUCE TO 3 (assertion ≈ modifier) |
| Modal | ✅ | ✅ | ✅ | ✅ Survive (modality via modifier) |
| Substructural (linear, relevance) | Varies | Varies | ❌ | ❓ UNKNOWN — needs investigation |

## Lines of Investigation

### Path 1: The Fuzzy Merge Question
Formalize: is there a Σ_UL-homomorphism from the fuzzy Assertion space [0,1] to the Modifier space G_m? If yes, Assertion reduces to a special case of Modifier in fuzzy settings. If no (because Assertions have *structural* properties beyond truth degree — e.g., content containment — that Modifiers lack), the 4 sorts are stable even under fuzzy logic.

### Path 2: Substructural Logics
Linear logic (resources are consumed) and relevance logic (premises must be relevant to conclusions) impose structural rules on reasoning. Analyze whether the Σ_UL operations respect linear/relevance constraints.

### Path 3: The Minimal Logic Core
Strip UL to the operations that survive ALL logics: probably `{predicate, modify_entity, modify_relation, embed, abstract, compose, invert}` — the non-logical core. Then add the logical operations (negate, conjoin, disjoin, quantify) as *parametric extensions* keyed to the chosen logic. This would make UL genuinely logic-agnostic.

## Deliverable

A classification:
1. **Logic-independent core** — which sorts and operations survive all logics?
2. **Logic-parametric extensions** — which operations require a logic parameter?
3. **Potential sort reduction** — under which logics (if any) do the 4 sorts collapse?

## Status

**Status:** ❌ OPEN — `formal-foundations.md` §6.3 flags "non-classical negation" but no analysis exists.
