# Pass 1 — Testing Rigor Master Plan

> **Mission:** Transform the test suite from "wide but not deep" syntax verification into a rigorous algebraic, structural, and semantic proof that UL Forge correctly implements the Σ_UL formal specification — and that UL itself is viable infrastructure for autonomous AI agents.

---

## Context

### What exists
- 575/575 tests passing across 12 test files
- 414 tests added in the aggressive pass (roundtrip-stress, agent-scenarios, edge-adversarial)
- Tests prove: parser and deparser handle the full grammar surface, SVG round-trip preserves strings, performance is reasonable

### What's missing (from self-critique)
1. **No semantic assertion in any test** — every test checks string equality or `nodes.length`, never GIR structure
2. **No algebraic law verified** — involution, associativity, De Morgan, commutativity are all untested
3. **No sort enforcement tested** — wrong-sort inputs may silently produce garbage
4. **No multi-operation chain tested** — each operation tested in isolation
5. **No cross-SVG-boundary workflow tested** — agent pipeline viability unproven
6. **Deparser bugs documented by workaround, not characterized** — failure boundaries unknown

---

## Plan Overview

| # | Plan | Focus | Tests | Priority |
|---|------|-------|-------|----------|
| 01 | [Semantic Structural](../01-semantic-structural/PLAN.md) | GIR graph properties: edge types, containment tree, topology, binding, modals, force | ~165 | **P0** — foundation for all other tests |
| 02 | [Algebraic Depth](../02-algebraic-depth/PLAN.md) | Sort correctness on all 13 ops, multi-step chains, injectivity | ~130 | **P0** — proves algebra works |
| 03 | [Deparser Characterization](../03-deparser-characterization/PLAN.md) | Bug 1 (operator loss), Bug 2 (left-arrow), Bug 3 (modal overlap) — exact failure boundaries | ~92 | **P1** — blocks confident pipeline use |
| 04 | [Agent Pipeline](../04-agent-pipeline/PLAN.md) | Cross-SVG-boundary operations, multi-agent relay, hostile input robustness | ~90 | **P1** — proves real-world viability |
| 05 | [Sci-Fi Scenarios](../05-scifi-scenarios/PLAN.md) | 9 multi-agent scenarios with structural GIR verification, not string labels | ~112 | **P2** — demonstrates UL value for AI |
| 06 | [Sort Boundary](../06-sort-boundary/PLAN.md) | All 13 ops × wrong-sort inputs, arity violations, error message quality, GIR validation | ~146 | **P1** — proves type safety |
| 07 | [Invariant Laws](../07-invariant-laws/PLAN.md) | Involution, associativity, De Morgan, commutativity, distribution, identity, embedding coherence, modal laws | ~83 | **P0** — proves specification compliance |
| 08 | [Cross-Operation](../08-cross-operation/PLAN.md) | Pairwise and deep operation chains, modal+force+modifier orthogonality, cross-SVG chains | ~90 | **P1** — proves composability |
| | | **TOTAL** | **~908** | |

---

## Dependency Graph

```
                    ┌─────────────────────┐
                    │  GIR Isomorphism     │
                    │  Checker (infra)     │
                    └──────────┬──────────┘
                               │
              ┌────────────────┼────────────────┐
              │                │                │
     ┌────────▼───────┐ ┌─────▼──────┐ ┌───────▼───────┐
     │ 01 Semantic     │ │ 02 Algebra │ │ 07 Invariant  │
     │ Structural      │ │ Depth      │ │ Laws          │
     └────────┬───────┘ └─────┬──────┘ └───────┬───────┘
              │                │                │
              │          ┌─────▼──────┐         │
              │          │ 06 Sort    │         │
              │          │ Boundary   │         │
              │          └─────┬──────┘         │
              │                │                │
     ┌────────▼────────────────▼────────────────▼──────┐
     │              08 Cross-Operation                  │
     └──────────────────────┬──────────────────────────┘
                            │
              ┌─────────────┼──────────────┐
              │             │              │
     ┌────────▼───────┐ ┌──▼──────┐ ┌─────▼──────────┐
     │ 03 Deparser    │ │ 04 Agent│ │ 05 Sci-Fi      │
     │ Characterize   │ │ Pipeline│ │ Scenarios       │
     └────────────────┘ └─────────┘ └────────────────┘
```

