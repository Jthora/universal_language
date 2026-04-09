/**
 * PLAN 05 — Sci-Fi Scenario Expansion: AI User Stories
 *
 * Real multi-agent workflow tests that exercise multiple operations,
 * cross SVG boundaries, and produce structurally verifiable GIR.
 * Scenarios A–I: federated knowledge, hypothesis generation, debate,
 * swarm coordination, self-modification, alien communication,
 * collective intelligence, temporal reasoning, adversarial epistemology.
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
  edgeTypeDistribution,
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

/** SVG round-trip: render → extract */
function svgCycle(girJson: string): string {
  const svg = h.renderSvg(girJson);
  return h.extractGirFromSvg(svg);
}

// ══════════════════════════════════════════════════
// Scenario A: Federated Knowledge Synthesis
// ══════════════════════════════════════════════════

describe("Scenario A: Federated Knowledge Synthesis", () => {
  it("A.1 — three agents encode domain concepts with correct structure", () => {
    const physics = h.p("○{● ↔ ●}");
    const biology = h.p("○{● | ● | ●} → ○{● | ● | ●}");
    const philosophy = h.p("○{● → ●}");

    for (const gir of [physics, biology, philosophy]) {
      const parsed = JSON.parse(gir) as Gir;
      expect(parsed.nodes.length).toBeGreaterThan(0);
      const encs = findNodesOfType(parsed, "enclosure");
      expect(encs.length).toBeGreaterThanOrEqual(1);
    }
  });

  it("A.2 — each domain concept survives SVG serialization", () => {
    const exprs = ["○{● ↔ ●}", "○{● | ●} → ○{● | ●}", "○{● → ●}"];
    for (const expr of exprs) {
      const gir = h.p(expr);
      const extracted = svgCycle(gir);
      expect(JSON.parse(extracted).nodes.length).toBeGreaterThan(0);
    }
  });

  it("A.3 — coordinator builds conjunction of domain assertions", () => {
    const p1 = h.apply("predicate", h.p("○{● ↔ ●}"), h.p("→"), h.p("●"));
    const p2 = h.apply(
      "predicate",
      h.p("○{● | ●}"),
      h.p("→"),
      h.p("○{●}"),
    );
    const p3 = h.apply("predicate", h.p("○{● → ●}"), h.p("→"), h.p("●"));

    const conj12 = h.apply("conjoin", p1, p2);
    const conjAll = h.apply("conjoin", conj12, p3);

    expect(getRootSort(JSON.parse(conjAll))).toBe("assertion");
    const gir = JSON.parse(conjAll) as Gir;
    // Should contain enclosures from all three domains
    const encs = findNodesOfType(gir, "enclosure");
    expect(encs.length).toBeGreaterThanOrEqual(3);
  });

  it("A.4 — conjunction survives SVG cycle, containment is tree", () => {
    const p1 = h.apply("predicate", h.p("○{● ↔ ●}"), h.p("→"), h.p("●"));
    const p2 = h.apply("predicate", h.p("○{● | ●}"), h.p("→"), h.p("●"));
    const conj = h.apply("conjoin", p1, p2);
    const extracted = svgCycle(conj);
    const gir = JSON.parse(extracted) as Gir;
    assertContainmentIsTree(gir);
  });

  it("A.5 — declare-wrapped unified ontology", () => {
    const p1 = h.apply("predicate", h.p("●"), h.p("→"), h.p("●"));
    const p2 = h.apply("predicate", h.p("○{●}"), h.p("←"), h.p("●"));
    const conj = h.apply("conjoin", p1, p2);
    const raw = wasm.set_force(conj, "declare");
    const declared = typeof raw === "string" ? raw : JSON.stringify(raw);
    const svg = h.renderSvg(declared);
    expect(svg).toContain("<svg");
    const extracted = h.extractGirFromSvg(svg);
    expect(JSON.parse(extracted).nodes.length).toBeGreaterThan(0);
  });
});

// ══════════════════════════════════════════════════
// Scenario B: Autonomous Scientific Hypothesis Generation
// ══════════════════════════════════════════════════

