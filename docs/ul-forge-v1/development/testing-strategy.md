# Testing Strategy

> How UL Forge is tested at every level.

---

## Testing Pyramid

```
         ╱╲
        ╱E2E╲         Few, slow, high-confidence
       ╱──────╲
      ╱ Integr. ╲     Medium count, medium speed
     ╱────────────╲
    ╱  Unit Tests   ╲  Many, fast, focused
   ╱──────────────────╲
  ╱  Property Tests     ╲  Exhaustive invariant checking
 ╱────────────────────────╲
```

---

## Unit Tests (Rust)

Each module has tests in a `#[cfg(test)] mod tests` block.

| Module | What's tested | Example |
|--------|--------------|---------|
| `tokenizer` | Unicode → Token mapping, ASCII fallback, error tokens | `assert_eq!(tokenize("◯"), vec![Token::Enclosure(Circle)])` |
| `parser` | Token stream → AST for each grammar rule | `assert!(parse("◯(•)").is_ok())` |
| `transform` | AST → GIR node/edge generation, ID assignment | `let gir = transform(ast); assert_eq!(gir.nodes.len(), 2)` |
| `validator` | Each validation rule independently | `assert!(validate(cyclic_gir).has_error(ContainmentCycle))` |
| `renderer` | Template matching, constraint solving, SVG output | `assert!(svg_output.contains("<circle"))` |

**Convention:** Test files live alongside source (`mod tests` in same file). Test fixtures go in `tests/fixtures/`.

---

## Property Tests (Rust — proptest)

Property tests generate random inputs and verify invariants hold for all of them.

| Property | Generator | Invariant |
|----------|-----------|-----------|
| Roundtrip | Random valid UL-Script | `deparse(parse(text))` ≈ `text` (modulo whitespace) |
| GIR validity | Random GIR document | If `validate(gir).valid`, then `render(gir)` succeeds |
| Sort preservation | Random GIR modification | Modified GIR has correct sort assignments |
| Idempotent validation | Random valid GIR | `validate(validate(gir))` = `validate(gir)` |
| SVG node count | Random GIR | SVG has exactly as many `data-ul-node` attributes as GIR has nodes |

```rust
proptest! {
    #[test]
    fn roundtrip(input in ul_script_strategy()) {
        let gir = parse(&input).unwrap();
        let output = deparse(&gir);
        let gir2 = parse(&output).unwrap();
        assert_eq!(gir, gir2);
    }
}
```

---

## Snapshot Tests (Rust — insta)

Snapshot tests capture expected output and fail if output changes unexpectedly.

| Snapshot type | What's captured | Review process |
|--------------|----------------|----------------|
| GIR snapshots | JSON output of `parse()` for each canonical glyph | `cargo insta review` |
| SVG snapshots | SVG output of `render()` for each canonical glyph | Visual diff via `cargo insta review` |
| Error snapshots | Error message format for known-bad inputs | `cargo insta review` |

```rust
#[test]
fn snapshot_concept_glyph() {
    let gir = parse("◯(•)").unwrap();
    insta::assert_json_snapshot!(gir);
}
```

The `tests/fixtures/canonical/` directory contains all 42 lexicon glyphs as `.ul` files. A test iterates over all of them and asserts snapshots.

---

## Integration Tests (Rust)

End-to-end tests that exercise the full pipeline.

```rust
#[test]
fn end_to_end_parse_render() {
    let input = "◯(•) → △(•)";
    let gir = parse(input).unwrap();
    let validation = validate(&gir);
    assert!(validation.valid);
    let svg = render(&gir, &RenderOptions::default()).unwrap();
    assert!(svg.contains("<svg"));
    assert!(svg.contains("data-ul-node"));
}
```

Integration tests live in `tests/integration/`.

---

## Web Editor Tests

### Unit Tests (Vitest)

```typescript
// bridge.test.ts
test('parse returns valid GIR', () => {
  const gir = parse('◯(•)');
  expect(gir.nodes).toHaveLength(2);
  expect(gir.edges).toHaveLength(1);
});

// gir-store.test.ts
test('undo reverts last edit', () => {
  const store = createGirStore();
  store.applyEdit(addNode({ type: 'point', sort: 'e' }));
  store.undo();
  expect(store.getState().gir.nodes).toHaveLength(0);
});
```

### E2E Tests (Playwright)

```typescript
test('type UL-Script and see SVG preview', async ({ page }) => {
  await page.goto('/');
  await page.locator('.monaco-editor textarea').fill('◯(•)');
  await page.waitForSelector('svg circle');
  const circles = await page.locator('svg circle').count();
  expect(circles).toBeGreaterThanOrEqual(1);
});
```

---

## API Server Tests

Integration tests spin up the Axum server and make HTTP requests:

```rust
#[tokio::test]
async fn test_parse_endpoint() {
    let app = create_app();
    let response = app
        .oneshot(Request::post("/parse").body("◯(•)".into()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
}
```

---

## Test Data

### Canonical Fixtures (`tests/fixtures/canonical/`)

All 42 lexicon entries as `.ul` files with expected GIR and SVG:

```
tests/fixtures/canonical/
├── 01-existence.ul          # •
├── 01-existence.ul.json     # expected GIR
├── 01-existence.svg         # expected SVG (snapshot)
├── 02-concept.ul            # ◯(•)
├── ...
└── 42-emergence.ul
```

### Edge Cases (`tests/fixtures/edge-cases/`)

- Empty input
- Single point
- Deeply nested enclosures (10 levels)
- Many adjacent marks (50+)
- All edge types in one glyph
- Unicode normalization edge cases
- Maximum-length input

### Invalid Inputs (`tests/fixtures/invalid/`)

- Containment cycle
- Dangling edge reference
- Sort violation
- Duplicate node IDs
- Malformed JSON
- Missing required fields

---

## Coverage

Target: **>80% line coverage** for `ul-core` crate.

```bash
# Generate coverage report
cargo llvm-cov --html
open target/llvm-cov/html/index.html
```

Coverage is tracked in CI but not enforced as a gate (to avoid coverage-chasing anti-patterns). The focus is on meaningful tests, not coverage numbers.

---

## Performance Benchmarks

```bash
# Run benchmarks
cargo bench

# Benchmarks measure:
# - parse() latency for each canonical glyph
# - render() latency for each canonical glyph
# - validate() latency for 100-node GIR
# - end-to-end pipeline latency
```

Benchmarks use `criterion` crate. Results are tracked over time but not enforced as CI gates.
