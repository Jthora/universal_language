# Historical Process Record — Superseded 2026-08-01

> **This directory (pass1 through pass3, and `pass1-summary.md`) is a historical record, not the
> current status of the project.**

These passes document the iterative process by which the project's formal claims (Σ_UL: 4 sorts,
13 operations, 23 theorems, the "Unique Grounding Theorem," "100% D2 completeness") were built up
between March and April 2026. They are genuinely useful as an honest record of *how* that process
worked — including finding **F7** (`pass1-1/findings/structural-gaps.md`), which correctly
identified the 4-sort/5-primitive tension years before it was actually resolved.

A direct audit in August 2026 (`docs/planning/audits/wiki-comparison-2026-08.md`) found:
- The "D2 completeness" score climbing from 32% to 100% across nine sequential passes, each
  triggered by a specific failing case and resolved by patching the theory — a pattern of fitting
  the theory to its own test rather than independent validation.
- The "Unique Grounding Theorem" these passes helped refine is, on direct inspection, close to
  circular: it defines semantic primitives with role properties constructed to mirror the geometric
  primitives already chosen, then proves a bijection between the two hand-matched lists.
- Finding F7 itself (the 4-sort vs. 5-primitive tension, logged here in April 2026) was never
  actually fixed in the live documentation, despite being marked "clear resolution — needs
  documentation additions."

**What replaced this process:** `docs/planning/emergence-investigation/PLAN.md` — a six-phase
investigation with pre-stated falsification criteria for each phase, explicitly designed to avoid
the patch-until-it-passes pattern documented in this directory's own resolution logs.

Read these passes for historical context on how the project's claims evolved, not as a source of
current, established results.
