/**
 * Tier 1: Core wrapper unit tests.
 *
 * Tests that all core/index.ts wrapper functions:
 * - Check initialization guard
 * - Serialize/deserialize correctly
 * - Pass correct arguments to WASM functions
 * - Return properly typed results
 *
 * Uses mocked WASM — no binary required.
 */

import { describe, it, expect, beforeEach, vi } from "vitest";

vi.mock("ul-wasm", async () => {
  const mod = await import("./helpers/wasm-mock");
  return mod.installWasmMock();
});

import {
  initialize,
  _resetForTesting,
  parse,
  validate,
  render,
  renderPreview,
  deparse,
  parseAndRender,
  parseValidateRender,
  createContext,
  evaluate,
  scoreComposition,
  evaluateJaneAttempt,
  validateSequence,
  getAnimationSequence,
  layout,
  loadCustomDefinitions,
  applyOperation,
  composeGir,
  detectOperations,
  analyzeStructure,
  getHints,
  analyzeHints,
  getNextPuzzle,
  queryLexicon,
  lookupLexiconEntry,
  clearCaches,
} from "../core";

import { wasmMock, resetWasmMock } from "./helpers/wasm-mock";
import { SINGLE_POINT, PREDICATE_GIR, ENCLOSURE_GIR } from "./helpers/gir-fixtures";

beforeEach(async () => {
  resetWasmMock();
  clearCaches();
  _resetForTesting();
  await initialize();
});

// ── Initialization guard ──

describe("initialization guard", () => {
  it("throws if WASM not initialized", () => {
    _resetForTesting();
    expect(() => parse("●")).toThrow("WASM not initialized");
  });

  it("initialize() is idempotent", async () => {
    await initialize();
    await initialize();
    // default (wasmInit) should only be called once (from beforeEach)
    expect(wasmMock.default).toHaveBeenCalledTimes(1);
  });
});

// ── UL-Core pass-throughs ──

describe("parse", () => {
  it("returns a parsed Gir object", () => {
    const gir = parse("●");
    expect(gir).toBeDefined();
    expect(gir.ul_gir).toBe("0.2");
    expect(wasmMock.parseUlScript).toHaveBeenCalledWith("●");
  });

  it("passes input string to WASM", () => {
    parse("○{●──●}");
    expect(wasmMock.parseUlScript).toHaveBeenCalledWith("○{●──●}");
  });
});

describe("validate", () => {
  it("serializes GIR and returns validation result", () => {
    const result = validate(SINGLE_POINT);
    expect(result.valid).toBe(true);
    expect(result.errors).toEqual([]);
    expect(wasmMock.validateGir).toHaveBeenCalledWith(
      JSON.stringify(SINGLE_POINT),
      false,
    );
  });

  it("passes checkGeometry flag", () => {
    validate(SINGLE_POINT, true);
    expect(wasmMock.validateGir).toHaveBeenCalledWith(
      JSON.stringify(SINGLE_POINT),
      true,
    );
  });
});

describe("render", () => {
  it("returns SVG string", () => {
    const svg = render(SINGLE_POINT);
    expect(svg).toContain("<svg");
    expect(wasmMock.renderSvg).toHaveBeenCalledWith(
      JSON.stringify(SINGLE_POINT),
      256,
      256,
    );
  });

  it("accepts custom dimensions", () => {
    render(SINGLE_POINT, 512, 512);
    expect(wasmMock.renderSvg).toHaveBeenCalledWith(
      JSON.stringify(SINGLE_POINT),
      512,
      512,
    );
  });
});

describe("renderPreview", () => {
  it("renders a compact preview", () => {
    const svg = renderPreview(SINGLE_POINT);
    expect(svg).toContain("<svg");
    expect(wasmMock.renderGlyphPreview).toHaveBeenCalled();
  });
});

