/**
 * @ul-forge/core — TypeScript wrapper for UL Forge WASM module.
 *
 * Provides typed interfaces over the raw wasm-bindgen bindings.
 * Wraps all 23 WASM entry points from ul-game with typed signatures.
 */

// ── GIR types (mirror Rust types) ──

export type Sort = "entity" | "relation" | "modifier" | "assertion";

export type NodeType = "point" | "line" | "angle" | "curve" | "enclosure";

export type EnclosureShape = "circle" | "triangle" | "square" | "ellipse" | "polygon" | "freeform";

export type EdgeType =
  | "contains"
  | "modified_by"
  | "adjacent"
  | "intersects"
  | "connects"
  | "references"
  | "binds"
  | "accessible_from";

export interface Node {
  id: string;
  type: NodeType;
  sort: Sort;
  label?: string;
  shape?: EnclosureShape;
  directed?: boolean;
  measure?: number;
  curvature?: number;
  curvature_profile?: number[];
  vertices?: number;
}

export interface Edge {
  source: string;
  target: string;
  type: EdgeType;
}

export interface GirMetadata {
  source?: string;
  generated_by?: string;
  timestamp?: string;
}

export interface Gir {
  ul_gir: string;
  root: string;
  nodes: Node[];
  edges: Edge[];
  metadata?: GirMetadata;
}

// ── Validation types ──

/** Per-layer error classification for the 4-layer validation pipeline. */
export interface ValidationLayers {
  schema: string[];
  sort: string[];
  invariant: string[];
  geometry: string[];
}

export interface ValidationResult {
  valid: boolean;
  errors: string[];
  warnings: string[];
  layers: ValidationLayers;
}

// ── Render options ──

export type OutputFormat = "svg" | "tikz";

export interface RenderOptions {
  format?: OutputFormat;
  width?: number;
  height?: number;
  embedGir?: boolean;
}

// ── Game types (mirror Rust ul-game types) ──

export type OperationName =
  | "predicate"
  | "modify_entity"
  | "modify_relation"
  | "negate"
  | "conjoin"
  | "disjoin"
  | "embed"
  | "abstract"
  | "compose"
  | "invert"
  | "quantify"
  | "necessity"
  | "possibility"
  | "counterfactual"
  | "set_force"
  | "infer_pragmatic";

export type Grade = "exact" | "close" | "partial" | "unrelated";

export type Tier = "forced" | "distinguished" | "conventional";

export type Easing = "linear" | "ease_in" | "ease_out" | "ease_in_out";

export interface PartialCredit {
  structural_match: number;
  sort_correctness: number;
  operation_correctness: number;
  sequence_order: number;
}

export interface EvaluationResult {
  score: number;
  grade: Grade;
  matched_rules: string[];
  violated_rules: string[];
  feedback: string[];
}

export interface ScoreResult {
  score: number;
  grade: Grade;
  partial_credit: PartialCredit;
  feedback: string[];
}

export interface JaneResult {
  score: number;
  grade: Grade;
  improvements: string[];
  proficiency_delta: Record<string, number>;
}

export interface SequenceResult {
  valid: boolean;
  errors: string[];
  pair_scores: number[];
}

export interface AnimationKeyframe {
  node_id: string;
  timestamp_ms: number;
  x: number;
  y: number;
  scale: number;
  rotation: number;
  opacity: number;
  easing: Easing;
}

export interface AnimationSequence {
  keyframes: AnimationKeyframe[];
  total_duration_ms: number;
}

export interface LoadResult {
  loaded: number;
  errors: string[];
}

/** Symmetry group classification (Erlangen Program). */
export type SymmetryGroup = "so2" | "d3" | "d4" | "d5" | "d6" | "bilateral" | "none";

/** Part of speech derived from symmetry group. */
export type PartOfSpeech = "determiner" | "noun" | "verb" | "adjective" | "proper_noun";

