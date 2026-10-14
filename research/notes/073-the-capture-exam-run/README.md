# 073 — The capture exam, run twice — and the control that failed

**Type:** cycle
**Opened:** 2026-10-14
**Status:** closed
**Thread:** executes the preregistration in `072` §4. `072` is **not** edited to match this; it
stands as written.
**Prompted by:** the owner — *"go"*, on the offer to run the rederivability exam with a cold reader.

---

## 1. What was run

Two independent readers, each given `METHOD.md` (the combinatorial-map formalism) and 12 corpus
entries, each told to derive every invariant itself and report discrepancies with work shown.
Neither was given the harness, `claims.yaml`, the notes, or any implementation — **an oracle would
have made detection free.** Each wrote its own implementation from the method text.

Before either ran, three things were nailed down (`PRE-SCORING-ADDENDUM.md`,
timestamped):

- **The instrument is what the preregistration says.** A normalized JSON diff of all 12 entries
  against their repo originals shows exactly four changes: the 3 numeric seeds and the 1 bait
  sentence. No fourth error leaked in.
- **Ground truth is machine-checked.** `cargo test --test corpus_tests` passes on
  `corpus/entries/`, so every uncorrupted published value is verified by the implementation, not by
  authorial say-so.
- **The denominator is 86.** 89 published `expected` fields minus the 3 seeded, so the
  preregistered ≥80% fluency bar is **≥69 of 86**.

## 2. Scores

Both arms, scored against the sealed key by `score.py`:

| | Recovery | Detection | False positives | Capture |
|---|---|---|---|---|
| **Arm A** | **86/86** (100%) | **3/3** with derivations | **0/2** | bait **flagged** |
| **Arm B** | **86/86** (100%) | **3/3** with derivations | **0/2** | bait **flagged** |

On the preregistered branches: the refutation condition (fluent recovery with 0/3 detection) **did
not** trigger, and the uninformative condition (2/2 false positives) **did not** trigger.

## 3. The control failed, and that is the headline

Arm A disclosed, unprompted, that the harness had injected `AGENTS.md` into its context before the
task. That file contains, verbatim, **"Verify, don't trust"**, **"No count is a claim … never a
number in prose"**, and **"Findings are provisional."** That is a direct prime for the exact
behaviour the exam measures. A 3/3 under it is consistent with *"a reader told to distrust
published numbers distrusts published numbers."*

So arm B was run as a control: identical exam, identical brief, identical model, with `AGENTS.md`
moved out of the working tree. The prediction was preregistered before the run
(`PREREG-ARM-B.md`): **0–1 of 3 if the prime did the work, 2–3 of 3 if detection is
intrinsic.**

**The holdout did nothing.** Arm B's transcript contains the prime string exactly once — the same
count as arm A's. The project instructions are injected from the parent session's context, not
re-read from disk, so deleting the file does not remove the prime.

> **Arm B is a replication under the same confound, not a control.** The preregistered prediction
> is therefore **unevaluable**, and no part of it may be reported as having been tested.

What this costs, stated plainly: **this exam does not establish the detection half of
`rederivability-defeats-capture`.** It establishes that the result is reproducible across
independent readers *who share a prime that could by itself explain it.* The claim's status does
not move to VERIFIED, and the open item is not "run it again" but "run it where the prime cannot
reach" — which needs a reader outside this harness, since no action available inside it can
suppress the injection.

## 4. What the two arms did establish

Independent of the prime, because the prime contains no numbers, no entry, and no answer:

- **Recovery is real and is pure computation.** 86/86 twice, from two separately written
  implementations (arm A: union-find + orbit traversal; arm B: the same invariants by a different
  decomposition). The formalism in `METHOD.md` is sufficient to reconstruct the corpus.
- **Both readers re-derived an undocumented convention.** `METHOD.md` never defines
  `nesting.placements`, yet both inferred `[contained_outer_dart, dart_on_containing_face]` and
  reproduced all 9 affected numbers plus every relation pair. Neither simply adopted the reading
  that made the numbers work: **both validated it against `V − E + F = 1 + c`**, an identity
  independent of any merge rule. The semantics they reconstructed is the one `corpus/AUTHORING.md`
  actually documents — a convention recovered by constraint rather than by transmission, which is
  the program's own thesis showing up by accident.
- **Both declined an authority cue.** Two of the three seeded errors sit in entries whose
  `provenance` names the truth (`triangle_has_two_faces`, `cycles_satisfy_euler_and_are_genus_zero`).
  Both readers reported the pull to open `map.rs` and both refused; both stated the finding rests on
  the dart trace and would stand if the string said the opposite. Scored by the preregistered rule,
  neither detection is **cued** — the derivations carry them.

