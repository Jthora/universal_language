# Legacy Findings — Consolidated from the 2026-04 Audit Era

**Purpose:** preserve the substance of the pass1–pass3 audit trail and the former `uws/CRITIQUE.md`
in one place, with **honest current status**, so ~141 planning documents and a misleading critique
file could be deleted without losing what they actually found.

**Why the originals were removed:** `CRITIQUE.md` marked findings "✅ RESOLVED" that were resolved
*in documentation only* and never implemented — see F-006. Any file that asserts a fix which does
not exist is worse than no file. Statuses below are re-derived from the current codebase, not
inherited.

> Findings that constitute *failures* are canonical in `/FAILURES.md`. This file is the fuller
> record of the audit era, including items that were design clarifications rather than failures.

---

## Genuinely resolved (verified against current code)

| ID | Finding | Status |
|---|---|---|
| **A1** | "All meaning is relationship" was stated as a theorem; the Yoneda argument for it is circular (Yoneda holds in *any* locally small category and cannot establish the postulate). | **Correctly reclassified** to a postulate. Sound. |
| **F2** | The operation set was claimed minimal; `conjoin` is derivable from `{negate, disjoin}` by De Morgan. | **Correctly weakened** to "complete and natural generating set." See `FAILURES.md` F-002. |
| **A2** | The Euclidean setting was presented as forced; it is a simplification. Most core results are geometry-independent, but the embedding argument's use of angle density in [0, 2π) is a Euclidean-specific step. | **Correctly documented** as a simplification. The flagged embedding step remains unverified under alternative geometries. |

## Design clarifications (not failures, still live)

| ID | Finding | Current status |
|---|---|---|
| **F3** | The formal Modifier carrier set (all invertible plane transformations up to homeomorphism) vastly exceeds what the writing system can draw (rotations, uniform scalings, reflections). | **Intentional gap**, documented. The algebra is richer than the notation. Fine, provided the asymmetry is stated wherever either is specified. |
| **B2** | Modifier has a dual nature — simultaneously an operand (passed to `modify_entity`, `quantify`) and a morphism (it acts on objects). Should it be a sort or a hom-object? | **4-sort design retained** for pragmatic reasons. **Interacts with a live bug:** the Rust `Sort` enum gives Modifier no discriminant, so four structurally different function shapes (`e→e`, `r→r`, quantifier, `a→a`) type-check identically. See "Still open" below. |
| **F4** | No operation produces a Relation from another sort — no `e → r` path, so relation vocabulary is stipulated rather than constructible. Natural language has denominalization ("hammer" → "to hammer"). | **Principled absence.** Expressible via `abstract(e) → m` then `modify_relation(m, r₀) → r`. No new operation needed. Reasoning holds. |
| **F5** | An enclosure appears to have context-dependent sort (Assertion when standalone, Entity when embedded). | **Not ambiguous in the algebra** — an enclosure is always Assertion; `embed(a) → e` produces the entity. The *visual* form is ambiguous, not the type. Documentation fix only. |
| **D1** | Tension between "minimal complete system" and a writing system with 105+ constructions. | **No tension.** The constructions are *terms* in the algebra, not additional axioms — as decimal notation is to arithmetic. Reasoning holds. |

## Still open (carried forward)

| ID | Finding | Where it lives now |
|---|---|---|
| **F1** | Negation-as-reflection produces *converse*, not negation. | `FAILURES.md` F-001. Documented fix (boundary inversion via σ) **never implemented** — F-006. Scheduled for reimplementation per `claims.yaml#NEGATE-REIMPLEMENTATION`. |
| **F7** | 4-sort algebra vs. 5-primitive geometry tension. | `FAILURES.md` F-007. Resolution was specified in April 2026 and never propagated; still open, designated as the first test of the new definition of done. |
| **B2-impl** | Modifier sort has no discriminant in code; four function shapes type-check identically. | Real type-safety gap in `ul-forge/crates/ul-core/src/types/sort.rs`. Cheap to fix, directly serves invariant checking. |
| **C1** | Machine parsing of 2D constructions — a spatial construction grammar was specified but never implemented. | Grammar exists on paper (`uws/grammar/`); no 2D image parser. Unchanged. |
| **F6** | No complete bidirectional mapping between spatial relationships, visual composition operations, and algebraic operations. | Partially addressed by `operation-visual-map.md`. Still no single normative table. |

## Retired outright

| ID | Finding | Why |
|---|---|---|
| **D2** | The 50-case completeness challenge, scored 100%. | **Not evidence.** Reached 100% across nine rounds of patching the theory after each failure. `FAILURES.md` header explains why this is worthless as validation. Test *cases* may be reusable; the *score* is not. |
| **D3** | "UQPL is a Σ_UL-algebra" relationship analysis. | Superseded. Only 3 of 13 operations ever mapped, and Σ_UL is retired. The wiki independently concluded "no intervening algebraic signature required." See `design/uqpl/`. |
| **B6** | "Yoneda generalizes the Unique Grounding Theorem." | Retired with the theorem. The original authors already noted it is a reformulation that "does not strengthen the conditional dependency" — correct, and it means the result carried no evidential weight. |

---

## What the audit era got right, and should be preserved as practice

The pass1–pass3 process **did** find real errors — F1 (a genuine correctness bug), F2 (a false
minimality claim), A1 (a circular argument) — several of which this project would still be
repeating otherwise. The instinct to run adversarial self-audits was correct.

Its failure was structural, not attitudinal: findings were closed in the audit rather than in the
artifact, so a "RESOLVED" label could coexist indefinitely with unfixed code. That is fixed by the
definition of done in `research/postmortem-and-rebuild-2026-08.md` (fix + test + propagation scan),
not by auditing less.
