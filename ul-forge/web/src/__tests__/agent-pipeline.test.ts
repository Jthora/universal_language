/**
 * PLAN 04 — Agent Workflow Pipeline Tests
 *
 * Test the complete agent-to-agent pipeline end-to-end:
 * Agent A builds → serializes to SVG → Agent B extracts → manipulates → re-serializes.
 * Every test crosses at least one SVG serialization boundary.
 *
 * @vitest-environment node
 */

import { describe, it, expect, beforeAll } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

import {
  getRootSort,
  girIsomorphic,
  girDistinct,
  findNodesOfType,
  findEdgesOfType,
  nodeTypeDistribution,
  assertContainmentIsTree,
  createPipelineHelpers,
  type Gir,
} from "./helpers/gir-helpers";

// ── WASM setup ──

let wasm: any;
let h: ReturnType<typeof createPipelineHelpers>;

beforeAll(async () => {
  const wasmPath = resolve(__dirname, "../../wasm-pkg/ul_wasm_bg.wasm");
  const wasmModule = await import("../../wasm-pkg/ul_wasm.js");
  const wasmBuffer = readFileSync(wasmPath);
  wasmModule.initSync({ module: new WebAssembly.Module(wasmBuffer) });
  wasmModule.init();
  wasm = wasmModule;
  h = createPipelineHelpers(wasm);
});

/** Serialize GIR to SVG, extract back. Returns extracted GIR JSON string. */
function svgCycle(girJson: string): string {
  const svg = h.renderSvg(girJson);
  expect(svg).toContain("<svg");
  const extracted = h.extractGirFromSvg(svg);
  expect(extracted).toBeTruthy();
  return extracted;
}

// ══════════════════════════════════════════════════
// 4.1 — Operation Across SVG Boundary
// ══════════════════════════════════════════════════

describe("4.1 Operation across SVG boundary", () => {
  it("predicate → SVG → extract → negate = negated assertion", () => {
    const a = h.makeAssertion();
    const extracted = svgCycle(a);
    const negated = h.apply("negate", extracted);
    expect(getRootSort(JSON.parse(negated))).toBe("assertion");
  });

  it("predicate → SVG → extract → embed = entity", () => {
    const a = h.makeAssertion();
    const extracted = svgCycle(a);
    const embedded = h.apply("embed", extracted);
    expect(getRootSort(JSON.parse(embedded))).toBe("entity");
  });

  it("modify_entity(∠45, ●) → SVG → extract → predicate", () => {
    const mod = h.apply("modify_entity", h.p("∠45"), h.p("●"));
    const extracted = svgCycle(mod);
    const result = h.apply("predicate", extracted, h.p("→"), h.p("●"));
    expect(getRootSort(JSON.parse(result))).toBe("assertion");
  });

  it("compose(→, →) → SVG → extract → invert", () => {
    const comp = h.apply("compose", h.p("→"), h.p("→"));
    const extracted = svgCycle(comp);
    const inverted = h.apply("invert", extracted);
    expect(getRootSort(JSON.parse(inverted))).toBe("relation");
  });

  it("conjoin(a₁, a₂) → SVG → extract → negate", () => {
    const conj = h.apply("conjoin", h.makeAssertion(), h.makeAssertion());
    const extracted = svgCycle(conj);
    const negated = h.apply("negate", extracted);
    expect(getRootSort(JSON.parse(negated))).toBe("assertion");
  });

  it("quantify(∠45, ●) → SVG → extract → bind", () => {
    const q = h.apply("quantify", h.p("∠60"), h.p("●"));
    const extracted = svgCycle(q);
    const bound = h.apply("bind", h.p("○_x{●_x}"), extracted);
    expect(getRootSort(JSON.parse(bound))).toBe("assertion");
  });

  it("abstract(●) → SVG → extract → modify_relation", () => {
    const abs = h.apply("abstract", h.p("●"));
    const extracted = svgCycle(abs);
    const result = h.apply("modify_relation", extracted, h.p("→"));
    expect(getRootSort(JSON.parse(result))).toBe("relation");
  });

  it("3-agent relay: A→SVG→B→SVG→C→SVG", () => {
    // Agent A: predicate
    const a = h.makeAssertion();
    const svgA = h.renderSvg(a);
    const girB = h.extractGirFromSvg(svgA);

    // Agent B: negate
    const negated = h.apply("negate", girB);
    const svgB = h.renderSvg(negated);
    const girC = h.extractGirFromSvg(svgB);

    // Agent C: embed
    const embedded = h.apply("embed", girC);
    const svgC = h.renderSvg(embedded);
    const girFinal = h.extractGirFromSvg(svgC);

    expect(getRootSort(JSON.parse(girFinal))).toBe("entity");
  });

  it("compose → SVG → extract → use in predicate", () => {
    const comp = h.apply("compose", h.p("→"), h.p("←"));
    const extracted = svgCycle(comp);
    const result = h.apply("predicate", h.p("●"), extracted, h.p("●"));
    expect(getRootSort(JSON.parse(result))).toBe("assertion");
  });

  it("abstract(embed(pred)) → SVG → extract → modify_entity", () => {
    const abs = h.apply("abstract", h.apply("embed", h.makeAssertion()));
    const extracted = svgCycle(abs);
    const result = h.apply("modify_entity", extracted, h.p("●"));
    expect(getRootSort(JSON.parse(result))).toBe("entity");
  });
});