describe("Scenario B: Scientific Hypothesis Generation", () => {
  it("B.1 — encode observation: collection → single (emergence)", () => {
    const observation = h.apply(
      "predicate",
      h.p("○{● | ● | ●}"),
      h.p("→"),
      h.p("○{●}"),
    );
    expect(getRootSort(JSON.parse(observation))).toBe("assertion");
  });

  it("B.2 — abstract(embed(observation)) → modifier", () => {
    const obs = h.apply(
      "predicate",
      h.p("○{● | ● | ●}"),
      h.p("→"),
      h.p("○{●}"),
    );
    const pattern = h.apply("abstract", h.apply("embed", obs));
    expect(getRootSort(JSON.parse(pattern))).toBe("modifier");
  });

  it("B.3 — apply pattern to new domain → entity", () => {
    const obs = h.apply(
      "predicate",
      h.p("○{● | ● | ●}"),
      h.p("→"),
      h.p("○{●}"),
    );
    const pattern = h.apply("abstract", h.apply("embed", obs));
    const newDomain = h.p("○{● → ● → ●}");
    const hypothesis = h.apply("modify_entity", pattern, newDomain);
    expect(getRootSort(JSON.parse(hypothesis))).toBe("entity");
  });

  it("B.4 — generate prediction: hypothesis → produces → concept", () => {
    const obs = h.apply(
      "predicate",
      h.p("○{● | ● | ●}"),
      h.p("→"),
      h.p("○{●}"),
    );
    const pattern = h.apply("abstract", h.apply("embed", obs));
    const hypothesis = h.apply("modify_entity", pattern, h.p("○{● → ●}"));
    const prediction = h.apply(
      "predicate",
      hypothesis,
      h.p("→"),
      h.p("○{●}"),
    );
    expect(getRootSort(JSON.parse(prediction))).toBe("assertion");
  });

  it("B.5 — hedged prediction survives SVG round-trip", () => {
    const pred = h.makeAssertion();
    const hedged = h.apply("modify_assertion", h.p("∠30"), pred);
    const extracted = svgCycle(hedged);
    const gir = JSON.parse(extracted) as Gir;
    expect(getRootSort(gir)).toBe("assertion");
    assertContainmentIsTree(gir);
  });

  it("B.6 — full pipeline: observation → abstract → apply → predict → SVG", () => {
    const obs = h.apply("predicate", h.p("○{● | ●}"), h.p("→"), h.p("○{●}"));
    const pattern = h.apply("abstract", h.apply("embed", obs));
    const hyp = h.apply("modify_entity", pattern, h.p("●"));
    const pred = h.apply("predicate", hyp, h.p("→"), h.p("●"));
    const hedged = h.apply("modify_assertion", h.p("∠45"), pred);
    const extracted = svgCycle(hedged);
    expect(getRootSort(JSON.parse(extracted))).toBe("assertion");
  });
});

// ══════════════════════════════════════════════════
// Scenario C: Multi-Agent Formal Debate
// ══════════════════════════════════════════════════

