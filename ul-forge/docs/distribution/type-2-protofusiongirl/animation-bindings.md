# Type 2 — Animation Bindings

> Construction-order keyframe generation for Phaser tween integration.

---

## Design

The WASM module computes **what** to animate and **when** (in milliseconds). Phaser's tween system handles the actual rendering. The interface is an `AnimationSequence` — an array of keyframes plus total duration, describing each node's visual state at absolute timestamps.

UL Forge computes the choreography, Phaser performs the show.

---

## Keyframe Types

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationKeyframe {
    /// Which GIR node this keyframe applies to
    pub node_id: String,
    /// Absolute time in milliseconds
    pub timestamp_ms: u32,
    /// Position (from layout computation)
    pub x: f64,
    pub y: f64,
    /// Visual properties
    pub scale: f64,
    pub rotation: f64,    // radians
    pub opacity: f64,     // 0.0 – 1.0
    /// Easing function name (Phaser understands these)
    pub easing: Easing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationSequence {
    pub keyframes: Vec<AnimationKeyframe>,
    pub total_duration_ms: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}
```

---

## Animation Generation

Nodes appear in BFS order from the GIR root. Each node gets two keyframes: invisible → visible.

```rust
const NODE_DELAY_MS: u32 = 150;    // delay between successive node appearances
const APPEAR_DURATION_MS: u32 = 300; // duration of each node's appear tween

pub fn get_animation_sequence(gir: &Gir, width: f64, height: f64) -> AnimationSequence {
    let positioned = ul_core::compute_layout(gir, width, height);
    let bfs_order = bfs_order(gir);

    let mut keyframes = Vec::new();
    let mut node_index = 0u32;

    for node_id in &bfs_order {
        let (x, y) = match positioned.elements.iter().find(|e| e.node_id == *node_id) {
            Some(e) => (e.x, e.y),
            None => continue,
        };

        let start_time = node_index * NODE_DELAY_MS;

        // Invisible start state
        keyframes.push(AnimationKeyframe {
            node_id: node_id.clone(),
            timestamp_ms: start_time,
            x, y, scale: 0.0, rotation: 0.0, opacity: 0.0,
            easing: Easing::EaseOut,
        });

        // Visible end state
        keyframes.push(AnimationKeyframe {
            node_id: node_id.clone(),
            timestamp_ms: start_time + APPEAR_DURATION_MS,
            x, y, scale: 1.0, rotation: 0.0, opacity: 1.0,
            easing: Easing::EaseOut,
        });

        node_index += 1;
    }

    let total = if node_index == 0 { 0 }
        else { (node_index - 1) * NODE_DELAY_MS + APPEAR_DURATION_MS };

    AnimationSequence { keyframes, total_duration_ms: total }
}
```

---

## WASM Entry Point

```typescript
// TypeScript signature after WASM import
function getAnimationSequence(girJson: string, width: number, height: number): AnimationSequence;
```

Note: the animation function takes `width` and `height` parameters because it internally computes layout to determine node positions.

---

## Phaser Integration

```typescript
import { getAnimationSequence, layout } from '@ul-forge/game';

function animateGlyph(girJson: string, container: Phaser.GameObjects.Container) {
  const seq = getAnimationSequence(girJson, 400, 400);
  const positioned = JSON.parse(layout(girJson, 400, 400));

  // Create Phaser sprites for each positioned element
  const sprites: Map<string, Phaser.GameObjects.Sprite> = new Map();
  for (const elem of positioned.elements) {
    const sprite = createSpriteForShape(elem.shape);
    sprite.setPosition(elem.x, elem.y);
    sprite.setScale(0);
    sprite.setAlpha(0);
    container.add(sprite);
    sprites.set(elem.node_id, sprite);
  }

  // Build Phaser timeline from keyframes
  const timeline = container.scene.tweens.createTimeline();

  // Group keyframes by node
  const byNode = groupBy(seq.keyframes, k => k.node_id);

  for (const [nodeId, frames] of byNode) {
    const sprite = sprites.get(nodeId);
    if (!sprite) continue;

    for (let i = 0; i < frames.length - 1; i++) {
      const from = frames[i];
      const to = frames[i + 1];
      const tweenDuration = to.timestamp_ms - from.timestamp_ms;

      timeline.add({
        targets: sprite,
        x: to.x,
        y: to.y,
        scaleX: to.scale,
        scaleY: to.scale,
        rotation: to.rotation,
        alpha: to.opacity,
        duration: tweenDuration,
        delay: i === 0 ? from.timestamp_ms : 0,
        ease: mapEasing(to.easing),
      });
    }
  }

  timeline.play();
}
```

---

## Animation Variants

| Variant | Description | Status |
|---------|-------------|--------|
| **Construction** | Nodes appear one-by-one in BFS order from root | Implemented |
| **Destruction** | Nodes fade out in reverse BFS order | Future |
| **Transformation** | Morph between two GIR layouts | Future |
| **Highlight** | Pulse specific nodes (e.g. matched rule nodes) | Future |
