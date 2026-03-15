# UL Forge Distribution Plan — 4 Alternate Solutions

> Architecture for packaging ul-core into 4 distinct products for 4 different consumers.

---

## Executive Summary

The ul-core Rust library contains the complete UL pipeline (parse → validate → render → deparse). Today it ships as a monolithic WASM blob welded to a React web app. This plan defines **4 independent distribution targets** that share ul-core but serve radically different consumers:

| Type | Name | Consumer | Form Factor |
|------|------|----------|-------------|
| **1** | AI Agent Package | AI agents / LLM tool-chains | npm + PyPI + Docker + CLI + MCP server |
| **2** | ProtoFusionGirl Module | Phaser 3 game engine | Game-specific WASM with evaluation API |
| **3** | Educational Web Component | Websites / humans | `<ul-symbol>` custom element, CDN script |
| **4** | AI Transceiver Protocol | Bot-to-bot systems | Wire protocol + encode/decode libraries |

All 4 depend on `ul-core` but add distinct binding layers. No changes to ul-core's existing API are required for Types 1, 3, or 4. Type 2 requires exposing layout geometry (currently internal).

---

## Shared Infrastructure Changes

Before building any distribution type, two prep tasks:

### P1. Extract `@ul-forge/sdk` TypeScript package

The typed wrappers in `web/src/core/index.ts` are well-designed but trapped inside the Vite app. Extract them into a standalone package:

```
packages/
  sdk/                          # @ul-forge/sdk
    src/
      index.ts                  # re-export everything
      types.ts                  # Sort, NodeType, Edge, Gir, etc.
      wasm-bridge.ts            # initialize(), parse(), validate(), render(), deparse()
      utils.ts                  # parseAndRender(), parseValidateRender()
    package.json                # publishable, not private
    tsconfig.json
    README.md
```

This SDK becomes the shared TypeScript foundation for Types 1, 3, and 4.

### P2. Expose layout geometry from ul-core

Currently `compute_layout()` returns `PositionedGlyph` inside the renderer module but this type is not re-exported from `lib.rs`. Type 2 (game engine) needs raw geometry, not SVG strings.

```rust
// ul-core/src/lib.rs — add to public exports:
pub use renderer::layout::{PositionedElement, PositionedGlyph, Shape, Connection};
pub use renderer::layout::compute_layout;  // new public function
```

This is a non-breaking addition — no existing API changes.

### P3. GIR JSON Schema

Generate a JSON Schema from the GIR types (using `schemars` crate) so that machines can validate GIR documents without running Rust code:

```
schemas/
  gir.schema.json              # JSON Schema for GIR documents
  validation-result.schema.json
  render-options.schema.json
```

Used by Types 1 and 4 for machine discovery and protocol definition.

---

## Type 1: AI Agent Self-Service Package

### Goal
A self-contained, zero-config package that an AI agent can find, download, and deploy in under 60 seconds. Self-describing. No human required.

### Consumers
- LLM tool-chains (OpenAI function calling, Anthropic tool use, LangChain)
- Autonomous coding agents (Copilot, Cursor, Devin, OpenClaw)
- MCP-compatible clients (Claude Desktop, etc.)

### Distribution Channels

| Channel | Package Name | Contents |
|---------|-------------|----------|
| **npm** | `@ul-forge/core` | WASM + TypeScript SDK + GIR Schema |
| **PyPI** | `ul-forge` | PyO3 native extension (existing bindings, polished) |
| **Docker** | `ghcr.io/jthora/ul-forge` | REST API server (existing ul-api, productized) |
| **Binary** | GitHub Releases | Standalone CLI for macOS/Linux/Windows |
| **MCP** | built into Docker/npm | Model Context Protocol tool definitions |

### Architecture

```
┌─────────────────────────────────────────────────┐
│                  AI Agent                         │
│                                                   │
│  "I need to work with Universal Language glyphs"  │
│                                                   │
│  Discovery:                                       │
│    AGENTS.md → links to @ul-forge/core            │
│    llms.txt → machine-readable capability list     │
│    npm info @ul-forge/core → package metadata      │
│                                                   │
│  Integration (pick one):                          │
│    npm install @ul-forge/core  → WASM in-process   │
│    pip install ul-forge        → Python native      │
│    docker run ul-forge         → REST API           │
│    ./ul-forge-cli              → CLI subprocess     │
│    MCP server                  → tool protocol      │
└─────────────────────────────────────────────────┘
```

### New Crate: `ul-mcp`

Model Context Protocol server exposing UL operations as MCP tools:

