# Project Structure

> Repository layout for UL Forge v1.

---

## Directory Tree

```
ul-forge/
├── Cargo.toml                    # Rust workspace root
├── Cargo.lock
├── package.json                  # npm workspace root
├── .github/
│   └── workflows/
│       ├── rust.yml              # Rust CI (build, test, clippy, fmt)
│       ├── wasm.yml              # WASM build + test
│       ├── web.yml               # Web editor CI (test, build, E2E)
│       └── release.yml           # Multi-target release workflow
│
├── crates/                       # Rust workspace members
│   ├── ul-core/                  # Core library (parser, validator, renderer)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs            # Public API re-exports
│   │       ├── types/
│   │       │   ├── mod.rs
│   │       │   ├── node.rs       # Node, NodeType, Sort
│   │       │   ├── edge.rs       # Edge, EdgeType
│   │       │   └── gir.rs        # Gir (top-level document)
│   │       ├── parser/
│   │       │   ├── mod.rs
│   │       │   ├── tokenizer.rs  # UL-Script → Token stream
│   │       │   ├── grammar.pest  # PEG grammar (pest)
│   │       │   ├── ast.rs        # AST node types
│   │       │   └── transform.rs  # AST → GIR
│   │       ├── validator/
│   │       │   ├── mod.rs
│   │       │   ├── schema.rs     # Layer 1: structural validation
│   │       │   ├── sorts.rs      # Layer 2: sort constraints
│   │       │   ├── invariants.rs # Layer 3: graph invariants
│   │       │   └── geometry.rs   # Layer 4: geometric satisfiability
│   │       ├── renderer/
│   │       │   ├── mod.rs
│   │       │   ├── layout.rs     # Template + constraint layout
│   │       │   ├── templates.rs  # Template library
│   │       │   ├── constraints.rs # Constraint solver
│   │       │   ├── svg.rs        # SVG generation
│   │       │   └── tikz.rs       # TikZ generation
│   │       └── error.rs          # Error types (thiserror)
│   │
│   ├── ul-wasm/                  # WASM bindings
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs            # wasm-bindgen exports: parse, render, validate
│   │
│   ├── ul-cli/                   # CLI binary
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs           # clap commands: parse, render, validate, convert, watch
│   │
│   └── ul-server/                # API server binary
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs           # Axum server setup
│           ├── routes.rs         # REST endpoints
│           └── ws.rs             # WebSocket handler
│
├── web/                          # Web editor (React + TypeScript)
│   ├── package.json
│   ├── vite.config.ts
│   ├── tsconfig.json
│   ├── index.html
│   └── src/
│       ├── main.tsx              # App entry point
│       ├── App.tsx               # Root component
│       ├── store/
│       │   └── gir-store.ts      # Zustand GIR store
│       ├── components/
│       │   ├── Editor.tsx         # Monaco editor pane
│       │   ├── Preview.tsx        # SVG preview pane
│       │   ├── Builder.tsx        # Visual glyph builder
│       │   ├── TemplatePalette.tsx # Template sidebar
│       │   └── ErrorPanel.tsx     # Validation error display
│       ├── wasm/
│       │   └── bridge.ts         # WASM module loader + typed wrappers
│       ├── lang/
│       │   └── ul-script.ts      # Monaco language definition
│       └── styles/
│           └── index.css         # Tailwind entry
│
├── bindings/
│   └── python/                   # PyO3 bindings
│       ├── pyproject.toml        # Maturin build config
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs            # PyO3 module: parse, render, validate
│
├── extensions/
│   └── vscode/                   # VS Code extension
│       ├── package.json          # Extension manifest
│       ├── tsconfig.json
│       └── src/
│           ├── extension.ts      # Activation, command registration
│           ├── language.ts       # Language client + diagnostics
│           └── preview.ts        # Webview preview panel
│
├── tests/                        # Integration tests
│   ├── fixtures/                 # Test UL-Script files and expected outputs
│   │   ├── canonical/            # All 42 lexicon glyphs as .ul files
│   │   ├── edge-cases/           # Tricky inputs
│   │   └── invalid/              # Files that should fail validation
│   └── integration/
│       ├── roundtrip_test.rs     # UL-Script → GIR → UL-Script
│       └── pipeline_test.rs      # UL-Script → GIR → SVG end-to-end
│
├── docs/                         # This documentation
│   └── ul-forge-v1/
│       └── ...
│
├── templates/                    # Glyph template data (JSON)
│   ├── tier1/                    # Geometrically forced (10 templates)
│   ├── tier2/                    # Structurally distinguished
│   └── tier3/                    # Conventional
│
└── scripts/                      # Development scripts
    ├── setup.sh                  # Install all dependencies
    ├── build-all.sh              # Build Rust + WASM + Web
    └── test-all.sh               # Run all test suites
```

---

## Workspace Configuration

### Rust Workspace (`Cargo.toml`)

```toml
[workspace]
members = [
    "crates/ul-core",
    "crates/ul-wasm",
    "crates/ul-cli",
    "crates/ul-server",
    "bindings/python",
]
resolver = "2"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
```

### npm workspace (`package.json`)

```json
{
  "private": true,
  "workspaces": ["web", "extensions/vscode"]
}
```

---

## Naming Conventions

| Item | Convention | Example |
|------|-----------|---------|
| Rust crates | `ul-*` (kebab-case) | `ul-core`, `ul-cli` |
| Rust modules | `snake_case` | `tokenizer.rs`, `ast_to_gir.rs` |
| TypeScript packages | `@ul-forge/*` | `@ul-forge/core` |
| React components | PascalCase | `Editor.tsx`, `Preview.tsx` |
| Test files | `*_test.rs` or `*.test.ts` | `parser_test.rs`, `bridge.test.ts` |
| Config files | Standard names | `Cargo.toml`, `tsconfig.json`, `vite.config.ts` |
