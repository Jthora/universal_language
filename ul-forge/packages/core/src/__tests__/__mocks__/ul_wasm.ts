// Mock WASM module for testing — replaces the real ul_wasm.js at test time.
// See vitest.config.ts alias configuration.

// wasm-bindgen default export: loads and initializes the .wasm binary
export default async function __wbg_init(_input?: RequestInfo | URL): Promise<unknown> {
  return {};
}

// Separate init that runs after wasm binary is loaded
export function init(): void {}
export function parseUlScript(_input: string): string {
  return '{"nodes":[{"id":"n1","node_type":"point","label":"existence"}],"edges":[],"root":"n1"}';
}
export function deparseGir(_gir: string): string {
  return "●";
}
export function validateGir(_gir: string, _checkGeometry: boolean): unknown {
  return { valid: true, errors: [], warnings: [] };
}
export function renderSvg(_gir: string, _w: number, _h: number): string {
  return '<svg xmlns="http://www.w3.org/2000/svg"><circle r="4"/></svg>';
}
export function renderGlyphPreview(_gir: string): string {
  return '<svg xmlns="http://www.w3.org/2000/svg"><circle r="2"/></svg>';
}
export function createContext(_config: string): number {
  return 1;
}
export function evaluate(_ctx: number, _gir: string): unknown {
  return { grade: "exact", score: 1.0 };
}
export function scoreComposition(_ctx: number, _gir: string, _target: string): unknown {
  return { score: 0.9, grade: "close" };
}
export function evaluateJaneAttempt(_ctx: number, _attempt: string, _expected: string): unknown {
  return { correct: true };
}
export function validateSequence(_ctx: number, _glyphs: string): unknown {
  return { valid: true };
}
export function getAnimationSequence(_gir: string, _w: number, _h: number): unknown {
  return { keyframes: [] };
}
export function layout(_gir: string, _w: number, _h: number): unknown {
  return { elements: [], connections: [] };
}
export function loadCustomDefinitions(_ctx: number, _rules: string): unknown {
  return { loaded: true };
}
export function applyOperation(_op: string, _operands: string): string {
  return '{"nodes":[],"edges":[],"root":"n1"}';
}
export function composeGir(_a: string, _b: string, _op: string): string {
  return '{"nodes":[],"edges":[],"root":"n1"}';
}
export function detectOperations(_gir: string): unknown {
  return ["predicate"];
}
export function analyzeStructure(_gir: string): unknown {
  return { symmetry: "none", depth: 1 };
}
export function compareGir(_a: string, _b: string, _level: string): unknown {
  return { equivalent: true };
}
export function getHints(_attempt: string, _target: string): unknown {
  return [];
}
export function analyzeHints(_gir: string): unknown {
  return [];
}
export function getNextPuzzle(_proficiency: string): unknown {
  return { id: "p1", target: {} };
}
export function queryLexicon(_query: string): unknown {
  return [];
}
export function lookupLexiconEntry(_name: string): unknown {
  return null;
}
