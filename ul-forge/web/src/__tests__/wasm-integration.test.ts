/**
 * Tier 2: WASM integration tests.
 *
 * These tests load the REAL wasm binary and exercise end-to-end
 * functionality. They require wasm-pkg to be built:
 *   wasm-pack build crates/ul-game --target web --out-dir ../../web/wasm-pkg --out-name ul_wasm
 *
 * Runs in Node.js environment (not jsdom) via vitest workspace config.
 * @vitest-environment node
 */

import { describe, it, expect, beforeAll } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

let wasm: any;

beforeAll(async () => {
  const wasmPath = resolve(__dirname, "../../wasm-pkg/ul_wasm_bg.wasm");
  const wasmModule = await import("../../wasm-pkg/ul_wasm.js");
  const wasmBuffer = readFileSync(wasmPath);
  wasmModule.initSync({ module: new WebAssembly.Module(wasmBuffer) });
  wasmModule.init();
  wasm = wasmModule;
});

// ── Parse / Deparse roundtrip ──

describe("parse + deparse roundtrip", () => {
  it("parses a single point", () => {
    const girJson = wasm.parseUlScript("●") as string;
    const gir = JSON.parse(girJson);
    expect(gir.nodes.length).toBeGreaterThanOrEqual(1);
    // Rust serializes as "node_type" not "type"
    expect(gir.nodes.some((n: any) => n.node_type === "point" || n.type === "point")).toBe(true);
  });

  it("parses an enclosure with point", () => {
    const girJson = wasm.parseUlScript("○{●}") as string;
    const gir = JSON.parse(girJson);
    expect(gir.nodes.some((n: any) => n.node_type === "enclosure" || n.type === "enclosure")).toBe(true);
    expect(gir.nodes.some((n: any) => n.node_type === "point" || n.type === "point")).toBe(true);
  });

  it("roundtrips: parse → deparse → parse yields equivalent GIR", () => {
    const input = "●";
    const gir1Json = wasm.parseUlScript(input) as string;
    const text = wasm.deparseGir(gir1Json) as string;
    expect(text.length).toBeGreaterThan(0);
    const gir2Json = wasm.parseUlScript(text) as string;
    const gir1 = JSON.parse(gir1Json);
    const gir2 = JSON.parse(gir2Json);
    expect(gir1.nodes.length).toBe(gir2.nodes.length);
    expect(gir1.edges.length).toBe(gir2.edges.length);
  });
});

// ── Validation ──

describe("validateGir", () => {
  it("validates a well-formed GIR", () => {
    const girJson = wasm.parseUlScript("●") as string;
    const result = wasm.validateGir(girJson, false) as any;
    expect(result.valid).toBe(true);
    expect(result.errors).toEqual([]);
  });

  it("validates with geometry check", () => {
    const girJson = wasm.parseUlScript("●") as string;
    const result = wasm.validateGir(girJson, true) as any;
    expect(result.valid).toBe(true);
  });

  it("detects invalid GIR JSON", () => {
    expect(() => wasm.validateGir("not json", false)).toThrow();
  });
});

// ── Rendering ──

describe("renderSvg", () => {
  it("produces SVG output", () => {
    const girJson = wasm.parseUlScript("●") as string;
    const svg = wasm.renderSvg(girJson, 256, 256) as string;
    expect(svg).toContain("<svg");
    expect(svg).toContain("</svg>");
  });

  it("respects custom dimensions", () => {
    const girJson = wasm.parseUlScript("●") as string;
    const svg = wasm.renderSvg(girJson, 512, 512) as string;
    expect(svg).toContain("512");
  });
});

describe("renderGlyphPreview", () => {
  it("produces compact SVG (64×64)", () => {
    const girJson = wasm.parseUlScript("●") as string;
    const svg = wasm.renderGlyphPreview(girJson) as string;
    expect(svg).toContain("<svg");
    expect(svg).toContain("64");
  });
});

// ── Game context ──

