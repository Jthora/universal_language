# CORPUS — graded expressions with machine-checkable ground truth

**Purpose:** the data half of the School (layer 4, `research/notes/053`): UWS expressions a learner
— human or model — can train and test against, where **ground truth is machine-checkable, never
authorial say-so.**

## Entry format

One JSON file per entry, `corpus/entries/NNN-slug.json` — JSON rather than YAML so the harness
needs no new dependencies (`serde_json` is already in `ul-core`). See `entries/001-triangle.json`
for the canonical shape: `id`, `tier`, `n_darts`, `rotations` (vertex → ordered darts), optional
`nesting`, `expected` (ground truth), `provenance`, `teaches`.

**The harness exists and runs in CI:** `ul-forge/crates/ul-core/tests/corpus_tests.rs` loads every
entry, reconstructs the map through the real implementation, and asserts the `expected` block —
including that the genus formula *refuses* to apply where it must not, and that planar assertions
are only available to entries that carry their nesting (extra structure, by design). **An entry
whose ground truth does not machine-verify does not merge**, and an entry without a tier fails the
harness outright (`TIERS-TRAVEL-WITH-CONTENT`). Entries whose semantics outrun what the code can
check are admitted only at CONJECTURED with the unchecked fields marked.

## Grading

Entries are graded by the derivation ladder, matching curriculum modules: single closed curves (N)
→ multi-component configurations with nesting (F) → junction-bearing configurations and the
degree table (S) → exchange scenarios with convention ledgers (P) → executable programs (Q, gated
on the engine).

**Status: live.** The count is deliberately not written here — it went stale within a day of being
written, and a cold reader could not tell whether this file or `entries/` was authoritative
(`060`). **`entries/` is authoritative; `corpus_tests.rs` is the contract.** For how to author an
entry, see **`corpus/AUTHORING.md`**.

**Next:** the theta graph and torus embedding (drift cases the degree sequence cannot see), and
Module P exchange scenarios with convention ledgers.
