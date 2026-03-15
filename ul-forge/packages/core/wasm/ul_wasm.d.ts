/* tslint:disable */
/* eslint-disable */
/**
 * Initialize the WASM module (panic hook setup).
 */
export function init(): void;
/**
 * Create a new game context. Returns a context ID (u32).
 *
 * `config_json`: JSON string matching `GameConfig` schema.
 */
export function createContext(config_json: string): number;
/**
 * Evaluate a GIR against all active composition rules.
 *
 * `ctx_id`: context ID from `createContext`.
 * `gir_json`: GIR as JSON string.
 *
 * Returns a JSON-serialized `EvaluationResult`.
 */
export function evaluate(ctx_id: number, gir_json: string): any;
/**
 * Score a composition against a specific puzzle target.
 *
 * `target_json`: JSON string matching `PuzzleTarget` schema.
 */
export function scoreComposition(ctx_id: number, gir_json: string, target_json: string): any;
/**
 * Evaluate Jane's learning attempt with proficiency tracking.
 *
 * `attempt_json`: student's GIR (JSON).
 * `expected_json`: target GIR (JSON).
 */
export function evaluateJaneAttempt(ctx_id: number, attempt_json: string, expected_json: string): any;
/**
 * Validate ordering constraints across a sequence of GIR structures.
 *
 * `glyphs_json`: JSON array of GIR JSON strings.
 */
export function validateSequence(ctx_id: number, glyphs_json: string): any;
/**
 * Generate a construction-order animation sequence for a GIR.
 *
 * Returns a JSON-serialized `AnimationSequence`.
 */
export function getAnimationSequence(gir_json: string, width: number, height: number): any;
/**
 * Compute positioned geometry for rendering a GIR.
 *
 * Returns a JSON-serialized `PositionedGlyph`.
 */
export function layout(gir_json: string, width: number, height: number): any;
/**
 * Load custom composition rules into an existing context.
 *
 * `rules_json`: JSON array of `CompositionRule` objects.
 */
export function loadCustomDefinitions(ctx_id: number, rules_json: string): any;
/**
 * Parse UL-Script text into a GIR JSON string.
 *
 * This is the primary input path: players (or game systems) write UL-Script
 * text and get back a GIR that can be evaluated, scored, laid out, or animated.
 */
export function parseUlScript(input: string): any;
/**
 * Deparse a GIR back into UL-Script text.
 *
 * Enables display: the game can show the player what their GIR looks like
 * in UL-Script notation, closing the roundtrip.
 */
export function deparseGir(gir_json: string): any;
/**
 * Validate a GIR structure (4-layer validation).
 *
 * Returns a JSON-serialized object with `valid` (bool), `errors` (string[]),
 * and `warnings` (string[]).
 * When `check_geometry` is true, also checks geometry satisfiability (layer 4).
 */
export function validateGir(gir_json: string, check_geometry: boolean): any;
/**
 * Apply a Σ_UL operation to GIR operands.
 *
 * `operation`: operation name (e.g. "predicate", "negate", "embed").
 * `operands_json`: JSON array of GIR JSON strings (1–3 depending on operation arity).
 *
 * Returns the resulting GIR as a JSON string.
 */
export function applyOperation(operation: string, operands_json: string): any;
/**
 * Combine two GIRs using a named binary operation.
 *
 * Shortcut for `applyOperation` with exactly 2 operands.
 */
export function composeGir(gir_a_json: string, gir_b_json: string, operation: string): any;
/**
 * Detect which Σ_UL operations are expressed in a GIR.
 *
 * Returns a JSON array of operation name strings.
 */
export function detectOperations(gir_json: string): any;
/**
 * Analyze the structure of a GIR and return a comprehensive report.
 *
 * Returns a JSON-serialized `StructureReport` with node/edge counts,
 * primitive distribution, sort distribution, detected operations,
 * depth, breadth, and complexity score.
 */
export function analyzeStructure(gir_json: string): any;
/**
 * Compare two GIR structures at a given Erlangen equivalence level.
 *
 * `level` must be one of: "euclidean", "similarity", "affine", "projective", "topological".
 * Returns a JSON-serialized `EquivalenceResult` with score, equivalence flag, and sub-dimensions.
 */
export function compareGir(gir_a_json: string, gir_b_json: string, level: string): any;
/**
 * Render a GIR to an SVG string.
 *
 * `width` and `height` define the viewBox dimensions.
 */
export function renderSvg(gir_json: string, width: number, height: number): any;
/**
 * Render a compact preview SVG for a GIR (fixed 64×64 viewBox).
 */
export function renderGlyphPreview(gir_json: string): any;
/**
 * Generate contextual hints by comparing an attempt GIR to a target GIR.
 *
 * Returns a JSON array of `Hint` objects with severity, category, and message.
 */
export function getHints(attempt_json: string, target_json: string): any;
/**
 * Generate self-standing analysis hints for a GIR (no target needed).
 *
 * Returns a JSON array of `Hint` objects.
 */
export function analyzeHints(gir_json: string): any;
/**
 * Get the next appropriate puzzle for a student given their proficiency.
 *
 * `proficiency_json`: JSON object mapping operation names to proficiency scores (0.0–1.0).
 * Returns a JSON-serialized `Puzzle` object.
 */
export function getNextPuzzle(proficiency_json: string): any;
/**
 * Search the UL lexicon (42 canonical entries) by query string.
 *
 * Matches against name, labels, symbol, and sigma_ul expression.
 * Returns a JSON array of matching `LexiconEntry` objects.
 */
export function queryLexicon(query: string): any;
/**
 * Look up a specific lexicon entry by exact name (case-insensitive).
 *
 * Returns a JSON-serialized `LexiconEntry` or null if not found.
 */
export function lookupLexiconEntry(name: string): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly init: () => void;
  readonly createContext: (a: number, b: number, c: number) => void;
  readonly evaluate: (a: number, b: number, c: number, d: number) => void;
  readonly scoreComposition: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly evaluateJaneAttempt: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly validateSequence: (a: number, b: number, c: number, d: number) => void;
  readonly getAnimationSequence: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly layout: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly loadCustomDefinitions: (a: number, b: number, c: number, d: number) => void;
  readonly parseUlScript: (a: number, b: number, c: number) => void;
  readonly deparseGir: (a: number, b: number, c: number) => void;
  readonly validateGir: (a: number, b: number, c: number, d: number) => void;
  readonly applyOperation: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly composeGir: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly detectOperations: (a: number, b: number, c: number) => void;
  readonly analyzeStructure: (a: number, b: number, c: number) => void;
  readonly compareGir: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly renderSvg: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly renderGlyphPreview: (a: number, b: number, c: number) => void;
  readonly getHints: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly analyzeHints: (a: number, b: number, c: number) => void;
  readonly getNextPuzzle: (a: number, b: number, c: number) => void;
  readonly queryLexicon: (a: number, b: number, c: number) => void;
  readonly lookupLexiconEntry: (a: number, b: number, c: number) => void;
  readonly __wbindgen_export_0: (a: number, b: number) => number;
  readonly __wbindgen_export_1: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