export interface StructureReport {
  node_count: number;
  edge_count: number;
  primitive_counts: Record<string, number>;
  sort_distribution: Record<string, number>;
  detected_operations: string[];
  depth: number;
  breadth: number;
  complexity_score: number;
  symmetry_group: SymmetryGroup;
  part_of_speech: PartOfSpeech;
  node_symmetries: Record<string, SymmetryGroup>;
}

/** Erlangen hierarchy level for equivalence testing. */
export type ErlangenLevel = "euclidean" | "similarity" | "affine" | "projective" | "topological";

/** Sub-dimension scores for Erlangen equivalence. */
export interface EquivalenceDimensions {
  topology: number;
  types: number;
  sorts: number;
  shapes: number;
  metrics: number;
}

/** Result of comparing two GIRs at a given Erlangen level. */
export interface EquivalenceResult {
  level: ErlangenLevel;
  score: number;
  equivalent: boolean;
  dimensions: EquivalenceDimensions;
}

export interface Hint {
  severity: string;
  category: string;
  message: string;
}

export interface Puzzle {
  id: string;
  difficulty: number;
  level: number;
  required_operations: string[];
  description: string;
  target_gir_json: string;
}

export interface LexiconEntry {
  id: number;
  level: number;
  name: string;
  tier: string;
  symbol: string;
  sigma_ul: string;
  labels: string[];
}

export interface GameConfig {
  rules_json?: string;
  session_id?: string;
}

// ── Layout geometry types (mirror Rust ul-core::renderer::layout) ──

export type ShapeType =
  | { Point: { radius: number } }
  | { Circle: { radius: number } }
  | { Triangle: { size: number } }
  | { Square: { size: number } }
  | { Pentagon: { size: number } }
  | { Hexagon: { size: number } }
  | { Line: { x1: number; y1: number; x2: number; y2: number; directed: boolean } }
  | { Angle: { radius: number; degrees: number } }
  | { Curve: { x1: number; y1: number; x2: number; y2: number; curvature: number } };

export interface PositionedElement {
  node_id: string;
  x: number;
  y: number;
  shape: ShapeType;
}

export interface LayoutConnection {
  edge_id: string;
  x1: number;
  y1: number;
  x2: number;
  y2: number;
  directed: boolean;
  dashed: boolean;
}

export interface PositionedGlyph {
  elements: PositionedElement[];
  connections: LayoutConnection[];
  width: number;
  height: number;
}

// ── Core API ──

import wasmInit, {
  init as wasmModuleInit,
  parseUlScript as wasmParseUlScript,
  deparseGir as wasmDeparseGir,
  validateGir as wasmValidateGir,
  renderSvg as wasmRenderSvg,
  renderGlyphPreview as wasmRenderGlyphPreview,
  createContext as wasmCreateContext,
  evaluate as wasmEvaluate,
  scoreComposition as wasmScoreComposition,
  evaluateJaneAttempt as wasmEvaluateJaneAttempt,
  validateSequence as wasmValidateSequence,
  getAnimationSequence as wasmGetAnimationSequence,
  layout as wasmLayout,
  loadCustomDefinitions as wasmLoadCustomDefinitions,
  applyOperation as wasmApplyOperation,
  composeGir as wasmComposeGir,
  detectOperations as wasmDetectOperations,
  analyzeStructure as wasmAnalyzeStructure,
  compareGir as wasmCompareGir,
  getHints as wasmGetHints,
  analyzeHints as wasmAnalyzeHints,
  getNextPuzzle as wasmGetNextPuzzle,
  queryLexicon as wasmQueryLexicon,
  lookupLexiconEntry as wasmLookupLexiconEntry,
  set_force as wasmSetForce,
  infer_pragmatics as wasmInferPragmatics,
} from "ul-wasm";

let initialized = false;

function ensureInit(): void {
  if (!initialized) {
    throw new Error("WASM not initialized. Call initialize() first.");
  }
}

