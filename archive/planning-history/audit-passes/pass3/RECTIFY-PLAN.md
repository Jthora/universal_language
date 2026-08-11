# Rectification Plan — UL Forge Learning & App-Readiness

**Status:** PLANNED  
**Date Created:** 2026-04-09  
**Scope:** Fix all gaps identified in the learning/app-readiness critique  
**Reference:** Critique output from Phase 3/4 review session

---

## Overview

The critique identified issues in 5 categories. This plan addresses each with concrete, ordered work items grouped into 5 rectification phases (R1–R5), ordered by impact and dependency.

| Phase | Focus | Items | Impact |
|-------|-------|-------|--------|
| R1 | WASM: Kill the 14 stubs | 8 | **Critical** — unlocks game, teaching, lexicon, layout |
| R2 | Fix broken promises in docs/UX | 10 | **High** — honest docs, working examples |
| R3 | Interactive feedback loop | 7 | **High** — exercises become verifiable |
| R4 | Gallery & visual polish | 5 | **Medium** — placeholder SVGs → real renders |
| R5 | CI/CD & infrastructure fixes | 6 | **Medium** — pipeline correctness |

Total: **36 items**

---

## Phase R1: Kill the 14 WASM Stubs (Critical Path)

### Context
`ul-game` already compiles to WASM (zero dependency blockers, all `#[wasm_bindgen]` annotations in place, 25 exports). The 14 JS shims exist because `ul-wasm` only depends on `ul-core`, not `ul-game`. The fix is straightforward: add `ul-game` as a dependency and re-export its functions.

### R1-1: Add `ul-game` dependency to `ul-wasm`

**File:** `crates/ul-wasm/Cargo.toml`

Add:
```toml
ul-game = { path = "../ul-game" }
```

### R1-2: Add width/height params to `render()` in ul-wasm

**File:** `crates/ul-wasm/src/lib.rs`

Change render from:
```rust
pub fn render(gir_json: &str) -> Result<String, JsError> {
    // ...
    let options = ul_core::renderer::RenderOptions::default();
```
To:
```rust
pub fn render(gir_json: &str, width: f64, height: f64) -> Result<String, JsError> {
    // ...
    let options = ul_core::renderer::RenderOptions::default()
        .with_width(width)
        .with_height(height);
```

Also add `check_geometry` passthrough to `validate()`.

### R1-3: Re-export ul-game functions from ul-wasm

**File:** `crates/ul-wasm/src/lib.rs`

Add wasm_bindgen exports that delegate to ul-game:
- `create_context(config_json)` → `ul_game::create_context(config_json)`
- `evaluate(ctx_id, gir_json)` → `ul_game::evaluate(ctx_id, gir_json)`
- `score_composition(ctx_id, gir, target)` → `ul_game::score_composition(...)`
- `evaluate_jane_attempt(ctx_id, attempt, expected)` → `ul_game::evaluate_jane_attempt(...)`
- `validate_sequence(ctx_id, glyphs)` → `ul_game::validate_sequence(...)`
- `get_animation_sequence(gir, w, h)` → `ul_game::get_animation_sequence(...)`
- `layout(gir, w, h)` → `ul_game::layout(...)`
- `load_custom_definitions(ctx_id, rules)` → `ul_game::load_custom_definitions(...)`
- `compare_gir(a, b, level)` → `ul_game::compare_gir(...)`
- `get_hints(attempt, target)` → `ul_game::get_hints(...)`
- `analyze_hints(gir)` → `ul_game::analyze_hints(...)`
- `get_next_puzzle(proficiency)` → `ul_game::get_next_puzzle(...)`
- `query_lexicon(query)` → `ul_game::query_lexicon(...)`
- `lookup_lexicon_entry(name)` → `ul_game::lookup_lexicon_entry(...)`
- `render_glyph_preview(gir)` → render at 64×64

**Approach decision:** Rather than duplicating `#[wasm_bindgen]` (which would cause symbol conflicts since ul-game already has them), the cleanest approach is one of:
- (a) **Re-export via `pub use`** — re-export ul-game's lib functions from ul-wasm
- (b) **Build ul-game directly** via wasm-pack and import both modules in JS
- (c) **Consolidate** — move all WASM exports into ul-wasm, remove cdylib from ul-game

