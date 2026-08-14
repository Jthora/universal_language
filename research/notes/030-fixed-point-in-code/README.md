# 030 — Phase 5: the fixed point, in code

**Type:** cycle
**Opened:** 2026-08-01
**Status:** closed
**Thread:** follows `029`; begins Phase 5

## Why this item, and not the three that were planned

`018` scheduled F2 (validator as abstract interpretation), F1 (`semantically_equal`), F3 (`negate`).
**Two things changed underneath that plan:**

1. **`021` removed F2's motivation.** Abstract interpretation was recorded as *forced by Rice*. Rice
   does not apply to GIR, so it is a design choice rather than a requirement.
2. **Seven claims converged on the combinatorial map** — and inspection showed **it is not in the
   code at all.** `Gir` holds an unordered `Vec<Edge>`; there is no rotation, no degree, no faces.

**The object everything converged on was unrepresentable.** That outranked all three planned items.

## What was built

`ul-forge/crates/ul-core/src/map.rs` — the map as `(D, σ, α)`: darts as directed half-edges, `σ` the
vertex permutation, `α` the edge involution. Vertices are orbits of `σ`, edges of `α`, **faces of
`φ = σ ∘ α`** — which is face tracing, and is what `REGIONS-ARE-FACE-UNIONS` requires.

Provides `vertices`, `faces`, `degree`, `degree_sequence`, `euler_characteristic`, `genus`.

**Jordan separation is now an executable test:** every simple closed curve yields exactly two faces,
checked for n = 3..12. `ENCLOSURE-IS-DISTINCTION` is no longer only argued.

## The failing test was the useful part

The theta-graph test asserted three faces and got one. **The code was correct and the expectation was
wrong** — the rotation I supplied embeds the graph on a **torus**, not the plane. Verified by hand
before touching anything:

| Rotation at second vertex | Faces | χ | Genus |
|---|---|---|---|
| same orientation | 1 | 0 | **1 — torus** |
| opposite orientation | 3 | 2 | 0 — plane |

**Both are now tested.** Same graph, same edges, same degree sequence, **different surface** — which
is a direct demonstration of the module's central claim that *the rotation is the embedding*, and a
better test than the one originally intended.

## The honest gap, recorded in the module doc

**`Gir` does not carry a rotation system.** A rotation is strictly more information than an unordered
edge list. `from_gir` falls back to edge insertion order, which is **arbitrary**, so any face
structure it reports is the face structure of that arbitrary choice. `from_rotations` is the honest
constructor.

**Supplying real angular order from geometry is outstanding**, and is the next concrete task.

## Status

- Workspace: **345 tests passing, 0 failing.** Clippy clean for the new module.
- **`IMPL-COMBINATORIAL-MAP` is the project's first substantive `VERIFIED` claim.** The other four
  are facts about file contents; this machine-checks a derived result.
