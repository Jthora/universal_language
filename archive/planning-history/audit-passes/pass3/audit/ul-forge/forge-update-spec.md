# UL-Forge Update Specification — Pass 3

**Status:** 📋 PLANNED  
**Scope:** All changes to `ul-forge/` crates, web editor, VS Code extension, Python bindings  
**Reference:** [CHECKLIST.md](../CHECKLIST.md) Phase 0 + Phase 1

---

## 1. Current State (Verified 2026-04-08)

### 1.1 What Works Today

| Component | Lines | Tests | Status |
|-----------|-------|-------|--------|
| **ul-core** (parser, renderer, validator, composer) | ~3,200 | 99 | ✅ Production |
| **ul-game** (scoring, hints, adaptive difficulty, animation) | ~1,800 | 27 | ✅ Production |
| **ul-api** (REST + WebSocket server) | ~350 | 9 | ✅ Production |
| **ul-mcp** (MCP server, 6 tools) | ~900 | — | ✅ Production |
| **ul-transceiver** (AI-to-AI protocol) | ~400 | — | ✅ Production |
| **ul-cli** (4 subcommands) | ~200 | 18 | ✅ Production |
| **ul-wasm** (5 WASM functions) | ~60 | — | ✅ Production |
| **Web editor** (React 18 + Monaco + Zustand) | ~600 TS | 9 test files | ✅ Production |
| **VS Code extension** (syntax + preview + 6 commands) | ~200 TS | — | ✅ Production |
| **Python bindings** (PyO3, 4 functions + IPython magic) | ~100 Rust | — | ✅ Production |
| **GIR schema** (JSON Schema v2020-12) | — | — | ✅ Complete |

**Total: 7 crates, ~7,510 lines Rust, ~1,700 lines TS, ~157 tests, zero stubs**

### 1.2 What's Missing

**Operations (composer.rs):**
- `bind(e, a) → a` — NOT IMPLEMENTED
- `modify_assertion(m, a) → a` — NOT IMPLEMENTED

**Extensions:**
- Modal semantics (§7) — 0% in code
- Performative force (§8) — 0% in code
- Pragmatic inference (§9) — 0% in code

**Schema fields:**
- No `variable_id`, `binding_scope`, `assertion_modifier`
- No `modal_context`, `accessibility_relation`
- No `performative_force`
- No `pragmatic_annotations`

---

## 2. Phase 0: Implement `bind` and `modify_assertion`

### 2.1 `bind(e, a) → a` — Design

**Formal specification** (formal-foundations.md):
```
bind(x, a) replaces all free occurrences of the slot entity ○_x  
in assertion a with the bound entity ●_x, and marks the binding scope.
```

**Geometric realization** (syntax-dictionary.md C12):
- **Slot entities** ○_x: hollow marks with subscript identifier
- **Bound entities** ●_x: filled marks with matching subscript
- **Scope**: determined by nesting depth (innermost enclosure containing ○_x)
- **Binding arrow**: dashed line from ●_x back to introduction site

> **Note:** Code examples in this spec are **illustrative, not literal**. The actual Node struct has 11 fields (not 14 as previously claimed), and constructor patterns will need to follow the existing style in `node.rs`. Use these designs as directional guidance; adapt to real struct shapes during implementation.

**Changes to `types/node.rs`:**
```rust
// Add to Node struct (currently 11 fields; these add 2 more):
pub variable_id: Option<String>,  // e.g., "x", "y"  
pub is_bound: Option<bool>,       // false = slot (free), true = bound

// Add node factory:
impl Node {
    pub fn slot(variable_id: &str) -> Self { ... }  // ○_x  
    pub fn bound(variable_id: &str) -> Self { ... } // ●_x
}
```

**Changes to `types/edge.rs`:**
```rust
// Add to EdgeType enum:
Binds,  // binding arrow from binder to slot
```

**Changes to `composer.rs`:**
```rust
pub fn bind(entity_gir: &Gir, assertion_gir: &Gir) -> UlResult<Gir> {
    // 1. Validate entity_gir root is Entity sort
    // 2. Validate assertion_gir root is Assertion sort
    // 3. Find all slot entities in assertion_gir matching variable_id
    // 4. Mark them as bound
    // 5. Create Binds edges from entity to each slot
    // 6. Set binding_scope on the containing enclosure
    // 7. Return combined GIR
}
```