describe("deparse", () => {
  it("converts GIR back to UL-Script text", () => {
    const text = deparse(SINGLE_POINT);
    expect(text).toBe("●");
    expect(wasmMock.deparseGir).toHaveBeenCalledWith(
      JSON.stringify(SINGLE_POINT),
    );
  });
});

describe("parseAndRender", () => {
  it("chains parse → render", () => {
    const svg = parseAndRender("●");
    expect(wasmMock.parseUlScript).toHaveBeenCalled();
    expect(wasmMock.renderSvg).toHaveBeenCalled();
    expect(svg).toContain("<svg");
  });
});

describe("parseValidateRender", () => {
  it("returns gir, validation, and svg", () => {
    const result = parseValidateRender("●");
    expect(result.gir).toBeDefined();
    expect(result.validation.valid).toBe(true);
    expect(result.svg).toContain("<svg");
  });

  it("returns null svg when validation fails", () => {
    wasmMock.validateGir.mockReturnValueOnce({
      valid: false,
      errors: ["bad"],
      warnings: [],
    });
    const result = parseValidateRender("●");
    expect(result.svg).toBeNull();
  });
});

// ── Game context ──

describe("createContext", () => {
  it("returns a context ID", () => {
    const id = createContext();
    expect(id).toBe(1);
    expect(wasmMock.createContext).toHaveBeenCalledWith(JSON.stringify({}));
  });

  it("passes config to WASM", () => {
    createContext({ session_id: "test-123" });
    expect(wasmMock.createContext).toHaveBeenCalledWith(
      JSON.stringify({ session_id: "test-123" }),
    );
  });
});

describe("evaluate", () => {
  it("returns evaluation result", () => {
    const result = evaluate(1, SINGLE_POINT);
    expect(result.score).toBe(1.0);
    expect(result.grade).toBe("exact");
    expect(wasmMock.evaluate).toHaveBeenCalledWith(
      1,
      JSON.stringify(SINGLE_POINT),
    );
  });
});

describe("scoreComposition", () => {
  it("returns score result with partial credit", () => {
    const result = scoreComposition(1, SINGLE_POINT, "{}");
    expect(result.score).toBe(0.85);
    expect(result.partial_credit).toBeDefined();
    expect(result.partial_credit.structural_match).toBe(0.9);
  });
});

describe("evaluateJaneAttempt", () => {
  it("returns proficiency delta", () => {
    const result = evaluateJaneAttempt(1, SINGLE_POINT, PREDICATE_GIR);
    expect(result.proficiency_delta).toHaveProperty("predicate");
    expect(wasmMock.evaluateJaneAttempt).toHaveBeenCalledWith(
      1,
      JSON.stringify(SINGLE_POINT),
      JSON.stringify(PREDICATE_GIR),
    );
  });
});

describe("validateSequence", () => {
  it("validates a sequence of GIRs", () => {
    const result = validateSequence(1, [SINGLE_POINT, PREDICATE_GIR]);
    expect(result.valid).toBe(true);
    expect(result.pair_scores).toHaveLength(1);
  });
});

describe("getAnimationSequence", () => {
  it("returns keyframes and duration", () => {
    const seq = getAnimationSequence(SINGLE_POINT, 256, 256);
    expect(seq.keyframes).toHaveLength(1);
    expect(seq.total_duration_ms).toBe(500);
  });
});

describe("layout", () => {
  it("returns positioned geometry", () => {
    const result = layout(SINGLE_POINT, 256, 256);
    expect(result).toBeDefined();
    expect(wasmMock.layout).toHaveBeenCalledWith(
      JSON.stringify(SINGLE_POINT),
      256,
      256,
    );
  });
});

describe("loadCustomDefinitions", () => {
  it("returns load result", () => {
    const result = loadCustomDefinitions(1, "[]");
    expect(result.loaded).toBe(1);
    expect(result.errors).toEqual([]);
  });
});

// ── Algebraic composer ──

