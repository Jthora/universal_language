# VS Code Extension

> UL Forge integration for Visual Studio Code.

---

## Features

### Language Support

Register UL-Script as a VS Code language (`ul`):
- **File association** — `.ul` files open with UL-Script syntax highlighting
- **Syntax highlighting** — TextMate grammar for Unicode marks, operators, brackets, comments
- **Bracket matching** — `()`, `{}`, `[]` pairs
- **Auto-closing** — typing `(` inserts `)`
- **Snippets** — common patterns (e.g., `concept` → `◯(•)`, `relation` → `→`)

### Live Preview

Side panel showing the rendered SVG, updated on save (or on keystroke with debounce):

```
┌──────────────────┬──────────────────┐
│  input.ul        │  UL Preview      │
│                  │                  │
│  ◯(•) → △(•)    │  [rendered SVG]  │
│                  │                  │
└──────────────────┴──────────────────┘
```

Rendering via the WASM module bundled with the extension. No server required.

### Diagnostics

Parser and validator errors appear as VS Code diagnostics:
- Red squiggles for parse errors
- Yellow squiggles for validation warnings
- Problems panel integration

### Commands

| Command | Description |
|---------|-------------|
| `UL Forge: Open Preview` | Open side-by-side SVG preview |
| `UL Forge: Export SVG` | Render current file to SVG |
| `UL Forge: Export TikZ` | Render current file to TikZ |
| `UL Forge: Show GIR` | Show the GIR JSON in a new editor tab |
| `UL Forge: Validate` | Run full validation and show results |
| `UL Forge: Insert Glyph` | Open picker for 42 canonical glyphs |

---

## Architecture

```
VS Code
  │
  ├── Extension Host
  │   ├── Language Client (TypeScript)
  │   │   └── sends/receives LSP messages
  │   ├── WASM Module (ul-forge-core)
  │   │   └── parse, validate, render
  │   └── Preview Webview
  │       └── renders SVG output
  │
  └── Editor
      └── .ul files with syntax highlighting, diagnostics
```

The extension uses the **Language Server Protocol (LSP)** pattern:
- The language server is the WASM module providing parse/validate
- Diagnostics (errors, warnings) are pushed to VS Code via the standard diagnostics API
- No separate language server process — everything runs in the extension host via WASM

---

## Implementation Notes

- Extension is TypeScript, bundled with webpack/esbuild
- WASM module loaded once on activation, reused for all operations
- Preview webview uses VS Code's webview API with CSP-compliant SVG rendering
- Extension activates on `.ul` file open or `UL Forge:` command invocation

---

## Configuration

```json
{
  "ul-forge.preview.autoRefresh": true,
  "ul-forge.preview.debounceMs": 200,
  "ul-forge.render.width": 400,
  "ul-forge.render.height": 400,
  "ul-forge.render.embedGir": true,
  "ul-forge.validation.checkGeometry": false
}
```

---

## Installation

```
ext install ulforge.ul-forge-vscode
```

Or install from VSIX during development:
```bash
cd extensions/vscode
npm run package    # produces ul-forge-x.y.z.vsix
code --install-extension ul-forge-x.y.z.vsix
```
