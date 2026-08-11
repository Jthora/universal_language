# Pass 3 Checklist — Learning & App-Readiness

**Status:** 📋 NOT STARTED  
**Date Created:** 2026-04-08  
**Reference:** [PLAN.md](PLAN.md) for rationale and dependency graph

---

## Phase 0: Complete the Algebra in Code (13/13 Operations)

### P0: `bind(e, a) → a` Implementation
- [ ] P0-1: Add `VariableSlot` / slot entity type to `types/node.rs`
- [ ] P0-2: Add `Binds` edge type to `types/edge.rs`
- [ ] P0-3: Add `variable_id` and `binding_scope` fields to `types/gir.rs`
- [ ] P0-4: Implement `pub fn bind(ctx, entity, assertion) → Gir` in `composer.rs`
- [ ] P0-5: Add bind pattern detection to `detect_operations()` in `composer.rs`
- [ ] P0-6: Add binding scope validation to `validator.rs`
- [ ] P0-7: Add dangling variable detection to `validator.rs`
- [ ] P0-8: Add binding arrow rendering to `renderer/svg.rs`
- [ ] P0-9: Add scope bracket rendering to `renderer/layout.rs`
- [ ] P0-10: Add ○_x / ●_x syntax to `parser/ul_script.pest`
- [ ] P0-11: Update `parser/grammar.rs` for binding parse
- [ ] P0-12: Update `parser/transform.rs` for binding AST→GIR
- [ ] P0-13: Update `parser/deparser.rs` for GIR→UL-Script (bind)
- [ ] P0-14: Write unit tests for bind in `tests/`
- [ ] P0-15: Write integration tests for bind (parse→compose→validate→render)

### P0: `modify_assertion(m, a) → a` Implementation
- [ ] P0-16: Add `AssertionModifier` type (evidential/emphatic/hedged) to `types/node.rs`
- [ ] P0-17: Implement `pub fn modify_assertion(ctx, modifier, assertion) → Gir` in `composer.rs`
- [ ] P0-18: Add modify_assertion pattern detection to `detect_operations()`
- [ ] P0-19: Add assertion modifier validation to `validator.rs`
- [ ] P0-20: Add frame decoration rendering (dotted/double/wavy borders) to `renderer/svg.rs`
- [ ] P0-21: Add assertion modifier syntax to `parser/ul_script.pest`
- [ ] P0-22: Update parser transform for modify_assertion
- [ ] P0-23: Update deparser for modify_assertion
- [ ] P0-24: Write unit tests for modify_assertion
- [ ] P0-25: Write integration tests for modify_assertion

### P0: Schema & Cascade
- [ ] P0-26: Update `schemas/gir.schema.json` — add `variable_id`, `binding_scope`, `assertion_modifier` fields
- [ ] P0-26b: Fix pre-existing bug: add missing `references` value to GIR schema `EdgeType` enum (Rust has 6 variants, schema has 5)
- [ ] P0-27: Update `crates/ul-wasm/src/lib.rs` — expose bind + modify_assertion
- [ ] P0-28: Update `crates/ul-game/src/types.rs` — add Bind, ModifyAssertion to Operation enum
- [ ] P0-29: Update `crates/ul-game/src/evaluation.rs` — score bind/modify_assertion
- [ ] P0-30: Update `crates/ul-api/src/routes.rs` — operations available via API
- [ ] P0-31: Update `crates/ul-mcp/src/main.rs` — operations returned by tools
- [ ] P0-32: Update `crates/ul-cli/src/main.rs` — operations parseable from CLI
- [ ] P0-33: Update `bindings/python/src/lib.rs` — operations available in Python
- [ ] P0-34: **GATE: `cargo test --workspace` passes with 13/13 operations**

---

## Phase 1a: Modal Extension in Code

### P1a: Distinguished Elements & Types
- [ ] P1a-1: Create `crates/ul-core/src/distinguished.rs` — registry for named elements
- [ ] P1a-2: Register `w_current` (actual world entity)
- [ ] P1a-3: Register `r_satisfies` (satisfaction relation: w ⊨ a)
- [ ] P1a-4: Register `r_alethic` (alethic accessibility)
- [ ] P1a-5: Register `r_K_agent` (epistemic accessibility)
- [ ] P1a-6: Register `r_O` (deontic accessibility)
- [ ] P1a-7: Register `r_ability_agent` (ability accessibility)
- [ ] P1a-8: Register `r_closeness` (counterfactual similarity ordering)
- [ ] P1a-9: Export distinguished elements module from `lib.rs`

### P1a: Modal Operators
- [ ] P1a-10: Add `WorldFrame` wrapper type (enclosure with modal context)
- [ ] P1a-11: Add `AccessibleFrom` edge type with relation parameter
- [ ] P1a-12: Add `modal_context` field to Gir root
- [ ] P1a-13: Implement □ (necessity) as composed pattern in `composer.rs` or new `modal.rs`
- [ ] P1a-14: Implement ◇ (possibility) as composed pattern
- [ ] P1a-15: Implement □→ (counterfactual) as Lewis-closeness pattern
- [ ] P1a-16: Add modal well-formedness rules to `validator.rs`

