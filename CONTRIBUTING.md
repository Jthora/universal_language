# Contributing — Universal Language

## How to Contribute

The most valuable contribution you can make is **running the experiments yourself and sharing your results.**

The theoretical framework is established. The experimental infrastructure is built. What remains is independent replication across diverse models, evaluators, and contexts.

---

## Run the Experiments

### Prerequisites

- Python 3.10+
- An API key for at least one supported model provider:
  - **OpenAI** (`gpt-4o`, `gpt-4o-mini`)
  - **Anthropic** (`claude-3.5-sonnet`, `claude-3-opus`)
  - **Google** (`gemini-1.5-pro`, `gemini-1.5-flash`)
- Required packages:

```bash
pip install openai anthropic google-generativeai tiktoken pandas numpy scipy statsmodels pingouin
```

### Step 1: Pilot (5 trials, ~$0.50)

```bash
cd experiments

# Set your API key
export OPENAI_API_KEY="sk-..."

# Run pilot: 5 tasks × 1 model × 1 condition
python analysis/run_trials.py pilot --model gpt-4o
```

This generates 5 LLM responses to reasoning tasks with a UL-structured artifact injected as context. Output goes to `data/raw_outputs/pilot/`.

### Step 2: Score the Outputs

```bash
# Create blinded scoring sheets (condition labels removed)
python analysis/blind.py create pilot
```

Score each output using the rubrics in [`scoring/rubrics.md`](experiments/scoring/rubrics.md). The 5 metrics are:

| Metric | What It Measures |
|--------|-----------------|
| **M1** | Cross-domain depth — number and depth of domain connections |
| **M2** | Structural coherence — logical architecture, not just content |
| **M3** | Generative novelty — structural connections unlikely from training data |
| **M4** | Hierarchical integration — movement across abstraction levels |
| **M5** | Self-referential depth — awareness of own reasoning structure |

Each metric is scored 1–5 using the anchored rubric scales. **SQS** (Structural Quality Score) is the composite.

Domain lists and known-connection references are in `scoring/` to calibrate your scoring.

### Step 3: Reveal and Analyze

```bash
# Reveal condition labels and merge scores
python analysis/blind.py reveal pilot

# Run pre-committed statistical analysis
python analysis/analysis.py pilot
```

### Step 4: Full Alpha (630 trials, ~$50–100)

```bash
# Run across 3 models × 2 conditions × 5 tasks × 21 repetitions
python analysis/run_trials.py alpha --models gpt-4o,claude-3.5-sonnet,gemini-1.5-pro
```

Then score, blind, reveal, and analyze as above.

---

## What Makes a Good Contribution

### High Value
- **Replication data** — Run Alpha on any supported model and share raw outputs + scores
- **New model results** — Run on models not in the original protocol (Llama, Mistral, Gemini 2, etc.)
- **Inter-rater reliability** — Score the same outputs independently and compare
- **Cross-linguistic replication** — Translate the UL artifact and test in non-English contexts
- **New task designs** — Design reasoning tasks that probe different aspects of UL artifact effects

### Medium Value
- **Bug reports** on the analysis scripts or scoring infrastructure
- **Documentation improvements** — clarity, accessibility, translations
- **Statistical review** — audit the pre-committed analysis plan

### How to Submit
1. Fork the repository
2. Run experiments in your fork
3. Open a pull request with:
   - Raw output data in `data/raw_outputs/`
   - Scored data in `data/scored/`
   - A brief report describing your setup (model, API version, date, scorer background)
4. Or open an issue describing your findings

---

## Contribution Guidelines

- **Do not modify the test artifact texts** — the 17 variants are QC-verified and token-matched within ±5% of 490 tokens
- **Do not modify the analysis scripts** unless fixing a bug — the statistical plan is pre-committed
- **Use the blinding tool** — unblinded scoring is not valid for the protocol
- **Report negative results** — if the artifact has no effect in your setup, that is valuable data
- **Include your model version** — API model versions change; record the exact model string used

---

## Code of Conduct

Be rigorous. Be honest. Label your certainty levels. Report what you find, not what you hope to find.

This project uses explicit rigor labels (PROVEN, CONJECTURED, FRAMEWORK, ANALOGY) for good reason. If you extend the theory, use them too.

---

## Contributing to UL Forge (Rust)

The Forge is an 8-crate Rust workspace in `ul-forge/`.

### Prerequisites

- Rust 1.70+ (`rustup update stable`)
- `wasm-pack` for WASM builds (`cargo install wasm-pack`)

### Structure

| Crate | Purpose |
|-------|---------|
| `ul-core` | Parser, validator, renderer, 13 operations + modal/performative/pragmatic |
| `ul-cli` | Command-line interface (8 subcommands) |
| `ul-wasm` | WASM bindings (wasm-bindgen exports) |
| `ul-api` | HTTP API server (Actix-web, 9 routes + websocket) |
| `ul-game` | Game engine — scoring, templates, puzzles |
| `ul-mcp` | Model Context Protocol server (10 tools) |
| `ul-transceiver` | Agent-to-agent message protocol |
| `bindings/python` | PyO3 bindings (8 functions) |

### Running Tests

```bash
cd ul-forge
cargo test --workspace        # 337 tests, all must pass
cargo clippy --workspace      # No warnings allowed
```

### Code Style

- Follow standard `rustfmt` formatting
- Prefer `Result<T, UlError>` over panics
- Public API functions need doc comments
- Match arms for `Operation`/`Sort` enums must be exhaustive

---

## Contributing to Web Editor (TypeScript)

The web editor is in `ul-forge/web/` — a React + Vite + Zustand app.

### Prerequisites

- Node.js 18+
- WASM module built (`cd ul-forge && wasm-pack build --target web crates/ul-wasm`)

### Development

```bash
cd ul-forge/web
npm install
npm run dev         # Starts dev server at localhost:5173
npm test            # Run test suite
```

### Components

| Component | Purpose |
|-----------|---------|
| `App.tsx` | Three-pane layout (palette + editor/canvas + preview) |
| `ScriptEditor.tsx` | UL-Script text editor |
| `VisualCanvas.tsx` | Visual glyph composition canvas |
| `SvgPreview.tsx` | Live SVG preview panel |
| `TemplatePalette.tsx` | 42 canonical templates + modal/force/pragmatic extensions |
| `StatusBar.tsx` | Parse status, force picker (φ), pragmatic inference |
| `ExportButtons.tsx` | SVG/TikZ/GIR export |

---

## Contributing to Python Bindings

The Python bindings are in `ul-forge/bindings/python/` using PyO3 + maturin.

### Prerequisites

- Python 3.10+
- `maturin` (`pip install maturin`)

### Building

```bash
cd ul-forge/bindings/python
maturin develop    # Build + install in current venv
```

### Available Functions

`parse`, `validate`, `render`, `deparse`, `compose_bind`, `set_force`, `infer_pragmatics`, `analyze_structure`

---

## Contributing to UL Core (Documentation)

UL Core documentation is in `ul-core/` — the 5 siblings + writing system.

### Style Guide

- Cross-references use relative paths: `see [Symbology](symbology/symbol-map.md)`
- Σ_UL expressions use backtick code: `` `predicate(e₁, r, e₂) → a` ``
- ASCII art diagrams for visual representations
- Tier labels (T1/T2/T3) for all canonical constructions
- Operation counts must say **13** (not 11 — that was pre-Pass 1.2)

### Documents to Know

- `formal-foundations.md` — Source of truth for all algebraic definitions
- `NAVIGATION.md` — Cross-document navigation guide
- `SYNTHESIS.md` — System overview and expansion paths
