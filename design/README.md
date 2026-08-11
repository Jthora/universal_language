# design/ — What Is Intended

**Tier rule: everything here is NOT YET BUILT. Every document must say so at the top.**

This directory holds specifications for things that don't exist yet. That is a legitimate and
necessary kind of document — it just must never be mistaken for `spec/`.

## Requirements for a document here

1. Opens with an explicit **"Not yet built"** banner stating current implementation status.
2. Design decisions are registered in `/claims.yaml` as `DESIGN-CHOICE`, with rationale and
   alternatives considered.
3. Known blockers are named, not glossed.

## Contents

### `uqpl/`
Specification drafts for the Universal Quantum Programming Language.

**Status: not built.** `ul-forge` contains zero UQPL code — this is greenfield, which means no
legacy to tear out and everything to construct.

Known issues in the current drafts, carried from `research/deep-critique-2026-08-wiki-and-implementation.md`:

- **`uqpl-spec.md` §0 is invalid.** Its "STATUS AND HONESTY" table cites the Unique Grounding
  Theorem and the Σ_UL signature as *Proven*. Both are retired. The section must be rewritten
  before the spec is used; most of the spec body survives.
- **Two undeclared types.** `transform : Modifier × Process → Modifier` uses `Process`, which is
  not among the declared sorts; `bound : Set<Modifier> → Assertion` uses an undeclared `Set<_>`
  constructor. Likely leftovers from the 5-primitive geometry never reconciled with the 4-sort
  type system.
- **Arrow types are used but not declared.** `quantify : (Entity → Assertion) → Assertion` and
  β-reduction both require a function-type former that the base-type list omits.
- **Totality is a feature, not a gap.** With β-reduction and no `fix`, the language is strongly
  normalizing and therefore provably not Turing-complete. Adopt this deliberately — a repair loop
  that can diverge is unusable in a safety path. See `claims.yaml#UQPL-TOTALITY`.
- **No execution substrate exists.** The Rust composer is a graph *builder* (`Gir → Gir`), with no
  evaluator, substitution, or reduction relation. Building UQPL means writing an interpreter.

### Planned, not yet written

- `enforcement-plan.md` — CI checks for `claims.yaml`, glossary collisions, stale `RETIRED`
  references, and link resolution. Referenced by `claims.yaml`; needs authoring.
- `cure/` — the Encode → Check → Detect → Repair → Reconstruct loop. **Blocked** on the repair
  operator problem (`FAILURES.md` F-009), which is the program's central open engineering question.
