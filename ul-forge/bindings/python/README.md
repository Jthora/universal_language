# UL Forge — Python Bindings

Python bindings for the Universal Language parser, validator, and renderer.

## Installation

```bash
pip install ul-forge
```

## Usage

```python
import ul_forge

# Parse UL-Script to GIR
gir = ul_forge.parse("● → ●")

# Render to SVG
svg = ul_forge.render(gir)

# Validate
result = ul_forge.validate(gir)
print(result["valid"])  # True
```

## Jupyter Magic

```python
%load_ext ul_forge

%%ul
● → ●
```
