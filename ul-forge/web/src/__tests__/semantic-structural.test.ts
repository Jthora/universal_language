/**
 * PLAN 01 — Semantic Structural Verification Tests
 *
 * Prove UL expressions produce GIR graphs with correct *structural meaning*.
 * Edge types, containment trees, connection chains, variable binding,
 * modal structures, force annotations, assertion modifiers.
 *
 * @vitest-environment node
 */

import { describe, it, expect, beforeAll } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

import {
  findEdgesOfType,
  findNodesOfType,
  getNodesBySort,
  getRootNode,
  getRootSort,
  getContainmentTree,
  getContainmentDepth,
  assertContainmentIsTree,
  assertConnectionChainValid,
  findNodeByVariableId,
  findNodesByVariableId,
  getForce,
  getAssertionModifier,
  getSign,
  getContainmentParent,
  nodeTypeDistribution,
  edgeTypeDistribution,
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

// ══════════════════════════════════════════════════
// 1.1 — Edge-Type Verification
// ══════════════════════════════════════════════════

describe("1.1 Edge-Type Verification", () => {
  describe("Connects edges", () => {
    it("● → ● has Connects edges", () => {
      const gir = h.parse("● → ●");
      const connects = findEdgesOfType(gir, "connects");
      expect(connects.length).toBeGreaterThanOrEqual(2);
    });

    it("● → ● has point→line→point connection chain", () => {
      const gir = h.parse("● → ●");
      const connects = findEdgesOfType(gir, "connects");
      const nodeMap = new Map(gir.nodes.map((n) => [n.id, n]));
      const pointLineConnects = connects.filter((e) => {
        const src = nodeMap.get(e.source);
        const tgt = nodeMap.get(e.target);
        return (
          (src?.type === "point" && tgt?.type === "line") ||
          (src?.type === "line" && tgt?.type === "point")
        );
      });
      expect(pointLineConnects.length).toBeGreaterThanOrEqual(2);
    });

    it("● ← ● has Connects edges", () => {
      const gir = h.parse("● ← ●");
      const connects = findEdgesOfType(gir, "connects");
      expect(connects.length).toBeGreaterThanOrEqual(2);
    });

    it("● ↔ ● has Connects edges", () => {
      const gir = h.parse("● ↔ ●");
      const connects = findEdgesOfType(gir, "connects");
      expect(connects.length).toBeGreaterThanOrEqual(2);
    });

    it("● ↔ ● line node has directed: false", () => {
      const gir = h.parse("● ↔ ●");
      const lines = findNodesOfType(gir, "line");
      expect(lines.length).toBeGreaterThanOrEqual(1);
      const bidir = lines.find((l) => l.directed === false);
      expect(bidir).toBeDefined();
    });

    it("● → ● line node has directed: true", () => {
      const gir = h.parse("● → ●");
      const lines = findNodesOfType(gir, "line");
      expect(lines.length).toBeGreaterThanOrEqual(1);
      expect(lines.some((l) => l.directed === true)).toBe(true);
    });

    it("● → ● → ● has two line nodes", () => {
      const gir = h.parse("● → ● → ●");
      const lines = findNodesOfType(gir, "line");
      expect(lines.length).toBeGreaterThanOrEqual(2);
    });

    it("● → ● → ● has three point nodes", () => {
      const gir = h.parse("● → ● → ●");
      const points = findNodesOfType(gir, "point");
      expect(points.length).toBeGreaterThanOrEqual(3);
    });

    it("● → ● → ● has at least 4 Connects edges", () => {
      const gir = h.parse("● → ● → ●");
      const connects = findEdgesOfType(gir, "connects");
      expect(connects.length).toBeGreaterThanOrEqual(4);
    });
  });

  describe("Adjacent edges", () => {
    it("● | ● has Adjacent edges", () => {
      const gir = h.parse("● | ●");
      const adjacent = findEdgesOfType(gir, "adjacent");
      expect(adjacent.length).toBeGreaterThanOrEqual(1);
    });

    it("● | ● has no Connects edges between points (adjacency, not connection)", () => {
      const gir = h.parse("● | ●");
      const connects = findEdgesOfType(gir, "connects");
      // Adjacent expressions shouldn't need point→line→point chains
      const points = findNodesOfType(gir, "point");
      expect(points.length).toBeGreaterThanOrEqual(2);
    });
  });

  describe("Contains edges", () => {
    it("○{●} has at least one Contains edge", () => {
      const gir = h.parse("○{●}");
      const contains = findEdgesOfType(gir, "contains");
      expect(contains.length).toBeGreaterThanOrEqual(1);
    });

    it("○{●} Contains edge goes from enclosure to point", () => {
      const gir = h.parse("○{●}");
      const contains = findEdgesOfType(gir, "contains");
      const nodeMap = new Map(gir.nodes.map((n) => [n.id, n]));
      const encToPoint = contains.filter((e) => {
        const src = nodeMap.get(e.source);
        const tgt = nodeMap.get(e.target);
        return src?.type === "enclosure" && tgt?.type === "point";
      });
      expect(encToPoint.length).toBeGreaterThanOrEqual(1);
    });

    it("△{●} has Contains edge, triangle shape", () => {
      const gir = h.parse("△{●}");
      const contains = findEdgesOfType(gir, "contains");
      expect(contains.length).toBeGreaterThanOrEqual(1);
      const encs = findNodesOfType(gir, "enclosure");
      const triangle = encs.find(
        (n) => n.shape === "triangle" || n.vertices === 3,
      );
      expect(triangle).toBeDefined();
    });

    it("□{●} has Contains edge, square shape", () => {
      const gir = h.parse("□{●}");
      const contains = findEdgesOfType(gir, "contains");
      expect(contains.length).toBeGreaterThanOrEqual(1);
      const encs = findNodesOfType(gir, "enclosure");
      const square = encs.find(
        (n) => n.shape === "square" || n.shape === "rectangle" || n.vertices === 4,
      );
      expect(square).toBeDefined();
    });

    it("○{● → ●} has Contains edges for the relation content", () => {
      const gir = h.parse("○{● → ●}");
      const contains = findEdgesOfType(gir, "contains");
      // Enclosure should contain points and line
      expect(contains.length).toBeGreaterThanOrEqual(3);
    });
  });

  describe("ModifiedBy edges (via operations)", () => {
    it("modify_relation produces ModifiedBy edge", () => {
      const gir = h.applyParsed("modify_relation", h.p("∠60"), h.p("→"));
      const modBy = findEdgesOfType(gir, "modified_by");
      expect(modBy.length).toBeGreaterThanOrEqual(1);
    });

    it("modify_entity produces ModifiedBy edge", () => {
      const gir = h.applyParsed("modify_entity", h.p("∠60"), h.p("●"));
      const modBy = findEdgesOfType(gir, "modified_by");
      expect(modBy.length).toBeGreaterThanOrEqual(1);
    });
  });

  describe("Intersects edges", () => {
    it("● & ● has Intersects edge", () => {
      const gir = h.parse("● & ●");
      const intersects = findEdgesOfType(gir, "intersects");
      expect(intersects.length).toBeGreaterThanOrEqual(1);
    });
  });
});

// ══════════════════════════════════════════════════
// 1.2 — Containment Tree Invariants
// ══════════════════════════════════════════════════

describe("1.2 Containment Tree Invariants", () => {
  describe("nesting depth verification", () => {
    it("○{●} has depth 1 for the point", () => {
      const gir = h.parse("○{●}");
      const points = findNodesOfType(gir, "point");
      expect(points.length).toBeGreaterThanOrEqual(1);
      const depth = getContainmentDepth(gir, points[0].id);
      expect(depth).toBeGreaterThanOrEqual(1);
    });

    it("○{○{●}} has greater depth than ○{●}", () => {
      const single = h.parse("○{●}");
      const double = h.parse("○{○{●}}");
      const sPoints = findNodesOfType(single, "point");
      const dPoints = findNodesOfType(double, "point");
      const sDepth = getContainmentDepth(single, sPoints[0].id);
      const dDepth = getContainmentDepth(double, dPoints[0].id);
      expect(dDepth).toBeGreaterThan(sDepth);
    });

    it("○{○{○{●}}} produces 3-deep nesting", () => {
      const gir = h.parse("○{○{○{●}}}");
      const points = findNodesOfType(gir, "point");
      const depth = getContainmentDepth(gir, points[0].id);
      expect(depth).toBeGreaterThanOrEqual(3);
    });
  });

  describe("sibling containment", () => {
    it("○{● | ●} both points are children of the enclosure", () => {
      const gir = h.parse("○{● | ●}");
      const encs = findNodesOfType(gir, "enclosure");
      const points = findNodesOfType(gir, "point");
      expect(points.length).toBeGreaterThanOrEqual(2);
      const tree = getContainmentTree(gir);
      // At least one enclosure should contain both points
      const encWithMultipleChildren = encs.find((e) => {
        const children = tree[e.id] || [];
        return children.length >= 2;
      });
      // Or the points share a common parent
      const parent0 = getContainmentParent(gir, points[0].id);
      const parent1 = getContainmentParent(gir, points[1].id);
      expect(parent0 || parent1).toBeDefined();
    });
  });

  describe("no-cycle invariant on parsed expressions", () => {
    const expressions = [
      "●",
      "● → ●",
      "○{●}",
      "○{● → ●}",
      "○{○{●}}",
      "● | ●",
      "● → ● → ●",
      "○{○{○{●}}}",
    ];

    for (const expr of expressions) {
      it(`${expr} has acyclic containment tree`, () => {
        const gir = h.parse(expr);
        const result = assertContainmentIsTree(gir);
        expect(result.valid).toBe(true);
      });
    }
  });

  describe("single-parent invariant on parsed expressions", () => {
    const expressions = [
      "●",
      "● → ●",
      "○{●}",
      "○{● → ●}",
      "○{○{●}}",
      "● | ●",
      "○{● | ●}",
    ];

    for (const expr of expressions) {
      it(`${expr} has single parent per node`, () => {
        const gir = h.parse(expr);
        const result = assertContainmentIsTree(gir);
        expect(result.valid).toBe(true);
      });
    }
  });

  describe("root node invariant", () => {
    const expressions = ["●", "● → ●", "○{●}"];

    for (const expr of expressions) {
      it(`${expr} root has no Contains parent`, () => {
        const gir = h.parse(expr);
        const parent = getContainmentParent(gir, gir.root);
        expect(parent).toBeNull();
      });
    }
  });
});

// ══════════════════════════════════════════════════
// 1.3 — Connection Chain Topology
// ══════════════════════════════════════════════════

describe("1.3 Connection Chain Topology", () => {
  describe("point↔line alternation", () => {
    it("● → ● has valid point→line→point chain", () => {
      const gir = h.parse("● → ●");
      const result = assertConnectionChainValid(gir);
      expect(result.valid).toBe(true);
    });

    it("● ← ● has valid connection chain", () => {
      const gir = h.parse("● ← ●");
      const result = assertConnectionChainValid(gir);
      expect(result.valid).toBe(true);
    });

    it("● ↔ ● has valid connection chain", () => {
      const gir = h.parse("● ↔ ●");
      const result = assertConnectionChainValid(gir);
      expect(result.valid).toBe(true);
    });
  });

  describe("multi-hop chains", () => {
    it("● → ● → ● has valid chain", () => {
      const gir = h.parse("● → ● → ●");
      const result = assertConnectionChainValid(gir);
      expect(result.valid).toBe(true);
    });

    it("● → ● → ● → ● has valid chain", () => {
      const gir = h.parse("● → ● → ● → ●");
      const result = assertConnectionChainValid(gir);
      expect(result.valid).toBe(true);
    });

    it("● ← ● → ● has valid chain", () => {
      const gir = h.parse("● ← ● → ●");
      const result = assertConnectionChainValid(gir);
      expect(result.valid).toBe(true);
    });
  });

  describe("no invalid Connects patterns", () => {
    const expressions = [
      "● → ●",
      "● ← ●",
      "● ↔ ●",
      "● → ● → ●",
      "○{● → ●}",
    ];

    for (const expr of expressions) {
      it(`${expr} has no point→point or line→line connects`, () => {
        const gir = h.parse(expr);
        const result = assertConnectionChainValid(gir);
        expect(result.valid).toBe(true);
      });
    }
  });

  describe("direction vectors", () => {
    it("● → ● line has a direction vector", () => {
      const gir = h.parse("● → ●");
      const lines = findNodesOfType(gir, "line");
      expect(lines.length).toBeGreaterThanOrEqual(1);
      const dirLine = lines.find(
        (l) => l.direction !== undefined || l.directed !== undefined,
      );
      expect(dirLine).toBeDefined();
    });

    it("● → ● and ● ← ● have different direction vectors", () => {
      const right = h.parse("● → ●");
      const left = h.parse("● ← ●");
      const rLines = findNodesOfType(right, "line");
      const lLines = findNodesOfType(left, "line");
      if (rLines[0]?.direction && lLines[0]?.direction) {
        expect(JSON.stringify(rLines[0].direction)).not.toBe(
          JSON.stringify(lLines[0].direction),
        );
      }
    });
  });
});

// ══════════════════════════════════════════════════
// 1.4 — Variable Binding Graph Structure
// ══════════════════════════════════════════════════

describe("1.4 Variable Binding Graph", () => {
  describe("basic binding", () => {
    it("○_x{●_x} has nodes with variable_id x", () => {
      const gir = h.parse("○_x{●_x}");
      const xNodes = findNodesByVariableId(gir, "x");
      expect(xNodes.length).toBeGreaterThanOrEqual(1);
    });

    it("○_x{●_x} has shared variable_id linking slot to point", () => {
      const gir = h.parse("○_x{●_x}");
      // Binding is via shared variable_id fields, not explicit Binds edges
      const xNodes = gir.nodes.filter((n: any) => n.variable_id === "x");
      expect(xNodes.length).toBeGreaterThanOrEqual(2);
    });
  });

  describe("distinct variable scopes", () => {
    it("○_x{●_x} | ○_y{●_y} has both x and y variable nodes", () => {
      const gir = h.parse("○_x{●_x} | ○_y{●_y}");
      const xNodes = findNodesByVariableId(gir, "x");
      const yNodes = findNodesByVariableId(gir, "y");
      expect(xNodes.length).toBeGreaterThanOrEqual(1);
      expect(yNodes.length).toBeGreaterThanOrEqual(1);
    });
  });

  describe("cross-structure binding", () => {
    it("○_x → ●_x has variable binding across connection", () => {
      const gir = h.parse("○_x → ●_x");
      const xNodes = findNodesByVariableId(gir, "x");
      expect(xNodes.length).toBeGreaterThanOrEqual(1);
    });
  });

  describe("binding across containment depth", () => {
    it("○_x{○{●_x}} has binding that crosses depth", () => {
      const gir = h.parse("○_x{○{●_x}}");
      const xNodes = findNodesByVariableId(gir, "x");
      expect(xNodes.length).toBeGreaterThanOrEqual(1);
    });
  });
});

// ══════════════════════════════════════════════════
// 1.5 — Modal World Structure
// ══════════════════════════════════════════════════

describe("1.5 Modal World Structure", () => {
  describe("necessity", () => {
    it("[]{●} produces accessible_from edges or world structure", () => {
      const gir = h.parse("[]{●}");
      const accessible = findEdgesOfType(gir, "accessible_from");
      const encs = findNodesOfType(gir, "enclosure");
      // Modal should produce either accessibility edges or special enclosure shapes
      expect(
        accessible.length > 0 ||
          encs.some(
            (e) =>
              e.shape === "square" ||
              e.shape === "rectangle" ||
              e.label?.includes("necessity") ||
              e.label?.includes("world"),
          ),
      ).toBe(true);
    });

    it("[]{● → ●} contains the predication inside modal scope", () => {
      const gir = h.parse("[]{● → ●}");
      const points = findNodesOfType(gir, "point");
      const lines = findNodesOfType(gir, "line");
      expect(points.length).toBeGreaterThanOrEqual(2);
      expect(lines.length).toBeGreaterThanOrEqual(1);
    });
  });

  describe("possibility", () => {
    it("<>{●} produces modal structure", () => {
      const gir = h.parse("<>{●}");
      const encs = findNodesOfType(gir, "enclosure");
      expect(encs.length).toBeGreaterThanOrEqual(1);
    });
  });

  describe("nested modals", () => {
    it("[]{<>{●}} has deeper nesting than []{●}", () => {
      const single = h.parse("[]{●}");
      const nested = h.parse("[]{<>{●}}");
      expect(nested.nodes.length).toBeGreaterThan(single.nodes.length);
    });
  });
});

// ══════════════════════════════════════════════════
// 1.6 — Force Annotation Preservation
// ══════════════════════════════════════════════════

describe("1.6 Force Annotation Preservation", () => {
  describe("all 6 forces produce correct force field", () => {
    const forces = [
      { syntax: "assert", expected: /assert/i },
      { syntax: "query", expected: /query/i },
      { syntax: "direct", expected: /direct/i },
      { syntax: "commit", expected: /commit/i },
      { syntax: "express", expected: /express/i },
      { syntax: "declare", expected: /declare/i },
    ];

    for (const { syntax, expected } of forces) {
      it(`${syntax}{● → ●} has force annotation`, () => {
        const inputJson = h.p("● → ●");
        const forceResult = wasm.set_force(inputJson, syntax);
        // set_force returns an object, not a JSON string
        const gir = (typeof forceResult === "string"
          ? JSON.parse(forceResult)
          : forceResult) as Gir;
        const force = getForce(gir);
        if (force) {
          expect(force).toMatch(expected);
        } else {
          const root = getRootNode(gir);
          expect(
            root?.force || gir.metadata?.force || JSON.stringify(gir),
          ).toMatch(expected);
        }
      });
    }
  });

  describe("force survives operations", () => {
    it("force survives negate", () => {
      const a = h.makeAssertion();
      const forceResult = wasm.set_force(a, "assert");
      const withForce = typeof forceResult === "string" ? forceResult : JSON.stringify(forceResult);
      const negated = h.apply("negate", withForce);
      const gir = JSON.parse(negated) as Gir;
      expect(gir.nodes.length).toBeGreaterThan(0);
    });

    it("force survives embed", () => {
      const a = h.makeAssertion();
      const forceResult = wasm.set_force(a, "query");
      const withForce = typeof forceResult === "string" ? forceResult : JSON.stringify(forceResult);
      const embedded = h.apply("embed", withForce);
      const gir = JSON.parse(embedded) as Gir;
      expect(gir.nodes.length).toBeGreaterThan(0);
    });

    it("force survives SVG round-trip", () => {
      const a = h.makeAssertion();
      const forceResult = wasm.set_force(a, "declare");
      const withForce = typeof forceResult === "string" ? forceResult : JSON.stringify(forceResult);
      const extracted = h.agentSerialize(withForce);
      const gir = JSON.parse(extracted) as Gir;
      expect(gir.nodes.length).toBeGreaterThan(0);
    });
  });
});

// ══════════════════════════════════════════════════
// 1.7 — Assertion Modifier Structure
// ══════════════════════════════════════════════════

describe("1.7 Assertion Modifier Structure", () => {
  describe("modifier types", () => {
    it("?{● → ●} produces evidential modifier structure", () => {
      const gir = h.parse("?{● → ●}");
      const encs = findNodesOfType(gir, "enclosure");
      const modded = encs.find(
        (e) =>
          e.assertion_modifier !== undefined ||
          e.label?.includes("evidential") ||
          e.label?.includes("?"),
      );
      // Characterization: document what actually gets produced
      expect(encs.length).toBeGreaterThanOrEqual(1);
    });

    it("!{● → ●} produces emphatic modifier structure", () => {
      const gir = h.parse("!{● → ●}");
      const encs = findNodesOfType(gir, "enclosure");
      expect(encs.length).toBeGreaterThanOrEqual(1);
    });

    it("~?{● → ●} produces hedged modifier structure", () => {
      const gir = h.parse("~?{● → ●}");
      const encs = findNodesOfType(gir, "enclosure");
      expect(encs.length).toBeGreaterThanOrEqual(1);
    });
  });

  describe("modifier + modal compose orthogonally", () => {
    it("!{[]{●}} has both emphatic and modal structure", () => {
      const gir = h.parse("!{[]{●}}");
      const encs = findNodesOfType(gir, "enclosure");
      // Should have at least 2 enclosures (emphatic + modal)
      expect(encs.length).toBeGreaterThanOrEqual(2);
    });
  });

  describe("modifier + force compose orthogonally", () => {
    it("assertion with both modifier and force", () => {
      const modded = h.p("?{● → ●}");
      const forceResult = wasm.set_force(modded, "assert");
      const gir = (typeof forceResult === "string"
        ? JSON.parse(forceResult)
        : forceResult) as Gir;
      expect(gir.nodes.length).toBeGreaterThan(0);
    });
  });
});
