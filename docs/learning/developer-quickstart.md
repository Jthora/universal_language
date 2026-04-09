# Build Your First UL App in 5 Minutes

> Parse, validate, render, and compose UL-Script — from Rust, Python, or the browser.

## Prerequisites

| Track | Requirements | Setup Time |
|-------|-------------|------------|
| **A: Rust** | Rust toolchain (`rustup`, `cargo`) | ~2 min |
| **B: Python** | Python 3.8+, Rust toolchain, `maturin` (`pip install maturin`) | ~5 min |
| **C: CLI** | Rust toolchain (builds from source) | ~3 min |
| **D: WASM** | Node.js 18+, Rust toolchain, `wasm-pack` (build from source — npm package not yet published) | ~5 min |

> All tracks require the `ul-forge` repository cloned locally. There is no published crate, PyPI package, or npm package yet — everything builds from source.

---

## Option A: Rust (native)

### 1. Add the dependency

```bash
cd your-project
# If using the ul-forge workspace directly:
cargo add --path /path/to/ul-forge/crates/ul-core
```

### 2. Parse UL-Script → GIR

```rust
use ul_core::parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gir = parser::parse("○{ ● → ● }")?;
    println!("{}", gir.to_json_pretty()?);
    Ok(())
}
```

### 3. Validate

```rust
let result = ul_core::validator::validate(&gir, false);
if result.valid {
    println!("Valid GIR with {} nodes", gir.nodes.len());
} else {
    for err in &result.errors {
        eprintln!("Error: {err}");
    }
}
```

### 4. Render to SVG

```rust
let options = ul_core::renderer::RenderOptions::default();
let svg = ul_core::renderer::render(&gir, &options)?;
std::fs::write("output.svg", &svg)?;
```

### 5. Compose operations

```rust
let entity = parser::parse("●")?;
let relation = parser::parse("→")?;
let object = parser::parse("●")?;

let assertion = ul_core::composer::predicate(&entity, &relation, &object)?;
println!("{}", parser::deparse(&assertion)?);
```

### 6. Round-trip

```rust
let script = parser::deparse(&gir)?;
let gir2 = parser::parse(&script)?;
assert_eq!(gir.nodes.len(), gir2.nodes.len());
```

---

## Option B: Python (via PyO3 bindings)

### 1. Build and install

```bash
cd ul-forge/bindings/python
pip install maturin
maturin develop
```

### 2. Use

```python
import ul_forge

# Parse
gir = ul_forge.parse("○{ ● → ● }")

# Validate
result = ul_forge.validate(gir)
assert result["valid"]

# Render
svg = ul_forge.render(gir)
with open("output.svg", "w") as f:
    f.write(svg)

# Compose
composed = ul_forge.compose("conjoin", ["●", "●"])
print(composed["ul_script"])

# Round-trip
text = ul_forge.deparse(gir)
gir2 = ul_forge.parse(text)

# Set force
forced = ul_forge.set_force(gir, "query")

# Pragmatic inference
inferences = ul_forge.infer_pragmatics(gir)
print(f"{inferences['count']} inferences found")
```

### 3. Jupyter notebook

```python
%load_ext ul_forge
```

```
%%ul
○{ ● "knowledge" → ○{ ∠ "structured" ● "truth" } }
```

This renders inline SVG directly in the notebook.

---

## Option C: CLI

```bash
cd ul-forge
cargo build --release -p ul-cli

# Parse
echo '● → ●' | ./target/release/ul parse --pretty

# Render
echo '● → ●' | ./target/release/ul parse | ./target/release/ul render > out.svg

# Validate
echo '● → ●' | ./target/release/ul parse | ./target/release/ul validate

# Compose
./target/release/ul compose conjoin "●" "●"

# Analyze
echo '● → ●' | ./target/release/ul parse | ./target/release/ul analyze

# Set force
./target/release/ul force query "○{ ● }"

# Pragmatic inference
echo '● → ●' | ./target/release/ul parse | ./target/release/ul pragmatics
```

---

## Option D: WASM (browser)

> **Note:** The `@ul-forge/core` npm package is not yet published. Build from source with `wasm-pack`, or use the pre-built `web/wasm-pkg/` directory.

```bash
# Build WASM from source
cd ul-forge
wasm-pack build --target web crates/ul-wasm --out-dir ../../web/wasm-pkg
```

```javascript
import init, { parse, render, validate, compose, analyze, set_force, infer_pragmatics } from '@ul-forge/core';

await init();

const gir = parse("● → ●");
const svg = render(JSON.stringify(gir), 256, 256);
document.getElementById("preview").innerHTML = svg;

// Compose
const composed = compose("conjoin", JSON.stringify(["●", "●"]));
```

---

## What's Next

- **13 operations reference:** `foundations/formal-foundations.md`
- **Writing guide:** `ul-core/writing-system/writers-companion.md`
- **MCP server (for AI agents):** `ul-forge/crates/ul-mcp/` — 10 JSON-RPC tools
- **Learn UL concepts:** `docs/learning/quickstart.md`