**Phase 0 (foundation):** Build GIR isomorphism checker → 01, 02, 07 can proceed in parallel
**Phase 1 (enforcement):** 06 (sort boundary) depends on understanding from 02; 08 (cross-op) depends on 01+02+07
**Phase 2 (integration):** 03, 04, 05 depend on all of Phase 0+1 passing — they compose everything

---

## Critical Infrastructure: GIR Isomorphism Checker

Without this, plans 02, 07, and 08 cannot function. The checker must:

1. Compare two GIR objects for structural isomorphism (ignoring node IDs)
2. Verify node-type distributions match
3. Verify edge-type distributions match
4. Find a node-ID bijection that preserves all edges
5. Be tested itself (reflexivity, symmetry, known-different rejection)

This is the single highest-priority implementation task. If one thing gets built first, it's this.

---

## Implementation Order

### Wave 1 — Infrastructure
- [ ] GIR isomorphism checker (`girIsomorphic(gir1, gir2)`)
- [ ] GIR structure query helpers (`getEdgesByType()`, `getContainmentTree()`, `getNodesBySort()`)
- [ ] Sort extraction helper (`inferSort(girNode)`)
- [ ] Tests of the test infrastructure itself

### Wave 2 — P0 Plans (01, 02, 07)
- [ ] 01 Semantic Structural — ~165 tests
- [ ] 02 Algebraic Depth — ~130 tests
- [ ] 07 Invariant Laws — ~83 tests

### Wave 3 — P1 Plans (03, 06, 08)
- [ ] 06 Sort Boundary — ~146 tests
- [ ] 08 Cross-Operation — ~90 tests  
- [ ] 03 Deparser Characterization — ~92 tests

### Wave 4 — P2 Plans (04, 05)
- [ ] 04 Agent Pipeline — ~90 tests
- [ ] 05 Sci-Fi Scenarios — ~112 tests

---

## Success Criteria

After Pass 1:
- Every algebraic law from the formal spec has at least one test
- Every operation has at least one sort-mismatch rejection test
- Every GIR edge type has at least one structural verification test
- Every pairwise sort-compatible operation chain has been tested
- At least one 5-deep operation chain works end-to-end
- The deparser bugs have exact failure boundaries documented with tests
- At least 3 multi-agent scenarios cross SVG boundaries with structural GIR verification
- Total test count: **~908 new tests** (from current 575 → ~1,483 total)

## Test Count Summary

| Plan | New Tests | Cumulative |
|------|-----------|------------|
| Existing suite | — | 575 |
| 01 Semantic Structural | ~165 | 740 |
| 02 Algebraic Depth | ~130 | 870 |
| 03 Deparser Characterization | ~92 | 962 |
| 04 Agent Pipeline | ~90 | 1,052 |
| 05 Sci-Fi Scenarios | ~112 | 1,164 |
| 06 Sort Boundary | ~146 | 1,310 |
| 07 Invariant Laws | ~83 | 1,393 |
| 08 Cross-Operation | ~90 | 1,483 |

---

## What This Proves

If all ~1,483 tests pass:
- UL Forge correctly implements the Σ_UL formal specification (algebraic laws hold)
- The type system is operational, not decorative (sort errors are caught)
- Operations compose correctly to arbitrary depth (the algebra is real)
- GIR survives cross-agent SVG pipelines intact (infrastructure works)
- Multi-agent workflows produce correct structural results (UL is viable for AI)

If tests *fail* — that's equally valuable. Each failure is a precise, characterized gap between specification and implementation, documented with exact inputs and expected vs. actual outputs.
