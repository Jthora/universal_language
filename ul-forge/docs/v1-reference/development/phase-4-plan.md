# Phase 4 — Intelligence: Detailed Implementation Plan

> Detailed milestone breakdown for Phase 4 of UL Forge v1.
> Converts the high-level roadmap into actionable sub-milestones with
> ordered tasks, file deliverables, dependency chains, and exit criteria.

---

## Prerequisites

Phase 4 depends on Phases 1–3 being complete:

| Dependency | Status | What Phase 4 Needs From It |
|------------|--------|---------------------------|
| Phase 1 Core Pipeline | ✅ Complete | Parser (33 tests), Validator (14 tests), Renderer (23 tests) |
| Phase 2 Web Editor | ✅ Complete | Monaco editor, SVG preview, visual canvas, template palette |
| Phase 3 Ecosystem | ✅ Complete | VS Code ext, PyO3 bindings, API server (126 total tests) |

---

## Phase 4 Internal Dependency Graph

```
M4.1 Template Library ──────────────────────┐
    │                                       │
    │ (templates needed for AI prompts)     │ (templates needed for composition tests)
    ▼                                       │
M4.2a LLM Interface ────────┐               │
    │                       │               │
    │                       │               │
    ▼                       ▼               ▼
M4.2b AI Panel          M4.3a Diff/Merge ──→ M4.3b CRDT Collaboration
(web editor)             (git integration)    (real-time editing)
                              │
M4.4a TikZ Polish ◄──────────┘
M4.4b Batch Render
M4.4c GIR Query Language
M4.4d Performance

─── Stretch goals (v1 if time permits) ───
M4.2c GNN Interface
M4.2d Vision Interface
M4.2e Theorem Prover
```

**Critical path:** M4.1 → M4.2a → M4.2b → M4.3b (collaboration requires editor AI panel to exist)

**Parallel tracks after M4.1:**
- Track A: AI (M4.2a → M4.2b)
- Track B: Collaboration (M4.3a → M4.3b)
- Track C: Polish (M4.4a–d, independent)

---

## M4.1 — Complete Template Library

**Goal:** All 42 canonical lexicon entries have validated renderer templates with anchor points for composition.

**Duration estimate removed per policy — measure by deliverables.**

### M4.1.1: Tier 1 Template Completion (12 remaining of 24)

The renderer currently has 6 multi-node templates. The web palette lists 24 T1 entries. Complete the gap.

**Current state (6 implemented):** Existence ○{●}, Connection ● → ●, Boundary ○|○, Intersection ○&○, Containment ○{○}, Single-node primitives (●, ○, △, □, ~, ∠)

**To implement:**

| # | Template | Pattern | Complexity | File |
|---|----------|---------|------------|------|
| 1 | Entity-in-Triangle | `△{●}` | Low — mirror ○{●} for triangle | templates.rs |
| 2 | Entity-in-Square | `□{●}` | Low — mirror ○{●} for square | templates.rs |
| 3 | Distinguished Angle 0° | `∠0` | Low — collapsed angle mark | templates.rs |
| 4 | Distinguished Angle 90° | `∠90` | Low — right angle mark | templates.rs |
| 5 | Distinguished Angle 180° | `∠180` | Low — straight line mark | templates.rs |
| 6 | Distinguished Angle 360° | `∠360` | Low — full rotation arc | templates.rs |
| 7 | Modify Entity | `∠60 ●` | Medium — angle mark adjacent to point | templates.rs |
| 8 | Modify Relation | `● →∠45 ●` | Medium — angle on arrow midpoint | templates.rs |
| 9 | Negation | `¬(a)` | Medium — strike-through or bar over assertion | templates.rs |
| 10 | Conjunction | `a ∧ b` | Medium — two subglyphs joined at vertex | templates.rs |
| 11 | Disjunction | `a ∨ b` | Medium — two subglyphs joined at divergence | templates.rs |
| 12 | Composition | `● → ● → ●` | Medium — chained connection template | templates.rs |
| 13 | Embedding | `⌈a⌉` | Medium — assertion wrapped in enclosure | templates.rs |
| 14 | Abstraction | `⌊e⌋` | Medium — entity extracted as modifier | templates.rs |
| 15 | Inversion | `● ← ●` | Low — reverse arrow template | templates.rs |
| 16 | Cycle | `↻` | Low — closed curve path | templates.rs |
| 17 | Void Enclosure | `◌` | Low — dashed/empty enclosure | templates.rs |
| 18 | Coincident Points | `● \| ●` | Low — two overlapping points | templates.rs |

