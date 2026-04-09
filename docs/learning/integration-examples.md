# Integration Examples

> Copy-pasteable examples for React, Python, and CLI integration with UL Forge.

| Integration | Status | Notes |
|------------|--------|-------|
| React/WASM | ✅ Working | Build from source; npm package not yet published |
| Python | ⚠️ Requires build | Needs Rust toolchain + `maturin develop` |
| CLI | ✅ Working | Build from source with `cargo build` |
| MCP | ✅ Working | JSON-RPC tools via stdio transport |
| HTTP API | 🚧 Dev only | Not deployed; run locally with `cargo run -p ul-api` |

---

## 1. React / TypeScript (WASM)

```tsx
// App.tsx — Parse UL-Script and render inline SVG
import { useEffect, useState } from "react";
import { initialize, parse, validate, render } from "@ul-forge/core";

export default function UlViewer() {
  const [svg, setSvg] = useState("");
  const [error, setError] = useState("");
  const [input, setInput] = useState("○{● → ●}");

  useEffect(() => {
    initialize(); // Load WASM module once
  }, []);

  const handleRender = () => {
    try {
      const gir = parse(input);
      const result = validate(gir);
      if (!result.valid) {
        setError(result.errors.join(", "));
        return;
      }
      setSvg(render(gir, 256, 256));
      setError("");
    } catch (e) {
      setError(String(e));
    }
  };

  return (
    <div>
      <input value={input} onChange={(e) => setInput(e.target.value)} />
      <button onClick={handleRender}>Render</button>
      {error && <p style={{ color: "red" }}>{error}</p>}
      <div dangerouslySetInnerHTML={{ __html: svg }} />
    </div>
  );
}
```

### With Modal and Force Extensions

```tsx
import { parse, setForce, inferPragmatics } from "@ul-forge/core";

// Set performative force on an assertion
const gir = parse("○{● → ●}");
const question = setForce(gir, "query");    // → query{○{● → ●}}
const command = setForce(gir, "direct");    // → direct{○{● → ●}}

// Run pragmatic inference
const inferences = inferPragmatics(gir);
// → [{rule: "SI-1", surface: {...}, intended: {...}}, ...]
```

---

## 2. Python

```python
# ul_demo.py — Parse, validate, render, and analyze
import ul_forge

# Parse UL-Script to GIR
gir = ul_forge.parse("○{● → ●}")
print(f"Nodes: {len(gir['nodes'])}, Edges: {len(gir['edges'])}")

# Validate
result = ul_forge.validate(gir)
print(f"Valid: {result['valid']}")

# Render to SVG
svg = ul_forge.render(gir)
with open("output.svg", "w") as f:
    f.write(svg)
print("Saved output.svg")

# Round-trip: GIR → UL-Script
script = ul_forge.deparse(gir)
print(f"Round-trip: {script}")

# Analyze structure
report = ul_forge.analyze_structure(gir)
print(f"Operations: {report['detected_operations']}")
print(f"Symmetry: {report['symmetry_group']}")

# Set performative force
question = ul_forge.set_force(gir, "query")

# Run pragmatic inference
pragmatics = ul_forge.infer_pragmatics(gir)
print(f"Inferences: {pragmatics['count']}")
```

### Display in Jupyter

```python
from IPython.display import SVG, display
import ul_forge

gir = ul_forge.parse("[]{○{● → ●}}")  # Necessity
svg = ul_forge.render(gir)
display(SVG(data=svg))
```

---

## 3. CLI Pipeline

```bash
# Parse → Validate → Render (pipe-based)
echo '○{● → ●}' | ul-cli parse | ul-cli validate | ul-cli render > output.svg

# Parse to GIR JSON
ul-cli parse '○{● → ●}'

# Validate a GIR file
ul-cli validate --gir graph.json

# Render to SVG (256×256)
ul-cli render --gir graph.json --width 256 --height 256 > glyph.svg

# Render to TikZ (for LaTeX)
ul-cli render --gir graph.json --format tikz > glyph.tex

# Compose two operations
ul-cli compose bind '{"entity_json": "...", "assertion_json": "..."}'

# Set performative force
ul-cli force query '○{● → ●}'

# Run pragmatic inference
ul-cli pragmatics '○{● | ●}'

# Analyze structure
ul-cli analyze '○{● → ●}'
# → {"detected_operations": ["predicate", "embed"], "symmetry_group": "so2", ...}
```

### Batch Processing

```bash
# Process multiple UL-Script files
for f in *.ul; do
  ul-cli parse < "$f" | ul-cli render > "${f%.ul}.svg"
done
```

---

## 4. MCP (Model Context Protocol)

For AI agents using MCP tools:

```json
{
  "tool": "ul_parse",
  "arguments": { "input": "○{● → ●}" }
}
```

Available tools: `ul_parse`, `ul_validate`, `ul_render`, `ul_deparse`, `ul_compose`, `ul_analyze`, `ul_lexicon`, `ul_hints`, `ul_set_force`, `ul_infer_pragmatics`

---

## 5. HTTP API

```bash
# Parse
curl -X POST http://localhost:8080/parse \
  -H "Content-Type: application/json" \
  -d '{"input": "○{● → ●}"}'

# Validate
curl -X POST http://localhost:8080/validate \
  -H "Content-Type: application/json" \
  -d '{"gir": {...}}'

# Render
curl -X POST http://localhost:8080/render \
  -H "Content-Type: application/json" \
  -d '{"gir": {...}, "width": 256, "height": 256}'

# Set force
curl -X POST http://localhost:8080/force \
  -H "Content-Type: application/json" \
  -d '{"gir": {...}, "force": "query"}'

# Pragmatic inference
curl -X POST http://localhost:8080/pragmatics \
  -H "Content-Type: application/json" \
  -d '{"gir": {...}}'
```
