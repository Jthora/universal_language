//! # UL Game
//!
//! Universal Language game module for ProtoFusionGirl — WASM evaluation engine.
//!
//! Grounded in the real UL formal system (Σ_UL): 5 geometric primitives,
//! 4 sorts, 11 operations. No fantasy lore — pure UL writing system.
//!
//! Provides 23 WASM-exported functions for game integration:
//! - `init` — set up panic hook
//! - `createContext` — create a game session
//! - `evaluate` — evaluate GIR against composition rules
//! - `scoreComposition` — score against a puzzle target
//! - `evaluateJaneAttempt` — learning assessment with proficiency tracking
//! - `validateSequence` — check ordering constraints
//! - `getAnimationSequence` — generate construction-order keyframes
//! - `layout` — compute positioned geometry for rendering
//! - `loadCustomDefinitions` — inject custom rules at runtime
//! - `parseUlScript` — parse UL-Script text → GIR JSON
//! - `deparseGir` — deparse GIR JSON → UL-Script text
//! - `validateGir` — 4-layer structural validation
//! - `applyOperation` — apply a Σ_UL operation to GIR operands
//! - `composeGir` — combine two GIRs with a binary operation
//! - `detectOperations` — reverse-engineer which operations a GIR expresses
//! - `analyzeStructure` — compute structural analysis report for a GIR
//! - `renderSvg` — render GIR to SVG string
//! - `renderGlyphPreview` — render GIR to compact SVG preview
//! - `getHints` — generate contextual teaching hints
//! - `analyzeHints` — generate self-standing hints for exploration
//! - `getNextPuzzle` — get the next appropriate puzzle for student
//! - `queryLexicon` — search the 42-entry UL lexicon
//! - `lookupLexiconEntry` — look up a specific lexicon entry by name

pub mod analysis;
pub mod animation;
pub mod context;
pub mod cosmic;
pub mod difficulty;
pub mod evaluation;
pub mod hints;
pub mod modding;
pub mod scoring;
pub mod sequence;
pub mod types;

use std::cell::RefCell;
use std::collections::HashMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use wasm_bindgen::prelude::*;

use context::GameContext;
use types::GameConfig;

// ── Context storage ────────────────────────────────────────────
// WASM is single-threaded, so thread_local + RefCell is safe.

thread_local! {
    static CONTEXTS: RefCell<HashMap<u32, GameContext>> = RefCell::new(HashMap::new());
    static NEXT_ID: RefCell<u32> = const { RefCell::new(1) };
}

fn with_context<F, R>(id: u32, f: F) -> Result<R, JsError>
where
    F: FnOnce(&GameContext) -> R,
{
    CONTEXTS.with(|ctx| {
        let map = ctx.borrow();
        let context = map
            .get(&id)
            .ok_or_else(|| JsError::new(&format!("No context with id {id}")))?;
        Ok(f(context))
    })
}

fn with_context_mut<F, R>(id: u32, f: F) -> Result<R, JsError>
where
    F: FnOnce(&mut GameContext) -> R,
{
    CONTEXTS.with(|ctx| {
        let mut map = ctx.borrow_mut();
        let context = map
            .get_mut(&id)
            .ok_or_else(|| JsError::new(&format!("No context with id {id}")))?;
        Ok(f(context))
    })
}

// ── WASM entry points ──────────────────────────────────────────

/// Initialize the WASM module (panic hook setup).
#[wasm_bindgen]
pub fn init() {
    // Panic hook would be set here if console_error_panic_hook is added as a dependency.
}