**Changes to `validator.rs`:**
```rust
// Add to validate_invariants():
fn validate_bindings(gir: &Gir) -> Vec<ValidationError> {
    // 1. Check all bound marks have corresponding binder entity
    // 2. Check no dangling slots (free variables without binder)
    // 3. Check binding scope is well-nested (no cross-scope capture)
}
```

**Changes to `renderer/svg.rs`:**
- Slot entities render as hollow circles ○ with subscript
- Bound entities render as filled circles ● with matching subscript
- Binding arrows: dashed curved lines from binder to each slot occurrence
- Scope brackets: subtle background shade on containing enclosure

**Changes to `parser/ul_script.pest`:**
```pest
SlotMark = { "○" ~ Subscript | "o_" ~ Identifier }
BoundMark = { "●" ~ Subscript | "._" ~ Identifier }
Subscript = { "_" ~ Identifier }
```

### 2.2 `modify_assertion(m, a) → a` — Design

**Formal specification** (formal-foundations.md):
```
modify_assertion(m, a) applies modifier m to the assertion-level frame,  
changing its presentation without altering truth content.
Types: evidential (reported), emphatic (stressed), hedged (tentative)
```

**Geometric realization** (syntax-dictionary.md C13):
- **Evidential**: dotted frame border (···)
- **Emphatic**: double frame border (═══)
- **Hedged**: wavy frame border (≈≈≈)

**Changes to `types/node.rs`:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AssertionModifierType {
    Evidential,   // reported, hearsay
    Emphatic,     // stressed, certain
    Hedged,       // tentative, uncertain
    Custom(String),
}

// Add to Node struct:
pub assertion_modifier: Option<AssertionModifierType>,
```

**Changes to `composer.rs`:**
```rust
pub fn modify_assertion(modifier_gir: &Gir, assertion_gir: &Gir) -> UlResult<Gir> {
    // 1. Validate modifier_gir root is Modifier sort
    // 2. Validate assertion_gir root is Assertion sort
    // 3. Apply modifier to assertion frame (set assertion_modifier field)
    // 4. Create ModifiedBy edge from assertion to modifier
    // 5. Return combined GIR
}
```

**Changes to `renderer/svg.rs`:**
```rust
fn render_frame_border(modifier: &AssertionModifierType) -> String {
    match modifier {
        Evidential => "stroke-dasharray: 2,2",    // dotted
        Emphatic => double border (nested rect),    // double line
        Hedged => "stroke-dasharray: 4,2,1,2",     // wavy approximation
    }
}
```

**Changes to `parser/ul_script.pest`:**
```pest
AssertionMod = { "..." | "===" | "~~~" }
// ... = evidential, === = emphatic, ~~~ = hedged
```

### 2.3 Cascade Updates

After implementing both operations:

| Crate | Change |
|-------|--------|
| `ul-wasm` | Export `bind()` and `modifyAssertion()` as WASM functions |
| `ul-game/types.rs` | Add `Bind` and `ModifyAssertion` to `Operation` enum |
| `ul-game/evaluation.rs` | Score bind/modify_assertion in compositions |
| `ul-api/routes.rs` | New operations available via POST /compose endpoint |
| `ul-mcp/main.rs` | `ul_parse` tool returns bind/modify_assertion in detect_operations |
| `ul-cli/main.rs` | `parse` command handles new syntax |
| `bindings/python/lib.rs` | Parse/render/validate handle new operations |
| `schemas/gir.schema.json` | Add `variable_id`, `is_bound`, `assertion_modifier`, `Binds` edge type |

---

## 3. Phase 1a: Modal Extension

### 3.1 Distinguished Element Registry

**New file: `crates/ul-core/src/distinguished.rs`**

```rust
/// Registry of distinguished elements that have special semantic roles.
/// These are not new primitives — they are specific INSTANCES of existing sorts
/// with canonical names and roles used by modal/performative extensions.
pub struct DistinguishedRegistry {
    elements: HashMap<String, DistinguishedElement>,
}

pub struct DistinguishedElement {
    pub id: String,
    pub sort: Sort,       // Entity, Relation, etc.
    pub role: String,     // Description of what this element does
    pub extension: Extension, // Modal, Performative, etc.
}

pub enum Extension { Core, Modal, Performative }

