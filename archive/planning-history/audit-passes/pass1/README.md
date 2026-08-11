# Pass 1 — Foundations Restructuring Plan

**Date:** April 6, 2026  
**Scope:** Complete restructuring of the Universal Language mathematical foundations  
**Goal:** Transform the current proof corpus from a mixed-rigor oversold presentation into an honest, tier-labeled, dependency-tracked mathematical framework

---

## The Problem

The UL project has genuine mathematical results, but they are buried in a presentation that:

1. **Oversells proof status** — coverage arguments labeled PROVEN, conventions labeled THEOREMS
2. **Hides dependencies** — proofs depend on definitions crafted to succeed, without flagging this
3. **Creates unfalsifiability** — paradigm.md blocks empirical testing while proofs remain incomplete
4. **Conflates theory and experiments** — the artifact effect study is framed as mathematical proof

## The Inventory

| Category | Count | Description |
|----------|-------|-------------|
| **Genuinely proven** | ~62 theorems | Complete, rigorous proofs with no gaps |
| **Conditional theorems** | ~15 | Proven IF stated assumptions hold |
| **Asserted claims** | ~64 | Presented as results but lacking formal proof |
| **Open problems** | ~42 | Identified in gap-analysis.md and expeditions |
| **Total proof corpus** | ~18,720 lines | Across foundations/, frontier/ |

## Phase Structure

| Phase | Name | Duration | New Math? | Documents |
|-------|------|----------|-----------|-----------|
| [Phase 0](phase-0-honesty-pass/) | Honesty Pass | 2–3 weeks | No | 9 task specs |
| [Phase 1](phase-1-structural-gaps/) | Structural Gap Closure | 2–3 months | Minor | 5 task specs |
| [Phase 2](phase-2-hard-problems/) | Five Hard Problems | 6–18 months | Yes | 5 problem specs |
| [Phase 3](phase-3-extended-theory/) | Extended Theory | 12–36 months | Yes | 6 research specs |
| [Phase 4](phase-4-implementation/) | Implementation Sync | Ongoing | Code only | 4 task specs |

## Dependency Graph

See [dependencies.md](dependencies.md) for the full inter-task dependency graph.

## What Success Looks Like

After Pass 1 completion:

1. Every theorem in foundations/ carries an explicit tier label (PROVEN / CONDITIONAL / OPEN)
2. Every semantic convention is flagged as FORCED / CONSTRAINED / CONVENTIONAL
3. The embedding theorem is renamed and repositioned as an adequacy result
4. The unique grounding theorem is separated from its definitional assumptions
5. paradigm.md contains explicit falsification criteria
6. The experiments are reframed as artifact studies, not proof of UL
7. AGENTS.md, index.json, llms.txt reflect honest status

After all phases:

1. The 11 operations are proven algebraically independent (or reduced to a minimal set)
2. A formal semantic domain axiomatization exists — defining the class of compositional meaning systems that are Σ_UL-algebras
3. The semantic assignments are characterized (unique or documented degrees of freedom)
4. Self-description is proven via Gödel encoding or demoted to a permanent conjecture
5. Probability, modality, and dynamics are classified: either already projections of the core (derivable from the 11 operations) or characterized as requiring explicit extensions beyond the minimal skeleton
6. The initiality/terminality result is the headline: UL is not "one framework to rule them all" — it is the minimal algebraic skeleton that all stable meaning-bearing frameworks instantiate as specializations. Like ℤ mapping into every group, or ℕ seeding all of arithmetic.

## File Index

```
pass1/
├── README.md                          ← You are here
├── dependencies.md                    ← Full inter-task dependency graph
├── proof-inventory.md                 ← Every theorem, its status, its location
├── phase-0-honesty-pass/
│   ├── README.md                      ← Phase 0 overview
│   ├── 0.1-tier-label-formal-foundations.md
│   ├── 0.2-fix-gap-a-injectivity.md
│   ├── 0.3-completeness-roadmap.md
│   ├── 0.4-separate-role-properties.md
│   ├── 0.5-independent-role-derivation.md
│   ├── 0.6-remove-unfalsifiability.md
│   ├── 0.7-tier-label-derivation.md
│   ├── 0.8-update-metadata.md
│   └── 0.9-reframe-experiments.md
├── phase-1-structural-gaps/
│   ├── README.md                      ← Phase 1 overview
│   ├── 1.1-close-gap-a-permanently.md
│   ├── 1.2-reduce-rationals-to-sigma.md
│   ├── 1.3-robinson-q-verification.md
│   ├── 1.4-adjoint-functors-f3-f4.md
│   └── 1.5-weak-terminality-specialness.md
├── phase-2-hard-problems/
│   ├── README.md                      ← Phase 2 overview
│   ├── 2.1-semantic-domain-axiomatization.md
│   ├── 2.2-algebraic-independence.md
│   ├── 2.3-semantic-assignment-uniqueness.md
│   ├── 2.4-role-property-grounding.md
│   └── 2.5-self-description-encoding.md
├── phase-3-extended-theory/
│   ├── README.md                      ← Phase 3 overview
│   ├── 3.1-probability-uncertainty.md
│   ├── 3.2-modal-logic.md
│   ├── 3.3-riemannian-context-space.md
│   ├── 3.4-instantons-discontinuity.md
│   ├── 3.5-pragmatics.md
│   └── 3.6-meaning-dynamics.md
└── phase-4-implementation/
    ├── README.md                      ← Phase 4 overview
    ├── 4.1-gir-schema-tier-labels.md
    ├── 4.2-operation-independence-tests.md
    ├── 4.3-probability-extension.md
    └── 4.4-modal-operators.md
```
