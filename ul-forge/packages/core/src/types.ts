// SYNC: Canonical source is packages/sdk/src/types.ts
// To sync: cp ../sdk/src/types.ts src/types.ts
/**
 * @ul-forge/sdk — Type definitions for the Universal Language formal system.
 *
 * These types mirror the Rust types in ul-core and ul-game, providing
 * full type safety for TypeScript consumers of the UL WASM module.
 */

// ── GIR types (mirror Rust ul-core::types) ──

export type Sort = "entity" | "relation" | "modifier" | "assertion";

export type NodeType = "point" | "line" | "angle" | "curve" | "enclosure" | "variable_slot";

export type EnclosureShape = "circle" | "triangle" | "square" | "ellipse" | "polygon" | "freeform";

export type EdgeType =
  | "contains"
  | "modified_by"
  | "adjacent"
  | "intersects"
  | "connects"
  | "references"
  | "binds";

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
  variable_id?: string;
  assertion_modifier?: "evidential" | "emphatic" | "hedged";
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
  binding_scope?: string[];
}

// ── Validation ──

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

// ── Game types (mirror Rust ul-game::types) ──

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
  | "bind"
  | "modify_assertion";

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
