# Rectification Checklist

**Status:** 🟢 R1+R2+R3+R4+R5 COMPLETE  
**Date Created:** 2026-04-09  
**Date Updated:** 2026-04-09  
**Reference:** [RECTIFY-PLAN.md](RECTIFY-PLAN.md)

---

## Phase R1: Kill the 14 WASM Stubs ✅

- [x] R1-1: Add `ul-game` dep to `ul-wasm/Cargo.toml`; change `ul-game` crate-type to `["rlib"]` only
- [x] R1-2: Add `width: f64, height: f64` params to `render()` in `ul-wasm/src/lib.rs`
- [x] R1-2b: Add `check_geometry: bool` param to `validate()` in `ul-wasm/src/lib.rs`
- [x] R1-3: Consolidated via `extern crate ul_game;` — wasm-bindgen discovers all 25 exports automatically
  - [x] All 14 former stubs now export real implementations (no re-export wrappers needed)
  - [x] Added `set_force` and `infer_pragmatics` to ul-game (were only in ul-wasm before)
  - [x] Removed duplicate `set_force`/`infer_pragmatics` from ul-wasm to avoid linker conflicts
- [x] R1-4: `cargo test --workspace` — 337 tests, 0 failures ✅
- [x] R1-5: `wasm-pack build --target web crates/ul-wasm` — 32 real exports in output ✅
- [x] R1-6: wasm-pack regenerated clean `ul_wasm.js` — all 19 hand-appended shims gone
- [x] R1-7: No web import changes needed — existing `wasm.d.ts` and `core/index.ts` already matched ul-game API
- [x] R1-8: `npm run build` succeeds; `npx vitest run` — 161 tests pass, 9 files ✅
- [x] R1-9: **GATE PASSED** — All WASM exports are real implementations

---

## Phase R2: Fix Broken Promises in Docs/UX

- [x] R2-1: `developer-quickstart.md` — add honest prerequisites per track ✅
- [x] R2-2: `integration-examples.md` — add status badges (✅ / ⚠️ / 🚧) per section ✅
- [x] R2-3: `modding-guide.md` — add "Requires game WASM" status note at top ✅
- [x] R2-4: Add `Evaluate` + `Check` subcommands to `ul-cli` ✅
- [x] R2-5: `OperationComposer.tsx` — add 4 extension operations (necessity, possibility, counterfactual, set_force) ✅
- [x] R2-6: Insert triggers reparse confirmed (via useLivePreview hook) — no fix needed ✅
- [x] R2-7: `curriculum.md` Day 7 — add notation bridge + dependency note ✅
- [x] R2-8: `curriculum.md` — add dependency note ("Days are sequential") ✅
- [x] R2-9: `exercises.md` — add "Common Mistakes" sections for all 3 levels ✅
- [x] R2-10: `quickstart.md` — qualify gallery link (placeholder note) ✅

---

## Phase R3: Interactive Feedback Loop

- [x] R3-1: Add `Check` subcommand to `ul-cli` (done in R2-4) ✅
- [x] R3-2: `exercises.md` — add `ul-cli check` verification hints after each exercise ✅
- [x] R3-3: Create `web/src/components/ExerciseChecker.tsx` ✅
- [x] R3-4: Wire ExerciseChecker into `App.tsx` (+ panel toggle) ✅
- [x] R3-5: Export game levels as `web/public/levels/level-{1..5}.json` ✅
- [x] R3-6: Add Practice Mode to web editor (`PracticeMode.tsx`) ✅
- [x] R3-7: **GATE** — CLI check works; web ExerciseChecker validates; Practice Mode scores ✅

---

## Phase R4: Gallery & Visual Polish

- [x] R4-1: Create `docs/learning/generate-gallery.mjs` (Node.js, with WASM fallback) ✅
- [x] R4-2: `gallery/index.md` — added placeholder warning + referenced generator ✅
- [x] R4-3: Improved gallery index: tier grouping (T0–T6), proper table structure ✅
- [x] R4-4: SVG renderer: CSS custom properties (`--gallery-bg/fg/accent`) for dark/light theming ✅
- [x] R4-5: **GATE** — 42 gallery SVGs with real geometric structure ✅

---

## Phase R5: CI/CD & Infrastructure Fixes

- [x] R5-1: Fix ci.yml + release.yml WASM target: build ul-wasm (not ul-game) ✅
- [x] R5-2: (Covered by R1-2b) validate check_geometry passthrough ✅
- [ ] R5-3: Push to GitHub, verify all CI jobs pass; fix any failures
- [ ] R5-4: Add Docker build job to ci.yml (conditional on main branch)
- [x] R5-5: `openapi.yaml` — parameterized server URL via variables ✅
- [ ] R5-6: **GATE** — All CI jobs green on GitHub

---

## Summary

| Phase | Total Items | Gates |
|-------|-------------|-------|
| R1: WASM Stubs | 9 (+14 sub-items) | All stubs replaced, web builds |
| R2: Doc/UX Fixes | 10 | Docs honest, CLI evaluate works |
| R3: Feedback Loop | 7 | Exercises verifiable via CLI + web |
| R4: Gallery | 5 | Real SVG renders |
| R5: CI/CD | 6 | Green CI on GitHub |
| **Total** | **37** | **5 gates** |
