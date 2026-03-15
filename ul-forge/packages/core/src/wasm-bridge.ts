/**
 * @ul-forge/core — WASM bridge
 *
 * Wraps the wasm-bindgen output with typed functions, result caching,
 * and lazy initialization. This module is the runtime heart of the package.
 *
 * WASM module is loaded lazily on first `initialize()` call.
 */

import type {
  Gir,
  GameConfig,
  ValidationResult,
  EvaluationResult,
  ScoreResult,
  JaneResult,
  SequenceResult,
  AnimationSequence,
  PositionedGlyph,
  LoadResult,
  OperationName,
  StructureReport,
  ErlangenLevel,
  EquivalenceResult,
  Hint,
  Puzzle,
  LexiconEntry,
} from "./types.js";
import { ResultCache } from "./cache.js";

// ── WASM module handle ──

type WasmModule = {
  default: (input?: RequestInfo | URL) => Promise<unknown>;
  init: () => void;
  parseUlScript: (input: string) => string;
  deparseGir: (gir: string) => string;
  validateGir: (gir: string, checkGeometry: boolean) => unknown;
  renderSvg: (gir: string, w: number, h: number) => string;
  renderGlyphPreview: (gir: string) => string;
  createContext: (config: string) => number;
  evaluate: (ctx: number, gir: string) => unknown;
  scoreComposition: (ctx: number, gir: string, target: string) => unknown;
  evaluateJaneAttempt: (ctx: number, attempt: string, expected: string) => unknown;
  validateSequence: (ctx: number, glyphs: string) => unknown;
  getAnimationSequence: (gir: string, w: number, h: number) => unknown;
  layout: (gir: string, w: number, h: number) => unknown;
  loadCustomDefinitions: (ctx: number, rules: string) => unknown;
  applyOperation: (op: string, operands: string) => string;
  composeGir: (a: string, b: string, op: string) => string;
  detectOperations: (gir: string) => unknown;
  analyzeStructure: (gir: string) => unknown;
  compareGir: (a: string, b: string, level: string) => unknown;
  getHints: (attempt: string, target: string) => unknown;
  analyzeHints: (gir: string) => unknown;
  getNextPuzzle: (proficiency: string) => unknown;
  queryLexicon: (query: string) => unknown;
  lookupLexiconEntry: (name: string) => unknown;
};

let wasm: WasmModule | null = null;
let initialized = false;

function ensureInit(): void {
  if (!initialized) {
    throw new Error("WASM not initialized. Call initialize() first.");
  }
}

// ── Result caches ──

function girHash(gir: Gir): string {
  return JSON.stringify({ r: gir.root, n: gir.nodes.length, e: gir.edges.length });
}

const layoutCache = new ResultCache<PositionedGlyph>();
const evaluationCache = new ResultCache<EvaluationResult>();
const renderCache = new ResultCache<string>();
const structureCache = new ResultCache<StructureReport>();

/** Clear all result caches. */
export function clearCaches(): void {
  layoutCache.clear();
  evaluationCache.clear();
  renderCache.clear();
  structureCache.clear();
}

// ── Initialization ──

/**
 * Initialize the WASM module.
 *
 * @param wasmUrl - Optional URL or path to the `.wasm` binary.
 *   Defaults to the bundled `ul_wasm_bg.wasm` shipped with this package.
 *   Pass a custom URL if self-hosting or using a CDN.
 */
export async function initialize(wasmUrl?: string | URL): Promise<void> {
  if (initialized) return;

  // Dynamic import so the module isn't loaded until needed
  const mod = await import("../wasm/ul_wasm.js") as unknown as WasmModule;
  wasm = mod;

  if (wasmUrl) {
    await mod.default(wasmUrl);
  } else {
    await mod.default();
  }
  mod.init();
  initialized = true;
}

/** For testing: reset the initialized flag. */
export function _resetForTesting(): void {
  initialized = false;
  wasm = null;
}

// ── UL-Core pass-throughs ──

/** Parse UL-Script text into a GIR document. */
export function parse(input: string): Gir {
  ensureInit();
  return JSON.parse(wasm!.parseUlScript(input)) as Gir;
}

/** Validate a GIR document against Σ_UL constraints. */
export function validate(gir: Gir, checkGeometry = false): ValidationResult {
  ensureInit();
  return wasm!.validateGir(JSON.stringify(gir), checkGeometry) as ValidationResult;
}

/** Render a GIR document to SVG. Cached by GIR hash + dimensions. */
export function render(gir: Gir, width = 256, height = 256): string {
  ensureInit();
  const key = `${girHash(gir)}:${width}x${height}`;
  const cached = renderCache.get(key);
  if (cached) return cached;
  const svg = wasm!.renderSvg(JSON.stringify(gir), width, height) as string;
  renderCache.set(key, svg);
  return svg;
}

/** Render a GIR to a compact 64×64 SVG preview. */
export function renderPreview(gir: Gir): string {
  ensureInit();
  return wasm!.renderGlyphPreview(JSON.stringify(gir)) as string;
}

/** Convert a GIR document back to canonical UL-Script text. */
export function deparse(gir: Gir): string {
  ensureInit();
  return wasm!.deparseGir(JSON.stringify(gir)) as string;
}

/** Convenience: parse UL-Script directly to SVG. */
export function parseAndRender(input: string, width = 256, height = 256): string {
  return render(parse(input), width, height);
}