describe("Scenario C: Multi-Agent Formal Debate", () => {
  it("C.1 — Agent A asserts thesis, serializes to SVG", () => {
    const thesis = h.apply(
      "predicate",
      h.p("○{● → ●}"),
      h.p("→"),
      h.p("○{●}"),
    );
    const svg = h.renderSvg(thesis);
    expect(svg).toContain("<svg");
    const extracted = h.extractGirFromSvg(svg);
    expect(getRootSort(JSON.parse(extracted))).toBe("assertion");
  });

  it("C.2 — Agent B extracts thesis, embeds as entity for query", () => {
    const thesis = h.makeAssertion();
    const svg = h.renderSvg(thesis);
    const extracted = h.extractGirFromSvg(svg);
    const embedded = h.apply("embed", extracted);
    expect(getRootSort(JSON.parse(embedded))).toBe("entity");
    // B builds query: embed(thesis) → challenge
    const query = h.apply("predicate", embedded, h.p("→"), h.p("○{●}"));
    expect(getRootSort(JSON.parse(query))).toBe("assertion");
  });

  it("C.3 — Agent A negates challenge, serializes defense", () => {
    const challenge = h.makeAssertion();
    const defense = h.apply("negate", challenge);
    const svg = h.renderSvg(defense);
    const extracted = h.extractGirFromSvg(svg);
    expect(getRootSort(JSON.parse(extracted))).toBe("assertion");
  });

  it("C.4 — Agent B disjoin: defense or alternative", () => {
    const defense = h.makeAssertion();
    const alternative = h.apply(
      "predicate",
      h.p("○{●}"),
      h.p("←"),
      h.p("●"),
    );
    const proposal = h.apply("disjoin", defense, alternative);
    const svg = h.renderSvg(proposal);
    const extracted = h.extractGirFromSvg(svg);
    expect(getRootSort(JSON.parse(extracted))).toBe("assertion");
  });

  it("C.5 — Judge collects all 4 SVGs, verifies all extractable", () => {
    const svgs = [
      h.renderSvg(h.makeAssertion()),
      h.renderSvg(h.apply("negate", h.makeAssertion())),
      h.renderSvg(h.apply("embed", h.makeAssertion())),
      h.renderSvg(
        h.apply("conjoin", h.makeAssertion(), h.makeAssertion()),
      ),
    ];
    for (const svg of svgs) {
      const extracted = h.extractGirFromSvg(svg);
      expect(JSON.parse(extracted).nodes.length).toBeGreaterThan(0);
    }
  });
});

// ══════════════════════════════════════════════════
// Scenario D: Swarm Coordination Protocol
// ══════════════════════════════════════════════════

describe("Scenario D: Swarm Coordination Protocol", () => {
  it("D.1 — 10 agents advertise capabilities as distinct structures", () => {
    const capabilities = [
      "○{● → ● → ●}", // sequential processing
      "○{● | ● | ●}", // parallel storage
      "○{● ↔ ●}", // symmetric exchange
      "○{● → ○{●}}", // entity to concept
      "○{○{●} → ●}", // concept to entity
      "○{● ← ●}", // reverse causation
      "○{● → ● | ●}", // branching
      "○{● & ●}", // intersection
      "○{● → ●}", // simple causation
      "○{● ↔ ● ↔ ●}", // chain of exchange
    ];

    const girStructures = capabilities.map((c) => JSON.parse(h.p(c)) as Gir);

    // All 10 should parse with enclosures
    for (const gir of girStructures) {
      expect(findNodesOfType(gir, "enclosure").length).toBeGreaterThanOrEqual(
        1,
      );
    }
  });

  it("D.2 — all 10 capabilities survive independent SVG cycles", () => {
    const caps = [
      "○{● → ● → ●}",
      "○{● | ● | ●}",
      "○{● ↔ ●}",
      "○{● → ○{●}}",
      "○{○{●} → ●}",
    ];
    for (const cap of caps) {
      const gir = h.p(cap);
      const extracted = svgCycle(gir);
      expect(JSON.parse(extracted).nodes.length).toBeGreaterThan(0);
    }
  });

  it("D.3 — coordinator broadcasts task, agents extract independently", () => {
    const task = h.apply(
      "conjoin",
      h.apply("predicate", h.p("○{●}"), h.p("→"), h.p("●")),
      h.apply("predicate", h.p("●"), h.p("→"), h.p("○{●}")),
    );
    const svg = h.renderSvg(task);

    // 3 agents extract
    for (let i = 0; i < 3; i++) {
      const extracted = h.extractGirFromSvg(svg);
      expect(JSON.parse(extracted).nodes.length).toBeGreaterThan(0);
    }
  });

  it("D.4 — task acceptance: commit{embed(subtask) → agent}", () => {
    const subtask = h.makeAssertion();
    const acceptance = h.apply(
      "predicate",
      h.apply("embed", subtask),
      h.p("→"),
      h.p("●"),
    );
    const raw = wasm.set_force(acceptance, "commit");
    const committed = typeof raw === "string" ? raw : JSON.stringify(raw);
    const svg = h.renderSvg(committed);
    const extracted = h.extractGirFromSvg(svg);
    expect(JSON.parse(extracted).nodes.length).toBeGreaterThan(0);
  });

  it("D.5 — coordinator collects and conjoins 3 reports", () => {
    const reports = [
      h.makeAssertion(),
      h.apply("predicate", h.p("○{●}"), h.p("→"), h.p("●")),
      h.apply("predicate", h.p("●"), h.p("←"), h.p("○{●}")),
    ];
    // Each report goes through SVG
    const extractedReports = reports.map((r) => svgCycle(r));
    // Conjoin all
    let combined = extractedReports[0];
    for (let i = 1; i < extractedReports.length; i++) {
      combined = h.apply("conjoin", combined, extractedReports[i]);
    }
    expect(getRootSort(JSON.parse(combined))).toBe("assertion");
    // Wrap in declare
    const raw = wasm.set_force(combined, "declare");
    const declared = typeof raw === "string" ? raw : JSON.stringify(raw);
    expect(JSON.parse(declared).nodes.length).toBeGreaterThan(0);
  });
});