- [ ] Implement templates 1–18 in `crates/uws/src/renderer/templates.rs`
- [ ] Add `anchor_points: HashMap<String, (f64, f64)>` to `Template` struct
- [ ] Add snapshot tests for each new template (reference SVG comparison)
- [ ] Verify each template parses → validates → renders round-trip

**Exit criterion:** `cargo test` includes snapshot tests for all 24 T1 templates.

### M4.1.2: Tier 2 Template Completion (14 entries)

| # | Template | Pattern | Notes |
|---|----------|---------|-------|
| 1 | Ray | `●→` | Open-ended directed line |
| 2 | Harmony | `∠120` | Distinguished angle |
| 3 | Efficiency | `∠60` | Already a primitive angle |
| 4 | Foundation (composite) | `□{●,●,●,●}` | Square with 4 corner points |
| 5 | Spiral | `~(recursive)` | Curve with decreasing radius |
| 6 | Wave | `~(periodic)` | Sinusoidal curve |
| 7 | ○{●} | Entity-in-concept | (Already T1, verify composition) |
| 8 | △{●} | Entity-in-structure | (Cross-ref T1) |
| 9 | □{●} | Entity-in-foundation | (Cross-ref T1) |
| 10 | Duality | `● ↔ ●` | Bidirectional connection |
| 11 | Trinity | `△{●\|●\|●}` | Three entities in triangle |
| 12 | Self-Nesting | `○{○{○}}` | Recursive containment |
| 13 | Quantification | `∠ × ●` | Angle-modifier applied to entity as quantifier |
| 14 | Reflection | `● ∠180 ●` | Mirror opposition |

- [ ] Implement 14 T2 templates in `templates.rs`
- [ ] Add snapshot tests for each
- [ ] Cross-reference with lexicon.md for geometric accuracy

### M4.1.3: Tier 3 Template Completion (4 entries)

| # | Template | Pattern |
|---|----------|---------|
| 1 | Entity → Concept | `● → ○` |
| 2 | Concept ↔ Concept | `○ ↔ ○` |
| 3 | Nested Process | `○{~}` |
| 4 | Full Glyph | Complex multi-tier composition |

- [ ] Implement 4 T3 templates
- [ ] Add snapshot tests

### M4.1.4: Template Composition Engine

Templates with anchor points can be composed into larger glyphs:

```rust
pub struct AnchorPoint {
    pub name: String,       // e.g., "N", "S", "E", "W", "center"
    pub position: (f64, f64),
    pub compatible_sorts: Vec<Sort>, // what can attach here
}

pub fn compose_templates(
    base: &Template,
    attachment: &Template,
    base_anchor: &str,
    attachment_anchor: &str,
) -> Result<Gir, UlError>;
```

- [ ] Add `AnchorPoint` struct to template module
- [ ] Define anchor points for all 42 templates (N/S/E/W + sort-typed attachment points)
- [ ] Implement `compose_templates()` — translates attachment template to align anchors, merges GIR graphs
- [ ] Add composition tests (at least 10 composition combinations)
- [ ] Wire template composition into web editor visual canvas (compose mode)

### M4.1.5: Template Search

- [ ] Implement `search_templates(query: &str) -> Vec<Template>` — searches by label, geometric pattern description, and sort signature
- [ ] Implement `match_subgraph(gir: &Gir) -> Vec<(Template, NodeMapping)>` — find template matches within a larger GIR
- [ ] Add search to web editor template palette (filter-as-you-type)
- [ ] Add search to VS Code insert-glyph command (update `CANONICAL_GLYPHS` to full 42)

### M4.1 Exit Criteria

- [ ] All 42 canonical lexicon templates render correctly (snapshot tests)
- [ ] Template composition produces valid GIR for at least 10 tested combinations
- [ ] Template search returns relevant results for natural language and pattern queries
- [ ] Web palette and VS Code both offer all 42 templates

