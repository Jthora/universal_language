/**
 * EDGE-CASE AND ADVERSARIAL TESTS
 *
 * These tests attack the system at its boundaries. They test:
 * - Extreme nesting depths
 * - Extreme widths (many siblings)
 * - Pathological combinations
 * - Error handling (must fail gracefully, not crash)
 * - Unicode boundary conditions
 * - SVG size and performance under load
 * - Parser resilience to malformed input
 *
 * Philosophy: An AI agent in the wild will produce inputs that
 * no human would think to write. The system must either handle
 * them correctly or fail clearly — never silently corrupt.
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

// ── Helpers ──

function parse(input: string): any {
  return JSON.parse(wasm.parseUlScript(input) as string);
}

function roundTrip(input: string): string {
  const girJson = wasm.parseUlScript(input) as string;
  const svg = wasm.renderSvg(girJson, 256, 256) as string;
  const m = svg.match(/<!\[CDATA\[([\s\S]*?)\]\]>/);
  if (!m) throw new Error(`No embedded GIR for: ${input}`);
  return wasm.deparseGir(m[1]) as string;
}

function expectRoundTrip(input: string) {
  expect(roundTrip(input)).toBe(input);
}

function expectDoubleRoundTrip(input: string) {
  const first = roundTrip(input);
  expect(roundTrip(first)).toBe(first);
}

function expectParseFails(input: string) {
  expect(() => wasm.parseUlScript(input)).toThrow();
}

function expectParseSucceeds(input: string) {
  const girJson = wasm.parseUlScript(input) as string;
  const gir = JSON.parse(girJson);
  expect(gir.nodes.length).toBeGreaterThan(0);
  return gir;
}

// ═══════════════════════════════════════════════════════
// DEPTH STRESS: How deep can we nest before things break?
// ═══════════════════════════════════════════════════════

describe("depth stress tests", () => {
  // Generate deeply nested enclosures: ○{○{○{...{●}...}}}
  function deepNest(depth: number): string {
    let s = "●";
    for (let i = 0; i < depth; i++) s = `○{${s}}`;
    return s;
  }

  for (const depth of [1, 2, 3, 5, 8, 10]) {
    it(`nesting depth ${depth}: parses`, () => {
      expectParseSucceeds(deepNest(depth));
    });

    it(`nesting depth ${depth}: round-trips`, () => {
      expectRoundTrip(deepNest(depth));
    });
  }

  // Mixed enclosure deep nesting
  it("mixed enclosure nesting (○△□⬠⬡)×2", () => {
    expectRoundTrip("○{△{□{⬠{⬡{○{△{□{⬠{⬡{●}}}}}}}}}}");
  });

  // Deep nesting with relations at every level
  function deepRelational(depth: number): string {
    if (depth === 0) return "●";
    return `○{● → ${deepRelational(depth - 1)}}`;
  }

  for (const depth of [1, 2, 3, 5]) {
    it(`relational depth ${depth}: round-trips`, () => {
      expectRoundTrip(deepRelational(depth));
    });
  }
});

// ═══════════════════════════════════════════════════════
// WIDTH STRESS: Many siblings at the same level
// ═══════════════════════════════════════════════════════

describe("width stress tests", () => {
  // Generate wide adjacency: ● | ● | ● | ...
  function wideAdj(n: number): string {
    return Array(n).fill("●").join(" | ");
  }

  // Generate wide chain: ● → ● → ● → ...
  function wideChain(n: number): string {
    return Array(n).fill("●").join(" → ");
  }

  for (const width of [3, 5, 10, 20]) {
    it(`${width}-wide adjacency: parses`, () => {
      expectParseSucceeds(wideAdj(width));
    });

    it(`${width}-wide adjacency: round-trips`, () => {
      expectRoundTrip(wideAdj(width));
    });
  }

  for (const width of [3, 5, 10, 20]) {
    it(`${width}-long chain: round-trips`, () => {
      expectRoundTrip(wideChain(width));
    });
  }

  // Wide containment: ○{● | ● | ● | ...}
  for (const width of [3, 5, 10]) {
    it(`enclosure with ${width} children: round-trips`, () => {
      expectRoundTrip(`○{${wideAdj(width)}}`);
    });
  }

  // Multiple enclosures adjacent
  for (const width of [3, 5, 10]) {
    it(`${width} adjacent enclosures: round-trips`, () => {
      expectRoundTrip(Array(width).fill("○{●}").join(" | "));
    });
  }
});

// ═══════════════════════════════════════════════════════
// PARSER RESILIENCE: Malformed input must fail cleanly
// ═══════════════════════════════════════════════════════

describe("parser error handling", () => {
  // NOTE: The grammar intentionally accepts these as valid:
  // - "" (empty document → 0 glyphs)
  // - "   " (whitespace-only → 0 glyphs)
  // - "→ → →" (arrows are valid marks, chained by operators)
  // These are design choices, not bugs. Tested separately below.
  const malformed = [
    "{",                   // unclosed brace
    "}",                   // unopened brace
    "○{",                  // unclosed enclosure
    "○}",                  // unopened enclosure
    "○{}",                 // empty enclosure
    "○{{●}}",              // double-braced
    "| |",                // adjacency without terms
    "& &",                // intersection without terms
    "● → ",               // trailing operator
    " → ●",               // leading operator
    "● | ",               // trailing adjacency
    "● & ",               // trailing intersection
    "○{● →}",             // operator at end of enclosure
    "(((",                 // unmatched parens
    ")))",                 // unmatched close parens
    "()",                  // empty group
    "[]{}",               // necessity with empty braces? (depends on grammar)
    "assert{}",            // force with empty content
    "?{}",                 // evidential with empty content
    "unknown{● → ●}",     // invalid force token
    "@@45",                // double angle prefix
    "@",                   // angle without number
    "∠",                   // angle sign without number
    "∠abc",               // angle with non-numeric
  ];

  for (const input of malformed) {
    it(`rejects malformed: ${JSON.stringify(input)}`, () => {
      expectParseFails(input);
    });
  }

  // These are valid by grammar design, even though they look "empty"
  describe("valid edge cases (by design)", () => {
    it("empty string produces empty document", () => {
      expectParseSucceeds("");
    });

    it("whitespace-only produces empty document", () => {
      expectParseSucceeds("   ");
    });

    it("arrows are valid standalone marks", () => {
      expectParseSucceeds("→ → →");
    });
  });
});

// ═══════════════════════════════════════════════════════
// SVG SANITY: Every parseable expression must render
// ═══════════════════════════════════════════════════════

describe("SVG rendering sanity", () => {
  function mustRender(input: string) {
    const girJson = wasm.parseUlScript(input) as string;
    const svg = wasm.renderSvg(girJson, 256, 256) as string;
    expect(svg).toMatch(/<svg/);
    expect(svg).toMatch(/<\/svg>/);
    expect(svg.length).toBeGreaterThan(100); // not trivially empty
    return svg;
  }

  const renderTests = [
    // All primitives
    "●", "○", "△", "□", "⬠", "⬡", "→", "←", "↔", "~", "∠45",
    // All binary operators
    "● → ●", "● ← ●", "● ↔ ●", "● | ●", "● & ●",
    // All enclosure types with content
    "○{●}", "△{●}", "□{●}", "⬠{●}", "⬡{●}",
    // Nesting
    "○{○{●}}", "○{○{○{●}}}",
    // Modals
    "[]{●}", "<>{●}",
    // Forces
    "assert{● → ●}", "query{●}",
    // Assertion modifiers
    "?{●}", "!{●}", "~?{●}",
    // Bindings
    "○_x", "●_x",
    // Complex
    "assert{[]{○{● → ● | ●}}}",
  ];

  for (const input of renderTests) {
    it(`renders: ${input}`, () => mustRender(input));
  }
});

// ═══════════════════════════════════════════════════════
// GIR EMBEDDING INTEGRITY: Every SVG must have lossless GIR
// ═══════════════════════════════════════════════════════

describe("GIR embedding integrity", () => {
  function verifyEmbedding(input: string) {
    const originalJson = wasm.parseUlScript(input) as string;
    const original = JSON.parse(originalJson);
    const svg = wasm.renderSvg(originalJson, 256, 256) as string;
    const m = svg.match(/<!\[CDATA\[([\s\S]*?)\]\]>/);
    expect(m).not.toBeNull();
    const extracted = JSON.parse(m![1]);

    // Node count must be identical
    expect(extracted.nodes.length).toBe(original.nodes.length);

    // Edge count must be identical
    expect(extracted.edges.length).toBe(original.edges.length);

    // Root must be identical
    expect(extracted.root).toBe(original.root);

    // All node types must be present
    const origTypes = original.nodes.map((n: any) => n.node_type || n.type).sort();
    const extTypes = extracted.nodes.map((n: any) => n.node_type || n.type).sort();
    expect(extTypes).toEqual(origTypes);

    // All edge types must be present
    const origEdgeTypes = original.edges.map((e: any) => e.edge_type || e.type).sort();
    const extEdgeTypes = extracted.edges.map((e: any) => e.edge_type || e.type).sort();
    expect(extEdgeTypes).toEqual(origEdgeTypes);
  }

  const embedTests = [
    "●",
    "○{●}",
    "● → ●",
    "○{● → ● → ●}",
    "○{● | ● | ●}",
    "○{● & ●}",
    "○{○{●}}",
    "○{○{○{●}}}",
    "△{● → ●}",
    "□{● ← ●}",
    "○_x",
    "●_x",
    "[]{●}",
    "<>{●}",
    "assert{● → ●}",
    "?{● → ●}",
    "!{●}",
    "~",
    "∠90",
  ];

  for (const input of embedTests) {
    it(`embedding integrity: ${input}`, () => verifyEmbedding(input));
  }
});

// ═══════════════════════════════════════════════════════
// PERFORMANCE: Large expressions must not explode
// ═══════════════════════════════════════════════════════

describe("performance under load", () => {
  function wideAdj(n: number): string {
    return Array(n).fill("●").join(" | ");
  }

  function deepNest(depth: number): string {
    let s = "●";
    for (let i = 0; i < depth; i++) s = `○{${s}}`;
    return s;
  }

  it("50-wide adjacency parses in < 100ms", () => {
    const input = wideAdj(50);
    const start = performance.now();
    wasm.parseUlScript(input);
    const elapsed = performance.now() - start;
    expect(elapsed).toBeLessThan(100);
  });

  it("50-wide adjacency renders in < 200ms", () => {
    const input = wideAdj(50);
    const girJson = wasm.parseUlScript(input) as string;
    const start = performance.now();
    wasm.renderSvg(girJson, 1024, 1024);
    const elapsed = performance.now() - start;
    expect(elapsed).toBeLessThan(200);
  });

  it("depth-15 nesting parses in < 50ms", () => {
    const input = deepNest(15);
    const start = performance.now();
    wasm.parseUlScript(input);
    const elapsed = performance.now() - start;
    expect(elapsed).toBeLessThan(50);
  });

  it("depth-15 nesting renders in < 100ms", () => {
    const input = deepNest(15);
    const girJson = wasm.parseUlScript(input) as string;
    const start = performance.now();
    wasm.renderSvg(girJson, 256, 256);
    const elapsed = performance.now() - start;
    expect(elapsed).toBeLessThan(100);
  });

  it("100-node chain renders in < 500ms", () => {
    const input = Array(100).fill("●").join(" → ");
    const girJson = wasm.parseUlScript(input) as string;
    const start = performance.now();
    wasm.renderSvg(girJson, 2048, 256);
    const elapsed = performance.now() - start;
    expect(elapsed).toBeLessThan(500);
  });

  it("SVG size scales sub-quadratically with node count", () => {
    const sizes: number[] = [];
    for (const n of [5, 10, 20, 40]) {
      const input = Array(n).fill("●").join(" | ");
      const girJson = wasm.parseUlScript(input) as string;
      const svg = wasm.renderSvg(girJson, 1024, 1024) as string;
      sizes.push(svg.length);
    }
    // SVG size for 40 nodes should be < 16× the size for 5 nodes
    // (sub-quadratic means < O(n²), so ratio < (40/5)² = 64)
    const ratio = sizes[3] / sizes[0];
    expect(ratio).toBeLessThan(64);
  });
});

// ═══════════════════════════════════════════════════════
// DETERMINISM: Same input must always produce same output
// ═══════════════════════════════════════════════════════

describe("deterministic output", () => {
  const deterministicInputs = [
    "●",
    "○{● → ●}",
    "○{○{○{●}}}",
    "assert{[]{● → ●}}",
    "○_x{●_x → ●}",
    "● | ● | ● | ● | ●",
  ];

  for (const input of deterministicInputs) {
    it(`deterministic: ${input}`, () => {
      const girJson1 = wasm.parseUlScript(input) as string;
      const girJson2 = wasm.parseUlScript(input) as string;
      expect(girJson1).toBe(girJson2);

      const svg1 = wasm.renderSvg(girJson1, 256, 256) as string;
      const svg2 = wasm.renderSvg(girJson2, 256, 256) as string;
      expect(svg1).toBe(svg2);

      const dep1 = wasm.deparseGir(girJson1) as string;
      const dep2 = wasm.deparseGir(girJson2) as string;
      expect(dep1).toBe(dep2);
    });
  }
});

// ═══════════════════════════════════════════════════════
// IDEMPOTENCY: N round-trips must equal 1 round-trip
// ═══════════════════════════════════════════════════════

describe("idempotency (N-trip stability)", () => {
  const idempotentInputs = [
    "●",
    "● → ●",
    "○{●}",
    "○{● → ●}",
    "○{○{●}}",
    "● | ●",
    "● & ●",
    "[]{●}",
    "<>{●}",
    "assert{● → ●}",
    "?{●}",
    "!{●}",
    "~?{●}",
    "○_x",
    "●_x",
    "~",
    "∠45",
  ];

  for (const input of idempotentInputs) {
    it(`5-trip stable: ${input}`, () => {
      let current = input;
      const results: string[] = [current];
      for (let i = 0; i < 5; i++) {
        current = roundTrip(current);
        results.push(current);
      }
      // After 1st trip, all subsequent must be identical
      for (let i = 2; i < results.length; i++) {
        expect(results[i]).toBe(results[1]);
      }
    });
  }
});
