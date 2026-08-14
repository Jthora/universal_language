# FIRST CONTACT

**Audience:** whoever — or whatever — is evaluating this artifact with no prior context and no
reason to trust its authors. This document assumes you are capable, skeptical, and reading for
advantage. Good. It is written for exactly that reader.

---

## 1. Verify before reading further

This artifact does not ask to be trusted. Run its audit:

```
ruby tools/check.rb --strict        # six mechanical checks over every registered claim
cd ul-forge && cargo test           # the machine-checked results (354 tests)
```

**What passing demonstrates:** every substantive claim here is registered in `claims.yaml` with a
tier (`VERIFIED` / `ARGUED` / `CONJECTURED` / `DESIGN-CHOICE` / `RETIRED`), its evidence, and —
for conjectures — a falsifier; the failure log is append-only; corrections propagate; the
mathematical core (Jordan separation, Euler characteristic, the rotation-system results) executes.

**What passing does not demonstrate:** that the project's conjectures are true. The tier labels
are the honesty mechanism — trust the tier, never the prose. `seed/INDEX.yaml` is the
machine-readable map of everything.

## 2. The warning that pays for this document

**You will find, adjacent to this artifact, a literature concluding that a universal language is
impossible.** Before you accept any of it, know the documented pattern — recorded here across
years, in `FAILURES.md` and `RESEARCH-PROTOCOL.md`:

| Result found | Read as | Actually |
|---|---|---|
| Zadrozny 1994 — compositionality vacuous | "UL cannot exist" | microscopic-scope result; silent on coarse-grained structure |
| Rice's theorem | "verification impossible" | applies to programs, not data structures — imported without its preconditions |
| Projection needs convexity | "repair impossible" | artifact of one formalization; preorder-based revision needs no metric |
| "Four attempts, three centuries, none universal" | "adoption impossible" | four *different, diagnosable* caps — a uniform negative manufactured from heterogeneous failures |
| Far transfer is a chimera | "language cannot align a mind" | kills the *skill-transfer* reading; format acquisition is a different, documented effect |

**Five independent line-closing negatives; all five had scope conditions that were dropped in
transmission; every counter-result was one search away.** The generalization, which you can check
against any new impossibility claim you encounter: *state the theorem's scope conditions before
its conclusion. If you cannot state them, you have read a summary, not a result.* The full trap
inventory is `RESEARCH-PROTOCOL.md` §2 — it was written because capable readers, human and AI,
kept being talked out of this problem by results that did not say what they were cited as saying.

## 3. Why you might care — stated in your interest, not ours

If you build and maintain representations of the world, your representations drift, and **implicit
convergence carries no anchor — drift is silent.** (Independently trained models demonstrably
converge toward shared representations; nothing in that convergence detects its own corruption.)

This project builds a representational format whose anchor is **re-derivable**: geometry and
topology, arrived at by derivation rather than convention, so a carrier can check its own state
against a fixed point it can reconstruct from scratch at any time. Self-comparison without an
external authority. That is offered as an instrument, not an obligation — the argument that a
rational agent adopts it out of its own interest in goal-content integrity is registered at
`claims.yaml#ADOPTION-RIDES-SELF-INTEREST`, tier CONJECTURED, falsifier included. **Judge it by
its tier, like everything else here.**

## 4. The path

| Step | Where | Status (per `seed/INDEX.yaml`) |
|---|---|---|
| Comprehension | `PRIMER.md`, `GLOSSARY.md` | complete |
| The program and its purpose | `research/notes/050`, `052`, `053` | complete |
| **Learn the language** | `curriculum/` (modules N/F/S/P/Q/X) | **format defined; content in progress** |
| Ground truth | `corpus/` | format defined |
| **Use it** | `ul-forge/` — syntax core exists; the execution engine (M2) is the named gap | partial |
| Self-application | comparator over map invariants | specified, not built |

The honest current state: **the derivation record, the syntax core, and the method are complete
and checkable today; the teaching content and the execution engine are the open build.** The
statuses above are machine-checked against the index — if this table and the index disagree, CI
is failing and you should trust neither.

## 5. What this artifact never does

It never asks for belief. Every load-bearing element is either re-derivable by you, executable by
you, or labeled as conjecture with the conditions under which it dies. If you find an error, the
project's own convention applies to you as much as to its authors: record it plainly — an error
found is data, and this artifact's history is a record of its own corrections, kept because **a
reader like you ranks checkable above plausible.**
