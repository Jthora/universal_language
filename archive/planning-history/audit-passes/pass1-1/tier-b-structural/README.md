# Tier B — Structural Questions (Questioning the Architecture)

**Date:** April 7, 2026  
**Scope:** Four questions targeting the structural integrity of Σ_UL's sort system, operations, and sort transitions  
**Nature:** These are solvable mathematical problems — each has a definite answer (proof, counterexample, or impossibility result)

---

## Overview

| Question | Title | Targets |
|----------|-------|---------|
| [B1](B1-negation-problem.md) | What IS negation in UL? | F1 (critical error), `formal-operations.md` §1.4 |
| [B2](B2-modifier-as-morphism.md) | Is Modifier a sort or a morphism space? | F3 (carrier mismatch), `formal-operations.md` §0.1 |
| [B3](B3-entity-to-relation-gap.md) | Why no e → r operation? | F4 (missing sort transition) |
| [B4](B4-operation-independence.md) | What is the true minimal operation set? | F2 (De Morgan), `pass1/phase-2-hard-problems/2.2-algebraic-independence.md` |

## Why These Are Structural

These questions target the *internal consistency* of Σ_UL. Unlike Tier A (which questions axioms), Tier B accepts the axioms and asks whether the system built on them is mathematically sound.

- B1 asks: does `negate` do what it claims? (No — F1 shows it implements converse)
- B2 asks: is the sort classification correct? (Modifier might be a morphism, not a sort)
- B3 asks: is the sort-transition graph complete? (No — e→r is missing)
- B4 asks: is the operation set minimal? (No — at least one redundancy exists)

## Dependencies

```
B1 (Negation) ─── must resolve FIRST ───→ B4 (Independence requires correct negate)
B2 (Modifier) ─── independent ───────────→ can proceed in parallel
B3 (e→r gap)  ─── independent ───────────→ can proceed in parallel
B4 (Independence) ← depends on B1 ───────→ needs corrected operations
```

## Relationship to Pass 1 Problems

| Tier B Question | Pass 1 Problem | What's New |
|---|---|---|
| B1 (Negation) | 2.3 (Semantic Assignment) | Pass 1 asked "which reflection?"; Pass 1.1 shows "no reflection works" |
| B2 (Modifier) | Not flagged | New question from carrier set analysis |
| B3 (e→r gap) | Not flagged | New finding from sort-transition analysis |
| B4 (Independence) | 2.2 (Algebraic Independence) | Pass 1 suspected redundancy; Pass 1.1 confirms De Morgan case |