### P1a: Modal Rendering & Parsing
- [ ] P1a-17: Render world-enclosures with border styles (bold=□, dashed=◇, dashed-dot=□→)
- [ ] P1a-18: Render accessibility relation arrows between worlds
- [ ] P1a-19: Add □, ◇, □→ tokens to `parser/ul_script.pest`
- [ ] P1a-20: Update parser transform for modal syntax
- [ ] P1a-21: Update deparser for modal GIR→UL-Script
- [ ] P1a-22: Write unit + integration tests for all modal operators
- [ ] P1a-23: Update `schemas/gir.schema.json` — `modal_context`, accessibility fields

---

## Phase 1b: Performative Extension in Code

### P1b: Force Parameter
- [ ] P1b-1: Add `PerformativeForce` enum (assert/query/direct/commit/express/declare) to `types/node.rs`
- [ ] P1b-2: Add `force` field to assertion-sort nodes in `types/node.rs`
- [ ] P1b-3: Register `e_speaker` and `e_hearer` in `distinguished.rs`
- [ ] P1b-4: Implement force assignment in composer (default=assert)
- [ ] P1b-5: Implement force composition rules FC1–FC5

### P1b: Force Rendering & Parsing
- [ ] P1b-6: Render 6 force-specific border decorations in `renderer/svg.rs`
- [ ] P1b-7: Add force annotation tokens to parser
- [ ] P1b-8: Update deparser for force-annotated GIR
- [ ] P1b-9: Add force validation (one force per assertion frame)
- [ ] P1b-10: Write unit + integration tests for all 6 forces + composition
- [ ] P1b-11: Update `schemas/gir.schema.json` — `performative_force` field

---

## Phase 1c: Pragmatic Interface in Code

- [ ] P1c-1: Create `crates/ul-core/src/pragmatic.rs`
- [ ] P1c-2: Implement SI-1 (scalar implicature: "some" ⟹ "not all")
- [ ] P1c-3: Implement SI-2 (quantity scale: "warm" ⟹ "not hot")
- [ ] P1c-4: Implement SI-3 (disjunction: "A or B" ⟹ "not both")
- [ ] P1c-5: Implement CI-1 (conventional: "but" ⟹ contrast)
- [ ] P1c-6: Implement CI-2 (conventional: appositives)
- [ ] P1c-7: Implement CI-3 (forceful: indirect request mapping)
- [ ] P1c-8: Add `pragmatic_annotations` array to GIR
- [ ] P1c-9: Implement surface→intended GIR transformation
- [ ] P1c-10: Render inference arrows (⟹) between surface and intended
- [ ] P1c-11: Write tests for all 6 inference rules
- [ ] P1c-12: Update `schemas/gir.schema.json` — pragmatic fields

---

## Phase 1 Cascade

- [ ] P1-C1: Update `ul-wasm` — expose modal/performative/pragmatic via WASM
- [ ] P1-C2: Update `ul-game` types — add modal/force Operation variants
- [ ] P1-C3: Update `ul-game` scoring — modal/force-aware evaluation
- [ ] P1-C4: Update `ul-api` — extension-aware route handling
- [ ] P1-C5: Update `ul-mcp` — extension-aware tool responses
- [ ] P1-C6: Update `ul-cli` — extension-aware commands
- [ ] P1-C7: Update Python bindings — extension-aware functions
- [ ] P1-C8: Update web editor — modal UI, force picker, pragmatic panel
- [ ] P1-C9: Update VS Code extension — extension syntax + diagnostics
- [ ] P1-C10: Rebuild all dist/ outputs (fix stale "11 operations")
- [ ] P1-C11: **GATE: Full test suite passes, web editor renders extensions**

---

## Phase 2: UL-Core Documentation Updates

### P2: Writer's Companion — New Examples
> **Note:** Pass 2 already added Examples 11-19 (bind, modify_assertion, 3 modal, 2 performative, 1 pragmatic). The items below are ALREADY COMPLETE and included for tracking only.
- [x] ~~P2-1: Example 14: Modal — "She might be sleeping" (◇ epistemic)~~ — *already exists*
- [x] ~~P2-2: Example 15: Modal — "2+2 necessarily equals 4" (□ alethic)~~ — *already exists*
- [x] ~~P2-3: Example 16: Counterfactual — "He could have won if he had tried" (□→)~~ — *already exists*
- [x] ~~P2-4: Example 17: Performative — "I promise to return" (commissive)~~ — *already exists*
- [x] ~~P2-5: Example 18: Performative — "I pronounce you married" (declarative)~~ — *already exists*
- [x] ~~P2-6: Example 19: Pragmatic — "Can you pass the salt?" (CI-1)~~ — *already exists*
- [ ] P2-7: Example 20: Explicit `invert` — "The ball was kicked by the child" (passive voice)
- [ ] P2-8: Example 21: Explicit `abstract` — standalone modifier extraction demonstration
- [ ] P2-9: Example 22: Explicit `compose` — "grandfather" = compose(father, father)