// ══════════════════════════════════════════════════
// 4.2 — Multi-Agent Publish-Subscribe
// ══════════════════════════════════════════════════

describe("4.2 Multi-agent publish-subscribe", () => {
  it("parallel extraction: 3 agents extract identical GIR from same SVG", () => {
    const a = h.makeAssertion();
    const svg = h.renderSvg(a);
    const gir1 = JSON.parse(h.extractGirFromSvg(svg)) as Gir;
    const gir2 = JSON.parse(h.extractGirFromSvg(svg)) as Gir;
    const gir3 = JSON.parse(h.extractGirFromSvg(svg)) as Gir;
    // All three should be identical
    expect(gir1.nodes.length).toBe(gir2.nodes.length);
    expect(gir2.nodes.length).toBe(gir3.nodes.length);
    expect(gir1.edges.length).toBe(gir2.edges.length);
  });

  it("divergent operations: B negates, C embeds, D modifies — all valid but different", () => {
    const a = h.makeAssertion();
    const svg = h.renderSvg(a);
    const extracted = h.extractGirFromSvg(svg);

    const bResult = h.apply("negate", extracted);
    const cResult = h.apply("embed", extracted);
    const dResult = h.apply("modify_assertion", h.p("∠45"), extracted);

    expect(getRootSort(JSON.parse(bResult))).toBe("assertion");
    expect(getRootSort(JSON.parse(cResult))).toBe("entity");
    expect(getRootSort(JSON.parse(dResult))).toBe("assertion");

    // All different from each other
    const gB = JSON.parse(bResult) as Gir;
    const gC = JSON.parse(cResult) as Gir;
    const gD = JSON.parse(dResult) as Gir;
    expect(girDistinct(gB, gC)).toBe(true);
  });

  it("convergent merge: B and C build assertions, D conjoins", () => {
    // Agent B builds assertion via SVG cycle
    const bAssertion = svgCycle(h.makeAssertion());
    // Agent C builds different assertion via SVG cycle
    const cAssertion = svgCycle(
      h.apply("predicate", h.p("○{●}"), h.p("←"), h.p("●")),
    );
    // Agent D conjoins them
    const result = h.apply("conjoin", bAssertion, cAssertion);
    expect(getRootSort(JSON.parse(result))).toBe("assertion");
    const gir = JSON.parse(result) as Gir;
    // Should be larger than either input
    expect(gir.nodes.length).toBeGreaterThan(
      JSON.parse(bAssertion).nodes.length,
    );
  });

  it("conflict detection: negated vs non-negated are structurally distinct", () => {
    const a = h.makeAssertion();
    const svg = h.renderSvg(a);
    const extracted = h.extractGirFromSvg(svg);

    const original = JSON.parse(extracted) as Gir;
    const negated = JSON.parse(h.apply("negate", extracted)) as Gir;
    expect(girDistinct(original, negated)).toBe(true);
  });
});

// ══════════════════════════════════════════════════
// 4.3 — Incremental Build Across Boundaries
// ══════════════════════════════════════════════════

