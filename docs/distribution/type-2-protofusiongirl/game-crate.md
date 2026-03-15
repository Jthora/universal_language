# Type 2 — `ul-game` Crate Structure

> Rust crate that compiles to game-specific WASM for ProtoFusionGirl.

---

## Cargo.toml

```toml
[package]
name = "ul-game"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "Universal Language game module for ProtoFusionGirl — WASM evaluation engine"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
ul-core = { path = "../ul-core" }
wasm-bindgen = "=0.2.99"
serde = { workspace = true }
serde_json = { workspace = true }
serde-wasm-bindgen = "0.6"

[dev-dependencies]
wasm-bindgen-test = "0.3"

[profile.release]
opt-level = "z"     # optimize for size
lto = true
codegen-units = 1
strip = true
```

---

## Module Layout

```
crates/ul-game/src/
  lib.rs            # WASM-exported functions (9 entry points) + context storage
  types.rs          # All shared types: Operation, Tier, CompositionRule, Grade, PartialCredit, etc.
  context.rs        # GameContext: session state, RuleIndex, proficiency tracker
  cosmic.rs         # RuleIndex, pattern matching (backtracking search)
  evaluation.rs     # evaluate(): GIR vs composition rules
  scoring.rs        # score_composition(), evaluate_jane_attempt(), structural_similarity()
  sequence.rs       # validate_sequence(): ordering constraints across GIR sequence
  animation.rs      # get_animation_sequence(): BFS keyframes for Phaser tweens
  modding.rs        # load_custom_definitions(): runtime rule injection with validation
```

---

## lib.rs — WASM Entry Points

Context is stored in thread-local `HashMap<u32, GameContext>`. All WASM functions accept JSON strings (not `JsValue`-deserialized objects), making the API testable from both WASM and native Rust.

```rust
// Context lifecycle
pub fn init();
pub fn create_context(config_json: &str) -> Result<u32, JsError>;

// Core evaluation
pub fn evaluate(ctx_id: u32, gir_json: &str) -> Result<JsValue, JsError>;
pub fn score_composition(ctx_id: u32, gir_json: &str, target_json: &str) -> Result<JsValue, JsError>;

// Jane's learning system
pub fn evaluate_jane_attempt(ctx_id: u32, attempt_json: &str, expected_json: &str) -> Result<JsValue, JsError>;

// Sequence validation
pub fn validate_sequence(ctx_id: u32, glyphs_json: &str) -> Result<JsValue, JsError>;

// Animation
pub fn get_animation_sequence(gir_json: &str, width: f64, height: f64) -> Result<JsValue, JsError>;

// Layout (positioned geometry, NOT SVG — Phaser renders)
pub fn layout(gir_json: &str, width: f64, height: f64) -> Result<JsValue, JsError>;

// Modding
pub fn load_custom_definitions(ctx_id: u32, definitions_json: &str) -> Result<JsValue, JsError>;
```

All entry points use `catch_unwind(AssertUnwindSafe(...))` to convert panics into `JsError` results.

---

## context.rs — GameContext

```rust
pub struct GameContext {
    /// Active composition rules for evaluation.
    pub rule_index: RuleIndex,
    /// Per-primitive proficiency (0.0–1.0), keyed by name: "point", "line", "angle", "curve", "enclosure".
    pub proficiency: HashMap<String, f64>,
    /// Session identifier.
    pub session_id: String,
}

impl GameContext {
    pub fn from_config(config: &GameConfig) -> Result<Self, String>;
    pub fn proficiency_for(&self, primitive: &str) -> f64;     // default 0.5
    pub fn update_proficiency(&mut self, primitive: &str, score: f64) -> f64;  // EMA α=0.3
}

pub struct GameConfig {
    pub rules_json: String,    // JSON array of CompositionRule
    pub session_id: String,
}
```

Default proficiency: all 5 geometric primitives start at 0.5.

---

## Build

```bash
cd crates/ul-game
wasm-pack build --target web --release

# Optimize for size
wasm-opt -Oz -o pkg/ul_game_bg.wasm pkg/ul_game_bg.wasm

# Verify size
ls -la pkg/ul_game_bg.wasm  # target: < 500KB
```

---

## Testing

```bash
# Rust unit tests (27 tests across all modules)
cargo test -p ul-game

# WASM integration tests
wasm-pack test --headless --chrome
```