---

## M4.2 — AI Integration

### M4.2a: LLM Interface (Core)

**Goal:** Model-agnostic LLM client that translates natural language ↔ UL-Script with validation-in-the-loop.

#### M4.2a.1: AI Client Library

Create `crates/ul-ai/` — a Rust library with TypeScript mirror for the web editor.

**Rust crate (`crates/ul-ai/Cargo.toml`):**
```toml
[dependencies]
ul-core = { path = "../ul-core" }
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
```

- [ ] `src/config.rs` — `AiConfig { provider, model, temperature, max_retries, api_key_env_var }`
- [ ] `src/providers/mod.rs` — provider trait: `async fn complete(system: &str, user: &str) -> Result<String>`
- [ ] `src/providers/openai.rs` — OpenAI Chat Completions API client
- [ ] `src/providers/anthropic.rs` — Anthropic Messages API client
- [ ] `src/providers/ollama.rs` — Local Ollama API client (for offline use)
- [ ] `src/prompt.rs` — System prompt builder: embeds condensed Σ_UL spec + 42 canonical templates + sort rules (<2000 tokens)
- [ ] `src/compose.rs` — `async fn compose(description: &str, config: &AiConfig) -> Result<ComposeResult>` — natural language → UL-Script with validation loop (max 3 retries)
- [ ] `src/explain.rs` — `async fn explain(script: &str, config: &AiConfig) -> Result<String>` — UL-Script → natural language explanation
- [ ] `src/suggest.rs` — `async fn suggest(partial: &str, config: &AiConfig) -> Result<Vec<String>>` — partial UL-Script → 3 completions (each validated)
- [ ] `src/lib.rs` — Public API: `compose()`, `explain()`, `suggest()`
- [ ] Integration tests with mock HTTP server (no real API keys needed for CI)

**Validation-in-the-loop implementation:**
```rust
pub async fn compose(desc: &str, config: &AiConfig) -> Result<ComposeResult> {
    let system = build_system_prompt();
    let mut user_msg = format!("Compose UL-Script for: {desc}");

    for attempt in 0..config.max_retries {
        let response = config.provider.complete(&system, &user_msg).await?;
        match ul_core::parser::parse(&response) {
            Ok(gir) => {
                let validation = ul_core::validator::validate(&gir, false);
                if validation.valid {
                    return Ok(ComposeResult { script: response, gir, attempts: attempt + 1 });
                }
                // Feed errors back
                let errors: Vec<String> = validation.errors.iter().map(|e| e.to_string()).collect();
                user_msg = format!(
                    "Your previous output had validation errors:\n{}\nPlease fix and output only valid UL-Script.",
                    errors.join("\n")
                );
            }
            Err(e) => {
                user_msg = format!("Parse error: {e}\nPlease output only valid UL-Script.");
            }
        }
    }
    Err(AiError::MaxRetriesExceeded)
}
```

#### M4.2a.2: TypeScript AI Client (Web Editor)

Mirror the Rust AI client in TypeScript for the web editor:

- [ ] `web/src/ai/config.ts` — AI configuration types + localStorage key management (keys never leave browser)
- [ ] `web/src/ai/providers.ts` — Provider abstraction (OpenAI, Anthropic, Ollama)
- [ ] `web/src/ai/compose.ts` — Compose with validation loop (uses WASM parse/validate)
- [ ] `web/src/ai/explain.ts` — Explain function
- [ ] `web/src/ai/suggest.ts` — Suggest function
- [ ] `web/src/ai/prompt.ts` — System prompt builder (shared with Rust via embedded text)

**Security constraints:**
- API keys stored in browser `localStorage`, never sent to UL Forge servers
- All API calls made directly from browser to provider (no proxy)
- Keys are not logged or included in error reports

#### M4.2a.3: CLI Integration

