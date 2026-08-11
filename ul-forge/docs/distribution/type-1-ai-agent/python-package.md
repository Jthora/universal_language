# Type 1 — Python Package: `ul-forge`

> PyO3-powered native Python extension for the UL pipeline.

---

## Current State

Python bindings already exist at `ul-forge/bindings/python/`:
- PyO3 module with `parse()`, `render()`, `validate()`, `deparse()`
- Jupyter cell magic (`%%ul`)
- Maturin build configuration

### Gaps to Close

1. **Not published** — no PyPI release
2. **No README** — pip page would be empty
3. **No type stubs** — no `.pyi` file for autocomplete
4. **No CI publish step** — manual only

---

## Target

```
pip install ul-forge
```

```python
from ul_forge import parse, validate, render, deparse

gir = parse("enclosure(circle, point(existence))")
result = validate(gir)
if result.valid:
    svg = render(gir, width=400, height=400)
```

---

## Deliverables

### 1. Type Stubs

```python
# bindings/python/ul_forge.pyi

from typing import Any

def parse(input: str) -> dict[str, Any]:
    """Parse UL-Script text into a GIR document (dict)."""
    ...

def validate(gir: dict[str, Any], check_geometry: bool = False) -> dict[str, Any]:
    """Validate a GIR document. Returns {valid: bool, errors: list, warnings: list}."""
    ...

def render(gir: dict[str, Any], width: float = 400.0, height: float = 400.0, format: str = "svg") -> str:
    """Render a GIR document to SVG or TikZ."""
    ...

def deparse(gir: dict[str, Any]) -> str:
    """Convert a GIR document back to UL-Script text."""
    ...
```

### 2. README.md

Clear usage guide with install instructions, code examples, and Jupyter integration docs.

### 3. pyproject.toml / Cargo.toml polish

Ensure metadata is complete for PyPI:
- `description` — one-line summary
- `license` — CC0-1.0
- `readme` — points to README.md
- `repository` — GitHub URL
- `keywords` — universal-language, glyph, formal-language, symbology

### 4. CI Publish

```yaml
# .github/workflows/publish-pypi.yml
name: Publish PyPI
on:
  release:
    types: [published]
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: PyO3/maturin-action@v1
        with:
          command: publish
          args: --manifest-path ul-forge/bindings/python/Cargo.toml
        env:
          MATURIN_PYPI_TOKEN: ${{ secrets.PYPI_TOKEN }}
```

---

## Testing

```bash
# Build locally
cd bindings/python
maturin develop

# Test
python -c "from ul_forge import parse; print(parse('point(existence)'))"
```