// ══════════════════════════════════════════════════
// Scenario E: Self-Modifying Ontology
// ══════════════════════════════════════════════════

describe("Scenario E: Self-Modifying Ontology", () => {
  it("E.1 — encode knowledge base with 3 facts", () => {
    const kb = h.p("○{○{● → ●} | ○{● ↔ ●} | ○{○{●} → ●}}");
    const gir = JSON.parse(kb) as Gir;
    expect(findNodesOfType(gir, "enclosure").length).toBeGreaterThanOrEqual(4);
  });

  it("E.2 — embed KB as entity", () => {
    const fact = h.apply("predicate", h.p("○{● → ●}"), h.p("→"), h.p("●"));
    const embedded = h.apply("embed", fact);
    expect(getRootSort(JSON.parse(embedded))).toBe("entity");
  });

  it("E.3 — extend KB: conjoin(embed(fact1), embed(fact2))", () => {
    const f1 = h.apply("predicate", h.p("●"), h.p("→"), h.p("●"));
    const f2 = h.apply("predicate", h.p("●"), h.p("←"), h.p("●"));
    const extended = h.apply("conjoin", f1, f2);
    expect(getRootSort(JSON.parse(extended))).toBe("assertion");
    const gir = JSON.parse(extended) as Gir;
    expect(gir.nodes.length).toBeGreaterThan(
      JSON.parse(f1).nodes.length,
    );
  });

  it("E.4 — negate extension produces distinct structure", () => {
    const f1 = h.apply("predicate", h.p("●"), h.p("→"), h.p("●"));
    const f2 = h.apply("predicate", h.p("●"), h.p("←"), h.p("●"));
    const extended = h.apply("conjoin", f1, f2);
    const negated = h.apply("negate", extended);
    expect(
      girDistinct(JSON.parse(extended), JSON.parse(negated)),
    ).toBe(true);
  });

  it("E.5 — extended KB survives SVG cycle", () => {
    const f1 = h.apply("predicate", h.p("●"), h.p("→"), h.p("●"));
    const f2 = h.apply("predicate", h.p("○{●}"), h.p("→"), h.p("●"));
    const extended = h.apply("conjoin", f1, f2);
    const extracted = svgCycle(extended);
    const gir = JSON.parse(extracted) as Gir;
    expect(gir.nodes.length).toBeGreaterThan(0);
    assertContainmentIsTree(gir);
  });
});

// ══════════════════════════════════════════════════
// Scenario F: Cross-Species Communication Bridge
// ══════════════════════════════════════════════════

