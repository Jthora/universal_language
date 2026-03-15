/**
 * Tier 5: SVG rendering integration tests.
 *
 * Tests that renderSvg and renderGlyphPreview produce valid SVG
 * for various GIR structures.
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

describe("renderSvg", () => {
  it("produces valid SVG for single point", () => {
    const svg = wasm.renderSvg(p("●"), 256, 256) as string;
    expect(svg).toContain("<svg");
    expect(svg).toContain("</svg>");
  });

  it("produces valid SVG for enclosure with point", () => {
    const svg = wasm.renderSvg(p("○{●}"), 256, 256) as string;
    expect(svg).toContain("<svg");
    expect(svg).toContain("</svg>");
  });

  it("produces valid SVG for connected points", () => {
    const svg = wasm.renderSvg(p("●->●"), 256, 256) as string;
    expect(svg).toContain("<svg");
  });

  it("includes xmlns attribute", () => {
    const svg = wasm.renderSvg(p("●"), 256, 256) as string;
    expect(svg).toContain("xmlns");
  });

  it("respects width/height parameters", () => {
    const svg100 = wasm.renderSvg(p("●"), 100, 100) as string;
    const svg500 = wasm.renderSvg(p("●"), 500, 500) as string;
    // Different dimensions should produce different viewBox or size attributes
    expect(svg100).not.toBe(svg500);
  });

  it("renders multiple primitives", () => {
    // This should produce SVG with multiple visual elements
    const svg = wasm.renderSvg(p("○{●->●}"), 256, 256) as string;
    expect(svg).toContain("<svg");
    expect(svg).toContain("</svg>");
  });
});

describe("renderGlyphPreview", () => {
  it("produces compact SVG", () => {
    const svg = wasm.renderGlyphPreview(p("●")) as string;
    expect(svg).toContain("<svg");
    expect(svg).toContain("64");
  });

  it("works for complex structures", () => {
    const svg = wasm.renderGlyphPreview(p("○{●}")) as string;
    expect(svg).toContain("<svg");
  });
});

describe("animation sequences", () => {
  it("generates keyframes for each node", () => {
    const girJson = p("○{●}");
    const seq = wasm.getAnimationSequence(girJson, 256, 256) as any;
    expect(seq.keyframes.length).toBeGreaterThan(0);
    // Each keyframe should have position and timing
    for (const kf of seq.keyframes) {
      expect(kf).toHaveProperty("node_id");
      expect(kf).toHaveProperty("timestamp_ms");
      expect(kf).toHaveProperty("x");
      expect(kf).toHaveProperty("y");
      expect(typeof kf.x).toBe("number");
      expect(typeof kf.y).toBe("number");
    }
  });

  it("keyframes have valid timestamps", () => {
    const girJson = p("○{●}");
    const seq = wasm.getAnimationSequence(girJson, 256, 256) as any;
    for (const kf of seq.keyframes) {
      expect(kf.timestamp_ms).toBeGreaterThanOrEqual(0);
      expect(kf.timestamp_ms).toBeLessThanOrEqual(seq.total_duration_ms);
    }
  });

  it("total duration covers all keyframes", () => {
    const girJson = p("○{●}");
    const seq = wasm.getAnimationSequence(girJson, 256, 256) as any;
    const maxTimestamp = Math.max(...seq.keyframes.map((kf: any) => kf.timestamp_ms));
    expect(seq.total_duration_ms).toBeGreaterThanOrEqual(maxTimestamp);
  });
});

describe("layout", () => {
  it("returns positioned data for a GIR", () => {
    const girJson = p("●");
    const result = wasm.layout(girJson, 256, 256) as any;
    expect(result).toBeDefined();
  });

  it("positions differ for different canvas sizes", () => {
    const girJson = p("●");
    const small = wasm.layout(girJson, 100, 100) as any;
    const large = wasm.layout(girJson, 1000, 1000) as any;
    // Results should differ due to different canvas dimensions
    expect(JSON.stringify(small)).not.toBe(JSON.stringify(large));
  });
});
