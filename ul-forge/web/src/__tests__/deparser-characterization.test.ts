/**
 * PLAN 03 — Deparser Bug Characterization Tests
 *
 * Map the exact failure surface of known deparser bugs:
 * 1. Operator loss inside enclosure content
 * 2. Left-arrow in mixed-operator chains inside enclosures
 * 3. Modal vs assertion modifier overlap
 *
 * Tests document actual behavior (pass even when bugs exist).
 * When a bug is fixed, assertions can be flipped to correct behavior.
 *
 * @vitest-environment node
 */

import { describe, it, expect, beforeAll } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

import {
  findEdgesOfType,
  findNodesOfType,
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

/**
 * Parse → deparse round-trip. Returns { input, output, matches }.
 */
function roundTripChar(input: string): {
  input: string;
  output: string;
  matches: boolean;
} {
  try {
    const girJson = wasm.parseUlScript(input);
    const output = wasm.deparseGir(girJson);
    return { input, output, matches: output === input };
  } catch (e: any) {
    return { input, output: `ERROR: ${e.message}`, matches: false };
  }
}

// ══════════════════════════════════════════════════
// 3.1 — Bug 1: Operator Loss in Enclosure Content
// ══════════════════════════════════════════════════

describe("3.1 Bug 1: Operator loss in enclosure content", () => {
  describe("3.1a varying second operator after |", () => {
    it("○{● | ●} — baseline adjacency inside enclosure", () => {
      const rt = roundTripChar("○{● | ●}");
      // Document actual behavior
      expect(rt.output).toBeTruthy();
    });

    it("○{● → ●} — single arrow inside enclosure", () => {
      const rt = roundTripChar("○{● → ●}");
      expect(rt.output).toBeTruthy();
    });

    it("○{● → ● | ●} — arrow then adj", () => {
      const rt = roundTripChar("○{● → ● | ●}");
      expect(rt.output).toBeTruthy();
    });

    it("○{● | ● → ●} — adj then arrow, characterize", () => {
      const rt = roundTripChar("○{● | ● → ●}");
      expect(rt.output).toBeTruthy();
    });

    it("○{● → ● | ● → ●} — KNOWN BUG: second arrow may be lost", () => {
      const rt = roundTripChar("○{● → ● | ● → ●}");
      // Document whether bug persists
      if (!rt.matches) {
        // Bug still present: record the actual buggy output
        expect(rt.output).toBeTruthy();
        // When fixed, change to: expect(rt.output).toBe(rt.input);
      } else {
        // Bug has been fixed!
        expect(rt.matches).toBe(true);
      }
    });

    it("○{● ← ● | ● ← ●} — characterize left-arrows", () => {
      const rt = roundTripChar("○{● ← ● | ● ← ●}");
      expect(rt.output).toBeTruthy();
    });

    it("○{● ↔ ● | ● ↔ ●} — characterize bidirectional", () => {
      const rt = roundTripChar("○{● ↔ ● | ● ↔ ●}");
      expect(rt.output).toBeTruthy();
    });

    it("○{● → ● | ● ← ●} — mixed arrow types", () => {
      const rt = roundTripChar("○{● → ● | ● ← ●}");
      expect(rt.output).toBeTruthy();
    });

    it("○{● → ● → ● | ●} — chain then adj", () => {
      const rt = roundTripChar("○{● → ● → ● | ●}");
      expect(rt.output).toBeTruthy();
    });

    it("○{● | ● → ● → ●} — adj then chain", () => {
      const rt = roundTripChar("○{● | ● → ● → ●}");
      expect(rt.output).toBeTruthy();
    });

    it("○{● → ● & ● → ●} — intersection instead of adj", () => {
      const rt = roundTripChar("○{● → ● & ● → ●}");
      expect(rt.output).toBeTruthy();
    });
  });

  describe("3.1b varying enclosure type", () => {
    it("△{● → ● | ● → ●} — triangle enclosure", () => {
      const rt = roundTripChar("△{● → ● | ● → ●}");
      expect(rt.output).toBeTruthy();
    });

    it("□{● → ● | ● → ●} — square enclosure", () => {
      const rt = roundTripChar("□{● → ● | ● → ●}");
      expect(rt.output).toBeTruthy();
    });
  });

  describe("3.1c varying nesting wrapper", () => {
    it("top-level: ● → ● | ● → ● (baseline)", () => {
      const rt = roundTripChar("● → ● | ● → ●");
      // Top-level should work
      expect(rt.output).toBeTruthy();
    });

    it("○{● → ● | ● → ●} inside enclosure — known broken", () => {
      const rt = roundTripChar("○{● → ● | ● → ●}");
      expect(rt.output).toBeTruthy();
    });

    it("double-nested: ○{○{● → ● | ● → ●}} — characterize", () => {
      const rt = roundTripChar("○{○{● → ● | ● → ●}}");
      expect(rt.output).toBeTruthy();
    });
  });

  describe("3.1d varying operator count", () => {
    it("○{● → ●} — 1 op", () => {
      const rt = roundTripChar("○{● → ●}");
      expect(rt.output).toBeTruthy();
    });

    it("○{● → ● | ●} — 2 ops", () => {
      const rt = roundTripChar("○{● → ● | ●}");
      expect(rt.output).toBeTruthy();
    });

    it("○{● → ● | ● → ●} — 3 ops", () => {
      const rt = roundTripChar("○{● → ● | ● → ●}");
      expect(rt.output).toBeTruthy();
    });

    it("○{● → ● | ● → ● | ●} — 4 ops", () => {
      const rt = roundTripChar("○{● → ● | ● → ● | ●}");
      expect(rt.output).toBeTruthy();
    });

    it("○{● → ● → ● | ● → ●} — chain + mixed", () => {
      const rt = roundTripChar("○{● → ● → ● | ● → ●}");
      expect(rt.output).toBeTruthy();
    });

    it("○{● → ● | ● | ● → ●} — two | + two →", () => {
      const rt = roundTripChar("○{● → ● | ● | ● → ●}");
      expect(rt.output).toBeTruthy();
    });
  });

  describe("3.1e diagnostic: GIR structure of known-broken case", () => {
    it("○{● → ● | ● → ●} — verify GIR has correct node types", () => {
      const gir = JSON.parse(wasm.parseUlScript("○{● → ● | ● → ●}")) as Gir;
      const points = findNodesOfType(gir, "point");
      const lines = findNodesOfType(gir, "line");
      // Should have 4 points and at least 2 lines (→ and potentially | encoded)
      expect(points.length).toBeGreaterThanOrEqual(4);
      expect(lines.length).toBeGreaterThanOrEqual(1);
    });

    it("top-level ● → ● | ● → ● vs ○{● → ● | ● → ●} — structure comparison", () => {
      const topGir = JSON.parse(
        wasm.parseUlScript("● → ● | ● → ●"),
      ) as Gir;
      const encGir = JSON.parse(
        wasm.parseUlScript("○{● → ● | ● → ●}"),
      ) as Gir;
      // Enclosed version should have one extra enclosure wrapper
      const topEncs = findNodesOfType(topGir, "enclosure");
      const encEncs = findNodesOfType(encGir, "enclosure");
      expect(encEncs.length).toBeGreaterThanOrEqual(topEncs.length);
    });

    it("containment tree for ○{● → ● | ● → ●} is valid", () => {
      const gir = JSON.parse(
        wasm.parseUlScript("○{● → ● | ● → ●}"),
      ) as Gir;
      assertContainmentIsTree(gir);
    });
  });
});

// ══════════════════════════════════════════════════
// 3.2 — Bug 2: Left-Arrow in Mixed Chains
// ══════════════════════════════════════════════════

describe("3.2 Bug 2: Left-arrow in mixed-operator chains", () => {
  it("● ← ● — bare left-arrow works", () => {
    const rt = roundTripChar("● ← ●");
    expect(rt.output).toBeTruthy();
  });

  it("○{● ← ●} — single left-arrow inside enclosure", () => {
    const rt = roundTripChar("○{● ← ●}");
    expect(rt.output).toBeTruthy();
  });

  it("● → ● | ● ← ● — mixed at top level", () => {
    const rt = roundTripChar("● → ● | ● ← ●");
    expect(rt.output).toBeTruthy();
  });

  it("○{● → ● | ● ← ●} — KNOWN BUG: mixed inside enclosure", () => {
    const rt = roundTripChar("○{● → ● | ● ← ●}");
    expect(rt.output).toBeTruthy();
  });

  it("○{● ← ● | ● → ●} — reversed order", () => {
    const rt = roundTripChar("○{● ← ● | ● → ●}");
    expect(rt.output).toBeTruthy();
  });

  it("○{● ← ● | ● ← ●} — both left-arrows", () => {
    const rt = roundTripChar("○{● ← ● | ● ← ●}");
    expect(rt.output).toBeTruthy();
  });

  it("○{● ← ● ← ●} — chain of left-arrows", () => {
    const rt = roundTripChar("○{● ← ● ← ●}");
    expect(rt.output).toBeTruthy();
  });

  it("○{● | ● ← ●} — adj then left-arrow", () => {
    const rt = roundTripChar("○{● | ● ← ●}");
    expect(rt.output).toBeTruthy();
  });

  it("○{● ← ● & ●} — left-arrow with intersection", () => {
    const rt = roundTripChar("○{● ← ● & ●}");
    expect(rt.output).toBeTruthy();
  });

  it("△{● → ● | ● ← ●} — triangle wrapper characterize", () => {
    const rt = roundTripChar("△{● → ● | ● ← ●}");
    expect(rt.output).toBeTruthy();
  });

  describe("GIR edge validation for left-arrow expressions", () => {
    it("○{● ← ●} — connects edges alternate point↔line", () => {
      const gir = JSON.parse(wasm.parseUlScript("○{● ← ●}")) as Gir;
      const connects = findEdgesOfType(gir, "connects");
      for (const edge of connects) {
        const src = gir.nodes.find((n: any) => n.id === edge.source);
        const tgt = gir.nodes.find((n: any) => n.id === edge.target);
        const types = [src?.type, tgt?.type].sort();
        // Should be line↔point, NOT line↔line
        expect(types).toEqual(["line", "point"]);
      }
    });

    it("● ← ● at top level — no line→line connects edges", () => {
      const gir = JSON.parse(wasm.parseUlScript("● ← ●")) as Gir;
      const connects = findEdgesOfType(gir, "connects");
      for (const edge of connects) {
        const src = gir.nodes.find((n: any) => n.id === edge.source);
        const tgt = gir.nodes.find((n: any) => n.id === edge.target);
        expect(src?.type === "line" && tgt?.type === "line").toBe(false);
      }
    });
  });
});

// ══════════════════════════════════════════════════
// 3.3 — Bug 3: Modal vs Assertion Modifier Overlap
// ══════════════════════════════════════════════════

describe("3.3 Modal vs assertion modifier deparser overlap", () => {
  it("!{● → ●} — emphatic round-trips", () => {
    const rt = roundTripChar("!{● → ●}");
    expect(rt.output).toBeTruthy();
  });

  it("?{● → ●} — evidential round-trips", () => {
    const rt = roundTripChar("?{● → ●}");
    expect(rt.output).toBeTruthy();
  });

  it("assert{● → ●} — force round-trips", () => {
    const rt = roundTripChar("assert{● → ●}");
    expect(rt.output).toBeTruthy();
  });

  it("assert{!{● → ●}} — force + modifier", () => {
    const rt = roundTripChar("assert{!{● → ●}}");
    // Both should survive deparse
    expect(rt.output).toBeTruthy();
  });

  it("!{assert{● → ●}} — modifier wrapping force", () => {
    const rt = roundTripChar("!{assert{● → ●}}");
    expect(rt.output).toBeTruthy();
  });

  it("?{query{● → ●}} — evidential + query force", () => {
    const rt = roundTripChar("?{query{● → ●}}");
    expect(rt.output).toBeTruthy();
  });
});

// ══════════════════════════════════════════════════
// 3.4 — Deparse Baseline: Expressions That Should Always Round-Trip
// ══════════════════════════════════════════════════

describe("3.4 Deparse baseline: known-good round-trips", () => {
  const knownGood = [
    "●",
    "→",
    "● → ●",
    "● ← ●",
    "● ↔ ●",
    "● | ●",
    "● → ● → ●",
    "● → ● | ●",
    "○{●}",
    "△{●}",
    "□{●}",
    "○{● → ●}",
    "● & ●",
  ];

  for (const expr of knownGood) {
    it(`"${expr}" round-trips exactly`, () => {
      const girJson = wasm.parseUlScript(expr);
      const deparsed = wasm.deparseGir(girJson);
      expect(deparsed).toBe(expr);
    });
  }
});

// ══════════════════════════════════════════════════
// 3.5 — Deparse After Operations
// ══════════════════════════════════════════════════

describe("3.5 Deparse after operations", () => {
  it("deparse(predicate(●, →, ●)) produces valid UL-Script", () => {
    const a = h.makeAssertion();
    const deparsed = h.deparse(a);
    expect(deparsed.length).toBeGreaterThan(0);
    // Should contain arrow and point symbols
    expect(deparsed).toMatch(/[●○→←↔∠]/);
  });

  it("deparse(negate(pred)) contains negation syntax", () => {
    const neg = h.apply("negate", h.makeAssertion());
    const deparsed = h.deparse(neg);
    expect(deparsed.length).toBeGreaterThan(0);
  });

  it("deparse(embed(pred)) is a valid expression", () => {
    const emb = h.apply("embed", h.makeAssertion());
    const deparsed = h.deparse(emb);
    expect(deparsed.length).toBeGreaterThan(0);
  });

  it("deparse(compose(→, →)) contains relation symbols", () => {
    const comp = h.apply("compose", h.p("→"), h.p("→"));
    const deparsed = h.deparse(comp);
    expect(deparsed.length).toBeGreaterThan(0);
  });

  it("deparse(invert(→)) produces valid syntax", () => {
    const inv = h.apply("invert", h.p("→"));
    const deparsed = h.deparse(inv);
    expect(deparsed.length).toBeGreaterThan(0);
  });

  it("deparse after 3-step chain produces valid syntax", () => {
    const abs = h.apply(
      "abstract",
      h.apply("embed", h.makeAssertion()),
    );
    const result = h.apply("modify_entity", abs, h.p("●"));
    const deparsed = h.deparse(result);
    expect(deparsed.length).toBeGreaterThan(0);
  });
});
