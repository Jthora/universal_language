# P3 — Generate GIR JSON Schema

> Derive JSON Schema from Rust types so machines can validate GIR documents without running Rust code.

**Blocks:** Type 1 (agent discovery), Type 4 (protocol validation)

---

## Motivation

AI agents and protocol validators need to inspect and validate GIR documents in any language — not just Rust. A JSON Schema derived directly from the source-of-truth Rust types guarantees the schema never drifts from the implementation.

---

## Approach: `schemars` Crate

The `schemars` crate derives JSON Schema (Draft 2020-12 compatible) from Rust types via `#[derive(JsonSchema)]`. Since all our types already derive `Serialize`/`Deserialize`, adding `JsonSchema` is mechanical.

### Dependencies

```toml
# ul-core/Cargo.toml
[dependencies]
schemars = { version = "0.8", features = ["derive"] }
```

### Types to Annotate

| Type | File | Notes |
|------|------|-------|
| `Gir` | types/gir.rs | Top-level document schema |
| `GirMetadata` | types/gir.rs | Optional metadata block |
| `Node` | types/node.rs | All node fields including optional ones |
| `NodeType` | types/node.rs | Enum: Point, Line, Angle, Curve, Enclosure |
| `EnclosureShape` | types/node.rs | Enum: Circle, Triangle, Square, etc. |
| `Edge` | types/edge.rs | Source, target, edge_type |
| `EdgeType` | types/edge.rs | Enum: Contains, ModifiedBy, etc. |
| `Sort` | types/sort.rs | Enum: Entity, Relation, Modifier, Assertion |
| `ValidationResult` | validator.rs | valid, errors, warnings |
| `RenderOptions` | renderer/mod.rs | format, width, height, embed_gir |
| `OutputFormat` | renderer/mod.rs | Enum: Svg, TikZ |

### Geometry types (after P2)

| Type | File |
|------|------|
| `PositionedGlyph` | renderer/layout.rs |
| `PositionedElement` | renderer/layout.rs |
| `Shape` | renderer/layout.rs |
| `Connection` | renderer/layout.rs |

---

## Schema Generation Module

```rust
// ul-core/src/schema.rs

use schemars::schema_for;
use std::collections::HashMap;

pub fn generate_schemas() -> HashMap<&'static str, String> {
    let mut schemas = HashMap::new();

    let gir_schema = schema_for!(crate::types::gir::Gir);
    schemas.insert("gir", serde_json::to_string_pretty(&gir_schema).unwrap());

    let validation_schema = schema_for!(crate::validator::ValidationResult);
    schemas.insert("validation-result", serde_json::to_string_pretty(&validation_schema).unwrap());

    let render_schema = schema_for!(crate::renderer::RenderOptions);
    schemas.insert("render-options", serde_json::to_string_pretty(&render_schema).unwrap());

    schemas
}
```

---

## CLI Integration

```bash
# Generate all schemas to the schemas/ directory
ul-forge schema generate --output schemas/

# Validate a GIR file against the schema (bonus)
ul-forge schema validate schemas/gir.schema.json path/to/document.gir.json
```

Implementation in `ul-cli`:

```rust
// New subcommand
Schema {
    #[command(subcommand)]
    action: SchemaAction,
}

enum SchemaAction {
    Generate {
        #[arg(short, long, default_value = "schemas")]
        output: PathBuf,
    },
    Validate {
        schema: PathBuf,
        document: PathBuf,
    },
}
```

---

## Output Files

```
ul-forge/schemas/
  gir.schema.json                  # ~200 lines, full Gir document schema
  validation-result.schema.json    # ~30 lines
  render-options.schema.json       # ~20 lines
```

### Example: gir.schema.json (abbreviated)

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "Gir",
  "description": "Graph Intermediate Representation — the core data structure of Universal Language",
  "type": "object",
  "required": ["nodes", "edges", "root"],
  "properties": {
    "nodes": {
      "type": "array",
      "items": { "$ref": "#/$defs/Node" }
    },
    "edges": {
      "type": "array",
      "items": { "$ref": "#/$defs/Edge" }
    },
    "root": { "type": "string" },
    "metadata": { "$ref": "#/$defs/GirMetadata" }
  },
  "$defs": {
    "Node": { ... },
    "Edge": { ... },
    "Sort": { "enum": ["Entity", "Relation", "Modifier", "Assertion"] },
    "NodeType": { "enum": ["Point", "Line", "Angle", "Curve", "Enclosure"] },
    "EdgeType": { "enum": ["Contains", "ModifiedBy", "Adjacent", "Intersects", "Connects", "References"] }
  }
}
```

---

## CI Integration

Add a CI step that:
1. Regenerates schemas from current code
2. Diffs against committed schemas
3. Fails if they diverge (forces developer to regenerate before committing)

```yaml
# .github/workflows/ci.yml
schema-check:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - run: cargo run -p ul-cli -- schema generate --output schemas-check/
    - run: diff -r schemas/ schemas-check/
```

---

## Consumers

| Consumer | Usage |
|----------|-------|
| Type 1: AI Agent Package | Bundled in npm package for machine discovery; served via `/schema/gir` endpoint |
| Type 4: Transceiver | Wire protocol schema references GIR schema via `$ref` |
| MCP Server | Tool parameter schemas derived from GIR schema |
| IDE Extensions | Autocomplete and validation for `.gir.json` files |
