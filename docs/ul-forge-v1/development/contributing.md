# Contributing to UL Forge

> How to set up your environment, make changes, and submit contributions.

---

## Getting Started

### Prerequisites

- Rust 1.75+ (`rustup` recommended)
- Node.js 20+ (`nvm` recommended)
- wasm-pack 0.12+ (`cargo install wasm-pack`)

### Setup

```bash
git clone https://github.com/Jthora/ul-forge.git
cd ul-forge
./scripts/setup.sh
```

This installs Rust dependencies, npm dependencies, and builds the WASM module.

### Verify

```bash
cargo test                  # Rust tests pass
cd web && npm test          # Web tests pass
```

---

## Development Workflow

1. **Create a branch** from `main`:
   ```bash
   git checkout -b feature/my-feature
   ```

2. **Make changes** in the relevant crate or package

3. **Test locally**:
   ```bash
   cargo test                    # if you changed Rust code
   cd web && npm test            # if you changed web code
   ```

4. **Format and lint**:
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   cd web && npx prettier --write .
   ```

5. **Commit** with [Conventional Commits](https://www.conventionalcommits.org/):
   ```
   feat(parser): add ASCII fallback tokenizer
   fix(renderer): correct z-ordering for nested enclosures
   docs(api): add WebSocket protocol specification
   test(validator): add sort violation test cases
   ```

6. **Push and open PR** against `main`

---

## Code Style

### Rust
- `rustfmt` defaults (enforced in CI)
- `clippy` with no warnings (enforced in CI)
- Prefer explicit types over inference in public APIs
- Document public items with `///` doc comments
- Error types use `thiserror` derives

### TypeScript
- Prettier defaults (enforced in CI)
- ESLint with recommended rules
- Strict TypeScript (`strict: true` in tsconfig)
- React components as function components with hooks

---

## Where to Contribute

| Area | Skill | Difficulty |
|------|-------|-----------|
| New test fixtures | UL knowledge | Easy |
| Template library entries | UL + Rust | Medium |
| CLI improvements | Rust | Medium |
| Web editor features | TypeScript + React | Medium |
| Parser error recovery | Rust + PEG | Hard |
| Constraint solver tuning | Rust + math | Hard |
| AI integration | Python + ML | Hard |

---

## Architecture Orientation

Read these docs before diving into code:

1. [Architecture Overview](../architecture/overview.md) — system map
2. [Data Flow](../architecture/data-flow.md) — how data moves through the pipeline
3. [GIR Spec](../specifications/ul-gir-spec.md) — the central data structure
4. [Decision Log](../architecture/decision-log.md) — why things are the way they are

---

## Reporting Issues

- Use GitHub Issues
- Include: what you did, what you expected, what happened
- For parse/render bugs: include the UL-Script input and the incorrect output
- For crashes: include the panic backtrace

---

## License

Contributions are licensed under CC0-1.0, matching the repository license.