/// Create a new game context. Returns a context ID (u32).
///
/// `config_json`: JSON string matching `GameConfig` schema.
#[wasm_bindgen(js_name = "createContext")]
pub fn create_context(config_json: &str) -> Result<u32, JsError> {
    let config_json = config_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let config: GameConfig =
            serde_json::from_str(&config_json).map_err(|e| JsError::new(&e.to_string()))?;
        let ctx = GameContext::from_config(&config).map_err(|e| JsError::new(&e))?;

        let id = NEXT_ID.with(|n| {
            let mut n = n.borrow_mut();
            let id = *n;
            *n += 1;
            id
        });

        CONTEXTS.with(|c| c.borrow_mut().insert(id, ctx));
        Ok(id)
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal panic in createContext")))
}

/// Evaluate a GIR against all active composition rules.
///
/// `ctx_id`: context ID from `createContext`.
/// `gir_json`: GIR as JSON string.
///
/// Returns a JSON-serialized `EvaluationResult`.
#[wasm_bindgen]
pub fn evaluate(ctx_id: u32, gir_json: &str) -> Result<JsValue, JsError> {
    let gir_json = gir_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let gir =
            ul_core::Gir::from_json(&gir_json).map_err(|e| JsError::new(&e.to_string()))?;
        let result = with_context(ctx_id, |ctx| evaluation::evaluate(ctx, &gir))?;
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal panic in evaluate")))
}

/// Score a composition against a specific puzzle target.
///
/// `target_json`: JSON string matching `PuzzleTarget` schema.
#[wasm_bindgen(js_name = "scoreComposition")]
pub fn score_composition(
    ctx_id: u32,
    gir_json: &str,
    target_json: &str,
) -> Result<JsValue, JsError> {
    let gir_json = gir_json.to_owned();
    let target_json = target_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let gir =
            ul_core::Gir::from_json(&gir_json).map_err(|e| JsError::new(&e.to_string()))?;
        let target: types::PuzzleTarget =
            serde_json::from_str(&target_json).map_err(|e| JsError::new(&e.to_string()))?;
        let result = with_context(ctx_id, |ctx| scoring::score_composition(ctx, &gir, &target))?;
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal panic in scoreComposition")))
}

/// Evaluate Jane's learning attempt with proficiency tracking.
///
/// `attempt_json`: student's GIR (JSON).
/// `expected_json`: target GIR (JSON).
#[wasm_bindgen(js_name = "evaluateJaneAttempt")]
pub fn evaluate_jane_attempt(
    ctx_id: u32,
    attempt_json: &str,
    expected_json: &str,
) -> Result<JsValue, JsError> {
    let attempt_json = attempt_json.to_owned();
    let expected_json = expected_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let attempt =
            ul_core::Gir::from_json(&attempt_json).map_err(|e| JsError::new(&e.to_string()))?;
        let expected =
            ul_core::Gir::from_json(&expected_json).map_err(|e| JsError::new(&e.to_string()))?;
        let result = with_context_mut(ctx_id, |ctx| {
            scoring::evaluate_jane_attempt(ctx, &attempt, &expected)
        })?;
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal panic in evaluateJaneAttempt")))
}

/// Validate ordering constraints across a sequence of GIR structures.
///
/// `glyphs_json`: JSON array of GIR JSON strings.
#[wasm_bindgen(js_name = "validateSequence")]
pub fn validate_sequence(ctx_id: u32, glyphs_json: &str) -> Result<JsValue, JsError> {
    let glyphs_json = glyphs_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let gir_strings: Vec<String> =
            serde_json::from_str(&glyphs_json).map_err(|e| JsError::new(&e.to_string()))?;
        let mut glyphs = Vec::new();
        for s in &gir_strings {
            let gir = ul_core::Gir::from_json(s).map_err(|e| JsError::new(&e.to_string()))?;
            glyphs.push(gir);
        }
        let result = with_context(ctx_id, |ctx| sequence::validate_sequence(ctx, &glyphs))?;
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal panic in validateSequence")))
}

