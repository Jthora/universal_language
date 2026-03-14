# Parser Component: `ul-parse`

> Converts UL-Script text into UL-GIR (JSON graph-with-tree-spine).

---

## Responsibility

The parser is the entry point for human-authored UL content. It takes a Unicode text stream of UL-Script and produces a validated UL-GIR document.

**Input:** UL-Script text (`.ul` file, stdin, or extracted from Markdown `ul` fences)  
**Output:** UL-GIR JSON (to stdout, file, or API response)

---

## Pipeline

```
UL-Script (text)
  │
  ├── 1. Tokenizer         → Token stream
  ├── 2. Parser (PEG)      → Abstract Syntax Tree (AST)
  ├── 3. AST→GIR Transform → UL-GIR (graph)
  └── 4. Basic validation   → Validated UL-GIR
```

### Stage 1: Tokenizer

See [tokenizer.md](tokenizer.md) for full specification.

Splits the input into tokens:
- Primitive symbols: `●`, `○`, `△`, `□`, `→`, `←`, `↔`, `~`, `∠`
- Structural: `{`, `}`, `|`, `&`, `(`, `)`, `,`
- Numbers: `[0-9]+`
- Angle prefix: `@`
- Comments: `#` to end of line
- Whitespace (skipped)

ASCII fallback tokens (`/0`, `/3`, `/4`, `->`, `<-`, `<->`) are normalized to their Unicode equivalents during tokenization.

### Stage 2: Parser

See [grammar.md](grammar.md) for the PEG grammar.

Builds an AST from the token stream. The AST mirrors the UL-Script structure:
- `Glyph` → top-level unit
- `Composition` → sequence of terms connected by operators
- `Mark` → primitive with optional content
- `Content` → nested composition inside braces

### Stage 3: AST → GIR Transform

See [ast-to-gir.md](ast-to-gir.md) for the transformation rules.

Converts the hierarchical AST into the flat graph-with-tree-spine:
- Each AST `Mark` node → GIR `node` with type and sort
- AST `Content` (braces) → GIR `contains` edges
- AST operators (`|`, `→`, `&`) → GIR cross-edges (`adjacent`, `connects`, `intersects`)
- Angle modifiers → GIR `modified_by` edges
- Stable ID assignment: IDs derived from AST position for determinism

### Stage 4: Basic Validation

The parser performs lightweight validation:
- All node IDs unique
- All edge references valid
- Tree-spine is acyclic and connected
- Sort defaults applied

Full validation (geometry, complex sort constraints) is deferred to `ul-validate`.

---

## API Surface (Rust)

```rust
pub struct ParseResult {
    pub gir: Gir,
    pub warnings: Vec<Diagnostic>,
    pub errors: Vec<Diagnostic>,
}

/// Parse UL-Script text into GIR
pub fn parse(source: &str) -> ParseResult;

/// Parse UL-Script from file (auto-detects .ul vs .md)
pub fn parse_file(path: &Path) -> ParseResult;

/// Extract UL-Script blocks from Markdown
pub fn extract_ul_blocks(markdown: &str) -> Vec<(Range, String)>;

/// Convert GIR back to canonical UL-Script
pub fn deparse(gir: &Gir) -> String;
```

---

## WASM API

```typescript
// Compiled from Rust via wasm-pack
import { parse, deparse } from 'ul-forge-core';

const result = parse('○{●} →@60 △{●}');
// result.gir: JavaScript object (GIR)
// result.warnings: string[]
// result.errors: string[]

const script = deparse(result.gir);
// script: '○{●} →@60 △{●}'
```

---

## Error Handling

| Error class | Example | Recovery |
|-------------|---------|----------|
| `LexError` | Unknown character `¥` | Skip character, emit warning |
| `SyntaxError` | Missing closing `}` | Insert implicit `}` at EOL |
| `SortError` | Point used as container | Report error, mark GIR node as invalid |

The parser always produces *some* GIR output, even with errors. This enables live preview to show partial results while the user is typing. The `errors` array in `ParseResult` indicates what went wrong.

---

## Performance Target

- Parse time for typical glyphs (1-50 nodes): < 1ms
- Parse time for large documents (100+ glyphs): < 100ms
- Memory: O(n) where n = number of nodes

The parser should never be the bottleneck — layout computation dominates.

---

## Related Documents

- [tokenizer.md](tokenizer.md) — Tokenization rules
- [grammar.md](grammar.md) — PEG grammar
- [ast-to-gir.md](ast-to-gir.md) — AST → GIR transformation rules
- [UL-Script spec](../../specifications/ul-script-spec.md) — Language specification
- [UL-GIR spec](../../specifications/ul-gir-spec.md) — Output format specification