Recommended: **(c) Consolidate.** Change ul-game to `crate-type = ["rlib"]` only (library, not cdylib). Then ul-wasm becomes the single WASM entry point, calling into both ul-core and ul-game. This avoids duplicate `wasm_bindgen` symbol conflicts.

### R1-4: Remove JS compatibility shims

**File:** `web/wasm-pkg/ul_wasm.js` (or rebuild from wasm-pack)

After R1-2 and R1-3, the 19 JS shims are no longer needed. The real WASM exports now have the correct signatures. Steps:
1. Run `wasm-pack build --target web crates/ul-wasm`
2. The generated `ul_wasm.js` will have all exports natively
3. Delete the hand-appended shim block
4. Update `web/src/core/index.ts` imports to match new export names

### R1-5: Update `core/index.ts` wrapper imports

**File:** `web/src/core/index.ts`

Update the import block to use the new native WASM exports. Key changes:
- `parseUlScript` → direct import if renamed, or keep if ul-wasm exports as `parse_ul_script`
- `renderSvg(gir, w, h)` now passes real width/height
- `validateGir(gir, check_geometry)` now passes real flag
- All 14 game/teaching/lexicon functions import from real WASM, not stubs

### R1-6: Rebuild WASM binary

```bash
cd ul-forge
wasm-pack build --target web crates/ul-wasm
```

Verify: binary should be ~600-800 KB (larger than current 492 KB due to ul-game inclusion).

### R1-7: Update web editor to use real layout

With `layout()` now returning real positioned geometry (not linear row), the `VisualCanvas` component should render actual force-directed positions. Verify the existing rendering code handles `PositionedGlyph` correctly.

### R1-8: GATE — All 14 stubs replaced with real WASM calls

Verification:
- `createContext` returns a valid context that stores rules
- `evaluate` returns real rule-match scores (not always 1.0)
- `layout` positions nodes in 2D (not all at y=50)
- `queryLexicon("existence")` returns the lexicon entry
- `getHints` returns contextual hints
- Web build succeeds: `cd web && npm run build`

---

## Phase R2: Fix Broken Promises in Docs/UX (High Impact)

### R2-1: Developer quickstart — honest prerequisites

**File:** `docs/learning/developer-quickstart.md`

- Add "Prerequisites" section at top listing actual requirements per track
- Python track: clearly state `maturin develop` requires Rust toolchain
- WASM track: clearly state package is not yet on npm, show build-from-source instructions
- Add approximate setup times (Rust: 2 min if toolchain exists, Python: 5 min with Rust, WASM: 3 min)

### R2-2: Integration examples — mark which APIs are real vs pending

**File:** `docs/learning/integration-examples.md`

- Add status badges to each example section: ✅ Working | ⚠️ Requires build from source | 🚧 Pending publish
- React example: note width/height params require R1-2 fix
- CLI `evaluate` example: either implement the subcommand (R2-4) or remove the example

### R2-3: Modding guide — note loadCustomDefinitions status

**File:** `docs/learning/modding-guide.md`

After R1 completes, this becomes real. Add a "Status" note at top:
> Requires UL Forge WASM with game module. See developer-quickstart for build instructions.

### R2-4: Add CLI `evaluate` subcommand

**File:** `crates/ul-cli/src/main.rs`

The game-levels.md and modding-guide.md reference `ul-cli evaluate --rules ... --gir ...`. Add:
```rust
Evaluate {
    #[arg(long)]
    rules: String,     // path to rules JSON
    #[arg(long)]
    gir: String,       // path to GIR JSON
}
```
Implementation: load rules, create game context, evaluate GIR, print score + matched/violated rules.

### R2-5: OperationComposer — add 5 extension operations

**File:** `web/src/components/OperationComposer.tsx`

Add the 5 missing operations to the OPERATIONS array:
- `necessity`: arity 1, signature `a → a`, placeholder: "Assertion"
- `possibility`: arity 1, `a → a`
- `counterfactual`: arity 2, `a × a → a`, placeholders: "Antecedent", "Consequent"
- `set_force`: arity 2, `force × a → a`, placeholders: "Force (name)", "Assertion"
- `infer_pragmatic`: arity 1, `a → inferences`

### R2-6: OperationComposer — auto-trigger reparse after insert

**File:** `web/src/components/OperationComposer.tsx`

After `setSource(result)`, the editor store should automatically trigger reparse. Verify the store's `setSource` triggers the parse→validate→render pipeline. If not, add a `useEffect` or explicit call.