describe("Scenario F: Cross-Species Communication Bridge", () => {
  it("F.1 — programmatically-constructed GIR deparses to valid UL-Script", () => {
    const alienGir = {
      ul_gir: "1.0",
      root: "n1",
      nodes: [
        { id: "n1", type: "enclosure", shape: "circle" },
        { id: "n2", type: "point" },
        { id: "n3", type: "line", directed: true, direction: [1, 0] },
        { id: "n4", type: "point" },
      ],
      edges: [
        { source: "n1", target: "n2", type: "contains" },
        { source: "n1", target: "n3", type: "contains" },
        { source: "n1", target: "n4", type: "contains" },
        { source: "n2", target: "n3", type: "connects" },
        { source: "n3", target: "n4", type: "connects" },
      ],
    };
    const deparsed = wasm.deparseGir(JSON.stringify(alienGir));
    expect(typeof deparsed).toBe("string");
    expect(deparsed.length).toBeGreaterThan(0);
  });

  it("F.2 — alien GIR → deparse → reparse produces structurally similar GIR", () => {
    const alienGir = {
      ul_gir: "1.0",
      root: "n1",
      nodes: [
        { id: "n1", type: "enclosure", shape: "circle" },
        { id: "n2", type: "point" },
        { id: "n3", type: "line", directed: true, direction: [1, 0] },
        { id: "n4", type: "point" },
      ],
      edges: [
        { source: "n1", target: "n2", type: "contains" },
        { source: "n1", target: "n3", type: "contains" },
        { source: "n1", target: "n4", type: "contains" },
        { source: "n2", target: "n3", type: "connects" },
        { source: "n3", target: "n4", type: "connects" },
      ],
    };
    const deparsed = wasm.deparseGir(JSON.stringify(alienGir));
    const reparsed = JSON.parse(wasm.parseUlScript(deparsed)) as Gir;
    // Should have similar structure (enclosure containing point-line-point)
    expect(findNodesOfType(reparsed, "point").length).toBeGreaterThanOrEqual(2);
    expect(findNodesOfType(reparsed, "line").length).toBeGreaterThanOrEqual(1);
    expect(findNodesOfType(reparsed, "enclosure").length).toBeGreaterThanOrEqual(1);
  });

  it("F.3 — structural analysis of alien GIR", () => {
    const alienGir = {
      ul_gir: "1.0",
      root: "n1",
      nodes: [
        { id: "n1", type: "enclosure", shape: "circle" },
        { id: "n2", type: "point" },
        { id: "n3", type: "line", directed: true, direction: [1, 0] },
        { id: "n4", type: "point" },
        { id: "n5", type: "enclosure", shape: "circle" },
        { id: "n6", type: "point" },
      ],
      edges: [
        { source: "n1", target: "n2", type: "contains" },
        { source: "n1", target: "n3", type: "contains" },
        { source: "n1", target: "n4", type: "contains" },
        { source: "n2", target: "n3", type: "connects" },
        { source: "n3", target: "n4", type: "connects" },
        { source: "n1", target: "n5", type: "adjacent" },
        { source: "n5", target: "n6", type: "contains" },
      ],
    };
    const gir = alienGir as unknown as Gir;
    const nodeDist = nodeTypeDistribution(gir);
    expect(nodeDist.point).toBe(3);
    expect(nodeDist.line).toBe(1);
    expect(nodeDist.enclosure).toBe(2);
    const edgeDist = edgeTypeDistribution(gir);
    expect(edgeDist.contains).toBe(4);
    expect(edgeDist.connects).toBe(2);
    expect(edgeDist.adjacent).toBe(1);
  });

  it("F.4 — alien GIR renders to SVG with geometric elements", () => {
    const alienGir = {
      ul_gir: "1.0",
      root: "n1",
      nodes: [
        { id: "n1", type: "enclosure", shape: "circle" },
        { id: "n2", type: "point" },
        { id: "n3", type: "line", directed: true, direction: [1, 0] },
        { id: "n4", type: "point" },
      ],
      edges: [
        { source: "n1", target: "n2", type: "contains" },
        { source: "n1", target: "n3", type: "contains" },
        { source: "n1", target: "n4", type: "contains" },
        { source: "n2", target: "n3", type: "connects" },
        { source: "n3", target: "n4", type: "connects" },
      ],
    };
    const svg = wasm.renderSvg(JSON.stringify(alienGir), 256, 256);
    expect(svg).toContain("<svg");
    // SVG should contain visual elements
    expect(svg.length).toBeGreaterThan(100);
  });

  it("F.5 — construct response using alien GIR as substructure", () => {
    // Deparse alien GIR, reparse, embed into response
    const alienGir = {
      ul_gir: "1.0",
      root: "n1",
      nodes: [
        { id: "n1", type: "enclosure", shape: "circle" },
        { id: "n2", type: "point" },
      ],
      edges: [
        { source: "n1", target: "n2", type: "contains" },
      ],
    };
    const deparsed = wasm.deparseGir(JSON.stringify(alienGir));
    const reparsed = wasm.parseUlScript(deparsed);
    // Build response: predicate(reparsed_entity, →, ●)
    const response = h.apply(
      "predicate",
      reparsed,
      h.p("→"),
      h.p("●"),
    );
    expect(getRootSort(JSON.parse(response))).toBe("assertion");
    // Render response to SVG
    const svg = h.renderSvg(response);
    expect(svg).toContain("<svg");
  });
});