// ── Result cache for game-loop performance ──

/** Deterministic hash of a GIR for cache keys. */
function girHash(gir: Gir): string {
  return JSON.stringify({ r: gir.root, n: gir.nodes.length, e: gir.edges.length });
}

const MAX_CACHE_ENTRIES = 256;

class ResultCache<T> {
  private cache = new Map<string, T>();

  get(key: string): T | undefined {
    return this.cache.get(key);
  }

  set(key: string, value: T): void {
    if (this.cache.size >= MAX_CACHE_ENTRIES) {
      // Evict oldest entry (first inserted)
      const first = this.cache.keys().next().value;
      if (first !== undefined) this.cache.delete(first);
    }
    this.cache.set(key, value);
  }

  clear(): void {
    this.cache.clear();
  }
}

const layoutCache = new ResultCache<PositionedGlyph>();
const evaluationCache = new ResultCache<EvaluationResult>();
const renderCache = new ResultCache<string>();
const structureCache = new ResultCache<StructureReport>();

/** Clear all result caches (call when GIR changes). */
export function clearCaches(): void {
  layoutCache.clear();
  evaluationCache.clear();
  renderCache.clear();
  structureCache.clear();
}

/**
 * Initialize the WASM module. Must be called before any other function.
 * Safe to call multiple times — subsequent calls are no-ops.
 */
export async function initialize(): Promise<void> {
  if (!initialized) {
    await wasmInit();
    wasmModuleInit();
    initialized = true;
  }
}

/** For testing: reset the initialized flag. */
export function _resetForTesting(): void {
  initialized = false;
}

// ── UL-Core pass-throughs ──

/**
 * Parse UL-Script text into a GIR document.
 */
export function parse(input: string): Gir {
  ensureInit();
  const json = wasmParseUlScript(input) as string;
  return JSON.parse(json) as Gir;
}

/**
 * Validate a GIR document against Σ_UL constraints.
 */
export function validate(gir: Gir, checkGeometry = false): ValidationResult {
  ensureInit();
  const json = JSON.stringify(gir);
  return wasmValidateGir(json, checkGeometry) as ValidationResult;
}

/**
 * Render a GIR document to SVG string. Results are cached by GIR hash + dimensions.
 */
export function render(gir: Gir, width = 256, height = 256): string {
  ensureInit();
  const key = `${girHash(gir)}:${width}x${height}`;
  const cached = renderCache.get(key);
  if (cached) return cached;
  const json = JSON.stringify(gir);
  const svg = wasmRenderSvg(json, width, height) as string;
  renderCache.set(key, svg);
  return svg;
}

/**
 * Render a GIR to a compact 64×64 SVG preview.
 */
export function renderPreview(gir: Gir): string {
  ensureInit();
  const json = JSON.stringify(gir);
  return wasmRenderGlyphPreview(json) as string;
}

/**
 * Convert a GIR document back to canonical UL-Script text.
 */
export function deparse(gir: Gir): string {
  ensureInit();
  const json = JSON.stringify(gir);
  return wasmDeparseGir(json) as string;
}

/**
 * Convenience: parse UL-Script directly to SVG.
 */
export function parseAndRender(input: string): string {
  const gir = parse(input);
  return render(gir);
}

/**
 * Convenience: parse, validate, and render in one call.
 */
export function parseValidateRender(input: string): {
  gir: Gir;
  validation: ValidationResult;
  svg: string | null;
} {
  const gir = parse(input);
  const validation = validate(gir);
  const svg = validation.valid ? render(gir) : null;
  return { gir, validation, svg };
}

/**
 * Extract embedded GIR JSON from an SVG string produced by render().
 * Returns the parsed GIR, or null if the SVG has no embedded metadata.
 *
 * This enables lossless round-trip:
 *   parse("○{●}") → render(gir) → extractGirFromSvg(svg) → deparse(gir) === "○{●}"
 */