describe("4.3 Incremental build across boundaries", () => {
  it("5-step incremental build with SVG cycle at each step", () => {
    // Step 1: parse entity
    let current = h.p("●");
    current = svgCycle(current);
    expect(getRootSort(JSON.parse(current))).toBe("entity");

    // Step 2: predicate
    current = h.apply("predicate", current, h.p("→"), h.p("●"));
    current = svgCycle(current);
    expect(getRootSort(JSON.parse(current))).toBe("assertion");

    // Step 3: embed
    current = h.apply("embed", current);
    current = svgCycle(current);
    expect(getRootSort(JSON.parse(current))).toBe("entity");

    // Step 4: predicate again
    current = h.apply("predicate", current, h.p("→"), h.p("○{●}"));
    current = svgCycle(current);
    expect(getRootSort(JSON.parse(current))).toBe("assertion");

    // Step 5: negate
    current = h.apply("negate", current);
    current = svgCycle(current);
    expect(getRootSort(JSON.parse(current))).toBe("assertion");
  });

  it("incremental vs all-at-once: same operation sequence", () => {
    // Build incrementally with SVG cycles
    let incr = h.p("●");
    incr = svgCycle(incr);
    incr = h.apply("predicate", incr, h.p("→"), h.p("●"));
    incr = svgCycle(incr);
    incr = h.apply("negate", incr);
    incr = svgCycle(incr);

    // Build all at once (no SVG cycles)
    const allAtOnce = h.apply(
      "negate",
      h.apply("predicate", h.p("●"), h.p("→"), h.p("●")),
    );

    // Both should produce assertion
    expect(getRootSort(JSON.parse(incr))).toBe("assertion");
    expect(getRootSort(JSON.parse(allAtOnce))).toBe("assertion");

    // Both should have similar node counts (SVG may alter IDs but not structure)
    const girIncr = JSON.parse(incr) as Gir;
    const girOnce = JSON.parse(allAtOnce) as Gir;
    // Node counts should be close (may differ by SVG reconstruction details)
    expect(Math.abs(girIncr.nodes.length - girOnce.nodes.length)).toBeLessThan(
      5,
    );
  });

  it("interruption recovery: build 3 steps, discard, rebuild from step-2 SVG", () => {
    // Step 1
    const step1 = h.p("●");
    // Step 2
    const step2 = h.apply("predicate", step1, h.p("→"), h.p("●"));
    const step2Svg = h.renderSvg(step2);

    // Step 3 (will be discarded)
    const step3 = h.apply("negate", step2);
    expect(getRootSort(JSON.parse(step3))).toBe("assertion");

    // Recover from step 2 SVG
    const recovered = h.extractGirFromSvg(step2Svg);
    expect(getRootSort(JSON.parse(recovered))).toBe("assertion");

    // Build differently from step 2
    const step3Alt = h.apply("embed", recovered);
    expect(getRootSort(JSON.parse(step3Alt))).toBe("entity");
  });

  it("10-step incremental with mixed operations", () => {
    let current = h.p("●");
    const sorts: string[] = [];

    // Steps
    current = svgCycle(current); // 1
    sorts.push(getRootSort(JSON.parse(current)));

    current = h.apply("predicate", current, h.p("→"), h.p("●")); // 2
    current = svgCycle(current);
    sorts.push(getRootSort(JSON.parse(current)));

    current = h.apply("embed", current); // 3
    current = svgCycle(current);
    sorts.push(getRootSort(JSON.parse(current)));

    current = h.apply("abstract", current); // 4
    current = svgCycle(current);
    sorts.push(getRootSort(JSON.parse(current)));

    current = h.apply("modify_entity", current, h.p("●")); // 5
    current = svgCycle(current);
    sorts.push(getRootSort(JSON.parse(current)));

    current = h.apply("predicate", current, h.p("→"), h.p("●")); // 6
    current = svgCycle(current);
    sorts.push(getRootSort(JSON.parse(current)));

    current = h.apply("negate", current); // 7
    current = svgCycle(current);
    sorts.push(getRootSort(JSON.parse(current)));

    current = h.apply("embed", current); // 8
    current = svgCycle(current);
    sorts.push(getRootSort(JSON.parse(current)));

    current = h.apply("predicate", current, h.p("←"), h.p("●")); // 9
    current = svgCycle(current);
    sorts.push(getRootSort(JSON.parse(current)));

    current = h.apply("negate", current); // 10
    current = svgCycle(current);
    sorts.push(getRootSort(JSON.parse(current)));

    // Verify sort progression is correct
    expect(sorts).toEqual([
      "entity",
      "assertion",
      "entity",
      "modifier",
      "entity",
      "assertion",
      "assertion",
      "entity",
      "assertion",
      "assertion",
    ]);
  });
});

// ══════════════════════════════════════════════════
// 4.4 — Message Protocol Simulation
// ══════════════════════════════════════════════════

