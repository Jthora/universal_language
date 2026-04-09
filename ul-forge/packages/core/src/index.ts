/**
 * @ul-forge/core — Universal Language for AI Agents
 *
 * Parse, validate, render, and compose geometric meaning structures.
 * 5 primitives, 4 sorts, 13 operations — unique up to isomorphism.
 *
 * @example
 * ```typescript
 * import { initialize, parse, render, validate } from '@ul-forge/core';
 *
 * await initialize();
 * const gir = parse('point(existence)');
 * const validation = validate(gir);
 * const svg = render(gir, 200, 200);
 * ```
 *
 * @packageDocumentation
 */

// Types — all Σ_UL types for TypeScript consumers
export type {
  Sort, NodeType, EnclosureShape, EdgeType, Node, Edge, GirMetadata, Gir,
  ValidationLayers, ValidationResult,
  OutputFormat, RenderOptions,
  OperationName, Grade, Tier, Easing,
  PartialCredit, EvaluationResult, ScoreResult, JaneResult,
  SequenceResult, AnimationKeyframe, AnimationSequence, LoadResult,
  SymmetryGroup, PartOfSpeech, StructureReport,
  ErlangenLevel, EquivalenceDimensions, EquivalenceResult,
  Hint, Puzzle, LexiconEntry, GameConfig,
  ShapeType, PositionedElement, LayoutConnection, PositionedGlyph,
} from "./types.js";

// Cache utility
export { ResultCache } from "./cache.js";

// WASM bridge — all 23 functions + initialization
export {
  initialize,
  clearCaches,
  _resetForTesting,
  // Core pipeline
  parse,
  validate,
  render,
  renderPreview,
  deparse,
  parseAndRender,
  parseValidateRender,
  // Game context
  createContext,
  evaluate,
  scoreComposition,
  evaluateJaneAttempt,
  validateSequence,
  getAnimationSequence,
  layout,
  loadCustomDefinitions,
  // Algebraic composer
  applyOperation,
  composeGir,
  detectOperations,
  analyzeStructure,
  compareGir,
  // Teaching
  getHints,
  analyzeHints,
  getNextPuzzle,
  // Lexicon
  queryLexicon,
  lookupLexiconEntry,
} from "./wasm-bridge.js";
