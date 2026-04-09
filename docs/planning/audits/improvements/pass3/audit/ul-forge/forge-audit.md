# UL Forge Implementation Audit

**Date:** 2026-04-08  
**Scope:** `ul-forge/` — Rust crates, web editor, VS Code extension, Python bindings, API, schemas  
**Goal:** Assess implementation completeness relative to formal specification (Σ_UL⁺)

---

## Executive Summary

**Grade: C (Architecturally solid, significantly incomplete)**

UL Forge has a well-designed multi-crate Rust workspace with 7 crates, a web editor, VS Code extension, Python bindings, and API server. The core pipeline (parse → validate → render) works reliably. However, the implementation lags the formal specification significantly: 2 of 13 operations are missing, and all three Pass 2 extensions (modality, performatives, pragmatics) have zero implementation.

**Readiness score: 55/100**

---

## 1. Workspace Architecture

### 1.1 Crate Inventory

| Crate | Purpose | Tests | Status |
|-------|---------|-------|--------|
| **ul-core** | Parser, renderer, validator, composer | 99 (29+33+23+14) | ✅ Solid |
| **ul-cli** | Command-line tools | 18 integration | ✅ Solid |
| **ul-wasm** | WebAssembly compilation | 4 functions | ✅ Solid |
| **ul-api** | REST + WebSocket server | 9 integration | ✅ Solid |
| **ul-mcp** | MCP server (6 tools) | — | ✅ Solid |
| **ul-game** | Game/learning mechanics | 27 | ✅ Solid |
| **ul-transceiver** | AI-to-AI protocol | — | ✅ Solid |

**Total: 7 crates, ~157 tests, all compiling**

### 1.2 Core Pipeline

```
UL-Script → Parser → GIR (JSON) → Validator → Renderer → SVG/TikZ
                                       ↓
                                  Composer (11/13 ops)
```

---

## 2. Operation Coverage

### 2.1 Implemented Operations (11/13)

All in `crates/ul-core/src/composer.rs`:

| # | Operation | Signature | Unit Tests | Notes |
|---|-----------|-----------|------------|-------|
| 1 | `predicate` | e × r × e → a | ✅ | Assertion creation |
| 2 | `modify_entity` | m × e → e | ✅ | Entity modification |
| 3 | `modify_relation` | m × r → r | ✅ | Relation modification |
| 4 | `negate` | a → a | ✅ | Involution verified |
| 5 | `conjoin` | a × a → a | ✅ | AND (Intersects edge) |
| 6 | `disjoin` | a × a → a | ✅ | OR (Adjacent edge) |
| 7 | `embed` | a → e | ✅ | Nominalization |
| 8 | `abstract` | e → m | ✅ | Adjectivalization |
| 9 | `compose` | r × r → r | ✅ | Transitivity |
| 10 | `invert` | r → r | ✅ | Relation reversal |
| 11 | `quantify` | m × e → a | ✅ | Quantifier application |

### 2.2 Missing Operations (2/13) ❌

| # | Operation | Signature | Required For |
|---|-----------|-----------|-------------|
| 12 | `bind` | e × a → a | Variable binding, co-reference, scope |
| 13 | `modify_assertion` | m × a → a | Evidentiality, emphasis, assertion-level modification |

**Impact:** Without `bind`, UL Forge cannot represent:
- Co-reference ("The person who knows that *they* are wise")
- Scope disambiguation (∀x∃y vs. ∃y∀x)
- Anaphora ("John said *he* left")

Without `modify_assertion`, UL Forge cannot represent:
- Evidentiality ("reportedly", "I believe that")
- Emphasis ("It IS the case that")
- Hedging ("It might be that")

### 2.3 Implementation Requirements

**For `bind(e, a) → a`:**
- New binding model in `types/` — variable slots, scope markers
- Co-reference resolution in validator
- Visual rendering of binding arrows/scope brackets
- ~200–400 lines estimated

**For `modify_assertion(m, a) → a`:**
- Frame decoration system (already partially in renderer for negation dashing)
- Assertion-level modifier types in GIR
- ~100–200 lines estimated

---

## 3. Extension Coverage

### 3.1 Modal Extensions ❌ (0% implemented)

**Formal spec (§7.1–7.9):** Defines □ (necessity), ◇ (possibility), □→ (counterfactual) via:
- 7 distinguished elements: `w_current`, `r_satisfies`, `r_alethic`, `r_K_agent`, `r_O`, `r_ability_agent`, `r_closeness`
- World-entities with accessibility relations
- All defined from existing operations (0 new primitives)

**What's needed in Forge:**
- World context layer in GIR (world frames as enclosure-entities)
- Distinguished element registry
- Accessibility relation handling in composer
- Modal rendering (world-enclosures with border styles: solid=actual, dashed=possible, dotted=hypothetical)
- Validation rules for modal well-formedness

### 3.2 Performative Extensions ❌ (0% implemented)

**Formal spec (§8.1–8.7):** Extends assertion tuple (F,C,σ) → (F,C,σ,φ) with:
- φ ∈ {assert, query, direct, commit, express, declare}
- 2 distinguished elements: `e_speaker`, `e_hearer`
- Force composition rules (FC1–FC5)

**What's needed in Forge:**
- Force parameter field on assertion nodes in GIR
- Speaker/hearer entity handling
- Force-specific rendering (assertion=solid, query=dashed-with-hook, directive=arrow)
- Force composition validation

### 3.3 Pragmatic Inference ❌ (0% implemented)