/** Convenience: parse, validate, and render in one call. */
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

// ── Game context ──

/** Create a new game context. Returns a numeric context ID. */
export function createContext(config: GameConfig = {}): number {
  ensureInit();
  return wasm!.createContext(JSON.stringify(config));
}

/** Evaluate a GIR against active composition rules. Cached. */
export function evaluate(ctxId: number, gir: Gir): EvaluationResult {
  ensureInit();
  const key = `${ctxId}:${girHash(gir)}`;
  const cached = evaluationCache.get(key);
  if (cached) return cached;
  const result = wasm!.evaluate(ctxId, JSON.stringify(gir)) as EvaluationResult;
  evaluationCache.set(key, result);
  return result;
}

/** Score a composition against a puzzle target. */
export function scoreComposition(ctxId: number, gir: Gir, targetJson: string): ScoreResult {
  ensureInit();
  return wasm!.scoreComposition(ctxId, JSON.stringify(gir), targetJson) as ScoreResult;
}

/** Evaluate a learning attempt with proficiency tracking. */
export function evaluateJaneAttempt(ctxId: number, attempt: Gir, expected: Gir): JaneResult {
  ensureInit();
  return wasm!.evaluateJaneAttempt(ctxId, JSON.stringify(attempt), JSON.stringify(expected)) as JaneResult;
}

/** Validate ordering constraints across a sequence of GIRs. */
export function validateSequence(ctxId: number, glyphs: Gir[]): SequenceResult {
  ensureInit();
  return wasm!.validateSequence(ctxId, JSON.stringify(glyphs.map(g => JSON.stringify(g)))) as SequenceResult;
}

/** Generate construction-order animation keyframes. */
export function getAnimationSequence(gir: Gir, width: number, height: number): AnimationSequence {
  ensureInit();
  return wasm!.getAnimationSequence(JSON.stringify(gir), width, height) as AnimationSequence;
}

/** Compute positioned geometry for game-engine rendering. Cached. */
export function layout(gir: Gir, width: number, height: number): PositionedGlyph {
  ensureInit();
  const key = `${girHash(gir)}:${width}x${height}`;
  const cached = layoutCache.get(key);
  if (cached) return cached;
  const result = wasm!.layout(JSON.stringify(gir), width, height) as PositionedGlyph;
  layoutCache.set(key, result);
  return result;
}

/** Load custom composition rules into a context. */
export function loadCustomDefinitions(ctxId: number, rulesJson: string): LoadResult {
  ensureInit();
  return wasm!.loadCustomDefinitions(ctxId, rulesJson) as LoadResult;
}

// ── Algebraic composer (Σ_UL operations) ──

/** Apply a Σ_UL operation to GIR operands. */
export function applyOperation(operation: OperationName, operands: Gir[]): Gir {
  ensureInit();
  return JSON.parse(wasm!.applyOperation(operation, JSON.stringify(operands.map(g => JSON.stringify(g))))) as Gir;
}

/** Combine two GIRs with a binary Σ_UL operation. */
export function composeGir(a: Gir, b: Gir, operation: OperationName): Gir {
  ensureInit();
  return JSON.parse(wasm!.composeGir(JSON.stringify(a), JSON.stringify(b), operation)) as Gir;
}

/** Detect which Σ_UL operations are expressed in a GIR. */
export function detectOperations(gir: Gir): string[] {
  ensureInit();
  return wasm!.detectOperations(JSON.stringify(gir)) as string[];
}

/** Compute structural analysis report. Cached. */
export function analyzeStructure(gir: Gir): StructureReport {
  ensureInit();
  const key = girHash(gir);
  const cached = structureCache.get(key);
  if (cached) return cached;
  const result = wasm!.analyzeStructure(JSON.stringify(gir)) as StructureReport;
  structureCache.set(key, result);
  return result;
}

/** Compare two GIRs at a given Erlangen equivalence level. */
export function compareGir(a: Gir, b: Gir, level: ErlangenLevel): EquivalenceResult {
  ensureInit();
  return wasm!.compareGir(JSON.stringify(a), JSON.stringify(b), level) as EquivalenceResult;
}

// ── Teaching system ──

/** Generate contextual hints by comparing attempt to target. */
export function getHints(attempt: Gir, target: Gir): Hint[] {
  ensureInit();
  return wasm!.getHints(JSON.stringify(attempt), JSON.stringify(target)) as Hint[];
}

/** Generate analysis hints for a standalone GIR. */
export function analyzeHints(gir: Gir): Hint[] {
  ensureInit();
  return wasm!.analyzeHints(JSON.stringify(gir)) as Hint[];
}

/** Get next puzzle given student proficiency map. */
export function getNextPuzzle(proficiency: Record<string, number>): Puzzle {
  ensureInit();
  return wasm!.getNextPuzzle(JSON.stringify(proficiency)) as Puzzle;
}

// ── Lexicon ──

/** Search the 42-entry UL lexicon. */
export function queryLexicon(query: string): LexiconEntry[] {
  ensureInit();
  return wasm!.queryLexicon(query) as LexiconEntry[];
}

/** Look up a lexicon entry by exact name. */
export function lookupLexiconEntry(name: string): LexiconEntry | null {
  ensureInit();
  return wasm!.lookupLexiconEntry(name) as LexiconEntry | null;
}