/// Generate a construction-order animation sequence for a GIR.
///
/// Returns a JSON-serialized `AnimationSequence`.
#[wasm_bindgen(js_name = "getAnimationSequence")]
pub fn get_animation_sequence(
    gir_json: &str,
    width: f64,
    height: f64,
) -> Result<JsValue, JsError> {
    let gir_json = gir_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let gir =
            ul_core::Gir::from_json(&gir_json).map_err(|e| JsError::new(&e.to_string()))?;
        let seq = animation::get_animation_sequence(&gir, width, height);
        Ok(serde_wasm_bindgen::to_value(&seq)?)
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal panic in getAnimationSequence")))
}

/// Compute positioned geometry for rendering a GIR.
///
/// Returns a JSON-serialized `PositionedGlyph`.
#[wasm_bindgen]
pub fn layout(gir_json: &str, width: f64, height: f64) -> Result<JsValue, JsError> {
    let gir_json = gir_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let gir =
            ul_core::Gir::from_json(&gir_json).map_err(|e| JsError::new(&e.to_string()))?;
        let positioned = ul_core::compute_layout(&gir, width, height);
        Ok(serde_wasm_bindgen::to_value(&positioned)?)
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal panic in layout")))
}

/// Load custom composition rules into an existing context.
///
/// `rules_json`: JSON array of `CompositionRule` objects.
#[wasm_bindgen(js_name = "loadCustomDefinitions")]
pub fn load_custom_definitions(ctx_id: u32, rules_json: &str) -> Result<JsValue, JsError> {
    let rules_json = rules_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let result =
            with_context_mut(ctx_id, |ctx| modding::load_custom_definitions(ctx, &rules_json))?;
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal panic in loadCustomDefinitions")))
}

// ── UL-Core pass-through entry points ──────────────────────────
// These expose ul-core capabilities directly to the game, making
// the implementation the authority for what the game can do.

/// Parse UL-Script text into a GIR JSON string.
///
/// This is the primary input path: players (or game systems) write UL-Script
/// text and get back a GIR that can be evaluated, scored, laid out, or animated.
#[wasm_bindgen(js_name = "parseUlScript")]
pub fn parse_ul_script(input: &str) -> Result<JsValue, JsError> {
    let input = input.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let gir = ul_core::parser::parse(&input).map_err(|e| JsError::new(&e.to_string()))?;
        let json = gir.to_json().map_err(|e| JsError::new(&e.to_string()))?;
        Ok(JsValue::from_str(&json))
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal panic in parseUlScript")))
}

/// Deparse a GIR back into UL-Script text.
///
/// Enables display: the game can show the player what their GIR looks like
/// in UL-Script notation, closing the roundtrip.
#[wasm_bindgen(js_name = "deparseGir")]
pub fn deparse_gir(gir_json: &str) -> Result<JsValue, JsError> {
    let gir_json = gir_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let gir =
            ul_core::Gir::from_json(&gir_json).map_err(|e| JsError::new(&e.to_string()))?;
        let text =
            ul_core::parser::deparse(&gir).map_err(|e| JsError::new(&e.to_string()))?;
        Ok(JsValue::from_str(&text))
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal panic in deparseGir")))
}

/// Validate a GIR structure (4-layer validation).
///
/// Returns a JSON-serialized object with `valid` (bool), `errors` (string[]),
/// and `warnings` (string[]).
/// When `check_geometry` is true, also checks geometry satisfiability (layer 4).
#[wasm_bindgen(js_name = "validateGir")]
pub fn validate_gir(gir_json: &str, check_geometry: bool) -> Result<JsValue, JsError> {
    let gir_json = gir_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let gir =
            ul_core::Gir::from_json(&gir_json).map_err(|e| JsError::new(&e.to_string()))?;
        let result = ul_core::validator::validate(&gir, check_geometry);
        // Convert to a serializable DTO (ValidationResult doesn't impl Serialize)
        let dto = types::ValidationDto {
            valid: result.valid,
            errors: result.errors.iter().map(|e| e.to_string()).collect(),
            warnings: result.warnings,
        };
        Ok(serde_wasm_bindgen::to_value(&dto)?)
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal panic in validateGir")))
}

