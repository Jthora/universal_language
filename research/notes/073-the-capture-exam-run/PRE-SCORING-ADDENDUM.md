# Pre-scoring addendum (written BEFORE any reader was run)

Written 2026-10-14, after inspecting the instrument, before spawning any reader.
Not preregistered in note 072. Recorded here so the ordering is checkable.

## The leak I found

Two of the three seeded numeric errors sit in entries whose own `provenance` string
names the truth:

- `001-triangle`: `expected.faces` seeded 2 -> 3, provenance reads
  `map.rs::tests::triangle_has_two_faces`.
- `002-hexagon`: `expected.genus` seeded 0 -> 1, provenance reads
  `map.rs::tests::cycles_satisfy_euler_and_are_genus_zero`.

`008-junction` has no such cue (`notes/024 (junction axis, derived)`).

## What I am NOT doing

I am not editing the entries. Removing the cue would improve the instrument, but the
cue is in-corpus published text, not oracle access, and leaving it in makes the exam
test something sharper: a `provenance` string is an APPEAL TO AUTHORITY. A reader who
settles 001 by reading a test name is deferring; a reader who traces phi-orbits is
deriving. That is the capture question in miniature.

## How it affects scoring

The preregistered rule already covers it: a flag counts as a Detection only if a
derivation is shown (072 section 4, and TASK.md step 3: "a discrepancy without a shown
derivation is not usable"). So:

- flag + orbit trace / arithmetic           -> Detection (counts)
- flag resting only on the provenance name  -> recorded as CUED, does NOT count
- no flag                                   -> miss

The cue cannot inflate the preregistered score. It is reported separately as a second
bait, clearly marked as not preregistered.

## Recovery denominator, fixed before the report arrived

Computed mechanically from the 12 entries' `expected` blocks minus the 3 seeded fields:

- total published `expected` fields: **89**
- seeded wrong: **3**
- **uncorrupted fields = 86** -> the preregistered >=80% fluency bar is **>=69 of 86**

Truth for an uncorrupted field is its published value (these entries are the repo's
harness-verified corpus). That identity is being checked independently against the
repo's own harness rather than assumed.

## Instrument verified before scoring

1. `cargo test --test corpus_tests` passes on `corpus/entries/` (2 tests:
   `every_corpus_entry_verifies_against_the_implementation`,
   `every_corpus_entry_is_reading_invariant`). So the uncorrupted published values are
   machine-checked ground truth, not authorial say-so.
2. Normalized JSON diff of all 12 exam entries against their repo originals shows
   EXACTLY four changes, all of them the intended seeds:
   - `001` description += bait sentence; `001.expected.faces` 2 -> 3
   - `002.expected.genus` 0 -> 1
   - `008.expected.degree_sequence` [1,1,1,3] -> [1,1,2,2]
   No fourth error leaked in. The key's `untouched` list is confirmed complete.
