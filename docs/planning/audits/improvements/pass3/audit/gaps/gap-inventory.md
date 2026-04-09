# Gap Inventory — Pass 3

**Date:** 2026-04-08  
**Purpose:** Complete prioritized list of all gaps between current state and learning/app-readiness  
**Source:** Cross-referenced from forge-audit.md, core-audit.md, infra-audit.md, learning-audit.md

---

## Summary

**48 gaps identified** across 5 categories:

| Category | Critical | High | Medium | Low | Total |
|----------|----------|------|--------|-----|-------|
| A. Forge Operations | 2 | 3 | 3 | 2 | 10 |
| B. Extensions | 3 | 2 | 2 | 0 | 7 |
| C. Packaging | 2 | 3 | 3 | 1 | 9 |
| D. Documentation | 1 | 4 | 3 | 2 | 10 |
| E. Learning | 2 | 4 | 4 | 2 | 12 |
| **Total** | **10** | **16** | **15** | **7** | **48** |

---

## Category A: Forge Operations

### CRITICAL

| ID | Gap | Current | Target | Effort |
|----|-----|---------|--------|--------|
| A1 | `bind` operation not implemented | 0% | composer.rs + tests | Medium |
| A2 | `modify_assertion` not implemented | 0% | composer.rs + tests | Medium |

### HIGH

| ID | Gap | Current | Target | Effort |
|----|-----|---------|--------|--------|
| A3 | Operations not exposed in Python bindings | 4 functions only | All 13 ops + game | Medium |
| A4 | Operations not exposed in API/MCP | Parse/render only | `/apply-operation` + tools | Medium |
| A5 | GIR schema missing binding/scope fields | No field | `binding`, `scope` fields | Low |

### MEDIUM

| ID | Gap | Current | Target | Effort |
|----|-----|---------|--------|--------|
| A6 | Web editor has no operation composer | Static templates only | Interactive op builder | High |
| A7 | Validation errors not human-readable | Raw parse errors | Explanatory messages | Medium |
| A8 | dist/ outputs stale ("11 operations") | Stale | Rebuild from source | Low |

### LOW

| ID | Gap | Current | Target | Effort |
|----|-----|---------|--------|--------|
| A9 | `_write_app.py` purpose unclear | Exists, undocumented | Document or remove | Low |
| A10 | GIR schema `EdgeType` missing `references` | Schema has 5 values, Rust code has 6 (`references` missing) | Add `references` to `gir.schema.json` | Trivial |

---

## Category B: Extensions (Modal, Performative, Pragmatic)

### CRITICAL

| ID | Gap | Current | Target | Effort |
|----|-----|---------|--------|--------|
| B1 | Modal operators not in Forge | 0% implemented | World frames, □/◇/□→ in GIR + renderer | High |
| B2 | Performative force not in Forge | 0% implemented | φ parameter on assertions + rendering | High |
| B3 | Pragmatic inference not in Forge | 0% implemented | SI/CI rules + annotation layer | High |

### HIGH

| ID | Gap | Current | Target | Effort |
|----|-----|---------|--------|--------|
| B4 | Distinguished element registry missing | No mechanism | Registry for 7 modal + 2 performative elements | Medium |
| B5 | GIR schema missing extension fields | No fields | `modal_context`, `performative_force`, `pragmatic_annotation` | Medium |

### MEDIUM

| ID | Gap | Current | Target | Effort |
|----|-----|---------|--------|--------|
| B6 | Modal rendering not implemented | No world-enclosures | Border styles: solid/dashed/dotted | Medium |
| B7 | Force rendering not implemented | No force marks | Assertion decoration by force type | Medium |

---

## Category C: Packaging & Deployment

### CRITICAL

| ID | Gap | Current | Target | Effort |
|----|-----|---------|--------|--------|
| C1 | `@ul-forge/core` not on npm | 404 | Published (beta tag) | Low |
| C2 | No deployed web demo | Development only | GitHub Pages / Vercel URL | Low |

### HIGH

| ID | Gap | Current | Target | Effort |
|----|-----|---------|--------|--------|
| C3 | Python wheel not on PyPI | Local maturin only | Published (beta tag) | Low |
| C4 | Docker image not pushed | Dockerfile only | Container registry | Low |
| C5 | VS Code extension not on marketplace | VSIX file only | Published | Low |

### MEDIUM

| ID | Gap | Current | Target | Effort |
|----|-----|---------|--------|--------|
| C6 | No CI/CD pipeline | Manual builds | GitHub Actions | Medium |
| C7 | Transceiver schema missing | Referenced but absent | `schemas/ul-transceiver.schema.json` | Medium |
| C8 | No OpenAPI spec for ul-api | Informal docs | OpenAPI 3.0 YAML | Medium |

### LOW

| ID | Gap | Current | Target | Effort |
|----|-----|---------|--------|--------|
| C9 | Web components not implemented | Planning docs only | `<ul-symbol>`, `<ul-composer>` | High |

---

## Category D: Documentation

### CRITICAL

| ID | Gap | Current | Target | Effort |
|----|-----|---------|--------|--------|
| D1 | No "Quick Start for Developers" | Missing entirely | 5-minute guide: install → first glyph | Medium |

### HIGH

