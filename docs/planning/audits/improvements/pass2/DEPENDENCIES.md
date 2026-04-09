# Pass 2 — Cross-Phase Dependencies

---

## Dependency Graph

```
                PHASE 0: Consolidation
                (stale refs, open proofs)
                         │
                         │ GATES all other phases
                         │ (don't extend on inconsistent base)
                         ▼
                PHASE 1: Modality
                (5 ❌ → ✅: Cat. 4)
                ┌────────┼────────────────┐
                │        │                │
                │  partial│         partial│
                │        │                │
                ▼        ▼                ▼
        PHASE 2: Performatives    (used by Phase 3 RSA)
        (5 ❌ → ✅: Cat. 9)
                │
                │ required
                ▼
        PHASE 3: Pragmatics
        (3 ❌ → ✅/⚠️: Cat. 6)
```

---

## Detailed Dependencies

### Phase 0 → Phase 1

| Dependency | What Phase 1 Needs from Phase 0 | Hard/Soft |
|-----------|--------------------------------|-----------|
| Stale refs clean | All docs say "13 operations" so Phase 1 docs don't inherit errors | **Hard** |
| formal-operations.md complete | Phase 1 may add defined patterns; base ops must be complete | **Hard** |
| Recursion depth proof | Phase 1 adds modal expressions with embed/bind nesting; depth proof prevents worry | **Soft** |

### Phase 0 → Phase 2

| Dependency | What Phase 2 Needs from Phase 0 | Hard/Soft |
|-----------|--------------------------------|-----------|
| Same as Phase 1 | Consistency baseline | **Hard** |
| CRITIQUE.md cleanup | Phase 2 may modify Assertion tuple; CRITIQUE.md should be accurate first | **Soft** |

### Phase 1 → Phase 2

| Dependency | What Phase 2 Needs from Phase 1 | Hard/Soft |
|-----------|--------------------------------|-----------|
| Epistemic modality | Performatives involve speaker beliefs: "I promise" presupposes speaker CAN do what's promised (ability modal) | **Soft** — Phase 2 can proceed with placeholders |
| Deontic modality | Promises create obligations (deontic); some performatives (commands) involve deontic necessity | **Soft** — the obligation result of performatives can reference future Phase 1 work |
| Modal-force interaction | "You should promise" — modality wrapping force. Phase 2 design-questions.md P4 depends on Phase 1's operator types | **Medium** — Phase 2 can design force independently, then verify interaction |
| Assertion tuple | Phase 1 may or may not extend the assertion tuple (if worlds need it). Phase 2 extends the tuple with φ. Both extensions must be compatible | **Hard** — Phase 1 must settle the assertion tuple before Phase 2 modifies it |

### Phase 1 → Phase 3

| Dependency | What Phase 3 Needs from Phase 1 | Hard/Soft |
|-----------|--------------------------------|-----------|
| Epistemic modality | Sarcasm decomposition requires `believes(speaker, ¬a)` — epistemic modal operator | **Hard** — cases 6.1, 6.2 cannot be represented without it |
| Possibility operator | Case 6.4 "Can you X?" requires ability/possibility modal ◇ | **Hard** — case 6.4 surface form uses modality |
| Scalar implicature (SI-3) | "Might → not must" rule requires both ◇ and □ | **Hard** |

### Phase 2 → Phase 3

| Dependency | What Phase 3 Needs from Phase 2 | Hard/Soft |
|-----------|--------------------------------|-----------|
| Illocutionary force | All 3 pragmatic cases involve force: sarcasm = assertion mismatch, indirection = force mismatch | **Hard** — φ_query, φ_direct needed for case 6.4 |
| Force/content separation | The pragmatic interface operates on the force/content split | **Hard** |
| embed(force(a)) | "The question whether..." — talking ABOUT speech acts requires embedded force | **Soft** — nice to have for interface specification |

---

## Critical Path

The critical path through Pass 2:

```
Phase 0 stale-ref sweep (FAST)
    → Phase 0 open proofs OP-1, OP-4 (FAST)
        → Phase 0 OP-2 extensional injectivity (MEDIUM)
            → Phase 1 design questions M1, M5 (settle assertion tuple)
                → Phase 1 Kripke realization + modal operations
                    → Phase 2 assertion tuple extension (φ)
                        → Phase 2 force algebra
                            → Phase 3 pragmatic interface
                                → Phase 3 scope boundary declaration
                                    → D2 re-scoring
```

**Estimated critical path length:** 10 sequential work items (parallelization limited — each feeds the next)

---

## Parallelizable Work

| Can run in parallel with... | Work item |
|----------------------------|-----------|
| Phase 0 proofs | Phase 1 design-questions brainstorming (doesn't need clean base) |
| Phase 1 Kripke realization | Phase 2 design-questions P1–P3 (force-specific, not modality-dependent) |
| Phase 1 writing system updates | Phase 2 geometric realization of force (visual design, independent) |
| Phase 2 force algebra | Phase 3 scalar implicature rules (independent of force) |
| Phase 2 D2 rework | Phase 3 principled boundary declaration (independent) |

---

## Risk: Assertion Tuple Conflict

The biggest dependency risk is the **assertion tuple**. Currently (F, C, σ). Both Phase 1 and Phase 2 may want to extend it:

| Phase | Extension | Tuple becomes |
|-------|-----------|---------------|
| Phase 1 | Modal world parameter? | (F, C, σ, w?) |
| Phase 2 | Illocutionary force | (F, C, σ, φ) |
| Both | Combined | (F, C, σ, φ, w?) |

**Mitigation:** Phase 1's recommended approach (modality via quantification over worlds, not assertion extension) avoids adding w to the tuple. If Phase 1 stays with defined operators and distinguished relations, the tuple remains (F, C, σ) and Phase 2 can safely extend to (F, C, σ, φ).

**If Phase 1 DOES need to extend the tuple**, it must be settled before Phase 2 begins its force algebra work.