### R2-7: Curriculum Day 7 — bridge to thesaurus notation

**File:** `docs/learning/curriculum.md`

Add a "Notation Bridge" subsection to Day 7 explaining how thesaurus.md's organizational scheme maps to the syntax they've learned. Include a small worked example of navigating from one meaning to a synonym using the thesaurus paths.

### R2-8: Curriculum — add concept dependency note

**File:** `docs/learning/curriculum.md`

Add a brief dependency note at the top of the Overview section:
> Days are sequential: each builds on the previous. Day 4 requires Days 2+3. Day 8 requires Days 1–7.

### R2-9: Exercises — add common mistake examples

**File:** `docs/learning/exercises.md`

Add a "Common Mistakes" section after each Level's exercises (before the answer key):
- Level 1: confusing Modifier with Entity (e.g., "big" is not an entity)
- Level 2: forgetting the assertion frame (`○{...}`)
- Level 3: binding scope errors (variable used outside its bind)

### R2-10: Quickstart — fix gallery cross-reference

**File:** `docs/learning/quickstart.md`

After R4 completes (real SVGs), the gallery link is fine. Until then, add "(visual examples)" qualifier or note that gallery is under construction.

---

## Phase R3: Interactive Feedback Loop (High Impact)

### R3-1: Add `ul-cli check` subcommand for exercise verification

**File:** `crates/ul-cli/src/main.rs`

Add a learner-friendly command:
```
ul-cli check "○{ ● \"fire\" → ● \"hot\" }" --expect "predicate"
```
- Parses input
- Validates
- Detects operations
- Prints human-readable feedback: ✓ Valid / ✗ Expected `predicate`, found `modify_entity`
- Returns exit code 0 (correct) or 1 (incorrect)

### R3-2: Add exercise verification hints to exercises.md

**File:** `docs/learning/exercises.md`

After each exercise, add a "Check your answer" callout:
```
💻 Verify: `ul-cli check "your-answer" --expect predicate`
```

### R3-3: Add WASM-based exercise checker to web editor

**File:** `web/src/components/ExerciseChecker.tsx` (new)

Simple component:
- Text input for UL-Script
- Dropdown to select exercise (1.1–3.3)
- "Check" button → parse + validate + detect operations + compare to expected
- Green/red feedback with hints

### R3-4: Wire ExerciseChecker into App.tsx

**File:** `web/src/App.tsx`

Add as a collapsible panel (similar to OperationComposer).

### R3-5: Add game-levels.json as loadable asset in web editor

**File:** `web/public/levels/` (new directory)

Export the 5 game level JSONs as static assets that the web editor can fetch and use with `createContext` + `evaluate` (once R1 stubs are killed).

### R3-6: Add "Practice Mode" toggle to web editor

**File:** `web/src/components/App.tsx` or new `PracticeMode.tsx`

Toggle that loads a game level, shows the challenge description, and evaluates the user's composition against the rules in real time using the now-real `evaluate()` WASM function.

### R3-7: GATE — Feedback loop works end-to-end

Verify:
- `ul-cli check "○{● → ●}" --expect predicate` → ✓
- `ul-cli check "● → ●" --expect predicate` → ✗ (missing frame)
- Web ExerciseChecker validates Level 1 exercises correctly
- Practice Mode loads Level 1 rules and scores a composition

---

## Phase R4: Gallery & Visual Polish (Medium Impact)

### R4-1: Generate real SVGs via WASM build script

**File:** `docs/learning/generate-gallery.py` (rewrite)

Instead of requiring Python bindings (`maturin develop`), use a Node.js script that:
1. Loads the built WASM module (`wasm-pkg/ul_wasm.js`)
2. For each of 42 lexicon entries, calls `parse()` then `render()`
3. Writes SVG files to `ul-core/lexicon/gallery/`

New file: `docs/learning/generate-gallery.mjs`

This works because the WASM build is always available after `wasm-pack build`.

### R4-2: Add "placeholder" label to current gallery SVGs

**File:** `ul-core/lexicon/gallery/index.md`

Until R4-1 is complete, add a clear note:
> ⚠️ Gallery images are placeholders. Run `node docs/learning/generate-gallery.mjs` after building WASM to generate real renders.

### R4-3: Improve gallery index layout

**File:** `ul-core/lexicon/gallery/index.md`

Add tier grouping, larger thumbnails, and link each entry to its lexicon definition.

### R4-4: Add dark/light theme support to SVG output