// Pre-registered elements:
// Modal (7):
//   w_current    : Entity   — actual world
//   r_satisfies  : Relation — satisfaction (w ⊨ a)
//   r_alethic    : Relation — alethic accessibility
//   r_K_agent    : Relation — epistemic accessibility (parameterized by agent)
//   r_O          : Relation — deontic accessibility
//   r_ability_agent : Relation — ability accessibility (parameterized by agent)
//   r_closeness  : Relation — counterfactual similarity ordering
//
// Performative (2):
//   e_speaker    : Entity   — the speaker/writer
//   e_hearer     : Entity   — the hearer/reader
```

### 3.2 Modal Operators as Composed Patterns

The three modal operators are NOT new operations. They are *macros* — composed patterns using existing operations:

**□ (necessity): "necessarily a"**
```
predicate(w_current, r_alethic, embed(
  quantify(m_all, embed(
    predicate(○_w, r_satisfies, embed(a))
  ))
))
```
Where: for all worlds w accessible from w_current via r_alethic, w satisfies a.

Rendered as: bold-bordered enclosure containing a.

**◇ (possibility): "possibly a"**
```
negate(predicate(w_current, r_alethic, embed(
  quantify(m_all, embed(
    negate(predicate(○_w, r_satisfies, embed(a)))
  ))
)))
```
= ¬□¬a (standard modal equivalence)

Rendered as: dashed-bordered enclosure containing a.

**□→ (counterfactual): "if p were true, q would be true"**
```
predicate(
  embed(predicate(closest_w, r_closeness, embed(p))),
  r_satisfies,
  embed(q)
)
```
Where closest_w selects the closest p-world.

Rendered as: dashed-dot-bordered enclosure.

### 3.3 GIR Schema Additions

```json
{
  "modal_context": {
    "type": "object",
    "properties": {
      "worlds": { "type": "array", "items": { "$ref": "#/$defs/world" } },
      "accessibility_relations": { "type": "array", "items": { "$ref": "#/$defs/accessibility" } },
      "actual_world": { "type": "string" }
    }
  }
}
```

### 3.4 Rendering Conventions

| Modal | Border Style | SVG |
|-------|-------------|-----|
| Actual (no modal) | Solid 1px | `stroke-width: 1` |
| □ (necessity) | Bold 3px | `stroke-width: 3` |
| ◇ (possibility) | Dashed | `stroke-dasharray: 6,3` |
| □→ (counterfactual) | Dash-dot | `stroke-dasharray: 6,3,2,3` |
| Epistemic (K) | Double thin | Two nested strokes |
| Deontic (O) | Dotted thick | `stroke-dasharray: 2,2; stroke-width: 2` |

---

## 4. Phase 1b: Performative Extension

### 4.1 Force Parameter

**Changes to `types/node.rs`:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PerformativeForce {
    Assert,   // default: declarative statement
    Query,    // question
    Direct,   // command/request
    Commit,   // promise/vow
    Express,  // exclamation/emotion
    Declare,  // institutional performative
}
```

Added as optional field on assertion-sort nodes. Default = Assert.

### 4.2 Force Composition Rules

From formal-foundations.md §8.4:

| Rule | Description | Implementation |
|------|-------------|----------------|
| FC1 | Embedding preserves force | embed(a_φ) → e carries force in metadata |
| FC2 | Conjunction: dominant force wins | conjoin(a_φ₁, a_φ₂) → a_{dominant(φ₁,φ₂)} |
| FC3 | Disjunction: weakest force | disjoin(a_φ₁, a_φ₂) → a_{weaker(φ₁,φ₂)} |
| FC4 | Negation preserves force | negate(a_φ) → a_φ |
| FC5 | Default is Assert | unannoted = Assert |

### 4.3 Rendering Conventions

| Force | Border Decoration | SVG |
|-------|------------------|-----|
| Assert (default) | Solid | Normal frame |
| Query | Dashed + hook (?) | `stroke-dasharray: 4,2` + `?` glyph |
| Direct | Arrow-out (→) | Frame with directional arrow |
| Commit | Arrow-in (←) | Frame with inward arrow |
| Express | Wavy (~) | `stroke-dasharray: wavy` |
| Declare | Box (□) | Double-width solid frame |

---

## 5. Phase 1c: Pragmatic Interface

### 5.1 Inference Engine

**New file: `crates/ul-core/src/pragmatic.rs`**

The pragmatic module is NOT a core algebra module. It's an *inference layer* that:
1. Takes a surface GIR (what was literally said/written)
2. Applies inference rules to produce an intended GIR (what was meant)
3. Returns both GIRs + the inference chain