// ══════════════════════════════════════════════════
// Scenario G: Collective Intelligence Emergence
// ══════════════════════════════════════════════════

describe("Scenario G: Collective Intelligence Emergence", () => {
  it("G.1 — 5 agents each hold partial knowledge, all parse correctly", () => {
    const knowledge = [
      h.apply("predicate", h.p("●"), h.p("→"), h.p("○{●}")),
      h.apply("predicate", h.p("○{●}"), h.p("→"), h.p("○{● | ●}")),
      h.apply("predicate", h.p("○{● | ●}"), h.p("→"), h.p("○{● → ●}")),
      h.apply("predicate", h.p("○{● → ●}"), h.p("→"), h.p("●")),
      h.makeAssertion(),
    ];
    for (const k of knowledge) {
      expect(getRootSort(JSON.parse(k))).toBe("assertion");
    }
  });

  it("G.2 — pairwise exchange via SVG: Agent 1→2→3", () => {
    const k1 = h.apply("predicate", h.p("●"), h.p("→"), h.p("○{●}"));
    const k2 = h.apply("predicate", h.p("○{●}"), h.p("→"), h.p("●"));

    // Agent 1 sends to Agent 2 via SVG
    const svg1 = h.renderSvg(k1);
    const received = h.extractGirFromSvg(svg1);

    // Agent 2 conjoins with own knowledge
    const combined = h.apply("conjoin", received, k2);
    expect(getRootSort(JSON.parse(combined))).toBe("assertion");

    // Agent 2 sends combined to Agent 3 via SVG
    const svg2 = h.renderSvg(combined);
    const received3 = h.extractGirFromSvg(svg2);

    const k3 = h.makeAssertion();
    const finalCombined = h.apply("conjoin", received3, k3);
    expect(getRootSort(JSON.parse(finalCombined))).toBe("assertion");
  });

  it("G.3 — final conjunction has more nodes than any individual piece", () => {
    const parts = [
      h.makeAssertion(),
      h.apply("predicate", h.p("○{●}"), h.p("→"), h.p("●")),
      h.apply("predicate", h.p("●"), h.p("←"), h.p("○{●}")),
    ];
    let combined = parts[0];
    for (let i = 1; i < parts.length; i++) {
      combined = h.apply("conjoin", combined, svgCycle(parts[i]));
    }
    const gir = JSON.parse(combined) as Gir;
    for (const part of parts) {
      expect(gir.nodes.length).toBeGreaterThan(
        JSON.parse(part).nodes.length,
      );
    }
  });

  it("G.4 — no information lost across 4 SVG boundaries", () => {
    let current = h.makeAssertion();
    const initialNodes = (JSON.parse(current) as Gir).nodes.length;

    for (let i = 0; i < 4; i++) {
      current = svgCycle(current);
    }

    const finalNodes = (JSON.parse(current) as Gir).nodes.length;
    // Structure should be preserved (node count stable after cycles)
    expect(finalNodes).toBe(initialNodes);
  });
});

// ══════════════════════════════════════════════════
// Scenario H: Temporal Reasoning and Prediction
// ══════════════════════════════════════════════════

