/**
 * WASM mock for unit tests.
 *
 * Provides deterministic, synchronous mock implementations of all 23 WASM
 * functions. Use vi.mock("ul-wasm", ...) in tests that should NOT load
 * the real WASM binary.
 */

import { vi } from "vitest";

/** A simple point GIR for parse results. */
const MOCK_GIR_JSON = JSON.stringify({
  ul_gir: "0.2",
  root: "p",
  nodes: [{ id: "p", node_type: "point", sort: "entity" }],
  edges: [],
});

const MOCK_VALIDATION = { valid: true, errors: [], warnings: [] };
const MOCK_SVG = '<svg xmlns="http://www.w3.org/2000/svg"><circle cx="10" cy="10" r="5"/></svg>';
const MOCK_UL_TEXT = "●";

export const wasmMock = {
  default: vi.fn().mockResolvedValue(undefined),
  init: vi.fn(),
  parseUlScript: vi.fn().mockReturnValue(MOCK_GIR_JSON),
  deparseGir: vi.fn().mockReturnValue(MOCK_UL_TEXT),
  validateGir: vi.fn().mockReturnValue(MOCK_VALIDATION),
  renderSvg: vi.fn().mockReturnValue(MOCK_SVG),
  renderGlyphPreview: vi.fn().mockReturnValue(MOCK_SVG),
  createContext: vi.fn().mockReturnValue(1),
  evaluate: vi.fn().mockReturnValue({
    score: 1.0,
    grade: "exact",
    matched_rules: ["point_exists"],
    violated_rules: [],
    feedback: [],
  }),
  scoreComposition: vi.fn().mockReturnValue({
    score: 0.85,
    grade: "close",
    partial_credit: {
      structural_match: 0.9,
      sort_correctness: 0.8,
      operation_correctness: 0.85,
      sequence_order: 0.8,
    },
    feedback: ["Good structure."],
  }),
  evaluateJaneAttempt: vi.fn().mockReturnValue({
    score: 0.7,
    grade: "close",
    improvements: ["Try adding more depth."],
    proficiency_delta: { predicate: 0.1 },
  }),
  validateSequence: vi.fn().mockReturnValue({
    valid: true,
    errors: [],
    pair_scores: [1.0],
  }),
  getAnimationSequence: vi.fn().mockReturnValue({
    keyframes: [
      {
        node_id: "p",
        timestamp_ms: 0,
        x: 128,
        y: 128,
        scale: 1.0,
        rotation: 0,
        opacity: 1.0,
        easing: "ease_in_out",
      },
    ],
    total_duration_ms: 500,
  }),
  layout: vi.fn().mockReturnValue({ nodes: [], edges: [] }),
  loadCustomDefinitions: vi.fn().mockReturnValue({
    loaded: 1,
    errors: [],
  }),
  applyOperation: vi.fn().mockReturnValue(MOCK_GIR_JSON),
  composeGir: vi.fn().mockReturnValue(MOCK_GIR_JSON),
  detectOperations: vi.fn().mockReturnValue([]),
  analyzeStructure: vi.fn().mockReturnValue({
    node_count: 1,
    edge_count: 0,
    primitive_counts: { point: 1 },
    sort_distribution: { entity: 1 },
    detected_operations: [],
    depth: 1,
    breadth: 0,
    complexity_score: 0.1,
  }),
  getHints: vi.fn().mockReturnValue([
    {
      severity: "info",
      category: "general",
      message: "Good start!",
    },
  ]),
  analyzeHints: vi.fn().mockReturnValue([]),
  getNextPuzzle: vi.fn().mockReturnValue({
    id: "single_point",
    difficulty: 1,
    level: 1,
    required_operations: [],
    description: "Place a single point.",
    target_gir_json: MOCK_GIR_JSON,
  }),
  queryLexicon: vi.fn().mockReturnValue([
    {
      id: 1,
      level: 1,
      name: "Single Point",
      tier: "T1",
      symbol: "●",
      sigma_ul: "e",
      labels: ["Existence", "Being", "Something"],
    },
  ]),
  lookupLexiconEntry: vi.fn().mockReturnValue({
    id: 1,
    level: 1,
    name: "Single Point",
    tier: "T1",
    symbol: "●",
    sigma_ul: "e",
    labels: ["Existence", "Being", "Something"],
  }),
};

/** Install the mock. Call in a vi.mock("ul-wasm", ...) factory. */
export function installWasmMock() {
  return wasmMock;
}

/** Reset all mock call counts. */
export function resetWasmMock() {
  Object.values(wasmMock).forEach((fn) => {
    if (typeof fn === "function" && "mockClear" in fn) {
      (fn as ReturnType<typeof vi.fn>).mockClear();
    }
  });
}
