# Type 2 — Phaser 3 Integration Guide

> How ProtoFusionGirl game client boots, uses, and persists the UL game module.

---

## Boot Sequence

```typescript
// src/systems/ULSystem.ts
import init, {
  createContext,
  evaluate,
  scoreComposition,
  evaluateJaneAttempt,
  validateSequence,
  getAnimationSequence,
  layout,
  loadCustomDefinitions,
} from '@ul-forge/game';

export class ULSystem {
  private ctxId: number | null = null;

  async boot(): Promise<void> {
    // 1. Initialize WASM
    await init();

    // 2. Load composition rules from game data
    const rulesJson = await fetch('/data/composition-rules.json').then(r => r.text());

    // 3. Load Jane's saved proficiency (if any)
    const saveData = this.loadSaveData();

    // 4. Create game context (returns a u32 context ID)
    this.ctxId = createContext(JSON.stringify({
      rules_json: rulesJson,
      session_id: crypto.randomUUID(),
    }));

    // 5. Load level-specific custom rules (if any)
    const levelRules = await this.loadLevelRules();
    if (levelRules) {
      const result = loadCustomDefinitions(this.ctxId, levelRules);
      if (!result.success) {
        console.warn('Failed to load custom rules:', result.errors);
      }
    }
  }

  // ... (see methods below)
}
```

---

## Core Game Loop Integration

### Player Submits a Glyph

```typescript
onPlayerSubmitGlyph(girJson: string, puzzleTargetJson: string): ScoreResult {
  if (this.ctxId === null) throw new Error('ULSystem not booted');

  // Score against puzzle target
  const result = scoreComposition(this.ctxId, girJson, puzzleTargetJson);

  // If acceptable, animate the glyph
  if (result.grade !== 'unrelated') {
    this.animateGlyph(girJson);
  }

  // Update Jane's learning model
  const expectedGirJson = JSON.parse(puzzleTargetJson).expected_gir_json;
  if (expectedGirJson) {
    const janeResult = evaluateJaneAttempt(this.ctxId, girJson, expectedGirJson);
    this.emit('janeProgress', janeResult);
  }

  return result;
}
```

### Free-Form Evaluation (Sandbox Mode)

```typescript
onFreeFormGlyph(girJson: string): EvaluationResult {
  if (this.ctxId === null) throw new Error('ULSystem not booted');
  return evaluate(this.ctxId, girJson);
}
```

### Get Positioned Geometry for Rendering

```typescript
getGlyphGeometry(girJson: string, width: number, height: number): PositionedGlyph {
  return layout(girJson, width, height);
}
```

---

## Phaser Scene Integration

```typescript
// src/scenes/GlyphPuzzleScene.ts
import { ULSystem } from '../systems/ULSystem';

export class GlyphPuzzleScene extends Phaser.Scene {
  private ulSystem: ULSystem;
  private glyphContainer: Phaser.GameObjects.Container;

  async create() {
    this.ulSystem = this.registry.get('ulSystem');
    this.glyphContainer = this.add.container(400, 300);
  }

  onSubmit(playerGirJson: string) {
    const result = this.ulSystem.onPlayerSubmitGlyph(playerGirJson, this.currentPuzzleJson);

    // Show score feedback
    this.showScorePopup(result.score, result.grade);

    // Show detailed feedback
    this.showFeedback(result.feedback);

    // Show partial credit breakdown
    this.showPartialCredit(result.partial_credit);

    // If close or exact, show the glyph with animation
    if (result.grade === 'exact' || result.grade === 'close') {
      const positioned = this.ulSystem.getGlyphGeometry(playerGirJson, 200, 200);
      this.renderPositionedGlyph(positioned, this.glyphContainer);
    }
  }

  private renderPositionedGlyph(glyph: PositionedGlyph, container: Phaser.GameObjects.Container) {
    container.removeAll(true);

    for (const elem of glyph.elements) {
      const go = this.createGameObjectForShape(elem.shape);
      go.setPosition(elem.x, elem.y);
      container.add(go);
    }

    for (const conn of glyph.connections) {
      const line = this.add.line(0, 0, conn.x1, conn.y1, conn.x2, conn.y2, 0xffffff);
      if (conn.dashed) line.setAlpha(0.5);
      container.add(line);
    }
  }

  private createGameObjectForShape(shape: Shape): Phaser.GameObjects.GameObject {
    // Map UL Shape types to Phaser primitives
    switch (shape.type) {
      case 'Point': return this.add.circle(0, 0, shape.radius, 0xffffff);
      case 'Circle': return this.add.circle(0, 0, shape.radius, 0x000000, 0).setStrokeStyle(2, 0xffffff);
      case 'Triangle': return this.add.triangle(0, 0, ...triangleVertices(shape.size), 0x000000, 0).setStrokeStyle(2, 0xffffff);
      case 'Square': return this.add.rectangle(0, 0, shape.size, shape.size, 0x000000, 0).setStrokeStyle(2, 0xffffff);
      case 'Line': return this.add.line(0, 0, shape.x1, shape.y1, shape.x2, shape.y2, 0xffffff);
      case 'Angle': return this.createAngleArc(shape);
      case 'Curve': return this.createBezierCurve(shape);
    }
  }
}
```

---

## Save / Load

Jane's proficiency is persisted via the game's save system:

```typescript
// Save
saveProficiency() {
  if (this.ctxId === null) return;
  // Extract proficiency from the context (via evaluateJaneAttempt result deltas)
  localStorage.setItem('ul_proficiency', JSON.stringify(this.proficiencyCache));
}

// Load (called during boot)
loadSaveData(): SaveData | null {
  const raw = localStorage.getItem('ul_proficiency');
  if (!raw) return null;
  return { ulProficiency: JSON.parse(raw) };
}
```

---

## Modding Support

Level designers and modders can add custom composition rules:

```typescript
async loadModRules(modId: string): Promise<void> {
  const rulesJson = await fetch(`/mods/${modId}/composition-rules.json`).then(r => r.text());
  const result = loadCustomDefinitions(this.ctxId!, rulesJson);

  if (result.success) {
    console.log(`Loaded ${result.rules_added} custom rules from mod ${modId}`);
  } else {
    console.error(`Mod ${modId} rule errors:`, result.errors);
  }
}
```

Custom rules follow the `CompositionRule` JSON format. The game validates rule structure on load and rejects invalid rules with descriptive error messages.

---

## File Organization in ProtoFusionGirl

```
protofusiongirl/
  node_modules/
    @ul-forge/game/              # npm package from wasm-pack
  public/
    data/
      composition-rules.json     # Base composition rules (ship with game)
  src/
    systems/
      ULSystem.ts                # UL game module wrapper
    scenes/
      GlyphPuzzleScene.ts        # Puzzle gameplay
      GlyphSandboxScene.ts       # Free-form exploration
    utils/
      glyph-renderer.ts          # PositionedGlyph → Phaser GameObjects
```