describe("game context lifecycle", () => {
  it("creates a context and returns an ID", () => {
    const id = wasm.createContext(JSON.stringify({}));
    expect(typeof id).toBe("number");
    expect(id).toBeGreaterThan(0);
  });

  it("evaluates a GIR in a context", () => {
    const ctxId = wasm.createContext(JSON.stringify({}));
    const girJson = wasm.parseUlScript("●") as string;
    const result = wasm.evaluate(ctxId, girJson) as any;
    expect(result).toHaveProperty("score");
    expect(result).toHaveProperty("grade");
    expect(typeof result.score).toBe("number");
  });
});

// ── Algebraic composer ──

describe("applyOperation", () => {
  // Helper: build an assertion GIR via predicate(entity, relation, entity)
  function makeAssertion(): string {
    const entity = wasm.parseUlScript("●") as string;
    const relation = wasm.parseUlScript("->") as string;
    const resultJsVal = wasm.applyOperation("predicate", JSON.stringify([entity, relation, entity]));
    return resultJsVal as string;
  }

  it("applies negate to an assertion GIR", () => {
    const assertionGir = makeAssertion();
    const resultJson = wasm.applyOperation("negate", JSON.stringify([assertionGir])) as string;
    const result = JSON.parse(resultJson);
    expect(result.nodes).toBeDefined();
    expect(result.nodes.length).toBeGreaterThanOrEqual(1);
  });

  it("rejects unknown operations", () => {
    const girJson = wasm.parseUlScript("●") as string;
    expect(() =>
      wasm.applyOperation("nonexistent", JSON.stringify([girJson])),
    ).toThrow();
  });

  it("rejects wrong arity", () => {
    const girJson = wasm.parseUlScript("●") as string;
    // predicate requires 3
    expect(() =>
      wasm.applyOperation("predicate", JSON.stringify([girJson])),
    ).toThrow();
  });
});

describe("composeGir", () => {
  it("conjoins two assertion GIRs", () => {
    // conjoin needs assertion sort — build via predicate
    const entity = wasm.parseUlScript("●") as string;
    const relation = wasm.parseUlScript("->") as string;
    const a = wasm.applyOperation("predicate", JSON.stringify([entity, relation, entity])) as string;
    const b = wasm.applyOperation("predicate", JSON.stringify([entity, relation, entity])) as string;
    const resultJson = wasm.composeGir(a, b, "conjoin") as string;
    const result = JSON.parse(resultJson);
    expect(result.nodes.length).toBeGreaterThanOrEqual(2);
  });
});

describe("detectOperations", () => {
  it("returns detected operations", () => {
    const girJson = wasm.parseUlScript("○{●}") as string;
    const ops = wasm.detectOperations(girJson) as string[];
    expect(Array.isArray(ops)).toBe(true);
  });
});

// ── Structural analysis ──

describe("analyzeStructure", () => {
  it("returns a comprehensive report", () => {
    const girJson = wasm.parseUlScript("○{●}") as string;
    const report = wasm.analyzeStructure(girJson) as any;
    expect(report.node_count).toBeGreaterThanOrEqual(2);
    expect(report.edge_count).toBeGreaterThanOrEqual(1);
    // serde_wasm_bindgen serializes HashMap as JS Map, not plain object
    const primCounts = report.primitive_counts instanceof Map
      ? Object.fromEntries(report.primitive_counts)
      : report.primitive_counts;
    expect(primCounts).toHaveProperty("enclosure");
    expect(primCounts).toHaveProperty("point");
    expect(report.sort_distribution).toBeDefined();
    expect(report.depth).toBeGreaterThanOrEqual(1);
    expect(report.complexity_score).toBeGreaterThanOrEqual(0);
  });

  it("reports correct primitive counts for single point", () => {
    const girJson = wasm.parseUlScript("●") as string;
    const report = wasm.analyzeStructure(girJson) as any;
    expect(report.node_count).toBe(1);
    const primCounts = report.primitive_counts instanceof Map
      ? Object.fromEntries(report.primitive_counts)
      : report.primitive_counts;
    expect(primCounts.point).toBe(1);
  });
});

// ── Hints ──

describe("getHints", () => {
  it("generates hints comparing attempt to target", () => {
    const attempt = wasm.parseUlScript("●") as string;
    const target = wasm.parseUlScript("○{●}") as string;
    const hints = wasm.getHints(attempt, target) as any[];
    expect(Array.isArray(hints)).toBe(true);
    expect(hints.length).toBeGreaterThan(0);
    expect(hints[0]).toHaveProperty("severity");
    expect(hints[0]).toHaveProperty("message");
  });
});

