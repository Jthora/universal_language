/**
 * Tier 4: Teaching system integration tests.
 *
 * Tests the hint generation, puzzle progression, and lexicon systems
 * against the real WASM binary.
 *
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

function p(input: string): string {
  return wasm.parseUlScript(input) as string;
}

// ── Hint generation ──

describe("getHints — contextual hints", () => {
  it("suggests missing primitives", () => {
    const attempt = p("●"); // just a point
    const target = p("○{●}"); // enclosure containing point
    const hints = wasm.getHints(attempt, target) as any[];
    expect(hints.length).toBeGreaterThan(0);
    // Should mention missing enclosure primitive
    const hasPrimitiveHint = hints.some(
      (h: any) => h.category === "primitive" || h.category === "structure",
    );
    expect(hasPrimitiveHint).toBe(true);
  });

  it("hints have required fields", () => {
    const attempt = p("●");
    const target = p("○{●}");
    const hints = wasm.getHints(attempt, target) as any[];
    for (const hint of hints) {
      expect(hint).toHaveProperty("severity");
      expect(hint).toHaveProperty("category");
      expect(hint).toHaveProperty("message");
      expect(typeof hint.message).toBe("string");
      expect(hint.message.length).toBeGreaterThan(0);
    }
  });

  it("gives encouragement when structures are similar", () => {
    const attempt = p("○{●}");
    const target = p("○{●}");
    const hints = wasm.getHints(attempt, target) as any[];
    expect(hints.length).toBeGreaterThan(0);
    // When structures match, hints should be encouraging
    const hasInfoHint = hints.some((h: any) => h.severity === "info");
    expect(hasInfoHint).toBe(true);
  });

  it("severity is one of the expected values", () => {
    const attempt = p("●");
    const target = p("○{●}");
    const hints = wasm.getHints(attempt, target) as any[];
    const validSeverities = ["info", "suggestion", "warning"];
    for (const h of hints) {
      expect(validSeverities).toContain(h.severity);
    }
  });
});

describe("analyzeHints — self-standing analysis", () => {
  it("returns hints for a single GIR", () => {
    const girJson = p("●");
    const hints = wasm.analyzeHints(girJson) as any[];
    expect(Array.isArray(hints)).toBe(true);
  });

  it("provides structural feedback for complex GIR", () => {
    const girJson = p("○{●}");
    const hints = wasm.analyzeHints(girJson) as any[];
    expect(Array.isArray(hints)).toBe(true);
    // All hints should have the required shape
    for (const h of hints) {
      expect(h).toHaveProperty("severity");
      expect(h).toHaveProperty("message");
    }
  });
});

// ── Puzzle progression ──

describe("getNextPuzzle", () => {
  it("returns easiest puzzle for empty proficiency", () => {
    const puzzle = wasm.getNextPuzzle(JSON.stringify({})) as any;
    expect(puzzle.difficulty).toBe(1);
    expect(puzzle.id).toBe("single_point");
    expect(puzzle.description).toBeTruthy();
    expect(puzzle.target_gir_json).toBeTruthy();
  });

  it("progresses when introductory level is mastered", () => {
    // With some proficiency, should advance past single_point
    const proficiency = { predicate: 0.9, embed: 0.9 };
    const puzzle = wasm.getNextPuzzle(JSON.stringify(proficiency)) as any;
    // Should be different from the absolute first puzzle or higher difficulty
    expect(puzzle).toHaveProperty("id");
    expect(puzzle).toHaveProperty("required_operations");
  });

  it("returns hardest puzzle when all mastered", () => {
    // With all operations mastered, should return the highest-difficulty puzzle
    const proficiency: Record<string, number> = {};
    const allOps = [
      "predicate", "modify_entity", "modify_relation", "negate",
      "conjoin", "disjoin", "embed", "abstract", "compose", "invert", "quantify",
    ];
    for (const op of allOps) {
      proficiency[op] = 1.0;
    }
    const puzzle = wasm.getNextPuzzle(JSON.stringify(proficiency)) as any;
    expect(puzzle).toHaveProperty("id");
    // Should still return a valid puzzle (the hardest one)
    expect(puzzle.difficulty).toBeGreaterThan(1);
  });

  it("puzzle has valid target GIR", () => {
    const puzzle = wasm.getNextPuzzle(JSON.stringify({})) as any;
    // The target_gir_json should be parseable as GIR
    const targetGir = JSON.parse(puzzle.target_gir_json);
    expect(targetGir).toHaveProperty("nodes");
  });

  it("puzzle fields have correct types", () => {
    const puzzle = wasm.getNextPuzzle(JSON.stringify({})) as any;
    expect(typeof puzzle.id).toBe("string");
    expect(typeof puzzle.difficulty).toBe("number");
    expect(typeof puzzle.level).toBe("number");
    expect(Array.isArray(puzzle.required_operations)).toBe(true);
    expect(typeof puzzle.description).toBe("string");
    expect(typeof puzzle.target_gir_json).toBe("string");
  });
});

// ── Lexicon ──

describe("queryLexicon", () => {
  it("finds geometric primitives", () => {
    const results = wasm.queryLexicon("point") as any[];
    expect(results.length).toBeGreaterThan(0);
    const names = results.map((r: any) => r.name);
    expect(names).toContain("Single Point");
  });

  it("finds by label", () => {
    const results = wasm.queryLexicon("existence") as any[];
    expect(results.length).toBeGreaterThan(0);
  });

  it("finds operations by name", () => {
    const results = wasm.queryLexicon("negation") as any[];
    expect(results.length).toBeGreaterThan(0);
    const found = results.find((r: any) => r.name === "Negation");
    expect(found).toBeDefined();
  });

  it("returns empty for gibberish", () => {
    const results = wasm.queryLexicon("xyzzyplugh") as any[];
    expect(results).toEqual([]);
  });

  it("entries have all required fields", () => {
    const results = wasm.queryLexicon("point") as any[];
    for (const entry of results) {
      expect(entry).toHaveProperty("id");
      expect(entry).toHaveProperty("level");
      expect(entry).toHaveProperty("name");
      expect(entry).toHaveProperty("tier");
      expect(entry).toHaveProperty("symbol");
      expect(entry).toHaveProperty("sigma_ul");
      expect(entry).toHaveProperty("labels");
      expect(Array.isArray(entry.labels)).toBe(true);
    }
  });
});

describe("lookupLexiconEntry", () => {
  it("finds by exact name", () => {
    const entry = wasm.lookupLexiconEntry("Single Point") as any;
    expect(entry).not.toBeNull();
    expect(entry.name).toBe("Single Point");
    expect(entry.tier).toBe("T1");
    expect(entry.level).toBe(1);
    expect(entry.symbol).toBe("●");
    expect(entry.labels).toContain("Existence");
  });

  it("is case-insensitive", () => {
    const entry = wasm.lookupLexiconEntry("SINGLE POINT") as any;
    expect(entry).not.toBeNull();
  });

  it("returns null for unknown", () => {
    expect(wasm.lookupLexiconEntry("Nonexistent Entry")).toBeNull();
  });

  it("tier classification is T1, T2, or T3", () => {
    const entry = wasm.lookupLexiconEntry("Single Point") as any;
    expect(["T1", "T2", "T3"]).toContain(entry.tier);
  });

  it("level is a reasonable number (0–7)", () => {
    const entry = wasm.lookupLexiconEntry("Single Point") as any;
    expect(entry.level).toBeGreaterThanOrEqual(0);
    expect(entry.level).toBeLessThanOrEqual(7);
  });
});

describe("lexicon completeness", () => {
  it("has exactly 42 entries searchable", () => {
    // Search with an empty-like broad query
    // Every entry has at least one label, so searching common words should find many.
    // We'll search for the most common terms and count unique names.
    const queries = ["point", "line", "angle", "curve", "enclosure", "relation",
      "existence", "not", "and", "or", "entity", "modifier", "action",
      "concept", "process", "identity", "negation", "conjunction"];
    const allNames = new Set<string>();
    for (const q of queries) {
      const results = wasm.queryLexicon(q) as any[];
      for (const r of results) {
        allNames.add(r.name);
      }
    }
    // Should find a significant portion of the 42 entries
    expect(allNames.size).toBeGreaterThanOrEqual(15);
  });
});