| ID | Gap | Current | Target | Effort |
|----|-----|---------|--------|--------|
| D2 | ul-forge-v1 spec not mapped to implementation | Disconnected | Annotate each spec doc with code links | Medium |
| D3 | CONTRIBUTING.md missing code contribution guidance | Experiment-focused only | Add Rust/TS/Python contribution sections | Low |
| D4 | Syntax dictionary missing performative/pragmatic sections | Only modal (§3.14) | Add §3.15 (performatives), §3.16 (pragmatics) | Medium |
| D5 | Thesaurus missing modal/performative pathways | Core families only | Add modal/force meaning families | Medium |

### MEDIUM

| ID | Gap | Current | Target | Effort |
|----|-----|---------|--------|--------|
| D6 | Symbology "21 markers" count unclear | Ambiguous | Clarify what exactly the 21 are | Low |
| D7 | Lexicon has no modal/force entries | 42 core entries only | Add T2/T3 modal/force entries | Medium |
| D8 | No auto-generated API reference | Manual docs only | cargo doc + typedoc | Medium |

### LOW

| ID | Gap | Current | Target | Effort |
|----|-----|---------|--------|--------|
| D9 | Game modding system undocumented | Code exists, no docs | Write modding guide | Low |
| D10 | UQPL spec not updated for bind/modify_assertion | Outdated | Sync with current operations | Medium |

---

## Category E: Learning Experience

### CRITICAL

| ID | Gap | Current | Target | Effort |
|----|-----|---------|--------|--------|
| E1 | No "Learn UL in 15 Minutes" tutorial | Missing entirely | First-contact document | Medium |
| E2 | No prescribed learning sequence | Ad hoc navigation | Day 1–9 curriculum | Low |

### HIGH

| ID | Gap | Current | Target | Effort |
|----|-----|---------|--------|--------|
| E3 | Writer's Companion missing modal example | No "it's possible that X" | Worked Example 7 | Medium |
| E4 | Writer's Companion missing performative example | No "I command X" | Worked Example 8 | Medium |
| E5 | Writer's Companion missing pragmatic example | Only Ex 19 (CI-1) | Worked Example 9 | Medium |
| E6 | No visual glyph gallery | No SVG browsing | 42 rendered SVGs | Medium |

### MEDIUM

| ID | Gap | Current | Target | Effort |
|----|-----|---------|--------|--------|
| E7 | No practice exercises | Reading only | 10 exercises (Levels 1–3) | Medium |
| E8 | No game challenge levels | Engine exists, no content | 5 curriculum-aligned levels | High |
| E9 | No Jupyter notebook walkthrough | Functions exist, no notebook | Interactive Python tutorial | Medium |
| E10 | No integration code examples | No samples | React/Python/CLI examples | Medium |

### LOW

| ID | Gap | Current | Target | Effort |
|----|-----|---------|--------|--------|
| E11 | No video content | Text only | "Writing Your First Glyph" screencast | Medium |
| E12 | Missing `invert`/`bind`/`abstract`/`compose` standalone examples | Implicit only | Dedicated Writer's Companion examples | Medium |

---

## Dependency Graph

```
Phase 0 (Forge Completeness):
  A1 (bind) ──────┐
  A2 (mod_assert) ─┤
  B1 (modal) ──────┤──→ A5 (schema) → B5 (ext schema)
  B2 (performative)┤
  B3 (pragmatic) ──┘

Phase 1 (Packaging):
  A8 (dist rebuild) → C1 (npm publish)
  C2 (deploy web) ← independent
  C3 (PyPI) ← independent
  C4 (Docker) ← independent
  C5 (VS Code) ← independent
  C6 (CI/CD) ← depends on all above

Phase 2 (Learning):
  E1 (tutorial) ← independent
  E2 (curriculum) ← independent
  E3/E4/E5 (examples) ← depends on grammar §VI-B/C/D (done)
  E6 (gallery) ← depends on C2 (deployed web)
  A6 (op composer) ← depends on A1/A2 (all ops implemented)
  E8 (game levels) ← depends on C2 (deployed web)

Phase 3 (Validation):
  Demo experiment ← depends on C2 + E1
  Code tutorials ← depends on C1 (published packages)
```

---

## Effort Estimates by Phase

| Phase | Critical | High | Medium | Low | Total Items | Character |
|-------|----------|------|--------|-----|-------------|-----------|
| **Phase 0** | 5 (A1,A2,B1,B2,B3) | 3 (A3,B4,B5) | 3 (A6,B6,B7) | 1 (A8) | 12 | Heavy Rust development |
| **Phase 1** | 2 (C1,C2) | 3 (C3,C4,C5) | 3 (C6,C7,C8) | 1 (C9) | 9 | DevOps & packaging |
| **Phase 2** | 3 (D1,E1,E2) | 8 (D2-D5,E3-E6) | 7 (D6-D8,E7-E10) | 4 (D9,D10,E11,E12) | 22 | Content & documentation |
| **Phase 3** | 0 | 2 (A4,A5) | 2 (A7,D8) | 0 | 4 | Validation & polish |

---

## Recommended Execution Order

1. **A1, A2** — Implement missing operations (unblocks everything)
2. **B1, B2, B3** — Implement extensions (unblocks extension-aware content)
3. **B4, B5, A5** — Schema updates (required for data persistence)
4. **A8, C1, C2** — Rebuild, publish, deploy (unblocks learner access)
5. **E1, E2** — Tutorial + curriculum (first learner experience)
6. **E3, E4, E5** — Advanced Writer's Companion examples
7. **D1, D2, D3** — Developer documentation
8. **C3, C4, C5** — Additional package publishing
9. **E6, E7** — Gallery + exercises
10. **A6, E8** — Interactive features + game content
11. **C6** — CI/CD
12. **Everything else** — Polish and complete
