# TypeScript API Reference — `@ul-forge/core`

> Complete typed function signatures, parameter descriptions, return types,
> and usage examples for every function exported by the UL Forge TypeScript
> wrapper layer (`web/src/core/index.ts`).

---

## Table of Contents

1. [Initialization](#1-initialization)
2. [Parsing & Deparsing](#2-parsing--deparsing)
3. [Validation](#3-validation)
4. [Rendering](#4-rendering)
5. [SVG Round-Trip](#5-svg-round-trip)
6. [Σ_UL Operations](#6-σ_ul-operations)
7. [Structural Analysis](#7-structural-analysis)
8. [Game Context Lifecycle](#8-game-context-lifecycle)
9. [Scoring & Evaluation](#9-scoring--evaluation)
10. [Animation & Layout](#10-animation--layout)
11. [Teaching System](#11-teaching-system)
12. [Lexicon](#12-lexicon)
13. [Performative & Pragmatic Extensions](#13-performative--pragmatic-extensions)
14. [Cache Management](#14-cache-management)
15. [Type Catalog](#15-type-catalog)

---

## 1. Initialization

### `initialize(): Promise<void>`

Load and compile the WASM binary, then initialize Rust-side state.
Must be called before any other function. Safe to call multiple times (idempotent).

```typescript
import * as core from "@ul-forge/core";

await core.initialize();
// Now all functions are available
```

**Throws:** Network/compilation errors if the `.wasm` file cannot be loaded.

### `_resetForTesting(): void`

Reset the initialization flag. **For testing only.** After calling this, the next call
to any function will throw `"WASM not initialized"` until `initialize()` is called again.

```typescript
core._resetForTesting();
// core.parse("●")  ← would throw "WASM not initialized"
await core.initialize();
// core.parse("●")  ← works again
```

---

## 2. Parsing & Deparsing

### `parse(input: string): Gir`

Parse UL-Script text into a GIR document.

| Parameter | Type | Description |
|-----------|------|-------------|
| `input` | `string` | UL-Script text (e.g., `"●"`, `"○{● -> ●}"`, `"∠90"`) |

**Returns:** `Gir` — the parsed graph intermediate representation.

**Throws:** If the input is invalid UL-Script (parse error with line/column info).

```typescript
const point = core.parse("●");
// { ul_gir: "0.1.0", root: "n1", nodes: [{ id: "n1", type: "point", sort: "entity" }], edges: [] }

const predicate = core.parse("● -> ●");
// 4 nodes (subject, relation, object, predicate root), 5 edges

const enclosure = core.parse("○{● -> ●}");
// Enclosure wrapping a predicate structure
```

**Valid UL-Script syntax:**

| Token | Meaning | Sort |
|-------|---------|------|
| `●` | Point (entity) | entity |
| `->` or `→` | Directed line (relation) | relation |
| `∠N` | Angle with measure N (e.g., `∠90`, `∠45`) | modifier |
| `~` | Curve (process) | relation |
| `○{...}` | Circle enclosure | entity |
| `△{...}` | Triangle enclosure | entity |
| `□{...}` | Square enclosure | entity |
| `● -> ●` | Predicate (subject → object) | assertion |
| `∠90 -> ●` | Modified predicate | assertion |

**Important syntax rules:**
- `∠` **requires** a numeric argument: `∠90` works, bare `∠` does not
- `●──●` (em-dash connection) **does not work** — use `● -> ●` with ASCII arrow
- Empty string `""` is accepted by the parser (does not throw)

### `deparse(gir: Gir): string`

Convert a GIR back to canonical UL-Script text.

| Parameter | Type | Description |
|-----------|------|-------------|
| `gir` | `Gir` | The GIR to deparse |

**Returns:** `string` — canonical UL-Script text.

```typescript
const gir = core.parse("○{●}");
const text = core.deparse(gir);
// "○{●}"
```

**Round-trip guarantee:** `deparse(parse(text))` produces canonical text. Running
`parse(deparse(gir))` produces a GIR equivalent to the original. After one
normalization cycle, the output is stable:

```typescript
const t1 = core.deparse(core.parse(input));
const t2 = core.deparse(core.parse(t1));
// t1 === t2  (idempotent after first cycle)
```

---

## 3. Validation

### `validate(gir: Gir, checkGeometry?: boolean): ValidationResult`

Run the 4-layer validation pipeline against a GIR.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `gir` | `Gir` | — | The GIR to validate |
| `checkGeometry` | `boolean` | `false` | Enable geometry-layer checks |

**Returns:** `ValidationResult`

```typescript
interface ValidationResult {
  valid: boolean;           // true if no errors at any layer
  errors: string[];         // aggregated error messages
  warnings: string[];       // non-fatal warnings
  layers: ValidationLayers; // per-layer breakdown
}

interface ValidationLayers {
  schema: string[];     // JSON schema violations
  sort: string[];       // sort constraint violations
  invariant: string[];  // structural invariant violations
  geometry: string[];   // geometric constraint violations (if checked)
}
```

**Example:**

```typescript
const gir = core.parse("●");
const result = core.validate(gir);
// { valid: true, errors: [], warnings: [], layers: { schema: [], sort: [], invariant: [], geometry: [] } }
```

**Known behavior:** GIRs produced by `applyOperation("predicate", ...)`,
`applyOperation("quantify", ...)`, and `setForce(...)` fail validation with
`"sort violation: contains edge from assertion to entity — contains source must be entity"`.
This is a structural limitation in the current WASM validator — predicate and quantify
operations create `contains` edges from assertion-sort nodes, but the validator requires
`contains` edges to originate from entity-sort nodes. See
[Known Issues](known-issues.md#predicate-validation) for details.

---

## 4. Rendering

### `render(gir: Gir, width?: number, height?: number): string`

Render a GIR to an SVG string. Results are cached.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `gir` | `Gir` | — | The GIR to render |
| `width` | `number` | `256` | SVG viewport width in pixels |
| `height` | `number` | `256` | SVG viewport height in pixels |

**Returns:** `string` — SVG markup with embedded GIR metadata in a `<![CDATA[...]]>` section.

```typescript
const svg = core.render(core.parse("●"), 512, 512);
// "<svg xmlns="http://www.w3.org/2000/svg" width="512" height="512">..."
```

**Caching:** Results are cached by `girHash + width + height`. Call `clearCaches()`
to invalidate.

### `renderPreview(gir: Gir): string`

Render a compact 64×64 SVG preview suitable for thumbnails and palette displays.

| Parameter | Type | Description |
|-----------|------|-------------|
| `gir` | `Gir` | The GIR to render |

**Returns:** `string` — compact SVG string.

### `parseAndRender(input: string): string`

Convenience: parse UL-Script directly to SVG in one call.

```typescript
const svg = core.parseAndRender("○{● -> ●}");
```

### `parseValidateRender(input: string): { gir: Gir; validation: ValidationResult; svg: string | null }`

Convenience: parse, validate, and (if valid) render in one call.

```typescript
const { gir, validation, svg } = core.parseValidateRender("○{●}");
if (validation.valid) {
  document.getElementById("preview")!.innerHTML = svg!;
}
```

**Returns `svg: null`** if validation fails.

---

## 5. SVG Round-Trip

### `extractGirFromSvg(svg: string): Gir | null`

Extract embedded GIR metadata from an SVG string produced by `render()`.

| Parameter | Type | Description |
|-----------|------|-------------|
| `svg` | `string` | SVG string with potential `<![CDATA[...]]>` GIR metadata |

**Returns:** `Gir | null` — the embedded GIR, or `null` if none found.

**Implementation:** Uses regex `/<!\[CDATA\[([\s\S]*?)\]\]>/` to extract the JSON.

```typescript
const svg = core.render(core.parse("●"));
const recovered = core.extractGirFromSvg(svg);
// recovered ≈ original GIR (lossless round-trip)
```

**Lossless round-trip chain:**

```
parse("○{●}") → render(gir) → extractGirFromSvg(svg) → deparse(gir) === "○{●}"
```

### `svgToUlScript(svg: string): string | null`

Full SVG → UL-Script text conversion.

```typescript
const text = core.svgToUlScript(someSvgString);
// "○{●}" or null if no embedded GIR
```

---

## 6. Σ_UL Operations

### `applyOperation(operation: OperationName, operands: Gir[]): Gir`

Apply one of the 13 Σ_UL algebraic operations (plus modal and performative extensions)
to GIR operands.

| Parameter | Type | Description |
|-----------|------|-------------|
| `operation` | `OperationName` | The operation to apply |
| `operands` | `Gir[]` | Array of GIR operands (1–3 depending on operation) |

**Returns:** `Gir` — the composed result.

**Operation signatures:**

| Operation | Operands | Input Sorts | Output Sort | Description |
|-----------|----------|-------------|-------------|-------------|
| `"predicate"` | 3 | (entity, relation, entity) | assertion | Subject-relation-object |
| `"modify_entity"` | 2 | (modifier, entity) | entity | Apply modifier to entity |
| `"modify_relation"` | 2 | (modifier, relation) | relation | Apply modifier to relation |
| `"negate"` | 1 | (assertion) | assertion | Negate a statement |
| `"conjoin"` | 2 | (assertion, assertion) | assertion | AND combination |
| `"disjoin"` | 2 | (assertion, assertion) | assertion | OR combination |
| `"embed"` | 1 | (assertion) | entity | Nominalization |
| `"abstract"` | 1 | (entity) | modifier | Adjectivalization |
| `"compose"` | 2 | (relation, relation) | relation | Chain relations |
| `"invert"` | 1 | (relation) | relation | Reverse direction |
| `"quantify"` | 2 | (modifier, entity) | assertion | Graduated quantification |
| `"necessity"` | 1 | (assertion) | entity | □ modal operator |
| `"possibility"` | 1 | (assertion) | entity | ◇ modal operator |
| `"counterfactual"` | 1 | (assertion) | entity | □→ modal operator |

```typescript
const e = core.parse("●");           // entity
const r = core.parse("->");           // relation
const m = core.parse("∠90");         // modifier

// Build a predicate: ● -> ●
const pred = core.applyOperation("predicate", [e, r, e]);

// Negate it: ¬(● -> ●)
const neg = core.applyOperation("negate", [pred]);

// Embed into entity: [¬(● -> ●)] as entity
const emb = core.applyOperation("embed", [neg]);

// Compose relations: -> ∘ ->
const comp = core.applyOperation("compose", [r, r]);

// Modal: □(● -> ●) — returns entity sort (enclosure)
const nec = core.applyOperation("necessity", [pred]);
```

**Important notes:**
- Modal operations (`necessity`, `possibility`, `counterfactual`) return **entity** sort
  (enclosure), not assertion
- `quantify` returns **assertion** sort
- `set_force` is a separate function, not an operation — see §13

### `composeGir(a: Gir, b: Gir, operation: OperationName): Gir`

Binary composition shorthand — combine two GIRs with a named operation.

```typescript
const conj = core.composeGir(pred1, pred2, "conjoin");
```

### `detectOperations(gir: Gir): string[]`

Reverse-engineer which Σ_UL operations are expressed in a GIR.

```typescript
const ops = core.detectOperations(core.parse("○{● -> ●}"));
// ["predicate", "embed"]  (or similar — depends on structure)
```

---

## 7. Structural Analysis

### `analyzeStructure(gir: Gir): StructureReport`

Compute a comprehensive structural analysis report. Results are cached.

**Returns:** `StructureReport`

```typescript
interface StructureReport {
  node_count: number;
  edge_count: number;
  primitive_counts: Record<string, number>;     // ⚠️ Runtime: Map<string, number>
  sort_distribution: Record<string, number>;    // ⚠️ Runtime: Map<string, number>
  detected_operations: string[];
  depth: number;
  breadth: number;
  complexity_score: number;
  symmetry_group: SymmetryGroup;
  part_of_speech: PartOfSpeech;
  node_symmetries: Record<string, SymmetryGroup>; // ⚠️ Runtime: Map<string, SymmetryGroup>
}
```

**⚠️ Critical: `Map` vs `Object` at runtime.** The `primitive_counts`, `sort_distribution`,
and `node_symmetries` fields are declared as `Record<string, T>` in TypeScript but
arrive as JavaScript `Map` objects at runtime (due to `serde-wasm-bindgen` serializing
Rust `HashMap` as JS `Map`). Use `.get()` method, not property access:

```typescript
const report = core.analyzeStructure(core.parse("●"));

// ❌ WRONG — returns undefined
report.primitive_counts["point"];

// ✅ CORRECT — use Map access
(report.primitive_counts as unknown as Map<string, number>).get("point"); // 1
```

### `compareGir(a: Gir, b: Gir, level: ErlangenLevel): EquivalenceResult`

Compare two GIR structures at a specified Erlangen equivalence level.

| Level | Strictness | Preserves |
|-------|-----------|-----------|
| `"euclidean"` | Strictest | Types, shapes, angles, sizes |
| `"similarity"` | — | Ignore absolute scale |
| `"affine"` | — | Ignore proportions, keep containment |
| `"projective"` | — | Incidence structure only |
| `"topological"` | Loosest | Graph isomorphism of types |

```typescript
interface EquivalenceResult {
  level: ErlangenLevel;
  score: number;           // 0.0–1.0 similarity
  equivalent: boolean;     // true if score === 1.0 at this level
  dimensions: EquivalenceDimensions;
}

interface EquivalenceDimensions {
  topology: number;
  types: number;
  sorts: number;
  shapes: number;
  metrics: number;
}
```

```typescript
const a = core.parse("●");
const b = core.parse("●");
const eq = core.compareGir(a, b, "topological");
// { level: "topological", score: 1.0, equivalent: true, dimensions: { ... } }
```

**Note:** `compareGir` is direction-insensitive at all levels.

---

## 8. Game Context Lifecycle

### `createContext(config?: GameConfig): number`

Create a new game session context. Returns a numeric context ID.

```typescript
interface GameConfig {
  rules_json?: string;    // Custom composition rules
  session_id?: string;    // Session identifier for tracking
}

const ctxId = core.createContext();                           // default config
const ctxId2 = core.createContext({ session_id: "tutorial" }); // named session
```

**Context storage:** Contexts are stored in WASM thread-local storage. They persist
for the lifetime of the WASM module (i.e., the page/process). There is no explicit
`destroyContext()` — contexts are cleaned up when the WASM module is unloaded.

### `loadCustomDefinitions(ctxId: number, rulesJson: string): LoadResult`

Inject custom composition rules into an existing context at runtime.

```typescript
interface LoadResult {
  loaded: number;     // number of rules successfully loaded
  errors: string[];   // any rule loading errors
}
```

**After loading custom definitions, clear the evaluation cache:**

```typescript
const result = core.loadCustomDefinitions(ctxId, rulesJson);
core.clearCaches(); // evaluation cache may be stale
```

---

## 9. Scoring & Evaluation

### `evaluate(ctxId: number, gir: Gir): EvaluationResult`

Evaluate a GIR against all active composition rules in a context.
Results are cached by context ID + GIR hash.

```typescript
interface EvaluationResult {
  score: number;            // 0.0–1.0
  grade: Grade;             // "exact" | "close" | "partial" | "unrelated"
  matched_rules: string[];  // rules that matched
  violated_rules: string[]; // rules that were violated
  feedback: string[];       // human-readable feedback messages
}
```

### `scoreComposition(ctxId: number, gir: Gir, targetJson: string): ScoreResult`

Score a composition against a specific puzzle target.

| Parameter | Type | Description |
|-----------|------|-------------|
| `ctxId` | `number` | Game context ID |
| `gir` | `Gir` | The student's composition |
| `targetJson` | `string` | **Must be** `JSON.stringify({ expected_gir_json: girJsonString })` |

**⚠️ Critical: Target format.** The `targetJson` parameter is NOT a raw GIR JSON string.
It must be a JSON object with an `expected_gir_json` field containing the GIR as a
JSON string:

```typescript
// ✅ CORRECT
const target = core.parse("○{●}");
const targetJson = JSON.stringify({ expected_gir_json: JSON.stringify(target) });
const score = core.scoreComposition(ctxId, attempt, targetJson);

// ❌ WRONG — will throw "missing field expected_gir_json"
const score = core.scoreComposition(ctxId, attempt, JSON.stringify(target));
```

```typescript
interface ScoreResult {
  score: number;                  // 0.0–1.0
  grade: Grade;
  partial_credit: PartialCredit;
  feedback: string[];
}

interface PartialCredit {
  structural_match: number;       // 0.0–1.0
  sort_correctness: number;       // 0.0–1.0
  operation_correctness: number;  // 0.0–1.0
  sequence_order: number;         // 0.0–1.0
}
```

### `evaluateJaneAttempt(ctxId: number, attempt: Gir, expected: Gir): JaneResult`

Evaluate a learning attempt with proficiency tracking (ProtoFusionGirl integration).

```typescript
interface JaneResult {
  score: number;
  grade: Grade;
  improvements: string[];
  proficiency_delta: Record<string, number>;  // per-operation proficiency change
}
```

### `validateSequence(ctxId: number, glyphs: Gir[]): SequenceResult`

Validate ordering constraints across a sequence of GIRs.

```typescript
interface SequenceResult {
  valid: boolean;
  errors: string[];
  pair_scores: number[];  // pairwise scores between consecutive glyphs
}
```

---

## 10. Animation & Layout

### `layout(gir: Gir, width: number, height: number): PositionedGlyph`

Compute raw positioned geometry for a GIR. Results are cached.

**This is the primary integration point for game engines** (Phaser, PixiJS, Babylon.js,
Three.js, etc.) where you need raw coordinates and shape parameters — not SVG strings.

```typescript
interface PositionedGlyph {
  elements: PositionedElement[];
  connections: LayoutConnection[];
  width: number;
  height: number;
}

interface PositionedElement {
  node_id: string;
  x: number;           // center X coordinate
  y: number;           // center Y coordinate
  shape: ShapeType;    // discriminated union — see below
}

interface LayoutConnection {
  edge_id: string;
  x1: number; y1: number;
  x2: number; y2: number;
  directed: boolean;
  dashed: boolean;
}
```

**`ShapeType` discriminated union:**

```typescript
type ShapeType =
  | { Point: { radius: number } }
  | { Circle: { radius: number } }
  | { Triangle: { size: number } }
  | { Square: { size: number } }
  | { Pentagon: { size: number } }
  | { Hexagon: { size: number } }
  | { Line: { x1: number; y1: number; x2: number; y2: number; directed: boolean } }
  | { Angle: { radius: number; degrees: number } }
  | { Curve: { x1: number; y1: number; x2: number; y2: number; curvature: number } };
```

**Usage in a game engine:**

```typescript
const gir = core.parse("○{● -> ●}");
const positioned = core.layout(gir, 800, 600);

for (const elem of positioned.elements) {
  if ("Point" in elem.shape) {
    // Draw a filled circle at (elem.x, elem.y) with radius elem.shape.Point.radius
    this.add.circle(elem.x, elem.y, elem.shape.Point.radius, 0xffffff);
  } else if ("Circle" in elem.shape) {
    // Draw an unfilled circle (enclosure)
    this.add.circle(elem.x, elem.y, elem.shape.Circle.radius).setStrokeStyle(2, 0xffffff);
  } else if ("Line" in elem.shape) {
    const line = elem.shape.Line;
    this.add.line(0, 0, line.x1, line.y1, line.x2, line.y2, 0xffffff);
    if (line.directed) {
      // Draw arrowhead
    }
  }
  // ... handle all ShapeType variants
}

for (const conn of positioned.connections) {
  this.add.line(0, 0, conn.x1, conn.y1, conn.x2, conn.y2, 0x888888);
}
```

### `getAnimationSequence(gir: Gir, width: number, height: number): AnimationSequence`

Generate construction-order animation keyframes for animating how a glyph is built
step by step.

```typescript
interface AnimationSequence {
  keyframes: AnimationKeyframe[];
  total_duration_ms: number;
}

interface AnimationKeyframe {
  node_id: string;
  timestamp_ms: number;
  x: number;
  y: number;
  scale: number;       // 0 → 1 (appear) or 1 → 0 (disappear)
  rotation: number;    // radians
  opacity: number;     // 0.0–1.0
  easing: Easing;      // "linear" | "ease_in" | "ease_out" | "ease_in_out"
}
```

---

## 11. Teaching System

### `getHints(attempt: Gir, target: Gir): Hint[]`

Generate contextual teaching hints by comparing a student's attempt to a target GIR.

```typescript
interface Hint {
  severity: string;    // "info" | "warning" | "error"
  category: string;    // hint category
  message: string;     // human-readable hint
}
```

### `analyzeHints(gir: Gir): Hint[]`

Generate standalone analysis hints for a GIR (no target needed).

### `getNextPuzzle(proficiency: Record<string, number>): Puzzle`

Get the next appropriate puzzle based on student proficiency levels.

```typescript
interface Puzzle {
  id: string;
  difficulty: number;
  level: number;
  required_operations: string[];
  description: string;
  target_gir_json: string;          // the target GIR as JSON string
}
```

```typescript
const puzzle = core.getNextPuzzle({ predicate: 0.7, negate: 0.3, embed: 0.5 });
const targetGir = JSON.parse(puzzle.target_gir_json);
```

---

## 12. Lexicon

### `queryLexicon(query: string): LexiconEntry[]`

Search the 42-entry canonical UL lexicon by query string.

```typescript
interface LexiconEntry {
  id: number;
  level: number;        // difficulty tier
  name: string;         // canonical name (e.g., "existence")
  tier: string;         // "forced" | "distinguished" | "conventional"
  symbol: string;       // UL-Script representation
  sigma_ul: string;     // Σ_UL operation or primitive
  labels: string[];     // semantic labels
}
```

```typescript
const entries = core.queryLexicon("exist");
// [{ id: 1, name: "existence", symbol: "●", tier: "forced", ... }]
```

### `lookupLexiconEntry(name: string): LexiconEntry | null`

Look up a specific lexicon entry by exact name (case-insensitive).

```typescript
const entry = core.lookupLexiconEntry("negation");
// { id: 4, name: "negation", symbol: "...", sigma_ul: "negate", ... }
```

---

## 13. Performative & Pragmatic Extensions

### `setForce(gir: Gir, force: ForceName): Gir`

Set the illocutionary force on an assertion GIR.

| Force | Speech Act | Example |
|-------|-----------|---------|
| `"assert"` | Statement of fact | "The cat is on the mat" |
| `"query"` | Question | "Is the cat on the mat?" |
| `"direct"` | Command/request | "Put the cat on the mat" |
| `"commit"` | Promise/commitment | "I will put the cat on the mat" |
| `"express"` | Emotional expression | "How nice that the cat is on the mat!" |
| `"declare"` | Performative declaration | "I hereby name this cat Fluffy" |

```typescript
const pred = core.applyOperation("predicate", [e, r, e]);
const question = core.setForce(pred, "query");
```

**Known behavior:**
- `setForce` returns GIR with `ul_gir: "1.0"` (different from parser's `"0.1.0"`)
- The result **fails validation** due to contains-assertion edge violations
- The result is still usable for rendering, deparsing, and further operations

### `inferPragmatics(gir: Gir): PragmaticInference[]`

Run pragmatic inference rules on a surface GIR to derive intended meanings.

```typescript
interface PragmaticInference {
  rule: string;       // name of the pragmatic rule applied
  surface: Gir;       // the surface (literal) meaning
  intended: Gir;      // the inferred (pragmatic) meaning
}
```

---

## 14. Cache Management

### `clearCaches(): void`

Clear all 4 result caches (render, layout, evaluation, structure).

```typescript
core.clearCaches();
```

**When to call:**
- After `loadCustomDefinitions()` (evaluation rules changed)
- When switching to a new document or game context
- When you suspect cache staleness
- Before measuring performance (to avoid cache hits skewing results)

---

## 15. Type Catalog

### Core GIR Types

```typescript
type Sort = "entity" | "relation" | "modifier" | "assertion";
type NodeType = "point" | "line" | "angle" | "curve" | "enclosure";
type EnclosureShape = "circle" | "triangle" | "square" | "ellipse" | "polygon" | "freeform";
type EdgeType = "contains" | "modified_by" | "adjacent" | "intersects"
              | "connects" | "references" | "binds" | "accessible_from";
```

### Operation Types

```typescript
type OperationName =
  | "predicate" | "modify_entity" | "modify_relation"
  | "negate" | "conjoin" | "disjoin"
  | "embed" | "abstract"
  | "compose" | "invert"
  | "quantify"
  | "necessity" | "possibility" | "counterfactual"
  | "set_force" | "infer_pragmatic";

type ForceName = "assert" | "query" | "direct" | "commit" | "express" | "declare";
```

### Game Types

```typescript
type Grade = "exact" | "close" | "partial" | "unrelated";
type Tier = "forced" | "distinguished" | "conventional";
type Easing = "linear" | "ease_in" | "ease_out" | "ease_in_out";
```

### Analysis Types

```typescript
type SymmetryGroup = "so2" | "d3" | "d4" | "d5" | "d6" | "bilateral" | "none";
type PartOfSpeech = "determiner" | "noun" | "verb" | "adjective" | "proper_noun";
type ErlangenLevel = "euclidean" | "similarity" | "affine" | "projective" | "topological";
```

### Layout Types

```typescript
type ShapeType =
  | { Point: { radius: number } }
  | { Circle: { radius: number } }
  | { Triangle: { size: number } }
  | { Square: { size: number } }
  | { Pentagon: { size: number } }
  | { Hexagon: { size: number } }
  | { Line: { x1: number; y1: number; x2: number; y2: number; directed: boolean } }
  | { Angle: { radius: number; degrees: number } }
  | { Curve: { x1: number; y1: number; x2: number; y2: number; curvature: number } };
```

---

*Last updated: 2026-04-09*