- [ ] Add `ul ai compose "description"` command to `crates/ul-cli/`
- [ ] Add `ul ai explain <file.ul>` command
- [ ] Add `ul ai suggest <file.ul>` command
- [ ] API key from environment variable (`OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, `OLLAMA_HOST`)

#### M4.2a.4: API Server Integration

- [ ] Add `POST /ai/compose` endpoint to `crates/ul-api/` — `{ description, config? }` → `{ script, gir, attempts }`
- [ ] Add `POST /ai/explain` endpoint — `{ script }` → `{ explanation }`
- [ ] Add `POST /ai/suggest` endpoint — `{ partial_script }` → `{ suggestions: string[] }`
- [ ] Server-side API keys from environment (for hosted deployments)

### M4.2a Exit Criteria

- [ ] `ul ai compose "balanced opposition"` produces valid UL-Script (with any configured provider)
- [ ] Validation loop correctly retries on parse/validation errors (test with mock provider returning invalid output)
- [ ] At least 2 providers work (OpenAI + Ollama recommended for testing)
- [ ] Integration tests pass with mock HTTP server

---

### M4.2b: AI Panel (Web Editor)

**Goal:** Inline AI assistant panel in the web editor.

- [ ] `web/src/components/AiPanel.tsx` — Collapsible side panel with:
  - Text input for "Compose" (natural language → UL-Script)
  - "Explain" button (selected text or full editor → explanation)
  - "Suggest" indicator (shows completions inline, accept with Tab)
  - Provider/model selector (dropdown)
  - API key input (stored in localStorage, masked display)
  - Retry count display and validation feedback
- [ ] `web/src/stores/aiStore.ts` — Zustand store for AI state (loading, results, config)
- [ ] Keyboard shortcut: `Ctrl+Shift+A` toggles AI panel
- [ ] Compose result: inserts UL-Script at cursor position in editor
- [ ] Explain result: shows in a tooltip/popover near selection
- [ ] Suggest result: shows ghost text completions (like Copilot)
- [ ] Settings persistence: provider config saved to localStorage

### M4.2b Exit Criteria

- [ ] User can type natural language, get valid UL-Script inserted into editor
- [ ] Explain shows meaningful descriptions of selected UL-Script
- [ ] Suggestions appear on pause after partial input
- [ ] API key management works without sending keys to any server

---

### M4.2c: GNN Interface (Stretch)

**Priority:** Medium. Only after M4.1 + M4.2a are solid.

- [ ] `bindings/python/gnn/` — PyTorch Geometric model for glyph embeddings
- [ ] `gir_to_pyg()` converter (GIR dict → PyG Data object)
- [ ] `GlyphEncoder` model (GIN architecture, 9→64→32 dimensions)
- [ ] Training script on 42 canonical glyphs + augmented compositions
- [ ] `ul_forge.similar(gir, k=5)` — find k nearest canonical glyphs by embedding
- [ ] `ul_forge.analogy(a, b, c)` — vector arithmetic analogy completion
- [ ] Export trained model weights as `.pt` file bundled with package

### M4.2d: Vision Interface (Stretch)

**Priority:** Low. Phase 4 stretch goal.

- [ ] `web/src/ai/vision.ts` — image upload → vision LLM → UL-Script
- [ ] Image preprocessing (crop, grayscale, normalize)
- [ ] Prompt template for vision models (GPT-4V, Claude Vision)
- [ ] Validation loop on vision output
- [ ] "Upload glyph" button in web editor toolbar

### M4.2e: Theorem Prover (Stretch)

**Priority:** Low. Research-grade feature.

- [ ] `crates/ul-prover/` — SMT encoding of sort constraints
- [ ] Z3 bindings (via `z3` crate) or SMT-LIB file generation
- [ ] `prove_sort_correctness(gir: &Gir) -> ProofResult`
- [ ] `prove_equivalence(a: &Gir, b: &Gir) -> ProofResult`
- [ ] CLI command: `ul prove <file.ul>`

---

## M4.3 — Collaboration

### M4.3a: Structural Diff & Merge

**Goal:** Graph-aware diff and three-way merge for GIR documents, with Git integration.

#### M4.3a.1: Diff Algorithm

- [ ] `crates/uws/src/diff.rs` — new module in ul-core
- [ ] `DiffOperation` enum: `AddNode`, `RemoveNode`, `ModifyNode`, `AddEdge`, `RemoveEdge`
- [ ] `pub fn diff(a: &Gir, b: &Gir) -> Vec<DiffOperation>` — structural diff
- [ ] Node matching: by ID first, then by `(type, sort, label)` triple fallback
- [ ] Diff serialization to JSON
- [ ] Tests: identical documents (empty diff), node additions, removals, modifications, edge changes

#### M4.3a.2: Three-Way Merge

- [ ] `crates/uws/src/merge.rs` — three-way merge
- [ ] `Conflict` struct: `{ type, node_id, field, base, ours, theirs }`
- [ ] `MergeResult { merged: Gir, conflicts: Vec<Conflict> }`
- [ ] `pub fn merge(base: &Gir, ours: &Gir, theirs: &Gir) -> MergeResult`
- [ ] Merge rules: accept non-conflicting changes, flag true conflicts
- [ ] Post-merge validation: run validator on merged GIR
- [ ] Tests: non-conflicting parallel edits, field conflicts, add-vs-remove conflicts

#### M4.3a.3: Git Integration

- [ ] Add `ul merge <base> <ours> <theirs>` CLI subcommand
- [ ] Add `ul diff <file-a> <file-b>` CLI subcommand
- [ ] Document `.gitattributes` config: `*.ul.json merge=ul-forge`
- [ ] Integration test: simulate Git merge scenario

### M4.3a Exit Criteria

- [ ] `ul diff` correctly identifies structural differences between two GIR documents
- [ ] `ul merge` produces correct merged output for non-conflicting edits
- [ ] Conflicts are correctly identified and serialized
- [ ] Git merge driver works end-to-end

---

### M4.3b: CRDT Real-Time Collaboration

**Goal:** Multiple users edit the same glyph simultaneously in the web editor.

#### M4.3b.1: Yjs Integration (Web Editor)

- [ ] Add `yjs` + `y-websocket` dependencies to `web/package.json`
- [ ] `web/src/collab/ydoc.ts` — Yjs document model: `Y.Map<NodeId, Node>` (nodes), `Y.Array<Edge>` (edges), `Y.Map` (metadata)
- [ ] `web/src/collab/gir-yjs.ts` — bidirectional sync: `girToYDoc()` and `yDocToGir()`
- [ ] `web/src/collab/awareness.ts` — cursor position sharing (user name + color + selection)
- [ ] `web/src/collab/provider.ts` — WebSocket provider connecting to collaboration server
- [ ] Integrate with `editorStore.ts`: Yjs observes changes → update GIR → re-render
- [ ] Offline support: IndexedDB persistence of Yjs document state

#### M4.3b.2: Collaboration Server

- [ ] `crates/ul-collab/` or `collab-server/` (Node.js) — WebSocket relay server
- [ ] Room management: create room per document ID, join/leave
- [ ] Yjs sync protocol: forward `sync_step_1`, `sync_step_2`, `update` messages
- [ ] Awareness protocol: forward cursor/selection state
- [ ] Document persistence: save Yjs state to disk (or database) on room idle
- [ ] Health endpoint for monitoring

**Decision: Node.js vs Rust for collab server.**
- Yjs is JavaScript-native. A Node.js server with `y-websocket` is 50 lines of code and production-proven.
- Rust alternative: use `y-crdt` (Yrs) crate — more complex, but keeps the stack homogeneous.
- **Recommendation:** Start with Node.js `y-websocket` for v1 (lowest risk). Migrate to Rust (Yrs) if server-side GIR processing is needed later.

#### M4.3b.3: Collaboration UI

- [ ] "Share" button in web editor toolbar → generates shareable room URL
- [ ] Collaborator cursors rendered on SVG canvas (colored markers with names)
- [ ] Collaborator list sidebar (who's in the room)
- [ ] Connection status indicator (connected / reconnecting / offline)
- [ ] Conflict resolution UI: when validator flags post-merge issues, highlight affected nodes

### M4.3b Exit Criteria

- [ ] Two browser tabs editing the same document see each other's changes in real time
- [ ] Offline edits sync correctly when reconnecting
- [ ] Cursor positions are visible to all collaborators
- [ ] Post-merge validation catches sort violations from concurrent edits

---

## M4.4 — Advanced Features

These are independent of each other and can be done in any order, in parallel with other tracks.

### M4.4a: TikZ Output Polish

- [ ] Complete TikZ generation in renderer (`OutputFormat::TikZ` path in `renderer/mod.rs`)
- [ ] Map all SVG elements to TikZ equivalents (circles, lines, arcs, text)
- [ ] LaTeX preamble generation (required packages: tikz, pgfplots)
- [ ] Test: render all 42 templates to TikZ, compile with `pdflatex`
- [ ] Add TikZ export to web editor, VS Code extension, and API server

### M4.4b: Batch Rendering

- [ ] `ul render --batch <directory>` — render all `.ul` files in a directory
- [ ] Parallel rendering (rayon for multi-core)
- [ ] Output: individual SVGs + optional combined SVG sheet (atlas)
- [ ] Progress reporting (file N of M)
- [ ] API endpoint: `POST /render/batch` — array of GIR documents → array of SVGs

### M4.4c: GIR Query Language

A mini query language for finding subgraph patterns in GIR documents:

```
# Find all enclosures containing exactly one point
MATCH (e:enclosure)-[:contains]->(p:point) WHERE degree(e, contains) == 1

# Find all directed connections between different sort entities
MATCH (a)-[:connects {directed: true}]->(b) WHERE a.sort != b.sort
```

- [ ] Design query grammar (Cypher-inspired, GIR-specific)
- [ ] Parser for query language (pest grammar)
- [ ] Query executor: `pub fn query(gir: &Gir, query: &str) -> Vec<NodeMapping>`
- [ ] CLI command: `ul query <file.ul> "MATCH ..."`
- [ ] Use in template search (M4.1.5) for pattern-based lookup

### M4.4d: Performance Optimization

- [ ] **Incremental parsing:** Track changed text regions, only re-parse affected AST subtrees
- [ ] **Render caching:** Hash GIR → cache SVG output, invalidate on GIR change
- [ ] **WASM size reduction:** `wasm-opt` pass, tree-shaking unused code
- [ ] **Layout caching:** Cache computed layouts for known template matches
- [ ] Benchmark suite: parse+validate+render for 1/10/100/1000 node GIRs
- [ ] Target: <50ms for 50-node GIR end-to-end in browser

---

## Implementation Order (Recommended)

```
Week-block 1:  M4.1.1 + M4.1.2       Tier 1+2 template completion (parallel with M4.4a TikZ)
Week-block 2:  M4.1.3 + M4.1.4       Tier 3 templates + composition engine
Week-block 3:  M4.1.5 + M4.2a.1–2    Template search + LLM client (Rust + TS)
Week-block 4:  M4.2a.3–4 + M4.2b     CLI/API AI integration + AI panel UI
Week-block 5:  M4.3a                  Diff + merge + Git integration
Week-block 6:  M4.3b                  CRDT + collaboration server + UI
Week-block 7:  M4.4b–d + polish      Batch render, GIR query, perf optimization
Stretch:       M4.2c–e               GNN, Vision, Theorem Prover (if time permits)
```

This ordering ensures:
1. Templates land first (everything else depends on them)
2. AI gets templates to reference in prompts
3. Collaboration comes after the editor is feature-complete with AI
4. Advanced features fill gaps at the end

---

## New Crates / Packages

| Crate/Package | Location | Purpose |
|---------------|----------|---------|
| `ul-ai` | `crates/ul-ai/` | LLM compose/explain/suggest + provider abstraction |
| `ul-collab` (or Node.js) | `collab-server/` | WebSocket relay for Yjs sync |
| `ul-prover` (stretch) | `crates/ul-prover/` | Z3/SMT sort verification |

**Changes to existing crates:**
- `ul-core`: add `diff.rs`, `merge.rs` modules; expand `templates.rs` to 42 entries
- `ul-cli`: add `ai`, `diff`, `merge`, `query` subcommands
- `ul-api`: add `/ai/*` endpoints
- `web/`: add `ai/` and `collab/` directories
- `bindings/python/`: add `gnn/` directory (stretch)

---

## Testing Strategy for Phase 4

| Component | Test Type | Target |
|-----------|-----------|--------|
| Templates (42) | Snapshot tests | 42 reference SVG comparisons |
| Template composition | Unit tests | 10+ composition combinations |
| LLM client | Mock-server integration tests | Compose, explain, suggest with mock provider |
| Validation loop | Unit tests | Invalid input → retry → success (and max-retry failure) |
| Diff | Unit + property tests | Identical docs, additions, removals, modifications |
| Merge | Unit tests | Non-conflicting merges, conflict detection, post-merge validation |
| CRDT | Integration tests | 2-client sync, offline-reconnect, concurrent edits |
| TikZ | Compile tests | All 42 templates compile with pdflatex |
| GIR query | Unit tests | Pattern matching on known GIR documents |
| Performance | Benchmarks | <50ms for 50-node GIR in WASM |

---

## Phase 4 Exit Criteria (Complete)

| # | Criterion | Validates |
|---|-----------|-----------|
| 1 | All 42 canonical glyphs have validated renderer templates with snapshot tests | M4.1 |
| 2 | Template composition produces valid GIR for 10+ tested combinations | M4.1.4 |
| 3 | `ul ai compose "balanced opposition"` produces valid UL-Script | M4.2a |
| 4 | Web editor AI panel: compose, explain, suggest all functional | M4.2b |
| 5 | `ul diff` correctly identifies structural differences | M4.3a |
| 6 | `ul merge` handles non-conflicting edits and flags conflicts | M4.3a |
| 7 | Two browser tabs can edit the same glyph with real-time sync | M4.3b |
| 8 | TikZ output compiles for all 42 templates | M4.4a |
| 9 | 126+ existing tests still pass (no regressions) | All |

**Stretch criteria (not blocking v1 ship):**
- GNN similarity search returns meaningful results for canonical glyphs
- Vision interface recognizes hand-drawn single-enclosure glyphs
- GIR query language can express template-search patterns

---

## File Deliverables Checklist

### Rust
- [ ] `crates/uws/src/renderer/templates.rs` — expanded from 6 to 42 templates
- [ ] `crates/uws/src/diff.rs` — structural diff module
- [ ] `crates/uws/src/merge.rs` — three-way merge module
- [ ] `crates/ul-ai/Cargo.toml` — AI client crate
- [ ] `crates/ul-ai/src/lib.rs` — compose, explain, suggest public API
- [ ] `crates/ul-ai/src/providers/{openai,anthropic,ollama}.rs` — provider implementations
- [ ] `crates/ul-ai/src/prompt.rs` — system prompt builder
- [ ] `crates/ul-ai/src/config.rs` — AI configuration
- [ ] `crates/ul-cli/src/main.rs` — updated with `ai`, `diff`, `merge` subcommands
- [ ] `crates/ul-api/src/routes.rs` — updated with `/ai/*` endpoints

### TypeScript (Web Editor)
- [ ] `web/src/ai/config.ts` — AI configuration
- [ ] `web/src/ai/providers.ts` — provider abstraction
- [ ] `web/src/ai/compose.ts` — compose with validation loop
- [ ] `web/src/ai/explain.ts` — explain function
- [ ] `web/src/ai/suggest.ts` — suggest function
- [ ] `web/src/ai/prompt.ts` — system prompt builder
- [ ] `web/src/components/AiPanel.tsx` — AI assistant UI
- [ ] `web/src/stores/aiStore.ts` — AI state management
- [ ] `web/src/collab/ydoc.ts` — Yjs document model
- [ ] `web/src/collab/gir-yjs.ts` — GIR ↔ Yjs bidirectional sync
- [ ] `web/src/collab/awareness.ts` — cursor sharing
- [ ] `web/src/collab/provider.ts` — WebSocket provider

### Collaboration Server
- [ ] `collab-server/package.json` — Node.js y-websocket server
- [ ] `collab-server/server.js` — room management + Yjs relay

### Tests
- [ ] `crates/uws/tests/template_snapshots/` — 42 reference SVGs
- [ ] `crates/uws/tests/diff_tests.rs` — diff unit tests
- [ ] `crates/uws/tests/merge_tests.rs` — merge unit tests
- [ ] `crates/ul-ai/tests/` — mock-provider integration tests
