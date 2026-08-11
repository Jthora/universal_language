# Pass 3 Execution Plan — Learning & App-Readiness

**Status:** 📋 PLANNED  
**Date:** 2026-04-08  
**Prerequisites:** Pass 1 complete (13 ops, 4 sorts), Pass 2 complete (modal/performative/pragmatic, D2 100%)  
**Goal:** Bring UL-Forge code and UL-Core docs to parity with the formal spec; create learning path  

---

## What Changed in Pass 1 & Pass 2

Before planning forward, here's what's already been done that the codebase hasn't absorbed:

### Pass 1 (Formal Foundations)
- Negation redefined as **boundary inversion** σ ∈ {⊕, ⊖} (not geometric reflection)
- Operation count established: **13 operations** (12 independent + conjoin derived)
- `bind(e, a) → a` and `modify_assertion(m, a) → a` added to formal spec
- Montague extensional subsumption proven
- D2 completeness: 37/50 → 50/50 via iterative resolution

### Pass 2 (Extensions — Zero New Primitives)
- **Modal semantics:** □ (necessity), ◇ (possibility), □→ (counterfactual)
  - 7 distinguished elements: `w_current`, `r_satisfies`, `r_alethic`, `r_K_agent`, `r_O`, `r_ability_agent`, `r_closeness`
  - World-enclosures with accessibility relations
  - Border styles: bold=□, dashed=◇, dashed-dot=□→
- **Performative extension:** φ ∈ {assert, query, direct, commit, express, declare}
  - 2 distinguished elements: `e_speaker`, `e_hearer`
  - Force composition rules FC1–FC5
  - Visual conventions: 6 distinct frame decorations
- **Pragmatic interface:** 5-layer architecture
  - 6 inference rules: SI-1–SI-3 (scalar implicature), CI-1–CI-3 (conventional inference)
  - Surface → intended mapping formalized

### What's NOT Absorbed Yet
The formal spec (foundations/) and writing system docs (ul-core/) have been updated. But:

| Layer | Spec Status | Code Status | Gap |
|-------|------------|-------------|-----|
| 13 operations | ✅ Specified | ❌ 11 implemented | `bind`, `modify_assertion` missing |
| Modal semantics | ✅ Specified | ❌ 0% implemented | World frames, distinguished elements, rendering |
| Performatives | ✅ Specified | ❌ 0% implemented | Force parameter, speaker/hearer, rendering |
| Pragmatics | ✅ Specified | ❌ 0% implemented | Inference engine, annotation layer |
| GIR schema | ✅ Specified informally | ❌ No extension fields | modal_context, force, binding, pragmatic |
| Writer's Companion examples | ✅ Core 18 examples | ❌ No modal/perf/pragmatic examples | 3+ new examples needed |
| Learning path | ❌ No tutorial | ❌ No tutorial | Everything needed |
| Published packages | N/A | ❌ Nothing published | npm, PyPI, VS Code, Docker |

---

## Execution Phases

