# Pass 3 — Learning & App-Readiness

**Status:** � PLANNED — Audit complete, execution plan ready  
**Prerequisite:** Pass 2 complete (Σ_UL⁺: 4 sorts, 13 operations, D2 at 100%)  
**Goal:** Make UL learnable, buildable, and deployable — bridge theory→practice gap  
**Scope:** UL Forge implementation completeness, UL Core teaching readiness, packaging, tutorials, demos  
**Plan:** [PLAN.md](PLAN.md) — 84 tasks across 5 phases  
**Checklist:** [CHECKLIST.md](CHECKLIST.md) — 138 items with 5 phase gates

---

## The Problem

After Pass 1 (formal foundations, 74% D2) and Pass 2 (extensions to 100% D2), UL is **theoretically mature**: 13 operations, modality, performatives, pragmatics — all formally specified with zero new primitives. But the implementation and learning infrastructure lags significantly behind the theory:

- **UL Forge** implements all 13 operations + modal/performative/pragmatic extensions across 8 crates (337 tests passing)
- **No packages published** — `npm install @ul-forge/core` returns 404
- **Learning path created** — quickstart, curriculum, exercises, developer quickstart all complete
- **Web editor extended** — modal templates, force picker, pragmatic inference panel added
- **Theory ↔ implementation gap closed** — formal spec and code both at 13 operations + 3 extensions

**In short:** UL is ready for researchers. It is not yet ready for learners or app developers.

---

## Audit Summary

Full audit conducted 2026-04-08 across 5 areas. See `audit/` subdirectory for detailed findings.

| Area | Grade | Key Finding |
|------|-------|-------------|
| [UL Forge Implementation](audit/ul-forge/) | **C** | 11/13 ops, 0/3 extensions, unpublished packages |
| [UL Core Writing System](audit/ul-core/) | **A-** | Excellent specs, minor gaps in advanced examples |
| [Infrastructure & Distribution](audit/infrastructure/) | **C+** | Architecture solid, nothing published or deployable |
| [Learning & Teaching](audit/learning/) | **C+** | Good reference docs, no tutorial path or demos |
| [Gap Analysis](audit/gaps/) | — | 47 specific gaps identified, prioritized |

**Overall: C+ (Architecturally sound, operationally incomplete)**

---

## Pass 3 Phases (Proposed)

### Phase 0: Forge Completeness
Bring UL Forge to parity with the formal specification.
- Implement `bind` and `modify_assertion` operations
- Add modal context support (world frames, accessibility relations)
- Add performative force parameter (φ)
- Add pragmatic inference rules
- Update GIR schema for extensions
- Update all tests

### Phase 1: Packaging & Publishing
Make UL Forge installable and usable.
- Publish `@ul-forge/core` to npm
- Publish Python wheels to PyPI
- Build and publish web components (`<ul-symbol>`, `<ul-composer>`)
- Create Docker images
- Rebuild dist/ outputs (fix stale "11 operations" references)

### Phase 2: Learning Infrastructure
Create the tutorial path and learning experience.
- "Learn UL in 15 Minutes" quick-start guide
- 3 new Writer's Companion examples (modal, performative, pragmatic)
- Prescribed learning sequence (Day 1–9 curriculum)
- Visual glyph gallery (42 SVGs from lexicon)
- Interactive web demo
- Map ul-forge-v1 spec ↔ actual implementation

### Phase 3: Demo & Validation
Prove it works.
- Deploy playable ProtoFusionGirl demo level
- Run pre-registered causal efficacy experiment
- Create code tutorials ("Build Your First UL Parser")
- Jupyter notebook walkthrough

---

## Entry Points

### Planning Documents
| Document | Purpose |
|----------|---------|
| [PLAN.md](PLAN.md) | **Execution plan** — 84 tasks, 5 phases, dependency graph |
| [CHECKLIST.md](CHECKLIST.md) | **Checklist** — 138 items with phase gates |

### Audit Documents (completed 2026-04-08)
| Document | Purpose |
|----------|---------|
| [audit/ul-forge/forge-audit.md](audit/ul-forge/forge-audit.md) | Complete UL Forge implementation inventory |
| [audit/ul-core/core-audit.md](audit/ul-core/core-audit.md) | UL Core writing system readiness |
| [audit/infrastructure/infra-audit.md](audit/infrastructure/infra-audit.md) | Distribution, packaging, deployment state |
| [audit/learning/learning-audit.md](audit/learning/learning-audit.md) | Teaching materials and learning path assessment |
| [audit/gaps/gap-inventory.md](audit/gaps/gap-inventory.md) | Complete prioritized gap list (47 gaps) |

### Update Specifications (the work to be done)
| Document | Purpose |
|----------|---------|
| [audit/ul-forge/forge-update-spec.md](audit/ul-forge/forge-update-spec.md) | **UL-Forge code changes** — bind, modify_assertion, modal, performative, pragmatic |
| [audit/ul-core/core-update-spec.md](audit/ul-core/core-update-spec.md) | **UL-Core doc changes** — new examples, symbology, syntax, thesaurus |
| [audit/learning/learning-update-spec.md](audit/learning/learning-update-spec.md) | **Learning infrastructure** — tutorial, curriculum, exercises, gallery |

---

## Relationship to Prior Passes

| Pass | Focus | D2 Score | Outcome |
|------|-------|----------|---------|
| Pass 1 | Formal foundations | 37/50 → 37/50 (74%) | 13 operations, 4 sorts, embedding theorem |
| Pass 1.2–1.4 | Extensions + D2 | 37/50 → 37/50 (74%) | bind, modify_assertion added to spec |
| Pass 2 | Modality, performatives, pragmatics | 37/50 → 50/50 (100%) | Zero new primitives; 7+2 distinguished elements |
| **Pass 3** | **Learning & app-readiness** | **N/A** | **Bridge theory → practice** |