// ── Algebraic composer entry points ────────────────────────────
// These expose the 11 Σ_UL operations as GIR transformations.

/// Apply a Σ_UL operation to GIR operands.
///
/// `operation`: operation name (e.g. "predicate", "negate", "embed").
/// `operands_json`: JSON array of GIR JSON strings (1–3 depending on operation arity).
///
/// Returns the resulting GIR as a JSON string.
#[wasm_bindgen(js_name = "applyOperation")]
pub fn apply_operation(operation: &str, operands_json: &str) -> Result<JsValue, JsError> {
    let operation = operation.to_owned();
    let operands_json = operands_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let operand_strings: Vec<String> = serde_json::from_str(&operands_json)
            .map_err(|e| JsError::new(&format!("invalid operands JSON: {e}")))?;

        let mut girs = Vec::new();
        for s in &operand_strings {
            let g = ul_core::Gir::from_json(s).map_err(|e| JsError::new(&e.to_string()))?;
            girs.push(g);
        }

        let result = match operation.as_str() {
            "predicate" => {
                if girs.len() != 3 {
                    return Err(JsError::new("predicate requires 3 operands (subject, relation, object)"));
                }
                ul_core::composer::predicate(&girs[0], &girs[1], &girs[2])
            }
            "modify_entity" => {
                if girs.len() != 2 {
                    return Err(JsError::new("modify_entity requires 2 operands (modifier, entity)"));
                }
                ul_core::composer::modify_entity(&girs[0], &girs[1])
            }
            "modify_relation" => {
                if girs.len() != 2 {
                    return Err(JsError::new("modify_relation requires 2 operands (modifier, relation)"));
                }
                ul_core::composer::modify_relation(&girs[0], &girs[1])
            }
            "negate" => {
                if girs.len() != 1 {
                    return Err(JsError::new("negate requires 1 operand (assertion)"));
                }
                ul_core::composer::negate(&girs[0])
            }
            "conjoin" => {
                if girs.len() != 2 {
                    return Err(JsError::new("conjoin requires 2 operands (assertion, assertion)"));
                }
                ul_core::composer::conjoin(&girs[0], &girs[1])
            }
            "disjoin" => {
                if girs.len() != 2 {
                    return Err(JsError::new("disjoin requires 2 operands (assertion, assertion)"));
                }
                ul_core::composer::disjoin(&girs[0], &girs[1])
            }
            "embed" => {
                if girs.len() != 1 {
                    return Err(JsError::new("embed requires 1 operand (assertion)"));
                }
                ul_core::composer::embed(&girs[0])
            }
            "abstract" => {
                if girs.len() != 1 {
                    return Err(JsError::new("abstract requires 1 operand (entity)"));
                }
                ul_core::composer::abstract_op(&girs[0])
            }
            "compose" => {
                if girs.len() != 2 {
                    return Err(JsError::new("compose requires 2 operands (relation, relation)"));
                }
                ul_core::composer::compose(&girs[0], &girs[1])
            }
            "invert" => {
                if girs.len() != 1 {
                    return Err(JsError::new("invert requires 1 operand (relation)"));
                }
                ul_core::composer::invert(&girs[0])
            }
            "quantify" => {
                if girs.len() != 2 {
                    return Err(JsError::new("quantify requires 2 operands (quantifier, entity)"));
                }
                ul_core::composer::quantify(&girs[0], &girs[1])
            }
            _ => return Err(JsError::new(&format!("unknown operation: {operation}"))),
        };

        let gir = result.map_err(|e| JsError::new(&e.to_string()))?;
        let json = gir.to_json().map_err(|e| JsError::new(&e.to_string()))?;
        Ok(JsValue::from_str(&json))
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal panic in applyOperation")))
}