### Phase 0: Complete the Algebra in Code
**Goal:** UL-Forge implements 13/13 operations  
**Scope:** `ul-forge/crates/ul-core/src/` (composer, types, validator, renderer, parser) + cascade to all crates  
**Effort:** ~1,000 lines of Rust (bind ~400, modify_assertion ~200, cascade+tests ~400)  
**Blocks:** Phase 1, Phase 2  
**Checklist:** 34 items (P0-1 through P0-34) — see [CHECKLIST.md](CHECKLIST.md#phase-0-complete-the-algebra-in-code-1313-operations)

**Summary:** Implement `bind(e, a) → a` and `modify_assertion(m, a) → a` in composer.rs, add supporting types (VariableSlot, Binds edge, AssertionModifier), update validator (scope/dangling checks), renderer (binding arrows, frame decorations), parser (○_x /●_x syntax), and cascade to all 7 downstream crates.

**Pre-existing bug to fix:** GIR schema `EdgeType` enum has 5 values but Rust code has 6 (`references` missing from schema). Fix during P0-26.

**Verification:** `cargo test --workspace` passes, all 13 operations exercised.

---

### Phase 1: Extensions in Code
**Goal:** Modal, performative, pragmatic support in UL-Forge  
**Scope:** All crates + GIR schema + renderer + web editor  
**Effort:** ~2,000 lines of Rust + TypeScript  
**Blocks:** Phase 3  
**Checklist:** 57 items (P1a-1 through P1-C11) — see [CHECKLIST.md](CHECKLIST.md#phase-1a-modal-extension-in-code)

**Phase 1a — Modal (23 items):** Distinguished element registry (`distinguished.rs`), world-frame types, accessibility edges, □/◇/□→ as composed patterns, modal rendering (border styles), parser tokens, validation.

**Phase 1b — Performative (11 items):** `PerformativeForce` enum, force field on assertions, FC1–FC5 composition rules, 6 border decorations, parser tokens.

**Phase 1c — Pragmatic (12 items):** `pragmatic.rs` inference engine, SI-1–SI-3 + CI-1–CI-3 rules, annotation layer, surface→intended mapping, ⟹ rendering.

**Phase 1 Cascade (11 items):** Update ul-wasm, ul-game, ul-api, ul-mcp, ul-cli, Python bindings, web editor, VS Code extension.

**Verification:** Full test suite passes, web editor renders modal/performative glyphs.

---

### Phase 2: UL-Core Documentation Updates
**Goal:** Writing system docs cover all extensions with worked examples  
**Scope:** `ul-core/` markdown files  
**Effort:** ~1,500 lines of documentation  
**Blocks:** Phase 3 (learning materials reference these)  
**Checklist:** 25 items (P2-1 through P2-25) — see [CHECKLIST.md](CHECKLIST.md#phase-2-ul-core-documentation-updates)

> **Effort reduction note:** Pass 2 already added Writer's Companion Examples 11-19 (bind, modify_assertion, 3 modal, 2 performative, 1 pragmatic). Only 3 new examples needed (standalone `invert`, `abstract`, `compose`). Original estimate of ~3,000 lines was based on stale audit data.

**Writer's Companion** (3 new examples): Explicit demonstrations of `invert` (passive voice), `abstract` (modifier extraction), `compose` (relation chaining). ~400 lines.

**Symbology** (3 updates): Clarify "21 markers" count, add modal symbols (border styles), add force symbols. ~200 lines.

**Syntax Dictionary** (2 new sections): §3.15 performative syntax, §3.16 pragmatic notation. ~500 lines.

**Thesaurus** (3 new families): Modal meaning families, force meaning families, pragmatic inference pathways. ~400 lines.

**Other**: Lexicon §9 cross-ref, UQPL divergence sync, NAVIGATION learning sequence, verify visual map + formal grammar. ~100 lines.

**Verification:** All cross-references accurate, no stale operation counts, all 13 ops + 3 extensions exemplified.

---

### Phase 3: Learning Infrastructure
**Goal:** Someone can learn UL from zero with a guided path  
**Scope:** New documents + web editor features + gallery  
**Effort:** ~3,000 lines of new content + TypeScript development  
**Blocks:** Nothing (final deliverable layer)  
**Checklist:** 13 items (P3-1 through P3-13) — see [CHECKLIST.md](CHECKLIST.md#phase-3-learning-infrastructure)

**No-code-dependency items** (can start immediately): "Learn UL in 15 Minutes" quickstart (~800 lines), Day 1-9 curriculum (~400 lines), practice exercises (15 with solutions), CONTRIBUTING.md update.

**Code-dependent items** (require Phase 0+): Visual glyph gallery (42 SVGs via `ul-forge render`), developer quickstart (requires published packages), Jupyter notebook (requires Python wheel), game challenge levels.

---

### Phase 4: Packaging & Deployment
**Goal:** UL-Forge is installable and accessible  
**Scope:** CI/CD, package registries, hosting  
**Effort:** DevOps + configuration  
**Checklist:** 9 items (P4-1 through P4-9) — see [CHECKLIST.md](CHECKLIST.md#phase-4-packaging--deployment)

**Targets:** Deploy web editor (GitHub Pages/Vercel), publish @ul-forge/core to npm, publish Python wheel to PyPI, publish VS Code extension, push Docker image, create CI/CD pipeline, create OpenAPI + transceiver schemas.

---

## Prioritization & Dependencies

```
Phase 0 (13/13 ops) ──────────────────────┐
  ↓                                        │
Phase 1a (Modal) ───┐                      │
Phase 1b (Perf) ────┤──→ Phase 1 Cascade ──┤
Phase 1c (Pragmatic)┘                      │
  ↓                                        │
Phase 2 (UL-Core docs) ←──────────────────┘
  ↓
Phase 3 (Learning) ←── Phase 2
  ↓
Phase 4 (Packaging) ←── Phase 0 + Phase 1
```

**Critical path:** P0 → P1 → P2 → P3  
**Parallel track:** P4 can start after P0 (doesn't need P1/P2)

### Fallback Plan

If Phase 1 (extensions) proves harder than expected or is blocked:

**Ship with Phase 0 only ("13/13 release"):**
- Delivers bind + modify_assertion in all crates — the last two core operations
- Updates GIR schema to match
- All 13 operations work end-to-end (parse → compose → validate → render)
- Phase 2 docs and Phase 3 learning materials can proceed without modal/perf/pragmatic code support
- Writer's Companion examples 14-19 (modal/perf/pragmatic) describe the formal notation — they don't require Forge to render them

**What this defers:**
- Modal/performative/pragmatic rendering in web editor and SVG output
- Distinguished element registry in code
- Pragmatic inference engine
- Force-aware composition rules in code

**This is a valid release** because the formal spec is complete and the writing system docs are complete. The code just can't render the extensions yet.

---

## Effort Summary

| Phase | Items | Character | Estimated Lines | Notes |
|-------|-------|-----------|----------------|-------|
| Phase 0 | 34 | Rust development | ~1,000 | bind alone ~400 lines (types+composer+validator+renderer+parser) |
| Phase 1 | 57 | Rust + TypeScript + schema | ~2,000 | Distinguished registry + 3 extensions + cascade |
| Phase 2 | 25 | Documentation | ~1,500 | Reduced: Pass 2 already added 9 Writer's Companion examples |
| Phase 3 | 13 | Tutorials + content + web features | ~3,000 | Mostly new content creation |
| Phase 4 | 9 | DevOps + config | ~500 | CI/CD + package publishing |
| **Total** | **138** | Mixed | **~8,000** |

> **Note on Phase 2 reduction:** The original estimate of ~3,000 lines assumed Writer's Companion had no modal/performative/pragmatic examples. In fact, Pass 2 already added Examples 11-19 covering bind, modify_assertion, modal (◇/□/□→), performative (commit/declare), and pragmatic (CI-1). Only 3 new examples are needed (standalone `invert`, `abstract`, `compose`).

---

## Success Criteria

Pass 3 is complete when:

1. **`cargo test --workspace`** passes with all 13 operations + modal/performative/pragmatic support
2. **Writer's Companion** has ≥21 worked examples covering all operations + extensions
3. **"Learn UL in 15 Minutes"** exists and is navigable from README
4. **Web editor** renders modal/performative glyphs and is deployed publicly
5. **`npm install @ul-forge/core`** works (beta tag acceptable)
6. **No stale references** — zero "11 operations" anywhere in repo
7. **Day 1–9 curriculum** exists and is linked from NAVIGATION.md
8. **GIR schema** documents all extension fields
