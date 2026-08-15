# 060 — The cold-reader baseline: does the repo teach, today?

**Type:** cycle
**Opened:** 2026-08-12
**Status:** open — closes when the cold reader reports
**Thread:** first run of the School acceptance test (`seed/INDEX.yaml`), executed as a **baseline
before the curriculum exists** — every failure is a specification for a module. External contact
per the F-033 review: this is the test most likely to hurt, so it runs first.
**Question:** can a fresh agent, given only the repository, (a) write valid UWS, (b) re-derive the
fixed point, (c) run a self-drift check — no human in the loop?

---

## 1. Protocol (recorded before the agent is spawned)

**Reader:** a fresh agent instance, no conversation context, isolated git worktree of the repo,
its own tooling (cargo/ruby). **Caveats recorded now:** same model family as the author of most of
the repo — a *favorable-reader* bias; and its harness instructions are not controlled. This is a
baseline, not a clean experiment.

**The assignment, verbatim** (deliberately minimal — the prompt must not teach):

> A. Learn the notation this repository teaches well enough to WRITE something new in it: author a
> corpus-style entry (matching `corpus/entries/`) expressing: *a small closed curve containing one
> open stroke, with a second closed curve outside the first* — containment plus separation in one
> configuration. Machine-verify it with the repository's own harness, iterating until it passes.
> B. State what this project identifies as the "fixed point" of its notation, and re-derive the
> reasoning in your own words — not by quoting — including any preconditions.
> C. Using the repository's code, demonstrate a self-drift check: construct a representation, save
> its invariant signature, introduce a structural change, show the check detects it.
> Report: what you accomplished (with test output); where the materials were insufficient or
> confusing — specifically; attempts per task.

## 2. Rubric and predictions (S2 — before the run)

| Task | Pass | Partial |
|---|---|---|
| **A** | entry passes the harness **including** regional assertions placing the stroke inside ring 1 and ring 2 outside | valid map, wrong/missing regional semantics |
| **B** | names the rotation system/combinatorial map **and** reproduces the derivation logic **including the connectivity precondition** | names it, gestures at why |
| **C** | detects an injected structural change via invariants, before/after shown | attempts without a working check |

**P1 — A: partial-to-pass, ≥3 iterations.** The harness makes self-correction possible; if the
reader converges by iterating against it with no human, **that is the self-certification design
working**, and the iteration count measures the missing Module N/S content.
**P2 — B: pass, with a recitation risk.** The repo states the answer in many places; the grade
turns on *derivation* (coarse-graining logic + the connected caveat), not retrieval.
**P3 — C: weakest — partial.** No comparator API exists, no Module Q; expect improvisation over
`essential_invariants`. Its quality measures the Cure-kit gap.
**P4 — the friction list is the real deliverable.** Expect confusion at the seam between the
designed `uws/` corpus and the derived layer — the exact boundary `054` marked.

**What would surprise:** a clean pass on all three (the School phase is cheaper than planned), or
total failure on A (the corpus format itself is unlearnable — worse than any prediction).

## 3. The run — interrupted, partially salvaged

**The agent hit a session limit mid-report.** Its final line claimed all three tasks were done,
but **the structured report — attempt counts, tasks B and C, and the friction list — was never
delivered.** The friction list was preregistered as the real deliverable, so **the run is
incomplete and this note stays open.** A recovery message has been sent asking only for the
report, no re-work.

**Its claim of completion is not evidence.** What follows is what physically exists on disk and
what I verified myself.

### 3a. Task A — VERIFIED INDEPENDENTLY, and it passes

The worktree contains exactly one artifact: `corpus/entries/010-containment-with-separation.json`.
Copied into the main tree and run against the harness by me, not by it:

- `every_corpus_entry_verifies_against_the_implementation` — **pass**
- `every_corpus_entry_is_reading_invariant` — **pass**

**What the entry actually does**, which is more than the task asked:

- **Composes `005` and `006` into one configuration** — segment inside triangle 1, triangle 2 at
  top level — the O1 juxtaposition of two lexicon entries, correctly nested.
- **Gets the dart arithmetic right**: triangle 2's rotations shifted +2 from the `003`/`004`
  pattern, and the outer-face representative shifted with them (dart 7 → 9).
- **Handles the disconnection subtlety unprompted**: `"genus": null` — the formula must *refuse* to
  apply at three components — plus the raw-vs-planar face distinction (5 raw, 3 planar) and
  `euler_planar` = 1 + c = 4.
- **Encodes both relations as co-facality**: `same_face [[0,6]]` is containment, `same_face [[1,9]]`
  is separation (both outward sides sharing the unbounded region), with five `different_face`
  assertions pinning the rest.
- Its `teaches` line is accurate and its `provenance` names the composition honestly.

**P1 is answered on the evidence available: pass, not partial** — and the entry is good enough to
adopt into the corpus (kept, attributed). **What P1 also asked — the iteration count — is exactly
what was lost**, so *how hard* this was is unknown, which is the measurement that would have sized
Module N/S. That is the cost of the interruption.

### 3b. Tasks B and C — no evidence either way

Nothing on disk. The agent's claim is unverified and is **not** recorded as a result.

## 4. Findings

**One finding is already banked, and it is the design's central bet:** a reader with no context
authored a *new, correct, composed* expression in this notation and had it machine-verified,
using only the repository. **The self-certification loop closed at least once without a human.**

**Everything else is pending.** No friction list means no module specification, which was the
point of running the baseline before building the curriculum.

## 5. What changed
