/**
 * @ul-forge/core — TypeScript wrapper for UL Forge WASM module.
 *
 * Provides typed interfaces over the raw wasm-bindgen bindings.
 */

// ── GIR types (mirror Rust types) ──

export type Sort = "entity" | "relation" | "modifier" | "assertion";

export type NodeType = "point" | "line" | "angle" | "curve" | "enclosure";

export type EnclosureShape = "circle" | "triangle" | "square";

export type EdgeType =
  | "contains"
  | "modified_by"
  | "adjacent"
  | "intersects"
  | "connects"
  | "references";

export interface Node {
  id: string;
  type: NodeType;
  sort: Sort;
  label?: string;
  shape?: EnclosureShape;
  directed?: boolean;
  measure?: number;
  curvature?: number;
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

export interface ValidationResult {
  valid: boolean;
  errors: string[];
  warnings: string[];
}

// ── Render options ──

export type OutputFormat = "svg" | "tikz";

export interface RenderOptions {
  format?: OutputFormat;
  width?: number;
  height?: number;
  embedGir?: boolean;
}

// ── Core API ──

import init, {
  parse as wasmParse,
  validate as wasmValidate,
  render as wasmRender,
  deparse as wasmDeparse,
} from "ul-wasm";

let initialized = false;

/**
 * Initialize the WASM module. Must be called before any other function.
 * Safe to call multiple times — subsequent calls are no-ops.
 */
export async function initialize(): Promise<void> {
  if (!initialized) {
    await init();
    initialized = true;
  }
}

/**
 * Parse UL-Script text into a GIR document.
 * @throws Error if the input has syntax errors.
 */
export function parse(input: string): Gir {
  if (!initialized) {
    throw new Error("WASM not initialized. Call initialize() first.");
  }
  return wasmParse(input) as Gir;
}

/**
 * Validate a GIR document against Σ_UL constraints.
 */
export function validate(gir: Gir): ValidationResult {
  if (!initialized) {
    throw new Error("WASM not initialized. Call initialize() first.");
  }
  const json = JSON.stringify(gir);
  return wasmValidate(json) as ValidationResult;
}

/**
 * Render a GIR document to SVG string.
 * @throws Error if the GIR is invalid for rendering.
 */
export function render(gir: Gir): string {
  if (!initialized) {
    throw new Error("WASM not initialized. Call initialize() first.");
  }
  const json = JSON.stringify(gir);
  return wasmRender(json);
}

/**
 * Convert a GIR document back to canonical UL-Script text.
 */
export function deparse(gir: Gir): string {
  if (!initialized) {
    throw new Error("WASM not initialized. Call initialize() first.");
  }
  const json = JSON.stringify(gir);
  return wasmDeparse(json);
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
 * Returns validation result alongside the SVG (or null if invalid).
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