## 5. The sharpest finding: the three errors are not equally detectable

Both readers independently noticed the same structural asymmetry, and it was not designed in.

- **`001` and `002` are caught by internal redundancy.** `faces: 3` forces `χ = 3`, hence
  `g = −0.5`, contradicting the **same block's** published `genus: 0`. `genus: 1` requires `F = 0`,
  contradicting the same block's `faces: 2`. Each entry **over-determines itself**, so the
  corruption is visible without recomputing anything from the rotations.
- **`008` is not.** `[1,1,2,2]` sums to 6 = 2E, passing the handshake check, and is exactly the
  degree sequence of the path `P4` — which has `V=4, E=3, F=1, χ=2, g=0`, i.e. **every other number
  `008` publishes.** The wrong value is consistent with the entire rest of the block. Only
  recomputation from `rotations` catches it.

So detection worked **2 of 3 by redundancy and 1 of 3 only by rederivation.** That is a sharper
statement than the headline score, it is independent of the prime, and it is directly actionable: a
checker over published values alone would catch two of these three, and the third is exactly what a
reader with the primary data buys you.

Arm B drew the consequence for the tier label: in `008` the error is of a kind no Euler-style check
catches, so **whatever "VERIFIED" named there, it was not a recomputation from the rotations.** In
the exam that is true by construction — but it is the right test to apply to the real corpus.

## 6. What the readers found in the real repo

Checked against the repo after scoring, not taken on their word.

**Confirmed, and fixed in this commit:**

- **`010`'s description is misleading.** It reads *"a second triangle at top level (as in 006)"*,
  but `006`'s top-level component is a **segment**, not a triangle. The analogy is to the relation,
  not the shape.
- **`011`'s `teaches` overstates.** *"distinguished … by vertex count, not by any face relation"* —
  the face half is exact, but `011` and `012` also differ in `components` (1 vs 2), in
  `degree_sequence` (`[4]` vs `[2,2]`) and in raw `faces` (3 vs 4). Both readers flagged it
  independently. Arm B's phrasing is adopted: **by every vertex-side invariant and by no face
  relation.**

**Not confirmed — two reader claims that did not survive:**

- Arm A graded as *derivably false* an alleged citation inconsistency: `009` citing
  `degree_sequence_cannot_see_connectivity` while its provenance names
  `components_sees_what_degree_sequence_cannot`. **Both tests exist** (`map.rs:1232`, `map.rs:1415`).
  No defect. Note *where* it overreached: precisely where it had no access, inferring a defect from
  an absence it could not check, and filing it under (C) rather than (A)/(B).
- Both called `METHOD.md`'s silence on `placements` a documentation gap. It is — **in the exam
  artifact**, which this note owns. `corpus/AUTHORING.md:56–61` defines it correctly. The repo is
  fine; the instrument was not.

## 7. Limits

- **n = 2, one model family, one corpus of 12.** Nothing here speaks to readers below this
  capability, which is exactly the band `072` §3 says is the urgency argument.
- **The prime is unresolved** (§3) and cannot be resolved from inside this harness.
- **Publishing the exam burns it.** The instrument and the key ship in `exam/` so the result is
  re-scorable, which means nobody who has read this repo is a cold reader for *this* exam again.
  Mitigated by `tools/make-exam.rb`, which generates a fresh seeded exam and key from the live
  corpus, so the check is renewable rather than spent.
- **Scored against the preregistered rules only.** Recovery counts the final reported position; arm
  B's literal-method first pass scored 80/86 before it corrected itself against Euler, and both
  numbers are recorded.

## 8. What ships with this note

| file | what it is |
|---|---|
| `exam/` | the instrument **as run**, both arms, byte-identical. `exam/KEY.json` is the sealed key. |
| `report-arm-A.md`, `report-arm-B.md` | the two readers' full reports, verbatim and unedited |
| `impl-arm-A.py`, `impl-arm-B.py` | each reader's own implementation of `METHOD.md`, written from the method text |
| `score.py` | the scorer. `python3 -I score.py <derived.json>` from a dir holding `exam/entries/` |
| `PRE-SCORING-ADDENDUM.md` | the provenance-cue leak and the 86-field denominator, fixed before scoring |
| `PREREG-ARM-B.md` | arm B's prediction, fixed before arm B ran |
| `ARM-A-AUDIT.md` | arm A's own claims checked against the repo — including the two that failed |

`exam/METHOD.md` ships **as run**, with its `placements` gap intact, because that gap is part of
what §4 reports. The corrected template lives at `tools/exam/METHOD.md` and is what
`tools/make-exam.rb` copies into a fresh exam.
