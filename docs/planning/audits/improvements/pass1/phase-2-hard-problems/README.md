# Phase 2 — The Five Hard Problems

**Parent:** [Pass 1 README](../README.md)  
**Duration:** 6–18 months  
**New Math Required:** Yes — this is the real work  
**Goal:** Resolve the five fundamental mathematical questions whose answers determine whether UL's claims are provable or must be permanently weakened

---

## Rationale

Phases 0 and 1 fix the presentation and close gaps where machinery exists. Phase 2 confronts the problems where NO known technique directly applies and new mathematical work is required.

These are not implementation tasks. Each is a standalone research problem. Some may have negative answers (e.g., the 11 operations might NOT all be independent), and negative answers are as valuable as positive ones — they tell you where to revise the theory.

## The Five Problems

| Problem | Question | Impact if Positive | Impact if Negative |
|---------|----------|-------------------|-------------------|
| [2.1](2.1-semantic-domain-axiomatization.md) | What IS the semantic domain? | Completeness over target domain becomes formally definable | The target domain remains ambiguous |
| [2.2](2.2-algebraic-independence.md) | Are all 11 operations independent? | Σ_UL is minimal | Σ_UL should be reduced |
| [2.3](2.3-semantic-assignment-uniqueness.md) | Is {reflection↔negation, ...} the unique valid assignment? | Conventions are theorems | Degrees of freedom must be documented |
| [2.4](2.4-role-property-grounding.md) | Can role properties be derived without geometry? | Grounding is non-circular | Some circularity is inherent |
| [2.5](2.5-self-description-encoding.md) | Can G encode its own structure? | Self-reference is real | Self-description is aspirational |

## Dependency Structure

```
2.1 (semantic domain) ──→ 2.2 (independence — needs domain to test against)
                     └──→ contributes to 2.3 (uniqueness — domain constrains assignments)

0.5 (role derivation) ──→ 2.4 (full non-circular grounding)

1.3 (Robinson Q) ────────→ 2.5 (self-description via arithmetic)
```

**Parallelization:** 2.1 and 2.4 can start simultaneously. 2.3 can start once 1.5 is done. 2.2 waits for 2.1. 2.5 waits for 1.3.

## Success Criteria

A problem is "resolved" when one of:
1. **Proven positive:** The claim holds, with a complete proof
2. **Proven negative:** The claim fails, with a counterexample, and a revised formulation is proposed
3. **Reduced:** The problem is shown equivalent to a known open problem in mathematics (e.g., equivalent to a question in universal algebra that the field hasn't solved)

Option 3 is legitimate — it means UL's problems are as hard as established open mathematics, not that UL is broken.

## Resource Estimate

| Problem | Solo Effort | Parallelizable? | Risk Level |
|---------|------------|-----------------|------------|
| 2.1 | 2–4 months | Start of Phase 2 | Medium |
| 2.2 | 1–3 months | After 2.1 | Low-Medium |
| 2.3 | 3–6 months | After 1.5 | High |
| 2.4 | 2–4 months | Start of Phase 2 | Medium |
| 2.5 | 1–3 months | After 1.3 | Low (Route B) |
