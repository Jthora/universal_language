/**
 * Performance benchmarks for game-loop viability.
 *
 * Validates that key WASM operations complete within frame-budget constraints.
 * Target: all hot-path operations < 5ms individually, < 16ms combined per frame.
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

function measure(fn: () => void, iterations = 100): number {
  // Warm up
  for (let i = 0; i < 10; i++) fn();
  // Measure
  const start = performance.now();
  for (let i = 0; i < iterations; i++) fn();
  return (performance.now() - start) / iterations;
}

describe("parse performance", () => {
  it("parses simple glyph in < 1ms", () => {
    const avg = measure(() => wasm.parseUlScript("●"));
    expect(avg).toBeLessThan(1);
  });

  it("parses complex glyph in < 2ms", () => {
    const avg = measure(() => wasm.parseUlScript("○{●->●}"));
    expect(avg).toBeLessThan(2);
  });
});

describe("validate performance", () => {
  it("validates in < 1ms", () => {
    const girJson = p("○{●}");
    const avg = measure(() => wasm.validateGir(girJson, false));
    expect(avg).toBeLessThan(1);
  });
});

describe("render performance", () => {
  it("renders SVG in < 2ms", () => {
    const girJson = p("○{●}");
    const avg = measure(() => wasm.renderSvg(girJson, 256, 256));
    expect(avg).toBeLessThan(2);
  });

  it("renders preview in < 1ms", () => {
    const girJson = p("○{●}");
    const avg = measure(() => wasm.renderGlyphPreview(girJson));
    expect(avg).toBeLessThan(1);
  });
});

describe("layout performance", () => {
  it("computes layout in < 1ms", () => {
    const girJson = p("○{●->●}");
    const avg = measure(() => wasm.layout(girJson, 512, 512));
    expect(avg).toBeLessThan(1);
  });
});

describe("evaluation performance", () => {
  it("evaluates with empty rules in < 1ms", () => {
    const ctxId = wasm.createContext("{}");
    const girJson = p("○{●}");
    const avg = measure(() => wasm.evaluate(ctxId, girJson));
    expect(avg).toBeLessThan(1);
  });
});

describe("animation performance", () => {
  it("generates keyframes in < 2ms", () => {
    const girJson = p("○{●->●}");
    const avg = measure(() => wasm.getAnimationSequence(girJson, 512, 512));
    expect(avg).toBeLessThan(2);
  });
});

describe("algebra performance", () => {
  it("applies operation in < 1ms", () => {
    const entity = p("●");
    const relation = p("->");
    const operands = JSON.stringify([entity, relation, entity]);
    const avg = measure(() => wasm.applyOperation("predicate", operands));
    expect(avg).toBeLessThan(1);
  });
});

describe("lexicon performance", () => {
  it("queries lexicon in < 1ms", () => {
    const avg = measure(() => wasm.queryLexicon("point"));
    expect(avg).toBeLessThan(1);
  });

  it("lookups entry in < 0.5ms", () => {
    const avg = measure(() => wasm.lookupLexiconEntry("Single Point"));
    expect(avg).toBeLessThan(0.5);
  });
});

describe("combined game-frame budget", () => {
  it("parse + layout + animate fits in < 5ms", () => {
    const avg = measure(() => {
      const girJson = wasm.parseUlScript("○{●->●}") as string;
      wasm.layout(girJson, 512, 512);
      wasm.getAnimationSequence(girJson, 512, 512);
    });
    expect(avg).toBeLessThan(5);
  });

  it("parse + validate + render fits in < 5ms", () => {
    const avg = measure(() => {
      const girJson = wasm.parseUlScript("○{●}") as string;
      wasm.validateGir(girJson, false);
      wasm.renderSvg(girJson, 256, 256);
    });
    expect(avg).toBeLessThan(5);
  });
});

describe("structure analysis performance", () => {
  it("analyzes structure in < 1ms", () => {
    const girJson = p("○{●->●}");
    const avg = measure(() => wasm.analyzeStructure(girJson));
    expect(avg).toBeLessThan(1);
  });
});