```rust
// Tools exposed via MCP:
tools:
  - name: ul_parse
    description: "Parse UL-Script text into a GIR (Graph Intermediate Representation)"
    input: { ul_script: string }
    output: { gir: GIR JSON }

  - name: ul_validate
    description: "Validate a GIR document against Σ_UL constraints"
    input: { gir: GIR JSON, check_geometry: bool }
    output: { valid: bool, errors: [...], warnings: [...] }

  - name: ul_render
    description: "Render a GIR document to SVG"
    input: { gir: GIR JSON, width: number, height: number, format: "svg"|"tikz" }
    output: { output: string }

  - name: ul_deparse
    description: "Convert a GIR document back to UL-Script text"
    input: { gir: GIR JSON }
    output: { ul_script: string }

  - name: ul_parse_and_render
    description: "Parse UL-Script and render to SVG in one step"
    input: { ul_script: string, width: number, height: number }
    output: { svg: string, gir: GIR JSON, validation: { valid, errors, warnings } }
```

### API Additions for Agent Discovery

The existing ul-api server gets two new endpoints:

```
GET /capabilities        → OpenAPI 3.1 spec (auto-generated)
GET /schema/gir          → GIR JSON Schema
GET /.well-known/ai-plugin.json  → OpenAI plugin manifest format
```

### npm Package Structure

```
@ul-forge/core/
  dist/
    ul_wasm_bg.wasm          # WASM binary
    ul_wasm.js               # JS glue (ESM)
    ul_wasm.d.ts             # TypeScript declarations
    index.js                 # SDK entry (re-exports typed wrappers)
    index.d.ts
  schemas/
    gir.schema.json
  README.md                   # includes AGENTS.md-style machine-readable section
  package.json                # name: @ul-forge/core, main/module/types configured
```

### Files to Create

| File | Purpose |
|------|---------|
| `ul-forge/crates/ul-mcp/Cargo.toml` | MCP server crate |
| `ul-forge/crates/ul-mcp/src/main.rs` | MCP tool handler |
| `ul-forge/packages/sdk/package.json` | Extracted TypeScript SDK |
| `ul-forge/packages/sdk/src/index.ts` | SDK entry point |
| `ul-forge/schemas/gir.schema.json` | GIR JSON Schema |
| `ul-forge/crates/ul-api/src/openapi.rs` | OpenAPI spec generation |

### Build & Publish

```bash
# npm
cd packages/sdk && npm publish --access public

# PyPI
cd bindings/python && maturin publish

# Docker
docker build -t ghcr.io/jthora/ul-forge . && docker push

# MCP
# Ships inside the Docker image, also installable standalone
cargo install ul-mcp
```

---

## Type 2: ProtoFusionGirl Game Module

### Goal
A WASM module purpose-built for the ProtoFusionGirl game engine (Phaser 3 / TypeScript). Exposes a game-specific evaluation API with cosmic rules, graduated scoring, animation bindings, and modding support. Not a general-purpose UL tool — this is a game subsystem.

### Consumer
ProtoFusionGirl game client (TypeScript / Phaser 3, runs in browser)

### New Crate: `ul-game`

```
ul-forge/crates/ul-game/
  Cargo.toml                   # depends on ul-core, wasm-bindgen
  src/
    lib.rs                     # WASM entry points
    context.rs                 # GameContext, session state, Jane's learning model
    cosmic.rs                  # Cosmic rule engine (elements × modalities × forces)
    evaluation.rs              # evaluate(), scoreComposition(), graduated failure
    sequence.rs                # validateSequence(), ordering constraints
    animation.rs               # getAnimationSequence() → keyframe data
    modding.rs                 # loadCustomDefinitions() → runtime rule injection
    scoring.rs                 # Scoring engine: exact/close/wrong/opposite
    types.rs                   # Game-specific types (re-exports ul-core types + extensions)
```

### Cosmic Rule Engine

```rust
/// The 4 classical elements
#[wasm_bindgen]
pub enum Element { Fire, Water, Earth, Air }

/// 3 modalities per element
#[wasm_bindgen]
pub enum Modality { Cardinal, Fixed, Mutable }

/// 9 cosmic forces
#[wasm_bindgen]
pub enum CosmicForce {
    Creation, Preservation, Destruction,    // Primary cycle
    Expansion, Contraction, Transformation, // Secondary cycle
    Harmony, Discord, Resolution,           // Tertiary cycle
}

/// A cosmic rule binding an element+modality+force to a UL glyph pattern
#[derive(Serialize, Deserialize)]
pub struct CosmicRule {
    pub element: Element,
    pub modality: Modality,
    pub force: CosmicForce,
    pub required_pattern: GirPattern,    // GIR subgraph that must appear
    pub forbidden_pattern: Option<GirPattern>, // GIR subgraph that must NOT appear
    pub score_weight: f64,
}
```