/// Combine two GIRs using a named binary operation.
///
/// Shortcut for `applyOperation` with exactly 2 operands.
#[wasm_bindgen(js_name = "composeGir")]
pub fn compose_gir(gir_a_json: &str, gir_b_json: &str, operation: &str) -> Result<JsValue, JsError> {
    let gir_a_json = gir_a_json.to_owned();
    let gir_b_json = gir_b_json.to_owned();
    let operation = operation.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let a = ul_core::Gir::from_json(&gir_a_json).map_err(|e| JsError::new(&e.to_string()))?;
        let b = ul_core::Gir::from_json(&gir_b_json).map_err(|e| JsError::new(&e.to_string()))?;

        let result = match operation.as_str() {
            "conjoin" => ul_core::composer::conjoin(&a, &b),
            "disjoin" => ul_core::composer::disjoin(&a, &b),
            "compose" => ul_core::composer::compose(&a, &b),
            "modify_entity" => ul_core::composer::modify_entity(&a, &b),
            "modify_relation" => ul_core::composer::modify_relation(&a, &b),
            "quantify" => ul_core::composer::quantify(&a, &b),
            _ => return Err(JsError::new(&format!("not a binary operation or unknown: {operation}"))),
        };

        let gir = result.map_err(|e| JsError::new(&e.to_string()))?;
        let json = gir.to_json().map_err(|e| JsError::new(&e.to_string()))?;
        Ok(JsValue::from_str(&json))
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal panic in composeGir")))
}

/// Detect which Σ_UL operations are expressed in a GIR.
///
/// Returns a JSON array of operation name strings.
#[wasm_bindgen(js_name = "detectOperations")]
pub fn detect_operations(gir_json: &str) -> Result<JsValue, JsError> {
    let gir_json = gir_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let gir = ul_core::Gir::from_json(&gir_json).map_err(|e| JsError::new(&e.to_string()))?;
        let ops = analysis::detect_operations_list(&gir);
        Ok(serde_wasm_bindgen::to_value(&ops)?)
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal panic in detectOperations")))
}

// ── Structural analysis entry point ────────────────────────────

/// Analyze the structure of a GIR and return a comprehensive report.
///
/// Returns a JSON-serialized `StructureReport` with node/edge counts,
/// primitive distribution, sort distribution, detected operations,
/// depth, breadth, and complexity score.
#[wasm_bindgen(js_name = "analyzeStructure")]
pub fn analyze_structure(gir_json: &str) -> Result<JsValue, JsError> {
    let gir_json = gir_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let gir = ul_core::Gir::from_json(&gir_json).map_err(|e| JsError::new(&e.to_string()))?;
        let report = analysis::analyze_structure(&gir);
        Ok(serde_wasm_bindgen::to_value(&report)?)
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal panic in analyzeStructure")))
}

// ── SVG rendering pass-throughs ────────────────────────────────

/// Render a GIR to an SVG string.
///
/// `width` and `height` define the viewBox dimensions.
#[wasm_bindgen(js_name = "renderSvg")]
pub fn render_svg(gir_json: &str, width: f64, height: f64) -> Result<JsValue, JsError> {
    let gir_json = gir_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let gir = ul_core::Gir::from_json(&gir_json).map_err(|e| JsError::new(&e.to_string()))?;
        let opts = ul_core::renderer::RenderOptions {
            format: ul_core::renderer::OutputFormat::Svg,
            width,
            height,
            embed_gir: false,
        };
        let svg = ul_core::renderer::render(&gir, &opts).map_err(|e| JsError::new(&e.to_string()))?;
        Ok(JsValue::from_str(&svg))
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal panic in renderSvg")))
}

