# Technology Stack

> Detailed rationale for every technology choice in UL Forge v1.

---

## Core: Rust

| Aspect | Choice | Rationale |
|--------|--------|-----------|
| **Language** | Rust | Performance (no GC pauses), memory safety, compiles to WASM, strong type system for Σ_UL enforcement |
| **Parser generator** | `pest` (PEG) | Rust-native, PEG semantics match UL-Script's operator precedence, good error messages |
| **Serialization** | `serde` + `serde_json` | De facto Rust JSON library, derives for all GIR types |
| **CLI framework** | `clap` | Derive-based CLI args, shell completions, standard in Rust ecosystem |
| **Property testing** | `proptest` | Generate random GIR documents, verify invariants hold |
| **SVG generation** | Custom (string templates) | SVG output is simple enough that a library adds complexity without value |
| **Error handling** | `thiserror` | Derive `Error` for all error types, clean error messages |

### Why Not Other Languages?

- **C/C++**: No memory safety guarantees. UL Forge processes untrusted input (user-typed UL-Script, LLM-generated GIR) — memory safety is non-negotiable.
- **Go**: No WASM support for browser deployment. GC pauses affect rendering latency.
- **OCaml**: Strong type system, excellent for compilers. But: WASM story is immature (js_of_ocaml works but is a transpilation layer), smaller ecosystem for web tooling. Could be revisited if Rust WASM proves painful.
- **Python**: Too slow for rendering. Used only for bindings (PyO3) and experiment scripts.

---

## WASM Target

| Aspect | Choice | Rationale |
|--------|--------|-----------|
| **Build tool** | `wasm-pack` | Standard Rust→WASM toolchain, produces npm-compatible packages |
| **Target** | `wasm32-unknown-unknown` | Browser-compatible WASM, no WASI needed |
| **JS bindings** | `wasm-bindgen` | Type-safe Rust↔JS interop, handles string/JSON marshalling |
| **npm package** | `@ul-forge/core` | Consumed by web editor and VS Code extension |

> **Deep WASM Documentation:** See [wasm/README.md](../wasm/README.md) for the complete
> WASM architecture and build guide, [wasm/typescript-api-reference.md](../wasm/typescript-api-reference.md)
> for all 33 WASM exports and their TypeScript wrapper signatures, and
> [wasm/known-issues.md](../wasm/known-issues.md) for serialization gotchas (especially
> `serde-wasm-bindgen` `HashMap` → `Map` behavior and version string discrepancies).

---

## Web Frontend: TypeScript + React

| Aspect | Choice | Rationale |
|--------|--------|-----------|
| **Language** | TypeScript | Type safety, IDE support, matches WASM binding types |
| **Framework** | React 18+ | Component model, large ecosystem, Monaco/D3 integrations exist |
| **Build tool** | Vite | Fast dev server, ESM-native, WASM loader plugin available |
| **Editor** | Monaco Editor | VS Code's editor engine — syntax highlighting, diagnostics, autocomplete for free |
| **SVG rendering** | D3.js (selections, zoom) | Mature SVG manipulation, not used for layout (layout is in Rust) |
| **State** | Zustand | Lightweight (~1KB), TypeScript-native, no boilerplate |
| **Styling** | Tailwind CSS | Utility-first, no custom CSS files to maintain |
| **Testing** | Vitest + Playwright | Unit tests (vitest) + E2E browser tests (Playwright) |

### Why Not Other Frameworks?

- **Svelte/SvelteKit**: Excellent, but Monaco Editor integration is less mature.
- **Vue**: Viable but React's ecosystem for code editors (Monaco, CodeMirror) is deeper.
- **Vanilla JS**: Too much plumbing for a dual-pane editor with state management.

---

## Python Bindings: PyO3

| Aspect | Choice | Rationale |
|--------|--------|-----------|
| **Binding generator** | PyO3 | Direct Rust→Python FFI, no C intermediate layer |
| **Build system** | Maturin | Standard PyO3 build tool, produces wheels for pip |
| **Package** | `ul-forge` on PyPI | `pip install ul-forge` for Jupyter users |

---

## API Server: Axum

| Aspect | Choice | Rationale |
|--------|--------|-----------|
| **HTTP framework** | Axum | Tokio-based, Rust-native, good WebSocket support |
| **WebSocket** | `axum::extract::ws` | Built into Axum, no extra dependency |
| **Serialization** | `serde_json` | Same as core library |
| **Container** | Docker (Alpine-based) | Small image, reproducible deployment |

### Why Not Actix-Web, Warp, Rocket?

Axum is the current Tokio team recommendation. It shares the tower middleware ecosystem with other Tokio projects. No strong reason to prefer alternatives.

---

## CI/CD

| Aspect | Choice | Rationale |
|--------|--------|-----------|
| **CI** | GitHub Actions | Repository is on GitHub, native integration |
| **Rust CI** | `cargo test`, `cargo clippy`, `cargo fmt --check` | Standard Rust quality gates |
| **WASM CI** | `wasm-pack test --node` | Test WASM build in Node.js |
| **Web CI** | `npm test`, `npx playwright test` | Unit + E2E for web editor |
| **Release** | GitHub Releases + crates.io + npm + PyPI | Multi-target publishing |

---

## Version Control and Formatting

| Tool | Purpose |
|------|---------|
| `rustfmt` | Rust code formatting (enforced in CI) |
| `clippy` | Rust linting (enforced in CI) |
| `prettier` | TypeScript/JSON/Markdown formatting |
| `eslint` | TypeScript linting |
| Conventional Commits | Commit message format for changelogs |
