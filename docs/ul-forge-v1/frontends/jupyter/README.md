# Jupyter Integration

> UL Forge magic commands for Jupyter notebooks.

---

## Overview

The Jupyter integration allows researchers to compose, render, and analyze UL glyphs inline in notebooks. This is especially useful for:
- **Linguistics research** — compose UL expressions alongside natural language analysis
- **Mathematical exploration** — UL operations next to formal proofs
- **Teaching** — step-by-step glyph construction in notebook cells

---

## Cell Magic

### `%%ul`

Render UL-Script as inline SVG in a notebook cell:

```python
%%ul
◯(•) → △(•)
```

**Output:** Rendered SVG displayed as cell output.

### `%%ul_gir`

Show the GIR JSON for a UL-Script expression:

```python
%%ul_gir
◯(•) → △(•)
```

**Output:** Pretty-printed GIR JSON.

---

## Python API

```python
import ul_forge

# Parse UL-Script to GIR
gir = ul_forge.parse("◯(•) → △(•)")

# Render GIR to SVG string
svg = ul_forge.render(gir)

# Validate GIR
result = ul_forge.validate(gir)
print(result.valid)       # True
print(result.errors)      # []

# Display inline (IPython)
from IPython.display import SVG, display
display(SVG(data=svg))
```

---

## Implementation

The Python package wraps the Rust core via PyO3:

```
ul-forge-core (Rust)
    │
    ├── WASM → JavaScript (web editor, VS Code)
    └── PyO3 → Python (Jupyter, scripts)
```

The PyO3 build produces a native Python extension (`.so` / `.pyd`) that calls directly into the Rust library. No subprocess spawning or HTTP calls.

---

## Installation

```bash
pip install ul-forge
```

Or build from source:
```bash
cd bindings/python
pip install maturin
maturin develop    # build and install in current venv
```

---

## Jupyter Kernel Setup

After installing the package, register the magic commands:

```python
%load_ext ul_forge
```

Or add to `~/.ipython/profile_default/ipython_config.py`:
```python
c.InteractiveShellApp.extensions = ['ul_forge']
```

---

## Example Notebook

```python
# Cell 1: Load extension
%load_ext ul_forge

# Cell 2: Render a concept glyph
%%ul
◯(•)

# Cell 3: Compose programmatically
import ul_forge
gir = ul_forge.parse("◯(•) → △(•)")
print(f"Nodes: {len(gir['nodes'])}, Edges: {len(gir['edges'])}")

# Cell 4: Modify GIR and re-render
gir['nodes'].append({"id": "n6", "type": "point", "sort": "e"})
gir['edges'].append({"type": "adjacent", "source": "n5", "target": "n6"})
svg = ul_forge.render(gir)
from IPython.display import SVG
SVG(data=svg)
```