export function extractGirFromSvg(svg: string): Gir | null {
  const match = svg.match(/<!\[CDATA\[([\s\S]*?)\]\]>/);
  if (!match) return null;
  try {
    return JSON.parse(match[1]) as Gir;
  } catch {
    return null;
  }
}

/**
 * Full SVG round-trip: SVG → GIR → UL-Script text.
 * Returns null if the SVG has no embedded GIR metadata.
 */
export function svgToUlScript(svg: string): string | null {
  const gir = extractGirFromSvg(svg);
  if (!gir) return null;
  return deparse(gir);
}

// ── Game context ──

/**
 * Create a new game context. Returns a context ID.
 */
export function createContext(config: GameConfig = {}): number {
  ensureInit();
  return wasmCreateContext(JSON.stringify(config));
}

/**
 * Evaluate a GIR against all active composition rules in a context.
 * Results are cached by context + GIR hash.
 */
export function evaluate(ctxId: number, gir: Gir): EvaluationResult {
  ensureInit();
  const key = `${ctxId}:${girHash(gir)}`;
  const cached = evaluationCache.get(key);
  if (cached) return cached;
  const result = wasmEvaluate(ctxId, JSON.stringify(gir)) as EvaluationResult;
  evaluationCache.set(key, result);
  return result;
}

/**
 * Score a composition against a specific puzzle target.
 */
export function scoreComposition(
  ctxId: number,
  gir: Gir,
  targetJson: string,
): ScoreResult {
  ensureInit();
  return wasmScoreComposition(ctxId, JSON.stringify(gir), targetJson) as ScoreResult;
}

/**
 * Evaluate a learning attempt with proficiency tracking.
 */
export function evaluateJaneAttempt(
  ctxId: number,
  attempt: Gir,
  expected: Gir,
): JaneResult {
  ensureInit();
  return wasmEvaluateJaneAttempt(
    ctxId,
    JSON.stringify(attempt),
    JSON.stringify(expected),
  ) as JaneResult;
}

/**
 * Validate ordering constraints across a sequence of GIRs.
 */
export function validateSequence(ctxId: number, glyphs: Gir[]): SequenceResult {
  ensureInit();
  const jsons = glyphs.map((g) => JSON.stringify(g));
  return wasmValidateSequence(ctxId, JSON.stringify(jsons)) as SequenceResult;
}

/**
 * Generate construction-order animation keyframes for a GIR.
 */
export function getAnimationSequence(
  gir: Gir,
  width: number,
  height: number,
): AnimationSequence {
  ensureInit();
  return wasmGetAnimationSequence(JSON.stringify(gir), width, height) as AnimationSequence;
}

/**
 * Compute positioned geometry for rendering a GIR.
 * Returns raw element positions and shapes — ideal for game engine integration
 * (Phaser, PixiJS, etc.) where you need coordinates, not SVG strings.
 * Results are cached by GIR hash + dimensions.
 */
export function layout(gir: Gir, width: number, height: number): PositionedGlyph {
  ensureInit();
  const key = `${girHash(gir)}:${width}x${height}`;
  const cached = layoutCache.get(key);
  if (cached) return cached;
  const result = wasmLayout(JSON.stringify(gir), width, height) as PositionedGlyph;
  layoutCache.set(key, result);
  return result;
}

/**
 * Load custom composition rules into an existing context.
 */
export function loadCustomDefinitions(ctxId: number, rulesJson: string): LoadResult {
  ensureInit();
  return wasmLoadCustomDefinitions(ctxId, rulesJson) as LoadResult;
}

// ── Algebraic composer ──

/**
 * Apply a Σ_UL operation to GIR operands.
 */
export function applyOperation(operation: OperationName, operands: Gir[]): Gir {
  ensureInit();
  const jsons = operands.map((g) => JSON.stringify(g));
  const result = wasmApplyOperation(operation, JSON.stringify(jsons)) as string;
  return JSON.parse(result) as Gir;
}