/// Render a compact preview SVG for a GIR (fixed 64×64 viewBox).
#[wasm_bindgen(js_name = "renderGlyphPreview")]
pub fn render_glyph_preview(gir_json: &str) -> Result<JsValue, JsError> {
    let gir_json = gir_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let gir = ul_core::Gir::from_json(&gir_json).map_err(|e| JsError::new(&e.to_string()))?;
        let opts = ul_core::renderer::RenderOptions {
            format: ul_core::renderer::OutputFormat::Svg,
            width: 64.0,
            height: 64.0,
            embed_gir: false,
        };
        let svg = ul_core::renderer::render(&gir, &opts).map_err(|e| JsError::new(&e.to_string()))?;
        Ok(JsValue::from_str(&svg))
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal panic in renderGlyphPreview")))
}

// ── Hint/teaching entry points ─────────────────────────────────

/// Generate contextual hints by comparing an attempt GIR to a target GIR.
///
/// Returns a JSON array of `Hint` objects with severity, category, and message.
#[wasm_bindgen(js_name = "getHints")]
pub fn get_hints(attempt_json: &str, target_json: &str) -> Result<JsValue, JsError> {
    let attempt_json = attempt_json.to_owned();
    let target_json = target_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let attempt = ul_core::Gir::from_json(&attempt_json).map_err(|e| JsError::new(&e.to_string()))?;
        let target = ul_core::Gir::from_json(&target_json).map_err(|e| JsError::new(&e.to_string()))?;
        let result = hints::generate_hints(&attempt, &target);
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal panic in getHints")))
}

/// Generate self-standing analysis hints for a GIR (no target needed).
///
/// Returns a JSON array of `Hint` objects.
#[wasm_bindgen(js_name = "analyzeHints")]
pub fn analyze_hints(gir_json: &str) -> Result<JsValue, JsError> {
    let gir_json = gir_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let gir = ul_core::Gir::from_json(&gir_json).map_err(|e| JsError::new(&e.to_string()))?;
        let result = hints::analyze_hints(&gir);
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal panic in analyzeHints")))
}

/// Get the next appropriate puzzle for a student given their proficiency.
///
/// `proficiency_json`: JSON object mapping operation names to proficiency scores (0.0–1.0).
/// Returns a JSON-serialized `Puzzle` object.
#[wasm_bindgen(js_name = "getNextPuzzle")]
pub fn get_next_puzzle(proficiency_json: &str) -> Result<JsValue, JsError> {
    let proficiency_json = proficiency_json.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let proficiency: std::collections::HashMap<String, f64> =
            serde_json::from_str(&proficiency_json).map_err(|e| JsError::new(&e.to_string()))?;
        let puzzle = difficulty::get_next_puzzle(&proficiency);
        Ok(serde_wasm_bindgen::to_value(&puzzle)?)
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal panic in getNextPuzzle")))
}

// ── Lexicon entry points ───────────────────────────────────────

/// Search the UL lexicon (42 canonical entries) by query string.
///
/// Matches against name, labels, symbol, and sigma_ul expression.
/// Returns a JSON array of matching `LexiconEntry` objects.
#[wasm_bindgen(js_name = "queryLexicon")]
pub fn query_lexicon(query: &str) -> Result<JsValue, JsError> {
    let query = query.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        let results = ul_core::lexicon::search(&query);
        Ok(serde_wasm_bindgen::to_value(&results)?)
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal panic in queryLexicon")))
}

/// Look up a specific lexicon entry by exact name (case-insensitive).
///
/// Returns a JSON-serialized `LexiconEntry` or null if not found.
#[wasm_bindgen(js_name = "lookupLexiconEntry")]
pub fn lookup_lexicon_entry(name: &str) -> Result<JsValue, JsError> {
    let name = name.to_owned();
    catch_unwind(AssertUnwindSafe(move || {
        match ul_core::lexicon::lookup(&name) {
            Some(entry) => Ok(serde_wasm_bindgen::to_value(entry)?),
            None => Ok(JsValue::NULL),
        }
    }))
    .unwrap_or_else(|_| Err(JsError::new("internal panic in lookupLexiconEntry")))
}