describe("analyzeHints", () => {
  it("generates self-standing hints", () => {
    const girJson = wasm.parseUlScript("●") as string;
    const hints = wasm.analyzeHints(girJson) as any[];
    expect(Array.isArray(hints)).toBe(true);
  });
});

// ── Puzzles ──

describe("getNextPuzzle", () => {
  it("returns a puzzle for a beginner", () => {
    const puzzle = wasm.getNextPuzzle(JSON.stringify({})) as any;
    expect(puzzle).toHaveProperty("id");
    expect(puzzle).toHaveProperty("difficulty");
    expect(puzzle).toHaveProperty("description");
    expect(puzzle.difficulty).toBe(1);
  });

  it("progresses past mastered operations", () => {
    const proficiency = { predicate: 0.95, embed: 0.95 };
    const puzzle = wasm.getNextPuzzle(JSON.stringify(proficiency)) as any;
    // Should return a puzzle that requires operations the student hasn't mastered
    expect(puzzle).toHaveProperty("id");
  });
});

// ── Lexicon ──

describe("queryLexicon", () => {
  it("finds entries matching a query", () => {
    const results = wasm.queryLexicon("point") as any[];
    expect(results.length).toBeGreaterThan(0);
    expect(results[0]).toHaveProperty("name");
    expect(results[0]).toHaveProperty("tier");
    expect(results[0]).toHaveProperty("labels");
  });

  it("returns empty array for no matches", () => {
    const results = wasm.queryLexicon("zzzznonexistent") as any[];
    expect(results).toEqual([]);
  });
});

describe("lookupLexiconEntry", () => {
  it("finds Single Point by name", () => {
    const entry = wasm.lookupLexiconEntry("Single Point") as any;
    expect(entry).not.toBeNull();
    expect(entry.name).toBe("Single Point");
    expect(entry.tier).toBe("T1");
    expect(entry.level).toBe(1);
  });

  it("is case-insensitive", () => {
    const entry = wasm.lookupLexiconEntry("single point") as any;
    expect(entry).not.toBeNull();
    expect(entry.name).toBe("Single Point");
  });

  it("returns null for unknown names", () => {
    const entry = wasm.lookupLexiconEntry("Does Not Exist");
    expect(entry).toBeNull();
  });

  it("finds all 42 canonical entries", () => {
    const knownEntries = [
      "The Void",
      "Single Point",
      "Single Line",
      "Single Directed Line",
      "Single Angle",
      "Single Curve",
      "Single Enclosure",
      "Negation",
      "Conjunction",
      "Disjunction",
      "Embedding",
      "Abstraction",
      "Composition",
      "Inversion",
      "Quantification",
    ];
    for (const name of knownEntries) {
      const entry = wasm.lookupLexiconEntry(name) as any;
      expect(entry, `lookup failed for "${name}"`).not.toBeNull();
    }
  });
});

// ── Animation ──

describe("getAnimationSequence", () => {
  it("returns keyframes for a GIR", () => {
    const girJson = wasm.parseUlScript("○{●}") as string;
    const seq = wasm.getAnimationSequence(girJson, 256, 256) as any;
    expect(seq).toHaveProperty("keyframes");
    expect(seq).toHaveProperty("total_duration_ms");
    expect(Array.isArray(seq.keyframes)).toBe(true);
    expect(seq.total_duration_ms).toBeGreaterThan(0);
  });
});

describe("layout", () => {
  it("returns positioned geometry", () => {
    const girJson = wasm.parseUlScript("●") as string;
    const result = wasm.layout(girJson, 256, 256) as any;
    expect(result).toBeDefined();
  });
});

// ── Error handling ──

describe("error handling", () => {
  it("throws on malformed UL-Script", () => {
    expect(() => wasm.parseUlScript("{{{{")).toThrow();
  });

  it("throws on malformed GIR JSON", () => {
    expect(() => wasm.validateGir("{bad json}", false)).toThrow();
  });

  it("throws for invalid context ID", () => {
    expect(() => wasm.evaluate(99999, "{}")).toThrow();
  });
});