### P2: Symbology Updates
- [ ] P2-10: Clarify "21 markers" count methodology in symbol-map.md
- [ ] P2-11: Add modal symbols (world-enclosure marks, border styles) to §VII
- [ ] P2-12: Add force symbols (6 force type marks) to §VII
- [ ] P2-13: Add pragmatic inference symbol (⟹) to §VIII

### P2: Syntax Dictionary Updates
- [ ] P2-14: Add §3.15: Performative syntax (force annotation conventions)
- [ ] P2-15: Add §3.16: Pragmatic notation (inference arrow conventions)
- [ ] P2-16: Verify §3.12 (bind) and §3.13 (modify_assertion) fully specified

### P2: Thesaurus Updates
- [ ] P2-17: Add modal meaning families (necessity↔possibility↔counterfactual)
- [ ] P2-18: Add force meaning families (assert↔query↔direct↔commit↔express↔declare)
- [ ] P2-19: Add pragmatic inference pathways (surface↔intended synonymy)

### P2: Lexicon & Other
- [ ] P2-20: Evaluate if □, ◇, □→, φ deserve T2/T3 lexicon entries
- [ ] P2-21: Update UQPL spec — sync bind/modify_assertion divergence analysis
- [ ] P2-22: Verify operation-visual-map.md covers all 13 operations + extensions
- [ ] P2-23: Add prescribed Day 1–9 learning sequence to NAVIGATION.md
- [ ] P2-24: Verify formal-grammar.md C12 (bind) and C13 (modify_assertion) complete
- [ ] P2-25: **GATE: Zero stale references, all cross-references verified**

---

## Phase 3: Learning Infrastructure

### P3: Tutorial Content
- [ ] P3-1: Write "Learn UL in 15 Minutes" quickstart
- [ ] P3-2: Write Day 1–9 prescribed curriculum document
- [ ] P3-3: Create 10 practice exercises (Levels 1–3: recognition, writing, composition)
- [ ] P3-4: Write developer quickstart ("Build Your First UL Parser in 5 Minutes")
- [ ] P3-5: Create Jupyter notebook walkthrough (Python bindings)

### P3: Visual & Interactive Content
- [ ] P3-6: Generate 42 SVG gallery images from lexicon entries
- [ ] P3-7: Create 5 game challenge levels (curriculum-aligned)
- [ ] P3-8: Add validation explanation messages to web editor
- [ ] P3-9: Add operation composer UI to web editor

### P3: Developer Onboarding
- [ ] P3-10: Update CONTRIBUTING.md — add Rust/TS/Python contribution sections
- [ ] P3-11: Write modding guide (custom composition rules)
- [ ] P3-12: Write integration examples (React, Python, CLI)
- [ ] P3-13: **GATE: Someone can learn UL from zero using only the tutorial path**

---

## Phase 4: Packaging & Deployment

- [ ] P4-1: Deploy web editor to GitHub Pages or Vercel (public URL)
- [ ] P4-2: Publish `@ul-forge/core` to npm (beta tag)
- [ ] P4-3: Publish Python wheel to PyPI (beta tag)
- [ ] P4-4: Publish VS Code extension to marketplace
- [ ] P4-5: Push Docker image to GHCR / Docker Hub
- [ ] P4-6: Create CI/CD pipeline (GitHub Actions: test → build → publish on tag)
- [ ] P4-7: Create OpenAPI spec for ul-api (`schemas/openapi.yaml`)
- [ ] P4-8: Create transceiver schema (`schemas/ul-transceiver.schema.json`)
- [ ] P4-9: **GATE: `npm install @ul-forge/core` works, web editor is publicly accessible**

---

## Summary

| Phase | Items | Gates |
|-------|-------|-------|
| Phase 0: 13/13 Operations | 34 | `cargo test --workspace` passes |
| Phase 1a: Modal | 23 | Modal operators render correctly |
| Phase 1b: Performative | 11 | 6 forces compose + render |
| Phase 1c: Pragmatic | 12 | 6 inference rules exercised |
| Phase 1 Cascade | 11 | Full test suite + web editor |
| Phase 2: UL-Core Docs | 25 (6 pre-complete) | Zero stale refs, all examples verified |
| Phase 3: Learning | 13 | End-to-end learner path works |
| Phase 4: Packaging | 9 | Public install + access |
| **Total** | **138 (6 pre-complete, 132 remaining)** | **5 phase gates** |
