# Type 2 — ProtoFusionGirl Game Module

> A WASM module purpose-built for the ProtoFusionGirl game engine (Phaser 3 / TypeScript).

**Status:** Implementation complete (27 tests passing)  
**Prerequisites:** P2 (Expose layout geometry) — done  
**Wave:** 3

---

## Goal

ProtoFusionGirl is a game where the player character Jane interacts with Universal Language as a core game mechanic. The implementation exposes UL's formal system directly — the game adapts its mechanics to what UL provides:

### What UL provides (implementation authority)

1. **Parsing** — UL-Script text → GIR (Geometric Intermediate Representation)
2. **Validation** — 4-layer structural validation (schema, sorts, invariants, geometry)
3. **Pattern matching** — backtracking subgraph search with node type, sort, and label constraints
4. **Composition rules** — weighted evaluation of GIR against operation-grounded rules
5. **Layout** — template matching (12 canonical glyphs) + hierarchical constraint layout
6. **Deparsing** — GIR → UL-Script text roundtrip
7. **Scoring** — 4-dimension partial credit (structural, sort, operation, sequence)
8. **Animation** — BFS construction-order keyframes with absolute timestamps

### How the game consumes it

1. **Evaluate** player-composed GIR against composition rules grounded in Σ_UL operations
2. **Score** with 4-dimensional partial credit (not binary pass/fail)
3. **Animate** glyph construction in BFS order from root
4. **Track** Jane's learning progression per geometric primitive (EMA proficiency)
5. **Support modding** via runtime composition rule injection
6. **Perform** within a 60fps game loop (< 1ms evaluate, < 16ms score)

---

## Architecture

```
┌──────────────────────────────────────────┐
│          ProtoFusionGirl (Phaser 3)       │
│                                           │
│  ┌─────────┐  ┌──────────┐  ┌─────────┐ │
│  │  Input   │  │  Phaser  │  │  Save   │ │
│  │  System  │→ │  Render  │  │  System │ │
│  └────┬─────┘  └────▲─────┘  └────▲────┘ │
│       │              │              │      │
│       ▼              │              │      │
│  ┌────────────────────────────────────┐   │
│  │         UL Game Module (WASM)       │   │
│  │                                     │   │
│  │  evaluate()         → EvalResult    │   │
│  │  scoreComposition() → ScoreResult   │   │
│  │  evaluateJaneAttempt() → JaneResult │───┘
│  │  validateSequence() → SeqResult     │
│  │  getAnimationSequence() → Keyframes │
│  │  layout()           → PositionedGlyph│
│  │  loadCustomDefinitions() → LoadResult│
│  └────────────────────────────────────┘   │
│       ▲                                    │
│       │                                    │
│  ┌────┴──────────────┐                     │
│  │ composition-rules │  (JSON data files)  │
│  │ .json             │                     │
│  └───────────────────┘                     │
└──────────────────────────────────────────┘
```

Key design constraint: the WASM module returns **geometry** (PositionedGlyph) and **data** (scores, keyframes), never SVG. Phaser handles all actual rendering.

---

## Specifications

| Document | Content |
|----------|---------|
| [game-crate.md](game-crate.md) | `ul-game` crate structure, Cargo.toml, module layout |
| [composition-rules.md](composition-rules.md) | Rule engine: 5 primitives × 4 sorts × 11 operations |
| [evaluation-api.md](evaluation-api.md) | Scoring engine, graduated failure, partial credit |
| [animation-bindings.md](animation-bindings.md) | Keyframe generation for Phaser tweens |
| [integration-guide.md](integration-guide.md) | Phaser 3 boot sequence, data flow, save/load |

---

## WASM API Surface

All functions accept/return JSON strings. Context is stored server-side (thread-local `HashMap<u32, GameContext>`); functions take a `ctx_id: u32`.

