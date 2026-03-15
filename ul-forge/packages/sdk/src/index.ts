/**
 * @ul-forge/sdk — Universal Language SDK
 *
 * Standalone typed TypeScript API for the UL Forge WASM module.
 * Can be consumed by any framework (React, Phaser, PixiJS, Node.js, etc.).
 *
 * Usage:
 *   import { initialize, parse, layout, evaluate } from "@ul-forge/sdk";
 *   await initialize();
 *   const gir = parse("●->●");
 *   const positioned = layout(gir, 800, 600);
 */

export type {
  Sort,
  NodeType,
  EnclosureShape,
  EdgeType,
  Node,
  Edge,
  GirMetadata,
  Gir,
  ValidationResult,
  OutputFormat,
  RenderOptions,
  OperationName,
  Grade,
  Tier,
  Easing,
  PartialCredit,
  EvaluationResult,
  ScoreResult,
  JaneResult,
  SequenceResult,
  AnimationKeyframe,
  AnimationSequence,
  LoadResult,
  StructureReport,
  Hint,
  Puzzle,
  LexiconEntry,
  GameConfig,
  ShapeType,
  PositionedElement,
  LayoutConnection,
  PositionedGlyph,
} from "./types.js";

export { ResultCache, girHash } from "./cache.js";
