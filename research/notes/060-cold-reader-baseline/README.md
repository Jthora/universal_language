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

## 3. The run

## 4. Findings

## 5. What changed