```rust
pub struct PragmaticResult {
    pub surface: Gir,
    pub intended: Gir,
    pub inferences: Vec<InferenceStep>,
}

pub struct InferenceStep {
    pub rule: InferenceRule,
    pub trigger: String,      // what triggered this inference
    pub conclusion: String,   // what was inferred
}

pub enum InferenceRule {
    SI1,  // scalar: "some" ⟹ "not all"
    SI2,  // quantity: "warm" ⟹ "not hot"  
    SI3,  // disjunction: "A or B" ⟹ "not both"
    CI1,  // conventional: "but" ⟹ contrast
    CI2,  // conventional: appositive ⟹ supplementary
    CI3,  // forceful: indirect request ⟹ directive
}

pub fn infer(surface: &Gir) -> PragmaticResult { ... }
```

### 5.2 GIR Annotation

```json
{
  "pragmatic_annotations": [
    {
      "rule": "CI-3",
      "surface_node": "a_12",
      "intended_transform": "force: query → direct",
      "confidence": 0.85
    }
  ]
}
```

---

## 6. Web Editor Updates

After all Phase 0 + Phase 1 code changes:

| Feature | Priority | Description |
|---------|----------|-------------|
| Operation composer panel | HIGH | Drag-drop to apply operations (replaces template-only editing) |
| Modal UI | HIGH | World-frame creation tool, border style picker |
| Force picker | HIGH | Dropdown/toggle for performative force on assertions |
| Hints panel | MEDIUM | Surface ul-game hints in editor UI |
| Validation explanations | MEDIUM | Turn raw errors into human-readable suggestions |
| Pragmatic toggle | LOW | Show/hide pragmatic inference overlay |
| Glyph gallery | LOW | Browse 42 lexicon entries visually |

### 6.1 TypeScript Type Updates

`web/src/core/index.ts` needs:
- `PerformativeForce` type
- `AssertionModifierType` type
- `ModalContext` type
- `PragmaticAnnotation` type
- Updated WASM function signatures

### 6.2 New WASM Entry Points

| Function | Purpose |
|----------|---------|
| `applyBind(ctx, entity, assertion)` | Bind operation |
| `applyModifyAssertion(ctx, modifier, assertion)` | Modify assertion |
| `inferPragmatics(gir)` | Run pragmatic inference |
| `getDistinguished(name)` | Look up distinguished element |

---

## 7. VS Code Extension Updates

| Feature | Description |
|---------|-------------|
| Syntax highlighting | Add □, ◇, □→, force annotations, slot marks |
| Diagnostics | Binding scope errors, force constraint violations |
| Snippets | Quick-insert modal/performative constructions |
| Hover info | Tooltip explaining distinguished elements |

---

## 8. Python Binding Updates

Expand from 4 to ~10 functions:

| Function | Signature |
|----------|-----------|
| `parse(script)` | Existing |
| `render(gir, ...)` | Existing |
| `validate(gir, ...)` | Existing |
| `deparse(gir)` | Existing |
| `compose(op, *girs)` | **NEW**: Apply any of 13 operations |
| `detect_operations(gir)` | **NEW**: Reverse-engineer operations |
| `infer_pragmatics(gir)` | **NEW**: Run pragmatic inference |
| `get_distinguished(name)` | **NEW**: Look up named elements |
| `query_lexicon(query)` | **NEW**: Search lexicon |
| `analyze_structure(gir)` | **NEW**: Structure report |

---

## 9. Files Changed Summary

| Category | Files | Character |
|----------|-------|-----------|
| **New Rust files** | `distinguished.rs`, `pragmatic.rs`, possibly `modal.rs`, `performative.rs` | Core extensions |
| **Modified Rust** | `composer.rs`, `validator.rs`, `types/node.rs`, `types/edge.rs`, `types/gir.rs`, `parser/*`, `renderer/*`, `lexicon.rs`, `lib.rs` | Core updates |
| **Cascade Rust** | All 6 other crate `lib.rs`/`main.rs` files | Downstream updates |
| **Schema** | `schemas/gir.schema.json` | All new fields |
| **TypeScript** | `web/src/core/index.ts`, component files, store | Type updates + UI |
| **VS Code** | `extension.ts`, `ul.tmLanguage.json` | Syntax + diagnostics |
| **Python** | `bindings/python/src/lib.rs` | New functions |
| **Estimated total** | ~35 files modified/created | ~3,500 lines |
