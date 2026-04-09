# WASM Integration Patterns Guide

> Recipes for integrating UL Forge WASM modules into different environments:
> browser apps, game engines, Node.js, AI agents, web components, React apps,
> and custom toolchains.

---

## Table of Contents

1. [React + Vite (Standard Web Editor)](#1-react--vite-standard-web-editor)
2. [Vanilla Browser (No Framework)](#2-vanilla-browser-no-framework)
3. [Web Components (CDN Embed)](#3-web-components-cdn-embed)
4. [Game Engine Integration](#4-game-engine-integration)
5. [Node.js / Server-Side](#5-nodejs--server-side)
6. [AI Agent Integration](#6-ai-agent-integration)
7. [Electron App](#7-electron-app)
8. [iframe Embedding](#8-iframe-embedding)
9. [Web Worker (Off-Thread)](#9-web-worker-off-thread)
10. [Testing Harness](#10-testing-harness)
11. [Build System Integration](#11-build-system-integration)
12. [Performance Optimization](#12-performance-optimization)

---

## 1. React + Vite (Standard Web Editor)

This is the primary deployment model used by UL Forge's web editor.

### 1.1 Setup

```bash
# Install dependencies
cd ul-forge/web
npm install

# Build WASM (from workspace root)
cd ..
wasm-pack build --target web crates/ul-wasm --out-dir ../../web/wasm-pkg

# Start dev server
cd web
npm run dev
```

### 1.2 Vite Configuration

```typescript
// vite.config.ts
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import wasm from "vite-plugin-wasm";
import path from "path";

export default defineConfig({
  plugins: [react(), wasm()],
  resolve: {
    alias: {
      "ul-wasm": path.resolve(__dirname, "wasm-pkg/ul_wasm.js"),
      "@ul-forge/core": path.resolve(__dirname, "src/core/index.ts"),
    },
  },
  optimizeDeps: {
    exclude: ["ul-wasm"],     // Don't pre-bundle the WASM glue
  },
  build: {
    target: "esnext",          // Required for top-level await (WASM init)
  },
});
```

### 1.3 App Initialization

```typescript
// src/App.tsx
import { useEffect, useState } from "react";
import * as core from "@ul-forge/core";

function App() {
  const [ready, setReady] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    core.initialize()
      .then(() => setReady(true))
      .catch((err) => setError(err.message));
  }, []);

  if (error) return <div>Failed to load WASM: {error}</div>;
  if (!ready) return <div>Loading UL engine...</div>;

  return <Editor />;
}
```

### 1.4 Zustand Store Pattern

The web editor uses Zustand for state management. The store wraps core functions
and manages GIR state:

```typescript
// src/store/editorStore.ts
import { create } from "zustand";
import * as core from "@ul-forge/core";
import type { Gir, ValidationResult } from "@ul-forge/core";

interface EditorState {
  input: string;
  gir: Gir | null;
  svg: string | null;
  validation: ValidationResult | null;
  error: string | null;

  setInput: (input: string) => void;
  parseAndRender: () => void;
}

export const useEditorStore = create<EditorState>((set, get) => ({
  input: "",
  gir: null,
  svg: null,
  validation: null,
  error: null,

  setInput: (input) => set({ input }),

  parseAndRender: () => {
    const { input } = get();
    try {
      const gir = core.parse(input);
      const validation = core.validate(gir);
      const svg = validation.valid ? core.render(gir) : null;
      set({ gir, validation, svg, error: null });
    } catch (err) {
      set({ gir: null, validation: null, svg: null, error: String(err) });
    }
  },
}));
```

### 1.5 Monaco Editor Integration

The web editor uses Monaco for syntax-highlighted UL-Script editing with
real-time parse-and-render:

```typescript
import MonacoEditor from "@monaco-editor/react";
import { useEditorStore } from "../store/editorStore";

function ULEditor() {
  const { input, setInput, parseAndRender } = useEditorStore();

  return (
    <MonacoEditor
      language="plaintext"
      value={input}
      onChange={(value) => {
        setInput(value ?? "");
        parseAndRender();  // Real-time parse-validate-render cycle
      }}
      options={{
        minimap: { enabled: false },
        fontSize: 18,
        renderWhitespace: "none",
      }}
    />
  );
}
```

---

## 2. Vanilla Browser (No Framework)

For integration without React or any framework, use the raw WASM module directly.

### 2.1 ESM Import (Recommended)

```html
<!DOCTYPE html>
<html>
<head>
  <title>UL Forge</title>
</head>
<body>
  <textarea id="input" rows="4" cols="40">● -> ●</textarea>
  <button id="render">Render</button>
  <div id="output"></div>

  <script type="module">
    // Import the WASM glue code
    import wasmInit, {
      init as wasmModuleInit,
      parseUlScript,
      renderSvg,
      validateGir,
    } from "./wasm-pkg/ul_wasm.js";

    // Initialize
    await wasmInit();
    wasmModuleInit();

    document.getElementById("render").addEventListener("click", () => {
      const input = document.getElementById("input").value;
      try {
        const girJson = parseUlScript(input);
        const gir = JSON.parse(girJson);
        const validation = validateGir(JSON.stringify(gir), false);

        if (validation.valid) {
          const svg = renderSvg(girJson, 400, 400);
          document.getElementById("output").innerHTML = svg;
        } else {
          document.getElementById("output").textContent =
            "Errors: " + JSON.stringify(validation.errors);
        }
      } catch (err) {
        document.getElementById("output").textContent = "Parse error: " + err;
      }
    });
  </script>
</body>
</html>
```

### 2.2 Dynamic Import (Lazy Load)

For pages where UL rendering is optional, load WASM on demand:

```javascript
let wasmReady = false;
let wasm = null;

async function ensureWasm() {
  if (wasmReady) return wasm;

  const wasmModule = await import("./wasm-pkg/ul_wasm.js");
  const wasmInit = wasmModule.default;
  await wasmInit();
  wasmModule.init();
  wasm = wasmModule;
  wasmReady = true;
  return wasm;
}

// Call when the user triggers a UL-related action
async function renderULScript(input) {
  const w = await ensureWasm();
  const girJson = w.parseUlScript(input);
  return w.renderSvg(girJson, 256, 256);
}
```

---

## 3. Web Components (CDN Embed)

Zero-build integration via custom HTML elements.

### 3.1 Quick Start

```html
<script src="https://cdn.jsdelivr.net/npm/@ul-forge/components/dist/ul-components.iife.js"></script>

<!-- Render a symbol -->
<ul-symbol script="○{● -> ●}" width="200" height="200"></ul-symbol>

<!-- Interactive composer -->
<ul-composer width="400" height="400"></ul-composer>

<!-- Dictionary lookup -->
<ul-dictionary query="existence"></ul-dictionary>
```

### 3.2 Component Attributes

**`<ul-symbol>`**

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `script` | string | — | UL-Script text to render |
| `gir` | string (JSON) | — | GIR JSON to render (alternative to `script`) |
| `width` | number | 128 | Viewport width |
| `height` | number | 128 | Viewport height |
| `theme` | string | `"dark"` | `"dark"` or `"light"` |

**`<ul-composer>`**

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `width` | number | 400 | Canvas width |
| `height` | number | 400 | Canvas height |
| `operations` | string | all | Comma-separated enabled operations |
| `on-compose` | function name | — | Callback on composition change |

**`<ul-dictionary>`**

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `query` | string | `""` | Search query |
| `show-symbol` | boolean | `true` | Show symbol renderings |
| `show-sigma` | boolean | `true` | Show Σ_UL operation |

### 3.3 Events

```javascript
document.querySelector("ul-composer").addEventListener("ul-compose", (e) => {
  console.log("New composition:", e.detail.gir);
  console.log("UL-Script:", e.detail.script);
  console.log("SVG:", e.detail.svg);
});
```

### 3.4 Reactive Updates

```javascript
const symbol = document.querySelector("ul-symbol");

// Update via attribute change — automatically re-renders
symbol.setAttribute("script", "○{∠90 -> ●}");

// Direct property access
console.log(symbol.gir);  // current GIR object
```

---

## 4. Game Engine Integration

The `layout()` function is the primary API for game engine integrations. It returns
positioned geometry coordinates instead of SVG strings, allowing any 2D/3D renderer
to draw UL glyphs natively.

### 4.1 Phaser.js Integration

```typescript
import Phaser from "phaser";
import * as core from "@ul-forge/core";

class ULGlyphScene extends Phaser.Scene {
  private glyphContainer!: Phaser.GameObjects.Container;

  async create() {
    await core.initialize();
    this.glyphContainer = this.add.container(0, 0);
    this.drawGlyph("○{● -> ●}");
  }

  drawGlyph(script: string) {
    this.glyphContainer.removeAll(true);
    const gir = core.parse(script);
    const positioned = core.layout(gir, 800, 600);

    // Draw elements
    for (const elem of positioned.elements) {
      if ("Point" in elem.shape) {
        const circle = this.add.circle(elem.x, elem.y, elem.shape.Point.radius, 0xffffff);
        this.glyphContainer.add(circle);
      } else if ("Circle" in elem.shape) {
        const circle = this.add.circle(elem.x, elem.y, elem.shape.Circle.radius);
        circle.setStrokeStyle(2, 0x00ff88);
        this.glyphContainer.add(circle);
      } else if ("Triangle" in elem.shape) {
        const size = elem.shape.Triangle.size;
        const tri = this.add.triangle(elem.x, elem.y, 0, -size, -size, size, size, size, 0x00ff88);
        tri.setStrokeStyle(2, 0x00ff88);
        this.glyphContainer.add(tri);
      } else if ("Square" in elem.shape) {
        const size = elem.shape.Square.size;
        const rect = this.add.rectangle(elem.x, elem.y, size * 2, size * 2);
        rect.setStrokeStyle(2, 0x00ff88);
        this.glyphContainer.add(rect);
      } else if ("Line" in elem.shape) {
        const l = elem.shape.Line;
        const line = this.add.line(0, 0, l.x1, l.y1, l.x2, l.y2, 0x888888);
        this.glyphContainer.add(line);
        if (l.directed) this.drawArrowhead(l.x1, l.y1, l.x2, l.y2);
      } else if ("Angle" in elem.shape) {
        const arc = this.add.arc(elem.x, elem.y, elem.shape.Angle.radius, 0, elem.shape.Angle.degrees);
        arc.setStrokeStyle(2, 0xffaa00);
        this.glyphContainer.add(arc);
      } else if ("Curve" in elem.shape) {
        const c = elem.shape.Curve;
        const points = this.generateCurvePoints(c.x1, c.y1, c.x2, c.y2, c.curvature);
        const curve = this.add.curve(0, 0, new Phaser.Curves.Spline(points));
        curve.setStrokeStyle(2, 0x8888ff);
        this.glyphContainer.add(curve);
      }
    }

    // Draw connections
    for (const conn of positioned.connections) {
      const style = conn.dashed ? 0x444444 : 0x888888;
      const line = this.add.line(0, 0, conn.x1, conn.y1, conn.x2, conn.y2, style);
      this.glyphContainer.add(line);
      if (conn.directed) this.drawArrowhead(conn.x1, conn.y1, conn.x2, conn.y2);
    }
  }

  drawArrowhead(x1: number, y1: number, x2: number, y2: number) {
    const angle = Math.atan2(y2 - y1, x2 - x1);
    const headLength = 10;
    const points = [
      x2, y2,
      x2 - headLength * Math.cos(angle - 0.5), y2 - headLength * Math.sin(angle - 0.5),
      x2 - headLength * Math.cos(angle + 0.5), y2 - headLength * Math.sin(angle + 0.5),
    ];
    const tri = this.add.triangle(0, 0, ...points, 0x888888);
    this.glyphContainer.add(tri);
  }

  generateCurvePoints(x1: number, y1: number, x2: number, y2: number, curvature: number) {
    const midX = (x1 + x2) / 2 + curvature * 50;
    const midY = (y1 + y2) / 2 - curvature * 50;
    return [new Phaser.Math.Vector2(x1, y1), new Phaser.Math.Vector2(midX, midY), new Phaser.Math.Vector2(x2, y2)];
  }
}
```

### 4.2 PixiJS Integration

```typescript
import * as PIXI from "pixi.js";
import * as core from "@ul-forge/core";

async function drawULGlyph(app: PIXI.Application, script: string) {
  await core.initialize();

  const gir = core.parse(script);
  const positioned = core.layout(gir, app.screen.width, app.screen.height);
  const graphics = new PIXI.Graphics();

  for (const elem of positioned.elements) {
    if ("Point" in elem.shape) {
      graphics.circle(elem.x, elem.y, elem.shape.Point.radius);
      graphics.fill(0xffffff);
    } else if ("Circle" in elem.shape) {
      graphics.circle(elem.x, elem.y, elem.shape.Circle.radius);
      graphics.stroke({ width: 2, color: 0x00ff88 });
    } else if ("Line" in elem.shape) {
      const l = elem.shape.Line;
      graphics.moveTo(l.x1, l.y1);
      graphics.lineTo(l.x2, l.y2);
      graphics.stroke({ width: 2, color: 0x888888 });
    }
    // ... handle all ShapeType variants
  }

  app.stage.addChild(graphics);
}
```

### 4.3 Canvas 2D (Raw)

```typescript
import * as core from "@ul-forge/core";

async function renderToCanvas(canvas: HTMLCanvasElement, script: string) {
  await core.initialize();

  const ctx = canvas.getContext("2d")!;
  const gir = core.parse(script);
  const positioned = core.layout(gir, canvas.width, canvas.height);

  ctx.clearRect(0, 0, canvas.width, canvas.height);

  for (const elem of positioned.elements) {
    if ("Point" in elem.shape) {
      ctx.beginPath();
      ctx.arc(elem.x, elem.y, elem.shape.Point.radius, 0, Math.PI * 2);
      ctx.fillStyle = "#ffffff";
      ctx.fill();
    } else if ("Circle" in elem.shape) {
      ctx.beginPath();
      ctx.arc(elem.x, elem.y, elem.shape.Circle.radius, 0, Math.PI * 2);
      ctx.strokeStyle = "#00ff88";
      ctx.lineWidth = 2;
      ctx.stroke();
    } else if ("Line" in elem.shape) {
      const l = elem.shape.Line;
      ctx.beginPath();
      ctx.moveTo(l.x1, l.y1);
      ctx.lineTo(l.x2, l.y2);
      ctx.strokeStyle = "#888888";
      ctx.lineWidth = 2;
      ctx.stroke();
    } else if ("Angle" in elem.shape) {
      const a = elem.shape.Angle;
      ctx.beginPath();
      ctx.arc(elem.x, elem.y, a.radius, 0, (a.degrees * Math.PI) / 180);
      ctx.strokeStyle = "#ffaa00";
      ctx.lineWidth = 2;
      ctx.stroke();
    } else if ("Curve" in elem.shape) {
      const c = elem.shape.Curve;
      ctx.beginPath();
      ctx.moveTo(c.x1, c.y1);
      const cpX = (c.x1 + c.x2) / 2 + c.curvature * 50;
      const cpY = (c.y1 + c.y2) / 2 - c.curvature * 50;
      ctx.quadraticCurveTo(cpX, cpY, c.x2, c.y2);
      ctx.strokeStyle = "#8888ff";
      ctx.lineWidth = 2;
      ctx.stroke();
    }
  }

  // Draw connections
  for (const conn of positioned.connections) {
    ctx.beginPath();
    ctx.moveTo(conn.x1, conn.y1);
    ctx.lineTo(conn.x2, conn.y2);
    ctx.strokeStyle = conn.dashed ? "#444444" : "#888888";
    ctx.setLineDash(conn.dashed ? [5, 5] : []);
    ctx.lineWidth = 1;
    ctx.stroke();
    ctx.setLineDash([]);

    if (conn.directed) {
      // Arrowhead
      const angle = Math.atan2(conn.y2 - conn.y1, conn.x2 - conn.x1);
      const headLen = 8;
      ctx.beginPath();
      ctx.moveTo(conn.x2, conn.y2);
      ctx.lineTo(
        conn.x2 - headLen * Math.cos(angle - 0.4),
        conn.y2 - headLen * Math.sin(angle - 0.4),
      );
      ctx.lineTo(
        conn.x2 - headLen * Math.cos(angle + 0.4),
        conn.y2 - headLen * Math.sin(angle + 0.4),
      );
      ctx.closePath();
      ctx.fillStyle = "#888888";
      ctx.fill();
    }
  }
}
```

### 4.4 Three.js (3D Extrusion)

Use `layout()` coordinates as 2D base, then extrude into 3D:

```typescript
import * as THREE from "three";
import * as core from "@ul-forge/core";

async function createULMesh(script: string, scene: THREE.Scene) {
  await core.initialize();
  const gir = core.parse(script);
  const positioned = core.layout(gir, 10, 10);  // Normalized coordinates

  const group = new THREE.Group();

  for (const elem of positioned.elements) {
    if ("Point" in elem.shape) {
      const geometry = new THREE.SphereGeometry(elem.shape.Point.radius * 0.01, 16, 16);
      const material = new THREE.MeshStandardMaterial({ color: 0xffffff });
      const mesh = new THREE.Mesh(geometry, material);
      mesh.position.set(elem.x * 0.01, elem.y * 0.01, 0);
      group.add(mesh);
    } else if ("Circle" in elem.shape) {
      const geometry = new THREE.TorusGeometry(elem.shape.Circle.radius * 0.01, 0.005, 8, 48);
      const material = new THREE.MeshStandardMaterial({ color: 0x00ff88 });
      const mesh = new THREE.Mesh(geometry, material);
      mesh.position.set(elem.x * 0.01, elem.y * 0.01, 0);
      group.add(mesh);
    }
  }

  scene.add(group);
  return group;
}
```

### 4.5 Animation with Game Engine

Use `getAnimationSequence()` for construction-order animations:

```typescript
async function animateConstruction(script: string) {
  const gir = core.parse(script);
  const seq = core.getAnimationSequence(gir, 800, 600);

  for (const kf of seq.keyframes) {
    // Schedule each keyframe
    setTimeout(() => {
      const element = getElementByNodeId(kf.node_id);
      animateElement(element, {
        x: kf.x,
        y: kf.y,
        scale: kf.scale,
        rotation: kf.rotation,
        opacity: kf.opacity,
        easing: kf.easing,
      });
    }, kf.timestamp_ms);
  }
}
```

---

## 5. Node.js / Server-Side

### 5.1 Synchronous Initialization

```typescript
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

async function initWasm() {
  const wasmModule = await import("./wasm-pkg/ul_wasm.js");
  const wasmBuffer = readFileSync(resolve("./wasm-pkg/ul_wasm_bg.wasm"));
  wasmModule.initSync({ module: new WebAssembly.Module(wasmBuffer) });
  wasmModule.init();
  return wasmModule;
}

const wasm = await initWasm();

// Use raw WASM functions
const girJson = wasm.parseUlScript("● -> ●");
const svg = wasm.renderSvg(girJson, 256, 256);
```

### 5.2 Express.js REST Handler

```typescript
import express from "express";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const app = express();
app.use(express.json());

let wasm: any;

async function initialize() {
  const wasmModule = await import("./wasm-pkg/ul_wasm.js");
  const wasmBuffer = readFileSync(resolve("./wasm-pkg/ul_wasm_bg.wasm"));
  wasmModule.initSync({ module: new WebAssembly.Module(wasmBuffer) });
  wasmModule.init();
  wasm = wasmModule;
}

app.post("/api/parse", (req, res) => {
  try {
    const girJson = wasm.parseUlScript(req.body.input);
    res.json(JSON.parse(girJson));
  } catch (err) {
    res.status(400).json({ error: String(err) });
  }
});

app.post("/api/render", (req, res) => {
  try {
    const { gir, width = 256, height = 256 } = req.body;
    const svg = wasm.renderSvg(JSON.stringify(gir), width, height);
    res.type("image/svg+xml").send(svg);
  } catch (err) {
    res.status(400).json({ error: String(err) });
  }
});

app.post("/api/validate", (req, res) => {
  try {
    const result = wasm.validateGir(JSON.stringify(req.body.gir), req.body.checkGeometry ?? false);
    res.json(result);
  } catch (err) {
    res.status(400).json({ error: String(err) });
  }
});

app.post("/api/operation", (req, res) => {
  try {
    const { operation, operands } = req.body;
    const operandJsons = operands.map((g: any) => JSON.stringify(g));
    const result = wasm.applyOperation(operation, JSON.stringify(operandJsons));
    res.json(JSON.parse(result));
  } catch (err) {
    res.status(400).json({ error: String(err) });
  }
});

initialize().then(() => {
  app.listen(3000, () => console.log("UL API ready on :3000"));
});
```

### 5.3 CLI Batch Processing

```typescript
#!/usr/bin/env node
// batch-render.ts — render multiple UL-Script files to SVG
import { readFileSync, writeFileSync } from "node:fs";
import { resolve, basename } from "node:path";

async function main() {
  const wasmModule = await import("./wasm-pkg/ul_wasm.js");
  const wasmBuffer = readFileSync(resolve("./wasm-pkg/ul_wasm_bg.wasm"));
  wasmModule.initSync({ module: new WebAssembly.Module(wasmBuffer) });
  wasmModule.init();

  const files = process.argv.slice(2);
  for (const file of files) {
    const input = readFileSync(file, "utf-8").trim();
    try {
      const girJson = wasmModule.parseUlScript(input);
      const svg = wasmModule.renderSvg(girJson, 512, 512);
      const outFile = basename(file, ".ul") + ".svg";
      writeFileSync(outFile, svg);
      console.log(`✓ ${file} → ${outFile}`);
    } catch (err) {
      console.error(`✗ ${file}: ${err}`);
    }
  }
}

main();
```

### 5.4 SSR (Server-Side Rendering)

Pre-render UL glyphs during build time for static sites:

```typescript
// build-static-glyphs.ts
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { resolve } from "node:path";

async function prerender() {
  const wasmModule = await import("./wasm-pkg/ul_wasm.js");
  const wasmBuffer = readFileSync(resolve("./wasm-pkg/ul_wasm_bg.wasm"));
  wasmModule.initSync({ module: new WebAssembly.Module(wasmBuffer) });
  wasmModule.init();

  const lexicon = JSON.parse(wasmModule.queryLexicon(""));
  mkdirSync("dist/glyphs", { recursive: true });

  for (const entry of lexicon) {
    try {
      const girJson = wasmModule.parseUlScript(entry.symbol);
      const svg = wasmModule.renderSvg(girJson, 128, 128);
      writeFileSync(`dist/glyphs/${entry.name}.svg`, svg);
    } catch {
      console.warn(`Could not render: ${entry.name}`);
    }
  }
}

prerender();
```

---

## 6. AI Agent Integration

### 6.1 MCP (Model Context Protocol)

The `ul-mcp` crate provides a native MCP server. For WASM-based agent integration,
expose the WASM module as an MCP tool provider:

```typescript
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";

async function main() {
  const wasm = await initWasm();  // as in §5.1

  const server = new Server({
    name: "ul-forge",
    version: "1.0.0",
  }, {
    capabilities: { tools: {} },
  });

  server.setRequestHandler("tools/list", async () => ({
    tools: [
      {
        name: "parse_ul",
        description: "Parse UL-Script text into a GIR (Graph Intermediate Representation)",
        inputSchema: {
          type: "object",
          properties: { input: { type: "string", description: "UL-Script text" } },
          required: ["input"],
        },
      },
      {
        name: "render_ul",
        description: "Render a UL GIR to SVG",
        inputSchema: {
          type: "object",
          properties: {
            gir: { type: "object", description: "GIR JSON object" },
            width: { type: "number", default: 256 },
            height: { type: "number", default: 256 },
          },
          required: ["gir"],
        },
      },
      {
        name: "compose_ul",
        description: "Apply a Σ_UL operation to GIR operands",
        inputSchema: {
          type: "object",
          properties: {
            operation: { type: "string", enum: ["predicate", "negate", "embed", "conjoin", "disjoin", "compose", "invert", "abstract", "modify_entity", "modify_relation", "quantify", "necessity", "possibility", "counterfactual"] },
            operands: { type: "array", items: { type: "object" } },
          },
          required: ["operation", "operands"],
        },
      },
    ],
  }));

  server.setRequestHandler("tools/call", async (request) => {
    const { name, arguments: args } = request.params;
    try {
      switch (name) {
        case "parse_ul":
          return { content: [{ type: "text", text: wasm.parseUlScript(args.input) }] };
        case "render_ul":
          return { content: [{ type: "text", text: wasm.renderSvg(JSON.stringify(args.gir), args.width ?? 256, args.height ?? 256) }] };
        case "compose_ul": {
          const operandJsons = args.operands.map((g: any) => JSON.stringify(g));
          return { content: [{ type: "text", text: wasm.applyOperation(args.operation, JSON.stringify(operandJsons)) }] };
        }
        default:
          throw new Error(`Unknown tool: ${name}`);
      }
    } catch (err) {
      return { content: [{ type: "text", text: `Error: ${err}` }], isError: true };
    }
  });

  const transport = new StdioServerTransport();
  await server.connect(transport);
}

main();
```

### 6.2 UL Transceiver Protocol

The UL Transceiver v1.0 wire format enables AI-to-AI communication using UL as
a shared meaning language:

```typescript
interface ULTransceiverMessage {
  version: "1.0";
  sender: string;       // agent identifier
  receiver: string;     // agent identifier or "*" for broadcast
  timestamp: string;    // ISO 8601
  payload: {
    type: "gir" | "script" | "query" | "response";
    content: any;       // GIR JSON, UL-Script text, or structured query
  };
  metadata?: {
    force?: ForceName;
    context_id?: number;
    correlation_id?: string;
  };
}
```

Agent conversation via WASM:

```typescript
function createMessage(script: string, force: ForceName): ULTransceiverMessage {
  const gir = core.parse(script);
  const forcedGir = core.setForce(gir, force);

  return {
    version: "1.0",
    sender: "agent-A",
    receiver: "agent-B",
    timestamp: new Date().toISOString(),
    payload: { type: "gir", content: forcedGir },
    metadata: { force },
  };
}

function processMessage(msg: ULTransceiverMessage): string {
  const gir = msg.payload.content as Gir;
  const ops = core.detectOperations(gir);
  const analysis = core.analyzeStructure(gir);
  const pragmatics = core.inferPragmatics(gir);

  // Build response based on inference
  return JSON.stringify({ operations: ops, analysis, pragmatics });
}
```

### 6.3 LLM Tool Integration (OpenAI Function Calling)

```typescript
const tools = [
  {
    type: "function",
    function: {
      name: "ul_parse",
      description: "Parse Universal Language script into a semantic graph",
      parameters: {
        type: "object",
        properties: {
          input: { type: "string", description: 'UL-Script (e.g., "● -> ●", "○{●}")' },
        },
        required: ["input"],
      },
    },
  },
  {
    type: "function",
    function: {
      name: "ul_compose",
      description: "Compose two semantic structures using a UL algebraic operation",
      parameters: {
        type: "object",
        properties: {
          operation: { type: "string", enum: ["predicate", "negate", "conjoin", "disjoin", "embed", "abstract", "compose", "invert", "modify_entity", "modify_relation", "quantify"] },
          operand_scripts: { type: "array", items: { type: "string" }, description: "UL-Script texts for each operand" },
        },
        required: ["operation", "operand_scripts"],
      },
    },
  },
];

// Handle tool calls
function handleToolCall(name: string, args: any): string {
  switch (name) {
    case "ul_parse":
      return JSON.stringify(core.parse(args.input));
    case "ul_compose": {
      const operands = args.operand_scripts.map((s: string) => core.parse(s));
      const result = core.applyOperation(args.operation, operands);
      return JSON.stringify(result);
    }
  }
}
```

---

## 7. Electron App

### 7.1 Main Process (Node.js)

```typescript
// main.ts — Electron main process
import { app, BrowserWindow, ipcMain } from "electron";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

let wasm: any;

async function initWasm() {
  const wasmModule = await import("./wasm-pkg/ul_wasm.js");
  const wasmBuffer = readFileSync(resolve(__dirname, "../wasm-pkg/ul_wasm_bg.wasm"));
  wasmModule.initSync({ module: new WebAssembly.Module(wasmBuffer) });
  wasmModule.init();
  wasm = wasmModule;
}

app.whenReady().then(async () => {
  await initWasm();

  ipcMain.handle("ul:parse", (_event, input: string) => {
    return wasm.parseUlScript(input);
  });

  ipcMain.handle("ul:render", (_event, girJson: string, width: number, height: number) => {
    return wasm.renderSvg(girJson, width, height);
  });

  ipcMain.handle("ul:validate", (_event, girJson: string) => {
    return wasm.validateGir(girJson, false);
  });

  const win = new BrowserWindow({
    webPreferences: { preload: resolve(__dirname, "preload.js") },
  });
  win.loadFile("index.html");
});
```

### 7.2 Preload + Renderer

```typescript
// preload.ts
import { contextBridge, ipcRenderer } from "electron";

contextBridge.exposeInMainWorld("ulForge", {
  parse: (input: string) => ipcRenderer.invoke("ul:parse", input),
  render: (girJson: string, w: number, h: number) => ipcRenderer.invoke("ul:render", girJson, w, h),
  validate: (girJson: string) => ipcRenderer.invoke("ul:validate", girJson),
});

// renderer.ts
const gir = await window.ulForge.parse("● -> ●");
const svg = await window.ulForge.render(gir, 400, 400);
document.getElementById("preview")!.innerHTML = svg;
```

---

## 8. iframe Embedding

Embed the UL editor in any page:

```html
<iframe
  src="https://ul-forge.example.com/embed?script=○{●}"
  width="400"
  height="300"
  sandbox="allow-scripts allow-same-origin"
  style="border: 1px solid #333; border-radius: 4px;">
</iframe>
```

For live communication with the host page, use `postMessage`:

```javascript
// Inside iframe: post GIR updates
window.parent.postMessage({
  type: "ul-gir-update",
  gir: currentGir,
  svg: currentSvg,
}, "*");

// Host page: receive updates
window.addEventListener("message", (event) => {
  if (event.data.type === "ul-gir-update") {
    console.log("New GIR:", event.data.gir);
    document.getElementById("preview").innerHTML = event.data.svg;
  }
});
```

---

## 9. Web Worker (Off-Thread)

For complex operations that might block the main thread (e.g., large GIR analysis,
batch rendering), offload to a Web Worker:

### 9.1 Worker File

```typescript
// ul-worker.ts
import wasmInit, {
  init as wasmModuleInit,
  parseUlScript,
  renderSvg,
  analyzeStructure,
} from "./wasm-pkg/ul_wasm.js";

let ready = false;

self.onmessage = async (event) => {
  if (!ready) {
    await wasmInit();
    wasmModuleInit();
    ready = true;
  }

  const { id, action, params } = event.data;

  try {
    let result: any;
    switch (action) {
      case "parse":
        result = parseUlScript(params.input);
        break;
      case "render":
        result = renderSvg(params.girJson, params.width, params.height);
        break;
      case "analyze":
        result = analyzeStructure(params.girJson);
        break;
      case "batch-render":
        result = params.scripts.map((s: string) => {
          try {
            const girJson = parseUlScript(s);
            return { script: s, svg: renderSvg(girJson, params.width, params.height) };
          } catch (err) {
            return { script: s, error: String(err) };
          }
        });
        break;
    }
    self.postMessage({ id, result });
  } catch (err) {
    self.postMessage({ id, error: String(err) });
  }
};
```

### 9.2 Main Thread

```typescript
const worker = new Worker(new URL("./ul-worker.ts", import.meta.url), { type: "module" });

let nextId = 0;
const pending = new Map<number, { resolve: Function; reject: Function }>();

worker.onmessage = (event) => {
  const { id, result, error } = event.data;
  const p = pending.get(id);
  if (p) {
    pending.delete(id);
    if (error) p.reject(new Error(error));
    else p.resolve(result);
  }
};

function callWorker(action: string, params: any): Promise<any> {
  return new Promise((resolve, reject) => {
    const id = nextId++;
    pending.set(id, { resolve, reject });
    worker.postMessage({ id, action, params });
  });
}

// Usage
const girJson = await callWorker("parse", { input: "○{● -> ●}" });
const svg = await callWorker("render", { girJson, width: 512, height: 512 });

// Batch render — all on worker thread
const results = await callWorker("batch-render", {
  scripts: ["●", "● -> ●", "○{●}", "○{● -> ●}", "∠90", "~"],
  width: 64,
  height: 64,
});
```

---

## 10. Testing Harness

### 10.1 Vitest (Recommended)

The project uses Vitest for all testing. WASM tests use the `node` environment:

```typescript
// vitest.config.ts
export default defineConfig({
  test: {
    environmentMatchGlobs: [
      ["src/__tests__/*wasm*", "node"],
      ["src/__tests__/*integration*", "node"],
      ["src/__tests__/*App*", "jsdom"],
    ],
  },
});
```

### 10.2 Jest Configuration

If using Jest instead of Vitest:

```javascript
// jest.config.js
module.exports = {
  testEnvironment: "node",
  transform: {
    "^.+\\.tsx?$": "ts-jest",
  },
  moduleNameMapper: {
    "^ul-wasm$": "<rootDir>/wasm-pkg/ul_wasm.js",
    "^@ul-forge/core$": "<rootDir>/src/core/index.ts",
  },
  setupFilesAfterSetup: ["<rootDir>/tests/setup-wasm.ts"],
};
```

```typescript
// tests/setup-wasm.ts
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

beforeAll(async () => {
  const wasmModule = await import("../wasm-pkg/ul_wasm.js");
  const wasmBuffer = readFileSync(resolve(__dirname, "../wasm-pkg/ul_wasm_bg.wasm"));
  wasmModule.initSync({ module: new WebAssembly.Module(wasmBuffer) });
  wasmModule.init();
});
```

### 10.3 Playwright / E2E

For end-to-end testing with real browser WASM initialization:

```typescript
import { test, expect } from "@playwright/test";

test("renders UL glyph", async ({ page }) => {
  await page.goto("http://localhost:5173");

  // Wait for WASM initialization
  await page.waitForFunction(() => window.__UL_WASM_READY === true, { timeout: 10000 });

  // Type UL-Script
  const editor = page.locator(".monaco-editor textarea");
  await editor.fill("○{● -> ●}");

  // Check SVG rendered
  const svg = page.locator("#preview svg");
  await expect(svg).toBeVisible();
  await expect(svg).toHaveAttribute("width", "256");
});
```

---

## 11. Build System Integration

### 11.1 Webpack

```javascript
// webpack.config.js
const path = require("path");

module.exports = {
  experiments: {
    asyncWebAssembly: true,
  },
  resolve: {
    alias: {
      "ul-wasm": path.resolve(__dirname, "wasm-pkg/ul_wasm.js"),
    },
  },
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: "asset/resource",
      },
    ],
  },
};
```

### 11.2 Rollup

```javascript
// rollup.config.js
import { wasm } from "@rollup/plugin-wasm";
import resolve from "@rollup/plugin-node-resolve";

export default {
  input: "src/index.js",
  output: { dir: "dist", format: "es" },
  plugins: [wasm(), resolve()],
};
```

### 11.3 esbuild

```bash
esbuild src/index.ts --bundle --outfile=dist/index.js --format=esm \
  --external:./wasm-pkg/ul_wasm_bg.wasm
```

Copy the WASM binary separately:

```bash
cp wasm-pkg/ul_wasm_bg.wasm dist/
```

### 11.4 Deno

```typescript
import wasmInit, { init as wasmModuleInit, parseUlScript, renderSvg } from "./wasm-pkg/ul_wasm.js";

// Deno supports fetch-based WASM loading out of the box
await wasmInit();
wasmModuleInit();

const girJson = parseUlScript("● -> ●");
const svg = renderSvg(girJson, 256, 256);
console.log(svg);
```

---

## 12. Performance Optimization

### 12.1 Minimize Serialization

The biggest performance cost is JSON serialization at the WASM boundary.
Minimize it by:

- **Caching parsed GIRs.** Parse once, reuse the `Gir` object for render/validate/analyze.
- **Using the wrapper's caches.** render, layout, evaluation, and structure results are cached.
- **Batching operations.** Instead of parse → render for each keystroke, debounce user input.

### 12.2 Use `layout()` Instead of `render()` for Games

`render()` returns an SVG string that must be parsed by the DOM.
`layout()` returns raw coordinates — use it for game engines where you draw
with Canvas 2D, WebGL, or a framework's scene graph.

### 12.3 Streaming Compilation

In modern browsers, use `WebAssembly.instantiateStreaming` for faster startup:

```javascript
// The wasm-bindgen glue already does this when available
const response = fetch("ul_wasm_bg.wasm");
const result = await WebAssembly.instantiateStreaming(response, importObject);
```

### 12.4 Binary Size Reduction

If binary size is critical:

```toml
# Cargo.toml
[profile.release]
opt-level = "z"       # Optimize for size instead of speed
lto = true
codegen-units = 1
strip = true
```

Additional tools:

```bash
# Install wasm-opt
npm install -g wasm-opt

# Optimize the binary
wasm-opt -Os web/wasm-pkg/ul_wasm_bg.wasm -o web/wasm-pkg/ul_wasm_bg.wasm
```

### 12.5 Lazy Loading

For large applications where UL rendering is a secondary feature:

```typescript
let core: typeof import("@ul-forge/core") | null = null;

async function getCore() {
  if (!core) {
    core = await import("@ul-forge/core");
    await core.initialize();
  }
  return core;
}

// Only loads WASM when first UL feature is used
button.addEventListener("click", async () => {
  const c = await getCore();
  const svg = c.parseAndRender(input.value);
  preview.innerHTML = svg;
});
```

---

*Last updated: 2026-04-09*
