# Findings — Theory-Derived UL Primer Experiment

**Date:** 2025-07-15 (experiment run) / 2026-03-13 (findings organized)  
**Analyst:** GitHub Copilot (Claude Opus 4.6)  
**Status:** Post-hoc analysis of 56 completed trials

---

## Contents

| File | Description |
|------|-------------|
| [00-executive-summary.md](00-executive-summary.md) | Top-level results and verdict |
| [01-score-tables.md](01-score-tables.md) | Full M1–M5 and TES score tables for all 56 trials |
| [02-prediction-evaluation.md](02-prediction-evaluation.md) | Pass/fail evaluation of all 18 pre-registered predictions |
| [03-diagnosis.md](03-diagnosis.md) | Why the experiment produced a null result — 5 identified causes |
| [04-positive-findings.md](04-positive-findings.md) | What the results actually show (despite the null on primary hypotheses) |
| [05-recommendations.md](05-recommendations.md) | Concrete recommendations for future experimental design |
| [06-condition-notes.md](06-condition-notes.md) | Per-condition qualitative observations across all 56 outputs |

---

## Quick Verdict

**Primary hypotheses: null result.** 10 of 18 predictions failed, 3 passed trivially, 3 partial, 2 supported.

**Root cause:** Claude's baseline reasoning is already near-ceiling on these tasks. No ~490-token context injection — theory-derived primer, reference artifact, domain-matched control, or naive prompt — produces a measurable improvement over the no-context baseline, because the baseline is already excellent.

**Theory status:** UL theory is not falsified. The experiment is not powerful enough to test it on a frontier-class model.