**File:** `crates/ul-core/src/renderer/svg.rs`

Embed CSS variables in SVG output so renders look good in both dark and light contexts. Currently SVGs may have hardcoded colors.

### R4-5: GATE — Gallery shows real rendered glyphs

Verify: at least 10 of 42 SVGs show recognizable geometric structure (not text-in-a-box).

---

## Phase R5: CI/CD & Infrastructure Fixes (Medium Impact)

### R5-1: Fix release WASM target mismatch

**File:** `.github/workflows/release.yml`

The release pipeline builds WASM with `--target bundler` but the web editor expects `--target web`. Fix:
- Build WASM twice: `--target web` for web editor, `--target bundler` for npm package
- Or: standardize on `--target web` (works for both if the consumer uses dynamic import)

### R5-2: Add validate_gir check_geometry passthrough

Covered by R1-2 (add width/height + check_geometry params to ul-wasm).

### R5-3: Push and verify CI pipeline

**Action:** Push to GitHub, verify all 4 CI jobs pass. Fix any issues:
- wasm-pack install via `curl | sh` → may need `cargo install wasm-pack` as fallback
- npm ci in web → requires package-lock.json committed
- VS Code extension npm ci → requires package-lock.json committed

### R5-4: Add Dockerfile to CI

**File:** `.github/workflows/ci.yml`

Add optional Docker build job (only on main branch push):
```yaml
docker:
  name: Docker Build
  if: github.ref == 'refs/heads/main'
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - run: docker build -t ul-forge:ci ul-forge/
```

### R5-5: Parameterize OpenAPI server URL

**File:** `ul-forge/schemas/openapi.yaml`

Change from hardcoded localhost to:
```yaml
servers:
  - url: http://localhost:8080
    description: Local development
  - url: "{baseUrl}"
    description: Custom deployment
    variables:
      baseUrl:
        default: http://localhost:8080
```

### R5-6: GATE — CI passes on GitHub

Verify: push triggers ci.yml, all jobs green.

---

## Dependency Graph

```
R1 (WASM stubs) ─────────┬───→ R3 (feedback loop)
                          │
                          ├───→ R4-1 (real SVG gallery)
                          │
R2 (docs fixes) ──────── │ ──→ R3-2 (exercise hints)
                          │
R5 (CI/CD fixes) ────────┘
```

**Critical path:** R1 → R3 (the interactive feedback loop depends on real WASM functions).

R2 and R5 can be done in parallel with R1.  
R4 depends on R1 (needs working render in WASM).  
R3 depends on R1 (createContext, evaluate) and partially on R2 (CLI check subcommand).

---

## Effort Estimates

| Phase | Estimated Subtasks | Complexity |
|-------|--------------------|------------|
| R1 | ~30 | High — Rust/WASM integration |
| R2 | ~20 | Low–Medium — doc edits + 1 CLI subcommand + 2 component fixes |
| R3 | ~25 | Medium — new CLI command + new web component + level assets |
| R4 | ~15 | Medium — Node.js gallery script + SVG theming |
| R5 | ~10 | Low — YAML edits + push verification |
| **Total** | **~100** | |

---

## Priority Order for Execution

1. **R1-1 through R1-6** — unblock everything by killing the stubs
2. **R1-7, R1-8** — verify the integration works
3. **R2-1, R2-2, R2-4** — fix the most misleading docs + add CLI evaluate
4. **R2-5, R2-6** — OperationComposer fixes (quick wins)
5. **R3-1, R3-2** — CLI check command + exercise hints
6. **R4-1, R4-2** — real SVG gallery
7. **R5-1, R5-3** — CI fixes and first push
8. **R2-7, R2-8, R2-9, R2-10** — curriculum/exercise polish
9. **R3-3 through R3-6** — web exercise checker + practice mode
10. **R4-3, R4-4, R4-5** — gallery polish
11. **R5-4, R5-5, R5-6** — infrastructure polish

---

## Success Criteria

When rectification is complete:

1. **Zero WASM stubs** — all 25+ WASM exports are real
2. **Working feedback loop** — a learner can verify their exercise answers via CLI or web
3. **Honest docs** — every code example either runs as-is or has clear build-from-source instructions
4. **Real gallery** — SVGs show actual geometric structure
5. **Green CI** — all GitHub Actions jobs pass
6. **Game works in browser** — Level 1–5 rules load and score compositions correctly
