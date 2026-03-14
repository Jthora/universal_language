# Build System

> How to build, test, and package all UL Forge components.

---

## Prerequisites

| Tool | Version | Purpose |
|------|---------|---------|
| Rust | 1.75+ | Core library, CLI, server |
| wasm-pack | 0.12+ | Rust → WASM compilation |
| Node.js | 20+ | Web editor, VS Code extension |
| npm | 10+ | JavaScript package management |
| Python | 3.10+ | PyO3 bindings (optional) |
| Maturin | 1.0+ | Python extension build (optional) |
| Docker | 24+ | API server image (optional) |

---

## Quick Start

```bash
# Clone and setup
git clone https://github.com/Jthora/ul-forge.git
cd ul-forge
./scripts/setup.sh     # installs all dependencies

# Build everything
./scripts/build-all.sh

# Test everything
./scripts/test-all.sh
```

---

## Component Builds

### Rust Core + CLI

```bash
# Build all Rust crates
cargo build --release

# Run tests
cargo test

# Run lints
cargo clippy -- -D warnings
cargo fmt --check

# Build CLI binary
cargo build --release -p ul-cli
# Binary at: target/release/ul
```

### WASM Module

```bash
cd crates/ul-wasm

# Build for browser (produces pkg/ directory)
wasm-pack build --target web --release

# Build for Node.js (for testing)
wasm-pack build --target nodejs

# Test in Node.js
wasm-pack test --node

# The output in pkg/ is an npm-ready package
```

### Web Editor

```bash
cd web

# Install dependencies (links local WASM package)
npm install

# Development server with hot reload
npm run dev          # http://localhost:5173

# Production build
npm run build        # outputs to dist/

# Run tests
npm test             # Vitest unit tests
npx playwright test  # E2E browser tests

# Type checking
npx tsc --noEmit
```

### VS Code Extension

```bash
cd extensions/vscode

# Install dependencies
npm install

# Compile
npm run compile

# Package VSIX
npx vsce package     # produces ul-forge-x.y.z.vsix

# Install locally
code --install-extension ul-forge-*.vsix
```

### Python Bindings

```bash
cd bindings/python

# Build and install in current virtualenv
maturin develop

# Build wheel for distribution
maturin build --release

# Test
python -m pytest tests/
```

### API Server (Docker)

```bash
# Build Docker image
docker build -t ulforge/api:v1 .

# Run
docker run -p 3000:3000 ulforge/api:v1
```

---

## CI Pipeline

### On every push/PR:

```yaml
# .github/workflows/rust.yml
- cargo build
- cargo test
- cargo clippy -- -D warnings
- cargo fmt --check

# .github/workflows/wasm.yml
- wasm-pack build --target web
- wasm-pack test --node

# .github/workflows/web.yml
- npm ci (in web/)
- npm test
- npm run build
- npx playwright test
```

### On tag (release):

```yaml
# .github/workflows/release.yml
- Build CLI binaries (Linux x86_64, macOS x86_64, macOS aarch64, Windows)
- Build WASM package → publish to npm
- Build Python wheel → publish to PyPI
- Build Docker image → push to registry
- Build web editor → deploy to GitHub Pages
- Create GitHub Release with binaries
```

---

## WASM ↔ Web Linking

The web editor consumes the WASM package as an npm dependency. During development:

```json
// web/package.json
{
  "dependencies": {
    "@ul-forge/core": "file:../crates/ul-wasm/pkg"
  }
}
```

For published builds, the WASM package is published to npm as `@ul-forge/core` and consumed normally.

---

## Environment Variables

| Variable | Purpose | Default |
|----------|---------|---------|
| `UL_FORGE_LOG` | Log level (error, warn, info, debug, trace) | `info` |
| `UL_FORGE_PORT` | API server port | `3000` |
| `UL_FORGE_HOST` | API server bind address | `127.0.0.1` |