describe("applyOperation", () => {
  it("applies a unary operation", () => {
    const result = applyOperation("negate", [ENCLOSURE_GIR]);
    expect(result).toBeDefined();
    expect(result.ul_gir).toBe("0.2");
  });

  it("serializes operands as JSON array of JSON strings", () => {
    applyOperation("conjoin", [ENCLOSURE_GIR, ENCLOSURE_GIR]);
    const [op, operandsJson] = wasmMock.applyOperation.mock.calls[0];
    expect(op).toBe("conjoin");
    const operands = JSON.parse(operandsJson);
    expect(operands).toHaveLength(2);
    // Each operand should be a JSON string (double-serialized)
    expect(typeof operands[0]).toBe("string");
  });
});

describe("composeGir", () => {
  it("combines two GIRs with a binary operation", () => {
    const result = composeGir(ENCLOSURE_GIR, ENCLOSURE_GIR, "conjoin");
    expect(result.ul_gir).toBe("0.2");
    expect(wasmMock.composeGir).toHaveBeenCalledWith(
      JSON.stringify(ENCLOSURE_GIR),
      JSON.stringify(ENCLOSURE_GIR),
      "conjoin",
    );
  });
});

describe("detectOperations", () => {
  it("returns array of operation names", () => {
    wasmMock.detectOperations.mockReturnValueOnce(["predicate", "embed"]);
    const ops = detectOperations(PREDICATE_GIR);
    expect(ops).toEqual(["predicate", "embed"]);
  });
});

describe("analyzeStructure", () => {
  it("returns structure report", () => {
    const report = analyzeStructure(SINGLE_POINT);
    expect(report.node_count).toBe(1);
    expect(report.edge_count).toBe(0);
    expect(report.primitive_counts).toHaveProperty("point");
    expect(report.complexity_score).toBeGreaterThanOrEqual(0);
  });
});

// ── Teaching system ──

describe("getHints", () => {
  it("returns hints array", () => {
    const hints = getHints(SINGLE_POINT, PREDICATE_GIR);
    expect(hints).toHaveLength(1);
    expect(hints[0].severity).toBe("info");
    expect(hints[0].message).toBeTruthy();
  });
});

describe("analyzeHints", () => {
  it("returns hints for a standalone GIR", () => {
    const hints = analyzeHints(SINGLE_POINT);
    expect(Array.isArray(hints)).toBe(true);
  });
});

describe("getNextPuzzle", () => {
  it("returns a puzzle object", () => {
    const puzzle = getNextPuzzle({});
    expect(puzzle.id).toBe("single_point");
    expect(puzzle.difficulty).toBe(1);
    expect(puzzle.target_gir_json).toBeTruthy();
  });

  it("passes proficiency map to WASM", () => {
    getNextPuzzle({ predicate: 0.9, embed: 0.5 });
    expect(wasmMock.getNextPuzzle).toHaveBeenCalledWith(
      JSON.stringify({ predicate: 0.9, embed: 0.5 }),
    );
  });
});

// ── Lexicon ──

describe("queryLexicon", () => {
  it("returns matching entries", () => {
    const entries = queryLexicon("point");
    expect(entries).toHaveLength(1);
    expect(entries[0].name).toBe("Single Point");
    expect(entries[0].tier).toBe("T1");
  });

  it("passes query to WASM", () => {
    queryLexicon("existence");
    expect(wasmMock.queryLexicon).toHaveBeenCalledWith("existence");
  });
});

describe("lookupLexiconEntry", () => {
  it("returns entry by name", () => {
    const entry = lookupLexiconEntry("Single Point");
    expect(entry).not.toBeNull();
    expect(entry!.id).toBe(1);
    expect(entry!.labels).toContain("Existence");
  });

  it("returns null for unknown name", () => {
    wasmMock.lookupLexiconEntry.mockReturnValueOnce(null);
    const entry = lookupLexiconEntry("nonexistent");
    expect(entry).toBeNull();
  });
});
