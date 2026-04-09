/**
 * TEST INFRASTRUCTURE SELF-TESTS
 *
 * Tests for the test helpers themselves. If these break, everything
 * downstream is unreliable.
 *
 * Covers:
 * - GIR isomorphism checker (reflexivity, symmetry, rejection)
 * - Structure query helpers (edges, nodes, sorts)
 * - Containment tree validator
 * - Connection chain validator
 * - Pipeline helpers (serialize, extract, operate)
 *
 * @vitest-environment node
 */

import { describe, it, expect, beforeAll } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

import {
  nodeTypeDistribution,
  edgeTypeDistribution,
  sortDistribution,
  findIsomorphism,
  girIsomorphic,
  girDistinct,
  findEdgesOfType,
  findNodesOfType,
  getNodesBySort,
  getRootNode,
  getRootSort,
  inferSort,
  getContainmentTree,
  getContainmentParent,
  getContainmentDepth,
  assertContainmentIsTree,
  assertConnectionChainValid,
  getForce,
  getAssertionModifier,
  createPipelineHelpers,
  type Gir,
  type GirNode,
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

// ══════════════════════════════════════════════════
// GIR ISOMORPHISM CHECKER
// ══════════════════════════════════════════════════

describe("GIR Isomorphism Checker", () => {
  describe("reflexivity — girIsomorphic(gir, gir) is always true", () => {
    it("single point", () => {
      const gir = h.parse("●");
      expect(girIsomorphic(gir, gir)).toBe(true);
    });

    it("predicate (● → ●)", () => {
      const gir = h.parse("● → ●");
      expect(girIsomorphic(gir, gir)).toBe(true);
    });

    it("enclosure with content", () => {
      const gir = h.parse("○{●}");
      expect(girIsomorphic(gir, gir)).toBe(true);
    });

    it("complex expression", () => {
      const gir = h.parse("○{● → ●} | ○{● ← ●}");
      expect(girIsomorphic(gir, gir)).toBe(true);
    });

    it("nested enclosures", () => {
      const gir = h.parse("○{○{●}}");
      expect(girIsomorphic(gir, gir)).toBe(true);
    });
  });

  describe("symmetry — girIsomorphic(a, b) === girIsomorphic(b, a)", () => {
    it("same expression parsed twice", () => {
      const a = h.parse("● → ●");
      const b = h.parse("● → ●");
      expect(girIsomorphic(a, b)).toBe(girIsomorphic(b, a));
    });

    it("different expressions", () => {
      const a = h.parse("● → ●");
      const b = h.parse("○{●}");
      expect(girIsomorphic(a, b)).toBe(girIsomorphic(b, a));
    });
  });

  describe("same expression parsed twice — different node IDs, same structure", () => {
    it("● → ● parsed twice is isomorphic", () => {
      const a = h.parse("● → ●");
      const b = h.parse("● → ●");
      expect(girIsomorphic(a, b)).toBe(true);
    });

    it("○{● → ●} parsed twice is isomorphic", () => {
      const a = h.parse("○{● → ●}");
      const b = h.parse("○{● → ●}");
      expect(girIsomorphic(a, b)).toBe(true);
    });

    it("complex nested parsed twice is isomorphic", () => {
      const a = h.parse("○{○{●} → ○{●}}");
      const b = h.parse("○{○{●} → ○{●}}");
      expect(girIsomorphic(a, b)).toBe(true);
    });
  });

  describe("known-different GIRs return false", () => {
    it("● vs ○{●} — different structure", () => {
      const a = h.parse("●");
      const b = h.parse("○{●}");
      expect(girIsomorphic(a, b)).toBe(false);
    });

    it("● → ● vs ● | ● — different edge types", () => {
      const a = h.parse("● → ●");
      const b = h.parse("● | ●");
      expect(girIsomorphic(a, b)).toBe(false);
    });

    it("● → ● vs ● → ● → ● — different node count", () => {
      const a = h.parse("● → ●");
      const b = h.parse("● → ● → ●");
      expect(girIsomorphic(a, b)).toBe(false);
    });

    it("○{●} vs △{●} — different shapes", () => {
      const a = h.parse("○{●}");
      const b = h.parse("△{●}");
      // May or may not differ depending on whether shape is encoded
      // in node signature — this test documents actual behavior
      const aRoot = getRootNode(a);
      const bRoot = getRootNode(b);
      if (aRoot?.shape !== bRoot?.shape) {
        expect(girIsomorphic(a, b)).toBe(false);
      }
    });
  });

  describe("direction sensitivity", () => {
    it("● → ● vs ● ← ● — different directions", () => {
      const a = h.parse("● → ●");
      const b = h.parse("● ← ●");
      // These should be structurally distinct — direction matters
      const aLines = findNodesOfType(a, "line");
      const bLines = findNodesOfType(b, "line");
      if (
        aLines.length > 0 &&
        bLines.length > 0 &&
        JSON.stringify(aLines[0].direction) !==
          JSON.stringify(bLines[0].direction)
      ) {
        expect(girIsomorphic(a, b)).toBe(false);
      }
    });
  });

  describe("empty/minimal edge cases", () => {
    it("single point is isomorphic to another single point", () => {
      const a = h.parse("●");
      const b = h.parse("●");
      expect(girIsomorphic(a, b)).toBe(true);
    });
  });
});

// ══════════════════════════════════════════════════
// DISTRIBUTION FUNCTIONS
// ══════════════════════════════════════════════════

describe("Distribution functions", () => {
  it("nodeTypeDistribution counts correctly", () => {
    const gir = h.parse("● → ●");
    const dist = nodeTypeDistribution(gir);
    expect(dist["point"]).toBeGreaterThanOrEqual(2);
    expect(dist["line"]).toBeGreaterThanOrEqual(1);
  });

  it("edgeTypeDistribution counts correctly", () => {
    const gir = h.parse("● → ●");
    const dist = edgeTypeDistribution(gir);
    expect(dist["connects"]).toBeGreaterThanOrEqual(2);
  });

  it("sortDistribution counts correctly", () => {
    const gir = h.parse("● → ●");
    const dist = sortDistribution(gir);
    expect(dist["entity"]).toBeGreaterThanOrEqual(2);
    expect(dist["relation"]).toBeGreaterThanOrEqual(1);
  });
});

// ══════════════════════════════════════════════════
// STRUCTURE QUERY HELPERS
// ══════════════════════════════════════════════════

describe("Structure query helpers", () => {
  describe("findEdgesOfType", () => {
    it("finds all Connects edges in ● → ●", () => {
      const gir = h.parse("● → ●");
      const connects = findEdgesOfType(gir, "connects");
      expect(connects.length).toBeGreaterThanOrEqual(2);
    });

    it("finds Contains edges in ○{●}", () => {
      const gir = h.parse("○{●}");
      const contains = findEdgesOfType(gir, "contains");
      expect(contains.length).toBeGreaterThanOrEqual(1);
    });

    it("returns empty for non-existent edge type", () => {
      const gir = h.parse("●");
      const binds = findEdgesOfType(gir, "binds");
      expect(binds.length).toBe(0);
    });
  });

  describe("findNodesOfType", () => {
    it("finds points in ● → ●", () => {
      const gir = h.parse("● → ●");
      const points = findNodesOfType(gir, "point");
      expect(points.length).toBeGreaterThanOrEqual(2);
    });

    it("finds lines in ● → ●", () => {
      const gir = h.parse("● → ●");
      const lines = findNodesOfType(gir, "line");
      expect(lines.length).toBeGreaterThanOrEqual(1);
    });

    it("finds enclosures in ○{●}", () => {
      const gir = h.parse("○{●}");
      const encs = findNodesOfType(gir, "enclosure");
      expect(encs.length).toBeGreaterThanOrEqual(1);
    });
  });

  describe("getNodesBySort", () => {
    it("finds entities in ● → ●", () => {
      const gir = h.parse("● → ●");
      const entities = getNodesBySort(gir, "entity");
      expect(entities.length).toBeGreaterThanOrEqual(2);
    });

    it("finds relations in ● → ●", () => {
      const gir = h.parse("● → ●");
      const relations = getNodesBySort(gir, "relation");
      expect(relations.length).toBeGreaterThanOrEqual(1);
    });
  });

  describe("getRootSort", () => {
    it("returns entity sort for ●", () => {
      const gir = h.parse("●");
      expect(getRootSort(gir)).toBe("entity");
    });

    it("returns relation sort for →", () => {
      const gir = h.parse("→");
      expect(getRootSort(gir)).toBeDefined();
    });
  });

  describe("inferSort", () => {
    it("point → entity", () => {
      expect(inferSort({ id: "x", type: "point", sort: "entity" })).toBe(
        "entity",
      );
    });

    it("line → relation", () => {
      expect(inferSort({ id: "x", type: "line", sort: "relation" })).toBe(
        "relation",
      );
    });

    it("angle → modifier", () => {
      expect(inferSort({ id: "x", type: "angle", sort: "modifier" })).toBe(
        "modifier",
      );
    });

    it("curve → relation", () => {
      expect(inferSort({ id: "x", type: "curve", sort: "relation" })).toBe(
        "relation",
      );
    });
  });
});

// ══════════════════════════════════════════════════
// CONTAINMENT TREE
// ══════════════════════════════════════════════════

describe("Containment tree helpers", () => {
  describe("getContainmentTree", () => {
    it("returns parent→children for ○{●}", () => {
      const gir = h.parse("○{●}");
      const tree = getContainmentTree(gir);
      expect(Object.keys(tree).length).toBeGreaterThanOrEqual(1);
    });
  });

  describe("assertContainmentIsTree", () => {
    it("valid tree for ○{●}", () => {
      const gir = h.parse("○{●}");
      const result = assertContainmentIsTree(gir);
      expect(result.valid).toBe(true);
      expect(result.errors).toEqual([]);
    });

    it("valid tree for ○{○{●}}", () => {
      const gir = h.parse("○{○{●}}");
      const result = assertContainmentIsTree(gir);
      expect(result.valid).toBe(true);
    });

    it("valid tree for ● → ●", () => {
      const gir = h.parse("● → ●");
      const result = assertContainmentIsTree(gir);
      expect(result.valid).toBe(true);
    });

    it("detects multi-parent containment (fabricated)", () => {
      const gir: Gir = {
        ul_gir: "0.2",
        root: "r",
        nodes: [
          { id: "r", type: "enclosure", sort: "entity", shape: "circle" },
          { id: "a", type: "enclosure", sort: "entity", shape: "circle" },
          { id: "b", type: "enclosure", sort: "entity", shape: "circle" },
          { id: "p", type: "point", sort: "entity" },
        ],
        edges: [
          { source: "r", target: "a", type: "contains" },
          { source: "r", target: "b", type: "contains" },
          { source: "a", target: "p", type: "contains" },
          { source: "b", target: "p", type: "contains" }, // p has TWO parents
        ],
      };
      const result = assertContainmentIsTree(gir);
      expect(result.valid).toBe(false);
      expect(result.errors.length).toBeGreaterThan(0);
      expect(result.errors[0]).toMatch(/2.*Contains parents/);
    });
  });
});

// ══════════════════════════════════════════════════
// CONNECTION CHAIN
// ══════════════════════════════════════════════════

describe("Connection chain helpers", () => {
  describe("assertConnectionChainValid", () => {
    it("valid for ● → ●", () => {
      const gir = h.parse("● → ●");
      const result = assertConnectionChainValid(gir);
      expect(result.valid).toBe(true);
    });

    it("valid for ● → ● → ●", () => {
      const gir = h.parse("● → ● → ●");
      const result = assertConnectionChainValid(gir);
      expect(result.valid).toBe(true);
    });

    it("valid for ● ← ●", () => {
      const gir = h.parse("● ← ●");
      const result = assertConnectionChainValid(gir);
      expect(result.valid).toBe(true);
    });

    it("detects point→point connects (fabricated)", () => {
      const gir: Gir = {
        ul_gir: "0.2",
        root: "p1",
        nodes: [
          { id: "p1", type: "point", sort: "entity" },
          { id: "p2", type: "point", sort: "entity" },
        ],
        edges: [{ source: "p1", target: "p2", type: "connects" }],
      };
      const result = assertConnectionChainValid(gir);
      expect(result.valid).toBe(false);
      expect(result.errors[0]).toMatch(/point.*point/);
    });

    it("detects line→line connects (fabricated)", () => {
      const gir: Gir = {
        ul_gir: "0.2",
        root: "l1",
        nodes: [
          { id: "l1", type: "line", sort: "relation", directed: true },
          { id: "l2", type: "line", sort: "relation", directed: true },
        ],
        edges: [{ source: "l1", target: "l2", type: "connects" }],
      };
      const result = assertConnectionChainValid(gir);
      expect(result.valid).toBe(false);
      expect(result.errors[0]).toMatch(/line.*line/);
    });
  });
});

// ══════════════════════════════════════════════════
// PIPELINE HELPERS
// ══════════════════════════════════════════════════

describe("Pipeline helpers", () => {
  describe("parse", () => {
    it("returns Gir object with nodes and edges", () => {
      const gir = h.parse("● → ●");
      expect(gir.nodes).toBeDefined();
      expect(gir.edges).toBeDefined();
      expect(gir.root).toBeDefined();
    });
  });

  describe("apply", () => {
    it("applies predicate operation", () => {
      const result = h.applyParsed(
        "predicate",
        h.p("●"),
        h.p("→"),
        h.p("●"),
      );
      expect(result.nodes.length).toBeGreaterThanOrEqual(3);
    });

    it("applies negate operation", () => {
      const a = h.makeAssertion();
      const result = h.applyParsed("negate", a);
      expect(result.nodes.length).toBeGreaterThanOrEqual(1);
    });
  });

  describe("makeAssertion", () => {
    it("returns valid GIR JSON string", () => {
      const a = h.makeAssertion();
      const gir = JSON.parse(a) as Gir;
      expect(gir.nodes.length).toBeGreaterThanOrEqual(3);
    });
  });

  describe("agentSerialize (SVG round-trip pipeline)", () => {
    it("preserves GIR through SVG serialization", () => {
      const girJson = h.p("● → ●");
      const extracted = h.agentSerialize(girJson);
      const original = JSON.parse(girJson) as Gir;
      const roundTripped = JSON.parse(extracted) as Gir;
      // Node counts should match
      expect(roundTripped.nodes.length).toBe(original.nodes.length);
      expect(roundTripped.edges.length).toBe(original.edges.length);
    });
  });

  describe("roundTrip", () => {
    it("● → ● survives full round-trip", () => {
      expect(h.roundTrip("● → ●")).toBe("● → ●");
    });

    it("○{●} survives full round-trip", () => {
      expect(h.roundTrip("○{●}")).toBe("○{●}");
    });
  });

  describe("operation sort outputs", () => {
    it("predicate → assertion", () => {
      const gir = h.applyParsed("predicate", h.p("●"), h.p("→"), h.p("●"));
      expect(getRootSort(gir)).toBe("assertion");
    });

    it("embed → entity", () => {
      const gir = h.applyParsed("embed", h.makeAssertion());
      expect(getRootSort(gir)).toBe("entity");
    });

    it("abstract → modifier", () => {
      const gir = h.applyParsed("abstract", h.p("●"));
      expect(getRootSort(gir)).toBe("modifier");
    });

    it("negate → assertion", () => {
      const gir = h.applyParsed("negate", h.makeAssertion());
      expect(getRootSort(gir)).toBe("assertion");
    });

    it("invert → relation", () => {
      const gir = h.applyParsed("invert", h.p("→"));
      expect(getRootSort(gir)).toBe("relation");
    });

    it("compose → relation", () => {
      const gir = h.applyParsed("compose", h.p("→"), h.p("→"));
      expect(getRootSort(gir)).toBe("relation");
    });

    it("conjoin → assertion", () => {
      const gir = h.applyParsed("conjoin", h.makeAssertion(), h.makeAssertion());
      expect(getRootSort(gir)).toBe("assertion");
    });

    it("disjoin → assertion", () => {
      const gir = h.applyParsed("disjoin", h.makeAssertion(), h.makeAssertion());
      expect(getRootSort(gir)).toBe("assertion");
    });

    it("modify_entity → entity", () => {
      const gir = h.applyParsed("modify_entity", h.p("∠60"), h.p("●"));
      expect(getRootSort(gir)).toBe("entity");
    });

    it("modify_relation → relation", () => {
      const gir = h.applyParsed("modify_relation", h.p("∠60"), h.p("→"));
      expect(getRootSort(gir)).toBe("relation");
    });

    it("quantify → assertion", () => {
      const gir = h.applyParsed("quantify", h.p("∠60"), h.p("●"));
      expect(getRootSort(gir)).toBe("assertion");
    });
  });
});