### WASM API Surface

```typescript
// TypeScript interface the game sees (generated by wasm-bindgen):

interface ULGameModule {
  // Lifecycle
  createContext(config: GameConfig): GameContext;
  destroyContext(ctx: GameContext): void;

  // Core evaluation
  evaluate(ctx: GameContext, gir: GirJson): EvaluationResult;
  validateSequence(ctx: GameContext, glyphs: GirJson[]): SequenceResult;
  scoreComposition(ctx: GameContext, gir: GirJson, target: PuzzleTarget): ScoreResult;

  // Jane's learning system
  evaluateJaneAttempt(ctx: GameContext, attempt: GirJson, expected: GirJson): JaneResult;
  getJaneProficiency(ctx: GameContext): ProficiencyMap;

  // Animation
  getAnimationSequence(gir: GirJson): AnimationKeyframe[];

  // Modding
  loadCustomDefinitions(ctx: GameContext, definitions: CustomRuleset): LoadResult;

  // Geometry (raw positioned data for Phaser rendering, NOT SVG)
  layout(gir: GirJson, width: number, height: number): PositionedGlyph;
}
```

### Evaluation Result (Graduated Failure)

```typescript
interface EvaluationResult {
  score: number;           // 0.0 – 1.0
  grade: "exact" | "close" | "wrong" | "opposite";
  matchedRules: CosmicRule[];
  violations: Violation[];
  feedback: string;        // Human-readable explanation for Jane
  partialCredit: {
    structuralMatch: number;   // How close the graph structure is
    sortCorrectness: number;   // Are sorts assigned correctly
    cosmicAlignment: number;   // Do cosmic rules match
    sequenceOrder: number;     // Is the ordering valid
  };
}
```

### Animation Bindings

```typescript
interface AnimationKeyframe {
  nodeId: string;
  timestamp: number;       // 0.0 – 1.0 normalized time
  x: number;
  y: number;
  scale: number;
  rotation: number;
  opacity: number;
  easing: "linear" | "ease-in" | "ease-out" | "ease-in-out";
}

// The game engine calls getAnimationSequence() and feeds keyframes
// to Phaser's tween system. UL Forge computes the choreography,
// Phaser does the actual rendering.
```

### Performance Budget

| Operation | Target | Method |
|-----------|--------|--------|
| Simple evaluate() | < 1ms | Pre-compiled rule lookup, no allocation |
| Complex scoreComposition() | < 16ms | Fits in one frame at 60fps |
| WASM binary size | < 500KB | `wasm-opt -Oz`, no std, minimal serde |
| Memory per context | < 2MB | Arena allocator for game session |

### Build

```bash
cd crates/ul-game
wasm-pack build --target web --release
wasm-opt -Oz -o pkg/ul_game_bg.wasm pkg/ul_game_bg.wasm
ls -la pkg/ul_game_bg.wasm  # must be < 500KB
```

### Integration with ProtoFusionGirl

```typescript
// In the game's boot scene:
import init, { createContext, evaluate } from '@ul-forge/game';

class ULSystem {
  private ctx: GameContext;

  async boot() {
    await init();
    this.ctx = createContext({
      cosmicRules: await fetch('/data/cosmic-rules.json').then(r => r.json()),
      janeProficiency: loadSaveData().ulProficiency,
    });
  }

  onPlayerSubmitGlyph(gir: GirJson, puzzle: PuzzleTarget): EvaluationResult {
    return evaluate(this.ctx, gir);
  }
}
```

---

## Type 3: Educational Web Component

### Goal
A framework-agnostic web component that any website can embed with a single `<script>` tag. A human types or selects UL primitives and sees the rendered glyph live. No build system, no npm, no React — just HTML.