```typescript
interface ULGameModule {
  // Lifecycle
  init(): void;
  createContext(configJson: string): number;  // returns ctx_id

  // Core evaluation
  evaluate(ctxId: number, girJson: string): EvaluationResult;
  scoreComposition(ctxId: number, girJson: string, targetJson: string): ScoreResult;
  validateSequence(ctxId: number, glyphsJson: string): SequenceResult;

  // Jane's learning system
  evaluateJaneAttempt(ctxId: number, attemptJson: string, expectedJson: string): JaneResult;

  // Animation
  getAnimationSequence(girJson: string, width: number, height: number): AnimationSequence;

  // Geometry (positioned data for Phaser rendering, NOT SVG)
  layout(girJson: string, width: number, height: number): PositionedGlyph;

  // Modding
  loadCustomDefinitions(ctxId: number, definitionsJson: string): LoadResult;
}
```

---

## UL Formal System (what drives the game)

The game's mechanics emerge from UL's Σ_UL algebraic signature:

| UL Concept | Game Mechanic |
|------------|---------------|
| 5 Primitives (Point, Line, Angle, Curve, Enclosure) | The 5 building blocks Jane learns to compose |
| 4 Sorts (Entity, Relation, Modifier, Assertion) | Structural constraints on valid compositions |
| 11 Operations (predicate, embed, compose, ...) | What composition rules test for |
| 3 Tiers (Forced, Distinguished, Conventional) | Difficulty progression |
| Pattern matching (backtracking subgraph search) | How rules evaluate player compositions |
| Graduated scoring (Exact/Close/Partial/Unrelated) | Player feedback granularity |
| Proficiency tracking (EMA per primitive) | Jane's learning progression |

---

## Performance Budget

| Operation | Target | Strategy |
|-----------|--------|----------|
| `evaluate()` (simple) | < 1ms | Pre-compiled rule lookup, no heap allocation |
| `scoreComposition()` (complex) | < 16ms | Fits in one frame at 60fps |
| `layout()` | < 2ms | Template matching short-circuits, hierarchical layout fallback |
| `getAnimationSequence()` | < 5ms | BFS + layout computation |
| WASM binary size | < 500KB | `wasm-opt -Oz`, minimal serde, no std bloat |
| Memory per GameContext | < 2MB | HashMap-based context storage |
| serde_json (minimal) | ~80KB |
| wasm-bindgen glue | ~20KB |
| **Total** | **~350KB** |

---

## Data Flow

```
Player draws glyph
  → Game builds GirJson from canvas input
  → evaluate(ctx, gir) returns EvaluationResult
  → Game reads score + grade + feedback
  → If grade != "wrong":
      → layout(gir, w, h) returns PositionedGlyph
      → getAnimationSequence(gir) returns keyframes
      → Phaser renders glyph with animation
  → evaluateJaneAttempt(ctx, attempt, expected) updates proficiency
  → Game saves proficiency via getJaneProficiency(ctx)
```

---

## Files to Create

| File | Purpose |
|------|---------|
| `crates/ul-game/Cargo.toml` | Crate manifest |
| `crates/ul-game/src/lib.rs` | WASM entry points |
| `crates/ul-game/src/context.rs` | GameContext, session state |
| `crates/ul-game/src/cosmic.rs` | Cosmic rule engine |
| `crates/ul-game/src/evaluation.rs` | Scoring, graduated failure |
| `crates/ul-game/src/sequence.rs` | Sequence validation |
| `crates/ul-game/src/animation.rs` | Keyframe generation |
| `crates/ul-game/src/modding.rs` | Runtime rule injection |
| `crates/ul-game/src/scoring.rs` | Score computation |
| `crates/ul-game/src/types.rs` | Game-specific types |

---

## Success Criteria

1. WASM binary < 500KB after `wasm-opt -Oz`
2. `evaluate()` returns in < 1ms for a single-primitive glyph (microbenchmark)
3. `scoreComposition()` returns in < 16ms for a 20-node glyph with 10 cosmic rules
4. Game integration test: Phaser scene boots, loads WASM, evaluates a glyph, renders result
5. Custom rules load at runtime without WASM rebuild
6. Jane's proficiency persists across save/load cycles
