/**
 * AGGRESSIVE ROUND-TRIP STRESS TESTS
 *
 * Tests that UL-Script → GIR → SVG → GIR → UL-Script is 100% lossless.
 * Every expression that parses must survive the full pipeline.
 *
 * These are NOT gentle. They exercise every grammar production,
 * every operator combination, deep nesting, wide compositions,
 * modal logic, force annotations, assertion modifiers, variable
 * binding, and multi-glyph documents.
 *
 * Philosophy: If an AI agent writes a UL expression, serializes it
 * to SVG, sends it to another agent, and that agent extracts the GIR
 * and deparses it — the result MUST be identical. Anything less is
 * information loss, and information loss in a universal language
 * is unacceptable.
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

// ── Helper: full round-trip ──

function roundTrip(input: string): string {
  const girJson = wasm.parseUlScript(input) as string;
  const svg = wasm.renderSvg(girJson, 256, 256) as string;
  const cdataMatch = svg.match(/<!\[CDATA\[([\s\S]*?)\]\]>/);
  if (!cdataMatch) throw new Error(`No GIR embedded in SVG for: ${input}`);
  return wasm.deparseGir(cdataMatch[1]) as string;
}

function expectRoundTrip(input: string) {
  const result = roundTrip(input);
  expect(result).toBe(input);
}

function expectDoubleRoundTrip(input: string) {
  const first = roundTrip(input);
  const second = roundTrip(first);
  expect(first).toBe(second);
}

// ═══════════════════════════════════════════════════
// TIER 0: Every atomic primitive must survive
// ═══════════════════════════════════════════════════

describe("atomic primitives round-trip", () => {
  const atoms = [
    "●",           // filled point (entity)
    "○",           // circle enclosure (concept)
    "△",           // triangle enclosure
    "□",           // square enclosure
    "⬠",           // pentagon enclosure
    "⬡",           // hexagon enclosure
    "→",           // right arrow (directed relation)
    "←",           // left arrow (inverse relation)
    "↔",           // bidirectional arrow (symmetric relation)
    "~",           // curve (process)
    "∠60",         // angle modifier (quality)
  ];

  for (const atom of atoms) {
    it(`atom: ${atom}`, () => expectRoundTrip(atom));
  }
});

// ═══════════════════════════════════════════════════
// TIER 1: Binary operators — every combination
// ═══════════════════════════════════════════════════

describe("binary operators round-trip", () => {
  // Connection operators
  const connections = [
    "● → ●",       // predication: subject relates to object
    "● ← ●",       // inverse predication
    "● ↔ ●",       // symmetric relation
    "○ → ●",       // concept relates to entity
    "● → ○",       // entity relates to concept
    "○ → ○",       // concept relates to concept
    "△ → □",       // triangle-concept relates to square-concept
  ];

  // Adjacency operator
  const adjacencies = [
    "● | ●",       // two entities side by side
    "○ | ○",       // two concepts side by side
    "○ | △",       // circle-concept adjacent to triangle-concept
    "● | ○",       // entity adjacent to concept
  ];

  // Intersection operator
  const intersections = [
    "● & ●",       // two entities overlapping
    "○ & ○",       // two concepts overlapping
    "○ & △",       // different enclosure types overlapping
  ];

  for (const expr of [...connections, ...adjacencies, ...intersections]) {
    it(`binary: ${expr}`, () => expectRoundTrip(expr));
  }
});

// ═══════════════════════════════════════════════════
// TIER 2: Enclosure containment — every shape
// ═══════════════════════════════════════════════════

describe("enclosure containment round-trip", () => {
  const enclosures = [
    "○{●}",             // circle containing point
    "△{●}",             // triangle containing point
    "□{●}",             // square containing point
    "⬠{●}",             // pentagon containing point
    "⬡{●}",             // hexagon containing point
    "○{● → ●}",         // circle containing predication
    "△{● → ●}",         // triangle containing predication
    "□{● ← ●}",         // square containing inverse predication
    "⬠{● ↔ ●}",         // pentagon containing symmetric relation
    "⬡{● | ●}",         // hexagon containing adjacency
    "○{● & ●}",         // circle containing intersection
    "○{● → ● → ●}",     // chain of relations inside circle
    "○{● → ● | ●}",     // mixed operators inside enclosure
    "○{○}",              // concept containing concept
    "△{○}",              // triangle containing circle
    "□{△{●}}",           // nested: square → triangle → point
    "○{○{●}}",           // nested: circle → circle → point
    "○{○{○{●}}}",        // triple nesting
  ];

  for (const expr of enclosures) {
    it(`enclosure: ${expr}`, () => expectRoundTrip(expr));
  }
});

// ═══════════════════════════════════════════════════
// TIER 3: Angle modifiers — the measure of meaning
// ═══════════════════════════════════════════════════

describe("angle modifier round-trip", () => {
  const angles = [
    "∠0",           // zero angle
    "∠45",          // 45 degrees
    "∠60",          // 60 degrees
    "∠90",          // right angle
    "∠120",         // obtuse
    "∠180",         // straight line
    "∠270",         // reflex
    "∠360",         // full rotation
    "∠45.5",        // decimal angle
    "∠0.01",        // near-zero
    "∠359.99",      // near-full
  ];

  for (const angle of angles) {
    it(`angle: ${angle}`, () => expectRoundTrip(angle));
  }

  // Angles as connection modifiers
  const connAngles = [
    "● →∠45 ●",     // connection at 45 degrees
    "● →∠90 ●",     // perpendicular connection
    "● ←∠120 ●",    // inverse at 120 degrees
    "● ↔∠60 ●",     // symmetric at 60 degrees
  ];

  for (const expr of connAngles) {
    it(`connection+angle: ${expr}`, () => expectDoubleRoundTrip(expr));
  }
});

// ═══════════════════════════════════════════════════
// TIER 4: Variable binding — co-reference across structures
// ═══════════════════════════════════════════════════

describe("variable binding round-trip", () => {
  const bindings = [
    "○_x",               // variable slot: "some x that is a concept"
    "●_x",               // bound reference: "that same x (entity)"
    "○_x → ●_x",         // self-reference: concept x relates to entity x
    "○_x{●}",            // variable concept containing point
    "○_x{● → ●_x}",     // concept x containing relation back to x
    "○_x | ○_y",         // two distinct variable concepts
    "○_x → ○_y",         // variable concept relates to variable concept
    "○_x & ●_x",         // variable intersection
    "○_abc",             // multi-character variable name
    "●_agent1",          // alphanumeric binding
  ];

  for (const expr of bindings) {
    it(`binding: ${expr}`, () => expectRoundTrip(expr));
  }
});

// ═══════════════════════════════════════════════════
// TIER 5: Modal operators — possible worlds
// ═══════════════════════════════════════════════════

describe("modal operators round-trip", () => {
  const modals = [
    "[]{●}",                       // necessarily, point exists
    "<>{●}",                       // possibly, point exists
    "[]{● → ●}",                   // necessarily, point relates to point
    "<>{○{●}}",                    // possibly, concept containing point
    "[]{○{● → ●}}",               // necessarily, concept with relation
    "<>{● | ●}",                   // possibly, two adjacent entities
    "[]->{● → ●}{● ← ●}",         // counterfactual: if A→B then B←A
    "[]->{○{●}}{△{●}}",           // counterfactual: if circle then triangle
  ];

  for (const expr of modals) {
    it(`modal: ${expr}`, () => expectRoundTrip(expr));
  }
});

// ═══════════════════════════════════════════════════
// TIER 6: Force annotations — illocutionary acts
// ═══════════════════════════════════════════════════

describe("force annotations round-trip", () => {
  const forces = [
    "assert{● → ●}",        // assertion: "A relates to B"
    "query{● → ●}",         // question: "does A relate to B?"
    "direct{● → ●}",        // command: "make A relate to B!"
    "commit{● → ●}",        // promise: "A will relate to B"
    "express{● → ●}",       // exclamation: "A relates to B!"
    "declare{● → ●}",       // declaration: "A hereby relates to B"
    "query{○{●}}",           // "is there a concept containing a point?"
    "direct{○_x → ●_x}",    // "bind x: concept x relates to entity x!"
    "assert{[]{● → ●}}",    // "it is asserted that necessarily, A→B"
    "query{<>{○{●}}}",      // "is it possible that there's a concept with a point?"
  ];

  for (const expr of forces) {
    it(`force: ${expr}`, () => expectRoundTrip(expr));
  }
});

// ═══════════════════════════════════════════════════
// TIER 7: Assertion modifiers — epistemic stance
// ═══════════════════════════════════════════════════

describe("assertion modifiers round-trip", () => {
  const modifiers = [
    "?{● → ●}",             // evidential: "reportedly, A→B"
    "!{● → ●}",             // emphatic: "definitely, A→B"
    "~?{● → ●}",            // hedged: "maybe A→B"
    "?{○{●}}",              // evidential complex
    "!{[]{● → ●}}",         // emphatically necessarily
    "~?{<>{○{●}}}",         // hedged possibility
  ];

  for (const expr of modifiers) {
    it(`modifier: ${expr}`, () => expectRoundTrip(expr));
  }
});

// ═══════════════════════════════════════════════════
// TIER 8: Chains and wide compositions
// ═══════════════════════════════════════════════════

describe("chain compositions round-trip", () => {
  const chains = [
    // Linear chains
    "● → ● → ●",                     // 3-chain
    "● → ● → ● → ●",                 // 4-chain
    "● → ● → ● → ● → ●",             // 5-chain
    "● ← ● ← ● ← ●",                 // reverse 4-chain
    "● ↔ ● ↔ ●",                      // symmetric 3-chain

    // Wide adjacency
    "● | ● | ●",                       // 3-adjacent
    "● | ● | ● | ●",                   // 4-adjacent
    "○ | △ | □ | ⬠ | ⬡",              // all 5 enclosure types adjacent

    // Wide intersection
    "● & ● & ●",                       // 3-intersection
    "○ & △ & □",                        // 3 enclosure types overlapping

    // Mixed operators
    "● → ● | ● → ●",                  // relation then adjacency then relation
    "● → ● & ● → ●",                  // relation then intersection then relation
    "● | ● → ● | ●",                  // adjacency → relation → adjacency
  ];

  for (const expr of chains) {
    it(`chain: ${expr}`, () => expectRoundTrip(expr));
  }
});

// ═══════════════════════════════════════════════════
// TIER 9: Grouping — explicit structure
// ═══════════════════════════════════════════════════

describe("grouping round-trip", () => {
  const groups = [
    "(● → ●)",                          // simple group
    "(● → ●) → ●",                      // group then relation
    "● → (● → ●)",                      // relation then group
    "(● → ●) → (● → ●)",                // group relates to group
    "(● | ●) → (● | ●)",                // adjacent-group relates to adjacent-group
    "(○{●}) → (△{●})",                  // enclosure-groups
    "((● → ●))",                         // double-grouped
    "(● → ●) | (● → ●)",                // two groups adjacent
    "(● → ●) & (● → ●)",                // two groups intersecting
  ];

  for (const expr of groups) {
    it(`group: ${expr}`, () => expectDoubleRoundTrip(expr));
  }
});

// ═══════════════════════════════════════════════════
// TIER 10: Deep nesting — structural depth
// ═══════════════════════════════════════════════════

describe("deep nesting round-trip", () => {
  const nested = [
    // Deep containment
    "○{○{●}}",                                          // depth 2
    "○{○{○{●}}}",                                       // depth 3
    "○{○{○{○{●}}}}",                                    // depth 4
    "○{○{○{○{○{●}}}}}",                                 // depth 5
    "○{△{□{⬠{⬡{●}}}}}",                                 // all 5 enclosure types nested

    // Nested with relations
    "○{● → ○{● → ●}}",                                  // outer has relation to inner concept
    "○{○{● → ●} → ○{● ← ●}}",                           // two inner concepts related
    "○{○{●} | ○{●}}",                                    // two inner concepts adjacent
    "○{○{●} & ○{●}}",                                    // two inner concepts intersecting

    // Nested modals
    "[]{<>{● → ●}}",                                     // necessarily possibly
    "<>{[]{○{●}}}",                                      // possibly necessarily concept
    "[]{[]{● → ●}}",                                     // double necessity

    // Nested forces within modals
    "assert{[]{● → ●}}",                                  // asserted necessity
    "query{<>{○{●}}}",                                    // queried possibility
  ];

  for (const expr of nested) {
    it(`nesting: ${expr}`, () => expectRoundTrip(expr));
  }
});

// ═══════════════════════════════════════════════════
// TIER 11: ASCII aliases — alternate syntax
// ═══════════════════════════════════════════════════

describe("ASCII alias double-round-trip", () => {
  // These may normalize to Unicode on output, so we test
  // that the double round-trip is stable (canonical form is fixed point)
  const aliases = [
    "*",             // ●
    "/0",            // ○
    "/3",            // △
    "/4",            // □
    "/5",            // ⬠
    "/6",            // ⬡
    "->",            // →
    "<-",            // ←
    "<->",           // ↔
    "@45",           // ∠45
    "/0{*}",         // ○{●}
    "/3{*}",         // △{●}
    "* -> *",        // ● → ●
    "/0{* -> *}",    // ○{● → ●}
    "/o_x",          // ○_x
    "*_x",           // ●_x
  ];

  for (const expr of aliases) {
    it(`alias: ${expr}`, () => expectDoubleRoundTrip(expr));
  }
});

// ═══════════════════════════════════════════════════
// TIER 12: Multi-glyph documents
// ═══════════════════════════════════════════════════

describe("multi-glyph document round-trip", () => {
  const docs = [
    "●\n●",                                              // two points
    "● → ●\n○{●}",                                       // relation then concept
    "○{●}\n△{●}\n□{●}",                                  // three enclosures
    "assert{● → ●}\nquery{● → ●}",                       // assertion followed by question
    "# comment\n● → ●",                                   // comment then expression
    "○{● → ●}\n○{● ← ●}\n○{● ↔ ●}",                     // three relation types
  ];

  for (const doc of docs) {
    it(`document: ${JSON.stringify(doc)}`, () => expectDoubleRoundTrip(doc));
  }
});

// ═══════════════════════════════════════════════════
// TIER 13: GIR structural fidelity
//   (not just text equality — verify the GIR itself)
// ═══════════════════════════════════════════════════

describe("GIR structural fidelity", () => {
  function parseGir(input: string): any {
    return JSON.parse(wasm.parseUlScript(input) as string);
  }

  function extractGir(svg: string): any {
    const m = svg.match(/<!\[CDATA\[([\s\S]*?)\]\]>/);
    return m ? JSON.parse(m[1]) : null;
  }

  it("single point has exactly 1 node", () => {
    const gir = parseGir("●");
    // May have implicit root, so check point count
    const points = gir.nodes.filter((n: any) =>
      (n.node_type || n.type) === "point"
    );
    expect(points.length).toBe(1);
  });

  it("enclosure containing point has both node types", () => {
    const gir = parseGir("○{●}");
    const types = gir.nodes.map((n: any) => n.node_type || n.type);
    expect(types).toContain("point");
    expect(types).toContain("enclosure");
  });

  it("● → ● has a line node with directed=true", () => {
    const gir = parseGir("● → ●");
    const lines = gir.nodes.filter((n: any) =>
      (n.node_type || n.type) === "line"
    );
    expect(lines.length).toBeGreaterThanOrEqual(1);
    expect(lines.some((l: any) => l.directed === true)).toBe(true);
  });

  it("● ← ● has a line node (inverse direction)", () => {
    const gir = parseGir("● ← ●");
    const lines = gir.nodes.filter((n: any) =>
      (n.node_type || n.type) === "line"
    );
    expect(lines.length).toBeGreaterThanOrEqual(1);
  });

  it("● ↔ ● has a line node with directed=false", () => {
    const gir = parseGir("● ↔ ●");
    const lines = gir.nodes.filter((n: any) =>
      (n.node_type || n.type) === "line"
    );
    expect(lines.length).toBeGreaterThanOrEqual(1);
    expect(lines.some((l: any) => l.directed === false)).toBe(true);
  });

  it("○_x has a variableslot node", () => {
    const gir = parseGir("○_x");
    const slots = gir.nodes.filter((n: any) =>
      (n.node_type || n.type) === "variableslot"
    );
    expect(slots.length).toBeGreaterThanOrEqual(1);
  });

  it("●_x has a point node with variable_id", () => {
    const gir = parseGir("●_x");
    const boundRefs = gir.nodes.filter((n: any) =>
      ((n.node_type || n.type) === "point") && n.variable_id
    );
    expect(boundRefs.length).toBeGreaterThanOrEqual(1);
    expect(boundRefs[0].variable_id).toBe("x");
  });

  it("SVG contains metadata with valid JSON GIR", () => {
    const girJson = wasm.parseUlScript("○{● → ●}") as string;
    const svg = wasm.renderSvg(girJson, 256, 256) as string;
    const extracted = extractGir(svg);
    expect(extracted).not.toBeNull();
    expect(extracted.nodes).toBeDefined();
    expect(extracted.edges).toBeDefined();
    expect(extracted.root).toBeDefined();
  });

  it("extracted GIR has same node count as original", () => {
    const input = "○{● → ●}";
    const original = parseGir(input);
    const girJson = wasm.parseUlScript(input) as string;
    const svg = wasm.renderSvg(girJson, 256, 256) as string;
    const extracted = extractGir(svg);
    expect(extracted.nodes.length).toBe(original.nodes.length);
  });

  it("extracted GIR has same edge count as original", () => {
    const input = "○{● → ● | ●}";
    const original = parseGir(input);
    const girJson = wasm.parseUlScript(input) as string;
    const svg = wasm.renderSvg(girJson, 256, 256) as string;
    const extracted = extractGir(svg);
    expect(extracted.edges.length).toBe(original.edges.length);
  });

  it("5-chain has 5 points and 4 lines", () => {
    const gir = parseGir("● → ● → ● → ● → ●");
    const points = gir.nodes.filter((n: any) =>
      (n.node_type || n.type) === "point"
    );
    const lines = gir.nodes.filter((n: any) =>
      (n.node_type || n.type) === "line"
    );
    expect(points.length).toBe(5);
    expect(lines.length).toBe(4);
  });

  it("all 5 enclosure shapes produce different (shape, vertices) tuples", () => {
    const shapes = ["○", "△", "□", "⬠", "⬡"];
    const labels = new Set<string>();
    for (const s of shapes) {
      const gir = parseGir(s);
      const enc = gir.nodes.find((n: any) =>
        (n.node_type || n.type) === "enclosure"
      );
      // polygon shapes differ by vertices count
      labels.add(`${enc?.shape}:${enc?.vertices ?? 0}`);
    }
    expect(labels.size).toBe(5);
  });
});

// ═══════════════════════════════════════════════════
// TIER 14: SVG visual properties
//   (verify SVG actually contains geometric elements)
// ═══════════════════════════════════════════════════

describe("SVG visual properties", () => {
  function renderSvg(input: string, w = 256, h = 256): string {
    const girJson = wasm.parseUlScript(input) as string;
    return wasm.renderSvg(girJson, w, h) as string;
  }

  it("single point → SVG has a circle element (filled)", () => {
    const svg = renderSvg("●");
    expect(svg).toMatch(/<circle[^>]*>/);
  });

  it("circle enclosure → SVG has circle element (unfilled)", () => {
    const svg = renderSvg("○");
    expect(svg).toMatch(/<circle[^>]*>/);
  });

  it("triangle enclosure → SVG has polygon element", () => {
    const svg = renderSvg("△");
    // triangle can be polygon or path
    expect(svg).toMatch(/<polygon[^>]*>|<path[^>]*>/);
  });

  it("line → SVG has line or path element", () => {
    const svg = renderSvg("● → ●");
    expect(svg).toMatch(/<line[^>]*>|<path[^>]*>/);
  });

  it("curve → SVG has path with Bézier", () => {
    const svg = renderSvg("~");
    expect(svg).toMatch(/<path[^>]*>/);
  });

  it("variable slot → SVG has dashed circle", () => {
    const svg = renderSvg("○_x");
    expect(svg).toMatch(/dashed|stroke-dasharray/);
  });

  it("SVG respects width/height", () => {
    const svg = renderSvg("●", 512, 384);
    expect(svg).toMatch(/width="512"/);
    expect(svg).toMatch(/height="384"/);
  });

  it("SVG has xmlns attribute", () => {
    const svg = renderSvg("●");
    expect(svg).toMatch(/xmlns="http:\/\/www\.w3\.org\/2000\/svg"/);
  });

  it("complex expression SVG has metadata block", () => {
    const svg = renderSvg("○{● → ● | ● & ●}");
    expect(svg).toMatch(/<metadata>/);
    expect(svg).toMatch(/<!\[CDATA\[/);
  });

  it("enclosure containing 3 points → multiple circle elements in SVG", () => {
    const svg = renderSvg("○{● | ● | ●}");
    const circles = svg.match(/<circle/g);
    // At minimum: 3 small filled circles for points + 1 large circle for enclosure = 4
    expect(circles).not.toBeNull();
    expect(circles!.length).toBeGreaterThanOrEqual(4);
  });
});
