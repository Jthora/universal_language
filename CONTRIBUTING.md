# Contributing — Universal Language

## Where the work is

This project is mid-rebuild. The foundational framing changed in August 2026 — UL is now approached
as an **emergent universality class** rather than a derivable signature — and a large amount of
superseded material has been deleted rather than archived in-repo. Git history is the archive.

**Before contributing research, read `RESEARCH-PROTOCOL.md`.** It is short and it documents the
specific way research goes wrong here: impossibility results carry scope conditions that get dropped
in transmission, and the check that catches this is mechanical rather than a matter of judgment.

**Before contributing anything, read `PRIMER.md` and `GLOSSARY.md`.** `claims.yaml` is the registry —
if a claim is not there, it is not a claim of this project.

### Highest-value contributions right now

| Kind | Why it is valuable |
|---|---|
| **Proofs** | Several open questions have mathematical objects and are provable rather than empirical. See `research/framework/provable-geometry.md` and `research/engineering/obstructions.md`. |
| **Obstruction theorems** | "What theorem says this is impossible?" is a required question here and has been under-asked. Scope conditions matter as much as the result. |
| **Implementation** | `ul-forge/` is a working Rust workspace. Code is checkable in a way prose is not. |
| **Falsifications** | A result that fails a prediction is a valid contribution. Report it; do not revise it until it passes. |

### On experiments

The earlier D2 experiment series and its infrastructure have been **removed**. That scoring was
withdrawn as evidence — it reached 100% across nine rounds in which each failure triggered a theory
change and a re-score, which is not validation. Any new evaluation must pre-register its criteria
and analysis before running. `FAILURES.md` is append-only for the same reason: a recorded failure is
data, a repaired one is nothing.

---

## Code of Conduct

Be rigorous, in both directions. Overclaiming and premature dismissal are both motivated reasoning,
and this repo has documented instances of each.

Every substantive claim carries a tier in `claims.yaml` — `VERIFIED`, `ARGUED`, `CONJECTURED`,
`DESIGN-CHOICE`, `RETIRED` — with the evidence each tier requires. Use them when you extend
anything.

---

## Contributing to UL Forge (Rust)

The Forge is an 8-crate Rust workspace in `ul-forge/`.

### Prerequisites

- Rust 1.70+ (`rustup update stable`)
- `wasm-pack` for WASM builds (`cargo install wasm-pack`)

### Structure

| Crate | Purpose |
|-------|---------|
| `ul-core` | Parser, validator, renderer, composer constructors + modal/performative/pragmatic |
| `ul-cli` | Command-line interface (8 subcommands) |
| `ul-wasm` | WASM bindings (wasm-bindgen exports) |
| `ul-api` | HTTP API server (Actix-web, 9 routes + websocket) |
| `ul-game` | Game engine — scoring, templates, puzzles |
| `ul-mcp` | Model Context Protocol server (10 tools) |
| `ul-transceiver` | Agent-to-agent message protocol |
| `bindings/python` | PyO3 bindings (8 functions) |

### Running Tests

```bash
cd ul-forge
cargo test --workspace        # 337 tests, all must pass
cargo clippy --workspace      # No warnings allowed
```

### Code Style

- Follow standard `rustfmt` formatting
- Prefer `Result<T, UlError>` over panics
- Public API functions need doc comments
- Match arms for `Operation`/`Sort` enums must be exhaustive

---

## Contributing to Web Editor (TypeScript)

The web editor is in `ul-forge/web/` — a React + Vite + Zustand app.

### Prerequisites

- Node.js 18+
- WASM module built (`cd ul-forge && wasm-pack build --target web crates/ul-wasm`)

### Development

```bash
cd ul-forge/web
npm install
npm run dev         # Starts dev server at localhost:5173
npm test            # Run test suite
```

### Components

| Component | Purpose |
|-----------|---------|
| `App.tsx` | Three-pane layout (palette + editor/canvas + preview) |
| `ScriptEditor.tsx` | UL-Script text editor |
| `VisualCanvas.tsx` | Visual glyph composition canvas |
| `SvgPreview.tsx` | Live SVG preview panel |
| `TemplatePalette.tsx` | 42 canonical templates + modal/force/pragmatic extensions |
| `StatusBar.tsx` | Parse status, force picker (φ), pragmatic inference |
| `ExportButtons.tsx` | SVG/TikZ/GIR export |

---

## Contributing to Python Bindings

The Python bindings are in `ul-forge/bindings/python/` using PyO3 + maturin.

### Prerequisites

- Python 3.10+
- `maturin` (`pip install maturin`)

### Building

```bash
cd ul-forge/bindings/python
maturin develop    # Build + install in current venv
```

### Available Functions

`parse`, `validate`, `render`, `deparse`, `compose_bind`, `set_force`, `infer_pragmatics`, `analyze_structure`

---

## Contributing to UL Core (Documentation)

UL Core documentation is in `uws/` — the 5 siblings + writing system.

### Style Guide

- Cross-references use relative paths: `see [Symbology](uws/symbology/symbol-map.md)`
- Composition expressions use backtick code: `` `predicate(e₁, r, e₂) → a` ``
- ASCII art diagrams for visual representations
- **Do not state inventory counts.** Primitive, sort, and operation counts are properties of a
  presentation, not of the notation, and prior fixed counts are retired. Point at source
  (`ul-forge/crates/ul-core/src/`) instead of restating a number.

### Documents to Know

- `formal-foundations.md` — Source of truth for all algebraic definitions
- `NAVIGATION.md` — Cross-document navigation guide
- `SYNTHESIS.md` — System overview and expansion paths
