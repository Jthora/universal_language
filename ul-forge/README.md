# ul-forge

Working Rust implementation of the Universal Writing System notation: parser, validator, composer,
renderer, plus bindings and services.

**This is the part of the project that runs.** It is judged as software — does it parse, does it
render, do the tests pass — and not on any claim about the notation being necessary or unique. Such
claims were made in earlier documentation here and are retired; see `../claims.yaml` and
`../FAILURES.md`.

## Crates

| Crate | Purpose |
|---|---|
| `ul-core` | Parser, validator, composer, renderer. The library everything else builds on. |
| `ul-cli` | Command-line interface |
| `ul-wasm` | WASM bindings (`wasm-bindgen`) |
| `ul-api` | HTTP API server (Actix-web) |
| `ul-game` | Game engine — scoring, templates, puzzles |
| `ul-mcp` | Model Context Protocol server |
| `ul-transceiver` | Agent-to-agent message protocol |
| `bindings/python` | PyO3 bindings |

Web editor (React + Vite) lives in `web/`.

## Build and test

```bash
cargo test --workspace      # all tests must pass
cargo clippy --workspace    # no warnings
wasm-pack build --target web crates/ul-wasm
```

## Reading the source rather than the docs

**Counts of primitives, sorts, or operations are not properties of the notation** — they are
properties of a particular presentation, and the fixed counts asserted in older documentation are
retired. Read the source directly:

- `crates/ul-core/src/types/sort.rs` — the `Sort` enum
- `crates/ul-core/src/composer.rs` — the `Gir -> Gir` constructors
- `crates/ul-core/src/validator.rs` — what is actually checked (**graph** well-formedness: duplicate
  IDs, dangling references, edge-endpoint compatibility). **Semantic** invariants do not exist in
  code; see `GLOSSARY.md` for why that distinction matters.
- `schemas/gir.schema.json` — the IR schema

## Known debt

- `composer.rs::negate` marks negation with a self-loop edge, so double negation is not structurally
  identity, and no equivalence procedure exists to evaluate the semantic reading
  (`claims.yaml#NEG-INVOLUTION`).
- `Sort::Modifier` carries no discriminant, so distinct function shapes type-check identically.
- The IR is cyclic by design, which places general normalization in an undecidable regime. The
  decision to restrict the normalizing core to acyclic term graphs is recorded at
  `claims.yaml#IR-NORMALIZATION-STRATEGY` and is load-bearing for two separate results.

Prior reference documentation for this toolchain has been removed rather than corrected — it
asserted retired proofs as established fact. Git history has it.