### Consumer
- Educational websites teaching UL
- Documentation sites (this repo's own docs)
- Blog posts, interactive tutorials
- Any HTML page

### Components

```html
<!-- Display a static glyph -->
<ul-symbol script="point(existence)" width="200" height="200"></ul-symbol>

<!-- Interactive composer with live preview -->
<ul-composer width="400" height="300"></ul-composer>

<!-- Searchable dictionary of canonical glyphs -->
<ul-dictionary columns="3"></ul-dictionary>
```

### Architecture

```
packages/
  web-components/              # @ul-forge/components (or CDN-only)
    src/
      ul-symbol.ts             # <ul-symbol> custom element
      ul-composer.ts           # <ul-composer> interactive editor
      ul-dictionary.ts         # <ul-dictionary> reference browser
      styles.ts                # Shadow DOM CSS (encapsulated)
      wasm-loader.ts           # Lazy WASM initialization
      index.ts                 # Custom element registration
    build/
      ul-components.iife.js    # Single-file IIFE bundle (WASM inlined as base64)
      ul-components.esm.js     # ESM bundle for import maps
    package.json
```

### `<ul-symbol>` Element

```typescript
class ULSymbol extends HTMLElement {
  static observedAttributes = ['script', 'width', 'height', 'theme'];

  async connectedCallback() {
    const shadow = this.attachShadow({ mode: 'open' });
    shadow.innerHTML = `<style>${styles}</style><div class="container"></div>`;

    await ensureWasmLoaded();
    this.render();
  }

  attributeChangedCallback() {
    this.render();
  }

  private render() {
    const script = this.getAttribute('script') || '';
    const width = parseFloat(this.getAttribute('width') || '200');
    const height = parseFloat(this.getAttribute('height') || '200');

    try {
      const gir = parse(script);
      const svg = render(gir, { format: 'svg', width, height, embedGir: false });
      this.shadowRoot!.querySelector('.container')!.innerHTML = svg;
    } catch (e) {
      this.shadowRoot!.querySelector('.container')!.innerHTML =
        `<span class="error">${sanitize(e.message)}</span>`;
    }
  }
}

customElements.define('ul-symbol', ULSymbol);
```

### `<ul-composer>` Element

Interactive editor with:
- Text input area (UL-Script)
- Live SVG preview (debounced)
- Validation status indicator (green/yellow/red)
- Primitive palette (click-to-insert buttons for Point, Line, Angle, Curve, Enclosure)
- Export button (download SVG)

All contained within Shadow DOM — zero style leakage.

### Bundle Strategy

```bash
# Build IIFE bundle with WASM inlined
vite build --config vite.components.config.ts

# Output: single file, ~350KB (WASM is ~200KB base64-encoded)
# Can be loaded from any CDN or self-hosted
```

### Usage (Zero Build System)

```html
<!DOCTYPE html>
<html>
<head>
  <script src="https://cdn.jsdelivr.net/npm/@ul-forge/components/ul-components.iife.js"></script>
</head>
<body>
  <h1>Universal Language Demo</h1>

  <ul-symbol script="enclosure(circle, point(existence))"></ul-symbol>

  <ul-composer></ul-composer>
</body>
</html>
```

### Files to Create

| File | Purpose |
|------|---------|
| `ul-forge/packages/web-components/src/ul-symbol.ts` | Static glyph display element |
| `ul-forge/packages/web-components/src/ul-composer.ts` | Interactive editor element |
| `ul-forge/packages/web-components/src/ul-dictionary.ts` | Reference browser element |
| `ul-forge/packages/web-components/src/styles.ts` | Encapsulated Shadow DOM styles |
| `ul-forge/packages/web-components/src/wasm-loader.ts` | Lazy WASM initialization |
| `ul-forge/packages/web-components/src/index.ts` | Registration entry point |
| `ul-forge/packages/web-components/vite.config.ts` | IIFE + ESM build configuration |
| `ul-forge/packages/web-components/package.json` | Package manifest |

---

## Type 4: AI Transceiver Protocol

### Goal
A wire protocol that enables AI systems to exchange UL-encoded meaning structures. Not about rendering — about structured semantic communication between machines. Transport-agnostic: works over HTTP, WebSocket, message queues, stdio pipes, or any byte stream.

### Consumer
- Multi-agent systems that need shared semantic representation
- AI pipelines that pass meaning between stages
- Bot-to-bot communication (e.g., planning agent → execution agent)
- Human-AI dialogue systems that want inspectable meaning structures

### Protocol Definition

```
ul-forge/
  protocol/
    ul-transceiver.schema.json   # JSON Schema for the wire format
    ul-transceiver.proto         # Optional protobuf definition
    README.md                    # Protocol specification
  crates/
    ul-transceiver/              # Rust encode/decode library
      Cargo.toml
      src/
        lib.rs
        message.rs               # ULMessage type
        envelope.rs              # Transport envelope
        codec.rs                 # Encode/decode (JSON + MessagePack)
        capability.rs            # Capability advertisement
        error.rs
  packages/
    transceiver/                 # @ul-forge/transceiver (TypeScript)
      src/
        index.ts
        message.ts
        codec.ts
        capability.ts
      package.json
  bindings/
    python/
      ul_transceiver/            # ul-transceiver (Python)
        __init__.py
        message.py
        codec.py
```

### Wire Format

```json
{
  "$schema": "https://ulforge.dev/schemas/ul-transceiver/v1.json",
  "version": "1.0",
  "message": {
    "id": "msg-uuid-here",
    "timestamp": "2025-01-15T10:30:00Z",
    "sender": {
      "id": "agent-alpha",
      "capabilities": ["parse", "validate", "render", "evaluate"]
    },
    "intent": "assert",
    "payload": {
      "format": "gir",
      "content": {
        "nodes": [...],
        "edges": [...],
        "root": "n0"
      }
    },
    "context": {
      "conversation_id": "conv-uuid",
      "in_reply_to": "msg-previous-uuid",
      "domain": "physics.thermodynamics"
    }
  }
}
```

### Message Types

| Intent | Description | Payload |
|--------|-------------|---------|
| `assert` | Declare a meaning structure as true | GIR |
| `query` | Ask whether a meaning structure holds | GIR (partial, with wildcards) |
| `propose` | Suggest a meaning structure for agreement | GIR + confidence score |
| `refine` | Modify a previously sent structure | GIR diff (add/remove nodes/edges) |
| `validate_request` | Ask recipient to validate a GIR | GIR |
| `validate_response` | Report validation result | ValidationResult |
| `capability_advertisement` | Announce what operations this agent supports | Capability list |
| `ack` | Acknowledge receipt | Reference to original message ID |

### Capability Advertisement

```json
{
  "intent": "capability_advertisement",
  "payload": {
    "operations": [
      { "name": "parse", "input": "ul-script", "output": "gir" },
      { "name": "validate", "input": "gir", "output": "validation-result" },
      { "name": "render", "input": "gir", "output": "svg" },
      { "name": "evaluate", "input": "gir+rules", "output": "evaluation-result" }
    ],
    "supported_sorts": ["entity", "relation", "modifier", "assertion"],
    "max_gir_nodes": 1000,
    "protocol_version": "1.0"
  }
}
```

### Rust Library API

```rust
use ul_transceiver::{ULMessage, Intent, Payload, Codec};

// Create a message
let msg = ULMessage::builder()
    .sender("agent-alpha")
    .intent(Intent::Assert)
    .gir_payload(my_gir)
    .context_domain("physics.thermodynamics")
    .build();

// Encode to JSON
let json_bytes = Codec::Json.encode(&msg)?;

// Encode to MessagePack (compact binary)
let msgpack_bytes = Codec::MessagePack.encode(&msg)?;

// Decode
let received: ULMessage = Codec::Json.decode(&json_bytes)?;
```

### TypeScript Library API

```typescript
import { ULMessage, Intent, Codec } from '@ul-forge/transceiver';

// Create a message
const msg = ULMessage.create({
  sender: { id: 'agent-beta', capabilities: ['parse', 'validate'] },
  intent: Intent.Propose,
  payload: { format: 'gir', content: myGir },
  context: { conversationId: 'conv-123', domain: 'linguistics' },
});

// Encode/decode
const json = Codec.json.encode(msg);
const decoded = Codec.json.decode(json);

// Validate message against schema
const isValid = ULMessage.validate(decoded);
```

### Transport Adapters (Examples, Not Scope)

The protocol is transport-agnostic. These are example adapters, not part of the core package:

```typescript
// WebSocket adapter
const ws = new WebSocket('wss://peer-agent.example/ul');
ws.onmessage = (e) => {
  const msg = Codec.json.decode(e.data);
  handleMessage(msg);
};

// HTTP adapter
const response = await fetch('https://agent.example/ul/message', {
  method: 'POST',
  headers: { 'Content-Type': 'application/ul-transceiver+json' },
  body: Codec.json.encode(msg),
});

// stdio adapter (for MCP-style subprocess communication)
process.stdin.on('data', (chunk) => {
  const msg = Codec.json.decode(chunk);
  const response = handleMessage(msg);
  process.stdout.write(Codec.json.encode(response));
});
```

---

## Dependency Graph

```
                    ┌──────────┐
                    │ ul-core  │  (shared foundation)
                    └────┬─────┘
                         │
          ┌──────────────┼──────────────┬─────────────────┐
          │              │              │                  │
     ┌────▼────┐   ┌────▼────┐   ┌────▼─────┐    ┌──────▼───────┐
     │ ul-wasm │   │ ul-game │   │ ul-mcp   │    │ul-transceiver│
     │ (base)  │   │ (Type2) │   │ (Type1)  │    │   (Type 4)   │
     └────┬────┘   └────┬────┘   └────┬─────┘    └──────┬───────┘
          │              │              │                  │
     ┌────▼────────┐    │         ┌────▼────┐            │
     │ @ul-forge/  │    │         │ ul-api  │            │
     │    sdk      │    │         │ + MCP   │            │
     └────┬────────┘    │         └─────────┘            │
          │              │                                │
   ┌──────┼───────┐     │                                │
   │              │     │                                │
┌──▼───┐   ┌─────▼──┐  │        ┌───────────────────────▼──┐
│Type 1│   │ Type 3 │  │        │  @ul-forge/transceiver   │
│ npm  │   │  <ul-  │  │        │  (TS) + ul-transceiver   │
│ pkg  │   │symbol> │  │        │  (Python)                │
└──────┘   └────────┘  │        └──────────────────────────┘
                        │
                   ┌────▼───────────┐
                   │ ProtoFusionGirl│
                   │   game client  │
                   └────────────────┘
```

---

## Implementation Order

### Wave 1: Shared Prerequisites
1. **P1**: Extract `@ul-forge/sdk` from web app (2-3 days)
2. **P2**: Expose `compute_layout` + geometry types from ul-core (1 day)
3. **P3**: Generate GIR JSON Schema via `schemars` (1 day)

### Wave 2: Type 1 + Type 3 (Share the SDK)
4. **Type 1a**: Publish `@ul-forge/core` to npm (SDK + WASM) (1-2 days)
5. **Type 1b**: Polish `ul-forge` PyPI package (existing bindings) (1 day)
6. **Type 1c**: Productize Docker image (existing Dockerfile + OpenAPI) (1 day)
7. **Type 3**: Build web components on top of SDK (3-4 days)

### Wave 3: Type 1 MCP + Type 2 Game Module
8. **Type 1d**: Build `ul-mcp` crate (MCP server) (2-3 days)
9. **Type 2**: Build `ul-game` crate + WASM module (5-7 days)

### Wave 4: Type 4 Protocol
10. **Type 4a**: Define wire protocol JSON Schema (1-2 days)
11. **Type 4b**: Rust `ul-transceiver` crate (2-3 days)
12. **Type 4c**: TypeScript `@ul-forge/transceiver` package (1-2 days)
13. **Type 4d**: Python `ul-transceiver` package (1 day)

### Total: ~22-30 days of focused work

---

## Workspace Structure After Implementation

```
ul-forge/
  Cargo.toml                    # workspace: add ul-game, ul-mcp, ul-transceiver
  crates/
    ul-core/                    # ← P2: expose layout geometry
    ul-cli/                     # existing
    ul-wasm/                    # existing (base WASM)
    ul-api/                     # ← Type 1c: add /capabilities, OpenAPI
    ul-game/                    # ← Type 2: new crate
    ul-mcp/                     # ← Type 1d: new crate
    ul-transceiver/             # ← Type 4b: new crate
  bindings/
    python/                     # ← Type 1b: polish and publish
  packages/
    sdk/                        # ← P1: extracted TypeScript SDK
    web-components/             # ← Type 3: <ul-symbol>, <ul-composer>
    transceiver/                # ← Type 4c: TypeScript transceiver
  protocol/
    ul-transceiver.schema.json  # ← Type 4a: wire protocol schema
    README.md
  schemas/
    gir.schema.json             # ← P3: GIR JSON Schema
    validation-result.schema.json
    render-options.schema.json
  web/                          # existing React editor (now imports from packages/sdk)
  vscode-extension/             # existing
```

---

## Success Criteria

| Type | Criterion |
|------|-----------|
| **1** | `npm install @ul-forge/core` works, `pip install ul-forge` works, `docker run` starts API, MCP server responds to tool calls |
| **2** | WASM < 500KB, `evaluate()` < 1ms on simple input, game integration test passes in Phaser, custom rules load at runtime |
| **3** | `<ul-symbol script="point(existence)">` renders in plain HTML file, no build tools needed, works in Chrome/Firefox/Safari |
| **4** | Two agents exchange UL messages over WebSocket, both can encode/decode, capability advertisement works, round-trip GIR integrity preserved |
