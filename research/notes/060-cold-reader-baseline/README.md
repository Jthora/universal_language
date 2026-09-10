# 060 — The cold-reader baseline: does the repo teach, today?

**Type:** cycle
**Opened:** 2026-08-12
**Status:** closed — report recovered after a session-limit interruption
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

### 3b. Report recovered

The agent was resumed and delivered the full report. Tasks B and C are assessed below from its
report; only task A rests on artifacts I verified myself.

## 4. Findings

### 4a. All three passed — the preregistered SURPRISE condition, and P3 was wrong

| Task | Predicted | Actual |
|---|---|---|
| A — author an entry | partial-to-pass, ≥3 iterations | **pass, 1 attempt** — plus an unprompted negative control |
| B — re-derive the fixed point | pass, recitation risk | **pass, derived not recited** |
| C — self-drift check | **partial (weakest)** | **pass** — P3 wrong |

**On A:** it did not merely pass — it *falsified `planar_faces` to 4, confirmed the harness fired,
then restored it*, on the stated grounds that a first-try pass is weak evidence. **A cold reader
independently invented the negative-control habit this repo relies on.**

**On B:** the derivation is reconstructed, not retrieved. It named the Erlangen/RG distinction
unprompted, walked the group tower, derived why only topological conditions reach the top, invoked
Jordan for Enclosure and Heffter–Edmonds for the rotation system — and listed **five preconditions
including the two that hurt**: connectedness (F-025), and that *why topology is minimal remains
argued, not proved*. It also flagged reading-invariance as still CONJECTURED and the object as a
DCEL reduct. **It reproduced our own honest limits without being asked for them.**

**On C, which I predicted would be weakest:** it wrote a drift check, ran it, and produced the one
result I would have wanted:

- 𝔽₀ re-readings (mirror, relabel, subdivide×2) **do not fire** — re-reading is not drift;
- an added edge fires;
- **the theta case fires where the degree sequence cannot see it** — identical `[3,3]` degrees,
  embedding slid plane → torus, signature `(1,3,Some(0))` → `(1,1,Some(1))`;
- reconstruction from the recipe restores the recorded signature — **the re-derivable anchor
  `CURE-IS-COROLLARY` asserts, demonstrated by a stranger.**

### 4b. The friction list found four real defects — one worse than reported

Every claim was verified before acting. **All four were real:**

1. **Vacuous entries could pass.** Every `expected` key was optional and silently skipped, with no
   minimum assertion set. **And the truth was worse than reported:** indexing a missing key yields
   `Value::Null`, so an entry that merely *omitted* `genus` was silently asserting *"the formula
   must not apply"* — **an assertion its author never wrote.** The reader inferred
   absent-asserts-nothing; absent was asserting something false. Both fixed, verified by probe.
2. **`corpus/README.md` was factually stale** — "four entries" against ten on disk, and it listed
   as future work an entry that already existed. Fixed by *removing the count*: prose that
   duplicates the tree goes stale, so the tree is now named as authoritative.
3. **`cycle()` was duplicated** — a private test helper shadowing the public API, so `provenance`
   strings naming `map.rs::tests::*` pointed at a non-public code path. Collapsed.
4. **`README.md` documented a `design/` directory that does not exist.** Fixed.

**The new anti-vacuity rule immediately caught my own seed entries** (`001`–`004` asserted no
`components`) — and then caught a case where **the rule, not the entry, was wrong**: `008`'s
meaning *is* its degree sequence, so requiring *regional* assertions was too narrow. The rule now
names both structural kinds. A checker written from a stranger's finding, correcting its author
twice within minutes of existing.

### 4c. The gap that matters most, and I did not predict it

> *"Nothing connects the notation to the marks. The repo is titled a writing system… but every
> entry is a rotation system in JSON. There is no worked example anywhere of going from a drawing
> — a circle with a line in it — to darts and rotations."*

**P4 predicted friction at the designed/derived `uws/` seam. The real gap is deeper: there is no
bridge from drawings to darts at all.** Every corpus entry's `teaches` field cites curriculum
modules N and F **that do not exist**. The artifact currently teaches the *fixed point* and not
the *writing*.

Its second finding of the same shape: **the Cure's comparator and `essential_invariants()` are the
same mechanism described in two vocabularies that never cross-reference** — `spec/reading-invariance-v1.md`
documents it as a reading-invariance device and never links it to the Cure, so task C had no named
entry point.

### 4d. Its answer to "what would you write first" — written

**`corpus/AUTHORING.md`, "from a drawing to a verified entry"** — now exists, with every convention
it had to reverse-engineer stated as a rule: `α(d) = d^1`, the `[outgoing, incoming]` cycle
convention, **even/odd darts and which orbit is outward**, the `placements` argument order, the
absent-vs-null distinction, which face set the regional assertions consult, and the falsify-then-
restore habit. Its closing section names the rendering gap rather than papering it.

## 5. What changed

- `corpus/AUTHORING.md` created — the reader's own top ask, written.
- Harness: absent-vs-null distinction fixed; anti-vacuity rule added (structural floor mandatory;
  lexicon entries must machine-check regional **or** degree structure). Verified by probe.
- `001`–`004` gained the `components` assertions they were missing; `010` adopted (attributed).
- `corpus/README.md` de-staled (count removed, tree named authoritative); duplicate `cycle()`
  collapsed; phantom `design/` reference removed from `README.md`.
- `claims.yaml`: `CORPUS-IS-MACHINE-VERIFIED` scope amended with the vacuity hole and its fix.
- **Open, and now specified rather than guessed:** the **drawing → darts** bridge is the top
  curriculum gap (Modules N/F, cited by every entry, unbuilt); the comparator needs one named
  entry point linking `essential_invariants()` to the Cure vocabulary.
- **Caveats on this result, recorded:** the reader is the same model family as the repo's author
  (favorable-reader bias, stated in the protocol before the run), and one run is one run. What it
  demonstrates is that the materials are *sufficient for a capable reader*, not that they are good.
