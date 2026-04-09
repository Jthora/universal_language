# Tier C — Writing System Questions (Theory ↔ Practice Gap)

**Date:** April 7, 2026  
**Scope:** Three questions targeting the gap between the algebraic specification (Σ_UL) and its visual realization (the writing system)  
**Nature:** These are design/documentation problems — they don't challenge the algebra but expose where the writing system fails to realize it

---

## Overview

| Question | Title | Targets |
|----------|-------|---------|
| [C1](C1-machine-readability.md) | Is UL machine-readable? | All writing system docs — no formal grammar exists |
| [C2](C2-orphaned-operations.md) | Which operations have no visual realization? | `abstract`, `quantify`, `compose` |
| [C3](C3-missing-critique.md) | Where is CRITIQUE.md? | `ul-core/README.md` references it, file doesn't exist |

## Why These Matter

The algebra (Σ_UL) and the writing system are supposed to be two faces of the same coin — the algebra specifies the *structure* of meaning; the writing system *instantiates* that structure visually. If operations exist in the algebra but not the writing system (or vice versa), then either:
- The algebra is overcomplete (some operations are unnecessary for visual meaning), or
- The writing system is underdeveloped (some algebraic operations haven't been designed yet)

Both possibilities need documentation.

## Dependencies

```
C1 (Machine-readability) ─── independent ──→ can proceed immediately
C2 (Orphaned operations) ─── depends on B4 ──→ some "orphaned" ops may turn out to be redundant
C3 (Missing CRITIQUE.md)  ─── independent ──→ can proceed immediately, uses findings from this audit
```