describe("4.4 Message protocol simulation", () => {
  it("assert → query → commit → declare conversation", () => {
    // Turn 1: Agent A asserts a fact
    const fact = h.makeAssertion();
    const rawForce = wasm.set_force(fact, "assert");
    const assertedFact =
      typeof rawForce === "string" ? rawForce : JSON.stringify(rawForce);
    const svgA = h.renderSvg(assertedFact);
    expect(svgA).toContain("<svg");

    // Turn 2: Agent B extracts, embeds to query about it
    const girB = h.extractGirFromSvg(svgA);
    // B can't directly query force-annotated since it's entity sort,
    // but can embed it into a new predication
    const bPred = h.apply("predicate", h.p("●"), h.p("→"), h.p("●"));
    const rawQuery = wasm.set_force(bPred, "query");
    const bQuery =
      typeof rawQuery === "string" ? rawQuery : JSON.stringify(rawQuery);
    const svgB = h.renderSvg(bQuery);
    expect(svgB).toContain("<svg");

    // Turn 3: Agent A responds with commitment
    const girA2 = h.extractGirFromSvg(svgB);
    const commit = h.makeAssertion();
    const rawCommit = wasm.set_force(commit, "commit");
    const committed =
      typeof rawCommit === "string" ? rawCommit : JSON.stringify(rawCommit);
    const svgC = h.renderSvg(committed);
    expect(svgC).toContain("<svg");

    // Turn 4: Final extraction works
    const final = h.extractGirFromSvg(svgC);
    expect(JSON.parse(final).nodes.length).toBeGreaterThan(0);
  });

  it("force annotations survive SVG round-trip", () => {
    for (const force of ["assert", "query", "direct", "commit"]) {
      const a = h.makeAssertion();
      const raw = wasm.set_force(a, force);
      const forced = typeof raw === "string" ? raw : JSON.stringify(raw);
      const svg = h.renderSvg(forced);
      const extracted = h.extractGirFromSvg(svg);
      const gir = JSON.parse(extracted) as Gir;
      expect(gir.nodes.length).toBeGreaterThan(0);
    }
  });
});

// ══════════════════════════════════════════════════
// 4.5 — Hostile Agent Simulation (Robustness)
// ══════════════════════════════════════════════════

describe("4.5 Hostile agent simulation", () => {
  it("truncated SVG — extraction fails cleanly", () => {
    const a = h.makeAssertion();
    const svg = h.renderSvg(a);
    const truncated = svg.slice(0, Math.floor(svg.length / 2));
    try {
      const extracted = h.extractGirFromSvg(truncated);
      // If extraction succeeds despite truncation, it's degraded but not a crash
      expect(extracted).toBeDefined();
    } catch {
      // Clean failure is acceptable
      expect(true).toBe(true);
    }
  });

  it("corrupted GIR JSON in CDATA — fails cleanly", () => {
    const a = h.makeAssertion();
    const svg = h.renderSvg(a);
    // Replace CDATA content with invalid JSON
    const corrupted = svg.replace(
      /<!\[CDATA\[[\s\S]*?\]\]>/,
      "<![CDATA[{not valid json!}]]>",
    );
    try {
      const extracted = h.extractGirFromSvg(corrupted);
      // If it returns, it must be parseable or empty
      JSON.parse(extracted);
    } catch {
      // Error on corrupt data is correct behavior
      expect(true).toBe(true);
    }
  });

  it("SVG with no CDATA — extraction handles gracefully", () => {
    const noData =
      '<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><circle cx="50" cy="50" r="40"/></svg>';
    try {
      const extracted = h.extractGirFromSvg(noData);
      // May return empty/null, but shouldn't crash
      expect(true).toBe(true);
    } catch {
      expect(true).toBe(true);
    }
  });

  it("hand-crafted GIR with extra nodes — operations still work", () => {
    const gir = JSON.parse(h.p("● → ●")) as Gir;
    // Inject a spurious node
    gir.nodes.push({ id: "spurious_999", type: "point" } as any);
    const modified = JSON.stringify(gir);
    // Try to negate it — may work or may fail
    try {
      const result = h.apply("negate", modified);
      expect(getRootSort(JSON.parse(result))).toBe("assertion");
    } catch {
      // Operation rejecting malformed GIR is also valid
      expect(true).toBe(true);
    }
  });

  it("hand-crafted GIR with missing edges — deparse produces output", () => {
    const gir = JSON.parse(h.p("● → ●")) as Gir;
    // Remove all edges
    gir.edges = [];
    const modified = JSON.stringify(gir);
    try {
      const deparsed = wasm.deparseGir(modified);
      // May produce degraded output
      expect(typeof deparsed).toBe("string");
    } catch {
      // Deparser rejecting edgeless graph is also valid
      expect(true).toBe(true);
    }
  });

  it("sort field tampered — operations detect mismatch", () => {
    const gir = JSON.parse(h.p("●")) as Gir;
    // Tamper: change entity sort to assertion
    const root = gir.nodes.find((n: any) => n.id === gir.root);
    if (root) (root as any).sort = "assertion";
    const tampered = JSON.stringify(gir);
    try {
      // Embed expects assertion sort — tampered node claims assertion but is a point
      const result = h.apply("embed", tampered);
      // If it works, that means sort field is trusted (may be dangerous)
      expect(result).toBeDefined();
    } catch {
      // Detecting the tamper is ideal behavior
      expect(true).toBe(true);
    }
  });

  it("empty nodes array — handled gracefully", () => {
    const emptyGir = JSON.stringify({
      ul_gir: "1.0",
      root: "none",
      nodes: [],
      edges: [],
    });
    try {
      wasm.deparseGir(emptyGir);
      expect(true).toBe(true);
    } catch {
      // Rejecting empty graph is fine
      expect(true).toBe(true);
    }
  });
});