**Formal spec (§9.1–9.7):** Defines 6 inference rules:
- SI-1, SI-2, SI-3 (scalar implicature)
- CI-1, CI-2, CI-3 (conventional inference)
- 5-layer architecture

**What's needed in Forge:**
- Inference rule engine (surface → intended mapping)
- Pragmatic annotation layer in GIR
- Inference visualization (⟹ arrows between surface and intended forms)

---

## 4. Web Editor

### 4.1 Architecture

React 18 + Monaco + D3 + Zustand, WASM worker for off-thread processing.

| Component | Status | Notes |
|-----------|--------|-------|
| Monaco editor | ✅ | UL-Script syntax highlighting |
| Live SVG preview | ✅ | 100ms debounce, worker-based |
| Template palette | ✅ | 42 lexicon entries (T1/T2/T3) |
| Visual canvas | ✅ | 5 primitives, select, connect tools |
| Export (SVG/PNG/GIR) | ✅ | Download buttons |
| D3 zoom/pan | ✅ | Mouse wheel zoom |
| Undo/redo | ✅ | Zustand store |
| Keyboard shortcuts | ✅ | Ctrl+E mode toggle, Ctrl+Shift+P palette |

### 4.2 Missing for Learning App

| Feature | Priority | Description |
|---------|----------|-------------|
| Operation composer | HIGH | Drag-drop to apply operations (not just static templates) |
| Hints panel | HIGH | Surface ul-game hint system in UI |
| Validation explanations | HIGH | Human-readable error messages |
| Guided tutorials | HIGH | Step-by-step lessons within editor |
| Modal UI | MEDIUM | World-frame creation, border style picker |
| Performative UI | MEDIUM | Force parameter selector |
| Glyph gallery | LOW | Browse 42 canonical entries visually |

---

## 5. VS Code Extension

| Feature | Status |
|---------|--------|
| Syntax highlighting | ✅ TextMate grammar |
| `.ul` file recognition | ✅ |
| Error diagnostics | ✅ Parse/validation squiggles |
| Live preview | ✅ SVG webview |
| 6 commands | ✅ preview, exportSvg, exportTikz, showGir, validate, insertGlyph |
| WASM integration | ✅ |

**Minor issues:** Redundant activation event in package.json (non-blocking).

**Grade: A- (Functional, needs extension support)**

---

## 6. Python Bindings

4 functions exposed via PyO3 (`maturin develop`):

| Function | Status |
|----------|--------|
| `parse(script) → dict` | ✅ |
| `render(gir, w, h, embed) → str` | ✅ |
| `validate(gir, check_geom) → dict` | ✅ |
| `deparse(gir) → str` | ✅ |
| IPython magics (`%%ul`, `%%ul_gir`) | ✅ |

**Missing:**
- ❌ 13 operations not exposed (no `compose()`, `modify_entity()`, etc.)
- ❌ Game functions not exposed
- ❌ Lexicon lookup not exposed

---

## 7. API Server & MCP

### REST (ul-api)
`/parse`, `/validate`, `/render`, `/convert`, `/health` — all functional.

### MCP (ul-mcp)
6 tools: parse, validate, render, deparse, parse_and_render, lexicon.

**Missing in both:**
- ❌ No `/apply-operation` or operation tool endpoints
- ❌ No modal/performative/pragmatic support
- ❌ Game evaluation endpoints undocumented

---

## 8. Schemas

| Schema | Status |
|--------|--------|
| `schemas/gir.schema.json` | ✅ Exists (nodes, edges, sorts) |
| `schemas/ul-transceiver.schema.json` | ❌ Referenced but missing |

**GIR schema gaps:**
- No `modal_context` field
- No `performative_force` field
- No `pragmatic_annotation` field
- No `binding` or `scope` fields

---

## 9. Documentation Within Forge

| File | Status | Current? |
|------|--------|----------|
| `ul-forge/Cargo.toml` | ✅ | Yes |
| `ul-forge/Dockerfile` | ✅ | Yes |
| `ul-forge/_write_app.py` | ❓ | Purpose unclear |
| `docs/ul-forge-v1/` | ⚠️ | Comprehensive spec, disconnected from impl |

**Critical gap:** No mapping from ul-forge-v1 spec documents to actual file locations in `ul-forge/crates/`.

---

## 10. Stale References

| Location | Issue | Fix |
|----------|-------|-----|
| `crates/ul-core/src/composer.rs` doc comment | "13 operations" but only 11 implemented | Implement missing 2, or note "11 of 13" |
| `packages/core/dist/index.d.ts` | "11 operations" | Rebuild after source update |
| `packages/core/dist/index.js` | "11 operations" | Rebuild after source update |

---

## 11. Priority Actions for Pass 3

### P0: Complete Operations (Phase 0)
1. Implement `bind` in composer.rs + tests
2. Implement `modify_assertion` in composer.rs + tests
3. Update GIR schema with binding/scope fields

### P1: Modal Support (Phase 0)
4. Add world context to GIR types
5. Add distinguished element registry
6. Implement modal rendering
7. Add modal validation rules

### P2: Performative Support (Phase 0)
8. Add force parameter to assertion nodes
9. Implement force rendering
10. Add force composition validation

### P3: Packaging (Phase 1)
11. Rebuild dist/ outputs
12. Publish @ul-forge/core to npm
13. Publish Python wheel to PyPI
14. Create transceiver schema

### P4: Learning Features (Phase 2)
15. Operation composer UI in web editor
16. Hint panel integration
17. Guided tutorial system
18. Visual glyph gallery