/**
 * Combine two GIRs with a named binary operation.
 */
export function composeGir(a: Gir, b: Gir, operation: OperationName): Gir {
  ensureInit();
  const result = wasmComposeGir(
    JSON.stringify(a),
    JSON.stringify(b),
    operation,
  ) as string;
  return JSON.parse(result) as Gir;
}

/**
 * Detect which Σ_UL operations are expressed in a GIR.
 */
export function detectOperations(gir: Gir): string[] {
  ensureInit();
  return wasmDetectOperations(JSON.stringify(gir)) as string[];
}

/**
 * Compute a comprehensive structural analysis report for a GIR.
 * Results are cached by GIR hash.
 */
export function analyzeStructure(gir: Gir): StructureReport {
  ensureInit();
  const key = girHash(gir);
  const cached = structureCache.get(key);
  if (cached) return cached;
  const result = wasmAnalyzeStructure(JSON.stringify(gir)) as StructureReport;
  structureCache.set(key, result);
  return result;
}

/**
 * Compare two GIR structures at a given Erlangen equivalence level.
 *
 * Levels (strictest → loosest):
 * - "euclidean": identical types, shapes, angles, sizes
 * - "similarity": ignore absolute scale
 * - "affine": ignore proportions, keep containment structure
 * - "projective": same incidence structure
 * - "topological": graph isomorphism of types only
 */
export function compareGir(a: Gir, b: Gir, level: ErlangenLevel): EquivalenceResult {
  ensureInit();
  return wasmCompareGir(JSON.stringify(a), JSON.stringify(b), level) as EquivalenceResult;
}

// ── Teaching system ──

/**
 * Generate contextual hints by comparing an attempt GIR to a target.
 */
export function getHints(attempt: Gir, target: Gir): Hint[] {
  ensureInit();
  return wasmGetHints(JSON.stringify(attempt), JSON.stringify(target)) as Hint[];
}

/**
 * Generate self-standing analysis hints for a GIR.
 */
export function analyzeHints(gir: Gir): Hint[] {
  ensureInit();
  return wasmAnalyzeHints(JSON.stringify(gir)) as Hint[];
}

/**
 * Get the next appropriate puzzle given student proficiency.
 */
export function getNextPuzzle(proficiency: Record<string, number>): Puzzle {
  ensureInit();
  return wasmGetNextPuzzle(JSON.stringify(proficiency)) as Puzzle;
}

// ── Lexicon ──

/**
 * Search the 42-entry UL lexicon by query string.
 */
export function queryLexicon(query: string): LexiconEntry[] {
  ensureInit();
  return wasmQueryLexicon(query) as LexiconEntry[];
}

/**
 * Look up a lexicon entry by exact name (case-insensitive).
 */
export function lookupLexiconEntry(name: string): LexiconEntry | null {
  ensureInit();
  return wasmLookupLexiconEntry(name) as LexiconEntry | null;
}

// ── Performative & Pragmatic extensions ──

export type ForceName = "assert" | "query" | "direct" | "commit" | "express" | "declare";

export interface PragmaticInference {
  rule: string;
  surface: Gir;
  intended: Gir;
}

/**
 * Set performative force on an assertion GIR.
 */
export function setForce(gir: Gir, force: ForceName): Gir {
  ensureInit();
  const result = wasmSetForce(JSON.stringify(gir), force);
  return (typeof result === "string" ? JSON.parse(result) : result) as Gir;
}

/**
 * Run pragmatic inference on a surface GIR.
 * Returns an array of {rule, surface, intended} objects.
 */
export function inferPragmatics(gir: Gir): PragmaticInference[] {
  ensureInit();
  const result = wasmInferPragmatics(JSON.stringify(gir));
  return (typeof result === "string" ? JSON.parse(result) : result) as PragmaticInference[];
}
