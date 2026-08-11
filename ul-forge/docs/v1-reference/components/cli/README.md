# CLI Tools

> Unix-composable command-line interface for parsing, rendering, and validating UL.

---

## Commands

### `ul parse`

Parse UL-Script to GIR JSON.

```bash
ul parse input.ul                     # → stdout (GIR JSON)
ul parse input.ul -o output.ul.json   # → file
ul parse --stdin                      # read UL-Script from stdin
echo '◯(•) → △(•)' | ul parse --stdin
```

**Flags:**
- `--pretty` / `--compact` — JSON formatting (default: compact)
- `--stdin` — read from stdin instead of file
- `--strict` — fail on warnings (not just errors)

**Exit codes:** 0 = success, 1 = parse error, 2 = file not found

---

### `ul render`

Render GIR JSON to SVG (or TikZ).

```bash
ul render input.ul.json                    # → stdout (SVG)
ul render input.ul.json -o output.svg      # → file
ul render input.ul.json --format tikz      # → TikZ output
cat input.ul.json | ul render --stdin      # pipe from stdin
```

**Flags:**
- `--format svg|tikz` — output format (default: svg)
- `--width N` / `--height N` — canvas size in SVG units (default: 400×400)
- `--embed-gir` / `--no-embed-gir` — embed GIR JSON in SVG metadata (default: embed)
- `--stdin` — read GIR from stdin
- `--template-dir PATH` — custom template directory

**Exit codes:** 0 = success, 1 = render error, 2 = invalid GIR

---

### `ul validate`

Validate GIR JSON against schema, sort rules, and invariants.

```bash
ul validate input.ul.json
ul validate input.ul.json --geometry     # enable Layer 4 geometric checks
ul validate --stdin < input.ul.json
```

**Flags:**
- `--geometry` — enable geometric satisfiability checks (Layer 4)
- `--quiet` — exit code only, no output
- `--json` — output validation results as JSON

**Exit codes:** 0 = valid, 1 = invalid (errors found), 2 = file not found

---

### `ul convert`

Convert between formats. Shorthand for parse + render pipeline.

```bash
ul convert input.ul -o output.svg          # UL-Script → SVG
ul convert input.ul -o output.ul.json      # UL-Script → GIR (same as `ul parse`)
ul convert input.ul.json -o output.svg     # GIR → SVG (same as `ul render`)
ul convert input.svg -o output.ul.json     # SVG → GIR (extract embedded metadata)
```

Format is inferred from file extensions: `.ul` = UL-Script, `.ul.json` = GIR, `.svg` = SVG.

---

### `ul watch`

Watch a file for changes and re-render.

```bash
ul watch input.ul -o output.svg            # watch and re-render on save
ul watch input.ul --serve 8080             # serve SVG on localhost:8080 with live reload
```

---

## Pipeline Composition

Commands are designed for Unix pipe composition:

```bash
# Parse and render in one pipeline
ul parse input.ul | ul render --stdin -o output.svg

# Validate before rendering
ul parse input.ul | ul validate --stdin && ul parse input.ul | ul render --stdin -o output.svg

# Batch convert a directory
find . -name '*.ul' -exec ul convert {} -o {}.svg \;

# Pretty-print a GIR
ul parse input.ul | jq .

# Count nodes in a glyph
ul parse input.ul | jq '.nodes | length'

# Extract all edge types
ul parse input.ul | jq '[.edges[].type] | unique'
```

---

## Installation

```bash
# From source (Rust toolchain required)
cargo install ul-forge-cli

# Or build from repository
git clone https://github.com/Jthora/ul-forge.git
cd ul-forge
cargo build --release
# Binary at target/release/ul
```

The CLI is a thin wrapper around the `ul-forge-core` library. All parsing, validation, and rendering logic lives in the library.

---

## Configuration

Optional configuration via `.ul-forge.toml` in the project root or `~/.config/ul-forge/config.toml`:

```toml
[render]
format = "svg"
width = 400
height = 400
embed_gir = true

[validate]
geometry = false

[parse]
pretty = false
```

---

## Shell Completions

```bash
ul completions bash > /etc/bash_completion.d/ul
ul completions zsh > ~/.zfunc/_ul
ul completions fish > ~/.config/fish/completions/ul.fish
```
