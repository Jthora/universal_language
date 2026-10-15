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

## Use it as an MCP server

The most useful artifact here for an agent, and until now the one with no setup documentation at
all. Ten tools over stdio; the banner goes to stderr so stdout stays clean JSON-RPC.

```bash
cargo build -p ul-mcp     # -> target/debug/ul-mcp
```

Register it (Claude Desktop / Claude Code `mcpServers` shape; adjust the path):

```json
{
  "mcpServers": {
    "ul-forge": {
      "command": "/absolute/path/to/ul-forge/target/debug/ul-mcp",
      "args": []
    }
  }
}
```

| tool | takes |
|---|---|
| `ul_parse` | `ul_script` — **returns `{"gir": …}`, not a bare GIR** |
| `ul_deparse` | `gir` — canonical UL-Script back out |
| `ul_validate` | `gir` — Σ_UL sort and structure constraints |
| `ul_render` | `gir` — SVG or TikZ |
| `ul_parse_and_render` | `ul_script` — both steps at once |
| `ul_compose` | `operation` + `operands` — arity-checked |
| `ul_analyze` | `gir` — which Σ_UL operations produced it |
| `ul_set_force` | `gir` + force (assert / query / direct / commit / express / declare) |
| `ul_infer_pragmatics` | `gir` |
| `ul_lexicon` | a search over the canonical lexicon entries |

**The wart, stated rather than hidden:** `ul_parse` wraps its result in `{"gir": …}` while every
other tool expects a bare GIR, so the tools do **not** chain without unwrapping that key. Fixing
the shape would break any existing caller, so it is documented here instead of changed quietly.
Also note that the lexicon's entries are a reference table, not a parseable vocabulary: most of
them are not in the grammar, and several that are parse to the same GIR as the bare primitive.

## Use it from JavaScript

`packages/core`, `packages/sdk`, `packages/transceiver` and `packages/web-components` are
publishable (`@ul-forge/*`, pre-1.0) but **are not published to npm**. Until they are, consume
them locally:

```bash
cd packages/core && npm install && npm run build && npm link
cd ../../your-project && npm link @ul-forge/core
```

The web editor is `web/` — `npm install && npm run dev`.

## The GIR contract

`schemas/gir.schema.json` is the published shape. `ul_gir` is **required** and is the constant
`ul_core::types::gir::GIR_VERSION`, currently pre-1.0. Both directions of agreement between the
schema and the implementation are asserted by `crates/ul-core/tests/schema_agreement_tests.rs`:
every key the schema requires is emitted, every top-level key the code emits is declared, and the
version is uniform across the parser and every composer operation.

That test exists because none of it used to be true — see `FAILURES.md` F-038 and the note below.

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

### Fixed since this list was written

- `schemas/gir.schema.json` never mentioned `ul_gir`, which the deserializer requires, so a
  schema-valid document was rejected by the implementation. Now required and asserted both ways.
- The GIR version was **path-dependent**: `Gir::new` wrote `"1.0"`, the parser `"0.1.0"`, and
  ul-game's fixtures `"0.2"`, so the version a consumer saw depended on which operation produced
  the document. One constant now, asserted uniform.
- `ul-cli compose` indexed its operand vector without an arity check, so all nine multi-operand
  operations aborted with `index out of bounds` instead of reporting the error. The MCP server had
  the check; the CLI did not.
- `assert{…}` parsed to sort `Entity`, so no string produced an `Assertion` and negation,
  conjunction and disjunction were unreachable from text (F-038).
