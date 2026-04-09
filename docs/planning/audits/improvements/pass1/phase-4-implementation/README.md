# Phase 4 — Implementation Sync

**Timeframe:** Ongoing (parallel with Phases 0–3)  
**Prerequisites:** Each task has specific phase dependencies

---

## Purpose

Phase 4 ensures that the Rust implementation (ul-forge), the GIR schema, and any tooling stay synchronized with the theoretical work. As the foundations are clarified, relabeled, and extended, the implementation must reflect current truth — not stale claims.

This phase is NOT about building new features. It's about keeping the codebase honest relative to the theory.

## Tasks

| Task | Description | Depends On | Estimated Effort |
|------|-------------|-----------|-----------------|
| 4.1 | Add tier labels to GIR schema | Phase 0 completion | 1–2 weeks |
| 4.2 | Add operation independence tests | Phase 2.2 results | 2–4 weeks |
| 4.3 | Implement probability extension | Phase 3.1 results | 1–3 months |
| 4.4 | Implement modal operators | Phase 3.2 results | 1–3 months |

## Design Principles

1. **Theory leads, implementation follows.** Never add code capabilities that aren't theoretically justified.
2. **Labels propagate.** If a theorem is CONDITIONAL in the theory, it's `#[conditional]` in the code.
3. **Tests test what's proven.** Unit tests assert only properties that have been rigorously established. Properties that are conjectured get `#[ignore]` with a note pointing to the relevant conjecture.
4. **Schema is the source of truth.** The GIR schema defines what the implementation can express. If the schema says 4 sorts and 11 operations, the code has 4 sorts and 11 operations — no more.

## Relationship to ul-forge

ul-forge is the Rust implementation. Key components:
- `crates/ul-core/` — Core types and operations
- `crates/ul-parser/` — Parsing UL expressions
- `schemas/` — GIR schema definitions
- `bindings/` — FFI bindings (Python, JS)
- `packages/` — Distribution packages

As theory cleans up, the implementation changes are typically:
- Adding/modifying metadata fields (tier labels, proof status)
- Adding/removing operations (if Phase 2.2 changes the minimal set)
- Adding tests that exercise proven properties
- Marking untested properties with reasons
