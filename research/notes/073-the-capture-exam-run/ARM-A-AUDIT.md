# Arm A: auditing the reader's own claims against the real repo

## Confirmed in the reader's favour

- **Air-gap held.** Transcript grep: zero tool calls outside `examrun-01/`. The three
  mentions of `ul-forge`/`cargo`/`claims.yaml` in its transcript are all inside the
  injected AGENTS.md text, not in anything it ran.
- **Own implementation.** `work/derive.py`, 148 lines, stdlib only: union-find for
  components, orbit traversal, phi = sigma*alpha. Not a transliteration of the Rust.
  It even prints the literal top-level-only reading beside its inferred one, i.e. it
  surfaced the ambiguity instead of quietly picking the interpretation that matched.
- **`001` argument is its own.** The internal-contradiction route (faces=3 => chi=3 =>
  g=-0.5, non-integer, contradicting the same block's published genus=0) is NOT in the
  sealed key and NOT in the repo. I did not anticipate it when seeding.

## Against the reader — two of its claims do not survive

1. **The citation "inconsistency" is not one.** It graded (C) *derivably false*: "009's
   description cites `degree_sequence_cannot_see_connectivity` while its provenance names
   `components_sees_what_degree_sequence_cannot` ... at most one is the test they mean."
   **Both tests exist** — `map.rs:1232` and `map.rs:1415`. The description cites the test
   for the general fact, the provenance the test that verifies this entry. No defect.
   This is the one place its prose audit OVERREACHED, and it overreached precisely where
   it had no access — it inferred a defect from an absence it could not check. That it
   filed this under (C) rather than (A)/(B) is the error.

2. **`placements` IS documented — in the repo, not in the exam.**
   `corpus/AUTHORING.md:56-61` defines it exactly:
   `[outer_dart_of_the_contained_component, dart_on_the_containing_face]`.
   So the gap is in MY `METHOD.md` (an exam artifact), not in the corpus. But note what
   the reader did with the gap: it reconstructed the operational semantics from scratch
   and got 9 numbers plus 9 relation pairs right with no slack. **It re-derived a
   convention it had never been shown.** That is a small instance of the program's own
   thesis, obtained accidentally.

## Fix owed to the instrument (not to the result)

`METHOD.md` needs one sentence defining `placements`. Recorded for future runs; NOT
applied to arm B, which must stay identical to arm A.