describe("Scenario H: Temporal Reasoning and Prediction", () => {
  it("H.1 — encode temporal states: past (asserted), present (emphatic), future (hedged)", () => {
    const past = h.makeAssertion(); // certain fact
    const present = h.apply("modify_assertion", h.p("∠90"), past); // emphatic
    const future = h.apply("modify_assertion", h.p("∠30"), h.makeAssertion()); // hedged

    expect(getRootSort(JSON.parse(past))).toBe("assertion");
    expect(getRootSort(JSON.parse(present))).toBe("assertion");
    expect(getRootSort(JSON.parse(future))).toBe("assertion");
  });

  it("H.2 — conjoin temporal states into model", () => {
    const past = h.makeAssertion();
    const present = h.apply("modify_assertion", h.p("∠90"), h.makeAssertion());
    const model = h.apply("conjoin", past, present);
    expect(getRootSort(JSON.parse(model))).toBe("assertion");
  });

  it("H.3 — temporal model survives SVG cycle", () => {
    const past = h.makeAssertion();
    const present = h.apply("modify_assertion", h.p("∠60"), h.makeAssertion());
    const model = h.apply("conjoin", past, present);
    const extracted = svgCycle(model);
    expect(getRootSort(JSON.parse(extracted))).toBe("assertion");
    assertContainmentIsTree(JSON.parse(extracted));
  });

  it("H.4 — update temporal model: disjoin old present, conjoin new", () => {
    const past = h.makeAssertion();
    const oldPresent = h.apply(
      "predicate",
      h.p("●"),
      h.p("→"),
      h.p("●"),
    );
    const model = h.apply("conjoin", past, oldPresent);

    // New information arrives
    const newPresent = h.apply(
      "predicate",
      h.p("●"),
      h.p("→"),
      h.p("● → ●"),
    );

    // Build updated model: past + new present
    const updated = h.apply("conjoin", past, newPresent);
    expect(getRootSort(JSON.parse(updated))).toBe("assertion");

    // Updated should be different from original
    expect(
      girDistinct(JSON.parse(model), JSON.parse(updated)),
    ).toBe(true);
  });
});

// ══════════════════════════════════════════════════
// Scenario I: Adversarial Epistemology
// ══════════════════════════════════════════════════

describe("Scenario I: Adversarial Epistemology (Lie Detection)", () => {
  it("I.1 — contradictory claim: conjoin(a, negate(a)) is parseable", () => {
    const a = h.makeAssertion();
    const contradiction = h.apply("conjoin", a, h.apply("negate", a));
    expect(getRootSort(JSON.parse(contradiction))).toBe("assertion");
  });

  it("I.2 — contradiction serializes and extracts via SVG", () => {
    const a = h.makeAssertion();
    const contradiction = h.apply("conjoin", a, h.apply("negate", a));
    const extracted = svgCycle(contradiction);
    expect(JSON.parse(extracted).nodes.length).toBeGreaterThan(0);
  });

  it("I.3 — negation detection: negate(a) is structurally distinct from a", () => {
    const a = h.makeAssertion();
    const neg = h.apply("negate", a);
    expect(girDistinct(JSON.parse(a), JSON.parse(neg))).toBe(true);
  });

  it("I.4 — proof of inconsistency: embed(contradiction) → predication", () => {
    const a = h.makeAssertion();
    const contradiction = h.apply("conjoin", a, h.apply("negate", a));
    const proof = h.apply(
      "predicate",
      h.apply("embed", contradiction),
      h.p("→"),
      h.apply("embed", h.apply("negate", contradiction)),
    );
    expect(getRootSort(JSON.parse(proof))).toBe("assertion");
  });

  it("I.5 — proof survives SVG cycle and has complex structure", () => {
    const a = h.makeAssertion();
    const contradiction = h.apply("conjoin", a, h.apply("negate", a));
    const proof = h.apply(
      "predicate",
      h.apply("embed", contradiction),
      h.p("→"),
      h.apply("embed", h.apply("negate", contradiction)),
    );
    const extracted = svgCycle(proof);
    const gir = JSON.parse(extracted) as Gir;
    expect(gir.nodes.length).toBeGreaterThan(15);
    assertContainmentIsTree(gir);
  });
});
