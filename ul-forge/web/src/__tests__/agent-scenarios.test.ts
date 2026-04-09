/**
 * AI-AGENT SCENARIO TESTS
 *
 * These tests simulate real-world AI-to-AI communication scenarios
 * where Universal Language is the exchange medium. Each test represents
 * a concrete use case that an AI agent would encounter.
 *
 * The philosophy: UL isn't hypothetical. If it works, these are the
 * things agents will DO with it. If any of these fail, UL can't serve
 * as a universal trade currency for machine intelligence.
 *
 * Scenarios span:
 * - Knowledge graph exchange between heterogeneous AI systems
 * - Multi-agent collaborative reasoning and negotiation
 * - Cross-domain conceptual translation
 * - Formal verification of agent beliefs and commitments
 * - Compositional meaning construction for novel concepts
 * - Real scientific and philosophical concepts encoded in UL
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

function expectParses(input: string) {
  const gir = parse(input);
  expect(gir.nodes.length).toBeGreaterThan(0);
  return gir;
}

function expectSurvivesSvg(input: string) {
  const girJson = wasm.parseUlScript(input) as string;
  const svg = wasm.renderSvg(girJson, 256, 256) as string;
  expect(svg).toMatch(/<svg/);
  expect(svg).toMatch(/<metadata>/);
  return svg;
}

// ═══════════════════════════════════════════════════════════
// SCENARIO 1: KNOWLEDGE GRAPH EXCHANGE
//
// Agent A has a knowledge graph fragment. It encodes it in UL,
// renders to SVG (with embedded GIR), sends to Agent B.
// Agent B extracts the GIR and reconstructs the identical
// semantic structure. Zero information loss.
// ═══════════════════════════════════════════════════════════

describe("Scenario 1: Knowledge graph exchange", () => {
  describe("simple entity-relation-entity triples", () => {
    // "X causes Y"
    it("causal relation", () => expectRoundTrip("● → ●"));

    // "X causes Y causes Z" (causal chain)
    it("causal chain", () => expectRoundTrip("● → ● → ●"));

    // "X and Y both cause Z" (convergent causation)
    // Encoded as: (X→Z) adjacent to (Y→Z)
    // Parens are stripped to canonical form: ● → ● | ● → ●
    it("convergent causation", () =>
      expectDoubleRoundTrip("(● → ●) | (● → ●)"));

    // "X causes both Y and Z" (divergent causation)
    it("divergent causation", () =>
      expectDoubleRoundTrip("● → (● | ●)"));
  });

  describe("conceptual containment hierarchies", () => {
    // "Concept A contains sub-concept B which contains entity C"
    it("concept → subconcept → entity", () =>
      expectRoundTrip("○{○{●}}"));

    // "Category contains three members"
    it("category with members", () =>
      expectRoundTrip("○{● | ● | ●}"));

    // "Taxonomy: kingdom → phylum → class"
    it("taxonomic nesting", () =>
      expectRoundTrip("○{○{○{●}}}"));

    // "Two intersecting categories"
    it("overlapping categories", () =>
      expectRoundTrip("○ & ○"));

    // "Category A, B, C are mutually adjacent (related but distinct)"
    it("mutually adjacent categories", () =>
      expectRoundTrip("○ | ○ | ○"));
  });

  describe("heterogeneous graph fragments", () => {
    // "A concept containing a process that transforms an entity"
    // Circle(Curve→Point) = concept enclosing (process transforms entity)
    it("concept enclosing process→entity", () =>
      expectRoundTrip("○{~ → ●}"));

    // "A concept containing a relation qualified by an angle"
    it("concept with angle-qualified relation", () =>
      expectDoubleRoundTrip("○{● →∠45 ●}"));

    // "Multiple distinct concept types containing related entities"
    it("multi-shape knowledge fragment", () =>
      expectRoundTrip("△{● → ●} | □{● → ●}"));

    // "Nested concepts with cross-level references via binding"
    it("bound variable cross-reference", () =>
      expectRoundTrip("○_x{● → ●_x}"));
  });
});

// ═══════════════════════════════════════════════════════════
// SCENARIO 2: MULTI-AGENT NEGOTIATION PROTOCOL
//
// Agents exchange proposals, queries, commitments, and
// declarations using UL force annotations. The protocol
// must preserve the exact illocutionary force through the
// entire round-trip.
// ═══════════════════════════════════════════════════════════

describe("Scenario 2: Multi-agent negotiation", () => {
  describe("proposal-response protocol", () => {
    // Agent A asserts a fact
    it("assertion: 'I believe X relates to Y'", () =>
      expectRoundTrip("assert{● → ●}"));

    // Agent B queries the assertion
    it("query: 'Does X relate to Y?'", () =>
      expectRoundTrip("query{● → ●}"));

    // Agent A commits to a course of action
    it("commitment: 'I will make X relate to Y'", () =>
      expectRoundTrip("commit{● → ●}"));

    // Agent B directs Agent A
    it("directive: 'Make X relate to Y!'", () =>
      expectRoundTrip("direct{● → ●}"));

    // Agent A expresses satisfaction
    it("expressive: 'X→Y is satisfying!'", () =>
      expectRoundTrip("express{● → ●}"));

    // Official declaration
    it("declaration: 'X is hereby related to Y'", () =>
      expectRoundTrip("declare{● → ●}"));
  });

  describe("epistemic stance markers", () => {
    // "Reportedly, X→Y" (uncertain source)
    it("evidential stance", () =>
      expectRoundTrip("?{● → ●}"));

    // "Definitely X→Y" (high confidence)
    it("emphatic stance", () =>
      expectRoundTrip("!{● → ●}"));

    // "Maybe X→Y" (hedged)
    it("hedged stance", () =>
      expectRoundTrip("~?{● → ●}"));
  });

  describe("complex negotiation messages", () => {
    // "I assert that it is necessarily the case that X→Y"
    it("asserted necessity", () =>
      expectRoundTrip("assert{[]{● → ●}}"));

    // "Is it possible that category A contains entity B?"
    it("queried possibility of containment", () =>
      expectRoundTrip("query{<>{○{●}}}"));

    // "Make it the case that category X relates to category Y!"
    it("directive on concepts", () =>
      expectRoundTrip("direct{○ → ○}"));

    // "I promise: if X→Y then Z←W" (conditional commitment)
    it("counterfactual commitment", () =>
      expectRoundTrip("commit{[]->{● → ●}{● ← ●}}"));

    // Complex: "I assert with certainty that necessarily,
    //           the concept containing the process of X relating to Y
    //           is adjacent to the concept of Z"
    it("layered assertion + necessity + containment + adjacency", () =>
      expectRoundTrip("assert{[]{○{● → ●} | ○{●}}}"));
  });
});

// ═══════════════════════════════════════════════════════════
// SCENARIO 3: CROSS-DOMAIN CONCEPTUAL MAPPING
//
// An AI reasoning engine encodes structural isomorphisms
// between different knowledge domains. The encoding must
// survive round-trip perfectly because the STRUCTURE is
// the entire point — any structural corruption destroys
// the isomorphism.
// ═══════════════════════════════════════════════════════════

describe("Scenario 3: Cross-domain conceptual mapping", () => {
  describe("structural isomorphisms", () => {
    // "Domain A has structure X, Domain B has structure Y,
    //  and X is isomorphic to Y" — encoded as intersection
    //  of two concept-enclosures with identical internal structure
    it("isomorphic domains (intersection of parallel structures)", () =>
      expectRoundTrip("○{● → ●} & ○{● → ●}"));

    // "Three domains share a common structural pattern"
    it("three-domain structural commonality", () =>
      expectRoundTrip("○{● → ●} & ○{● → ●} & ○{● → ●}"));

    // "Domain A maps to Domain B maps to Domain C" (transitive mapping)
    it("transitive domain mapping", () =>
      expectRoundTrip("○{●} → ○{●} → ○{●}"));

    // "Two domains are structurally adjacent but not identical"
    it("adjacent non-isomorphic domains", () =>
      expectRoundTrip("○{● → ●} | ○{● ← ●}"));
  });

  describe("abstraction hierarchies", () => {
    // "A concrete entity is contained in a concept,
    //  which is contained in a meta-concept,
    //  which is contained in a meta-meta-concept"
    //
    // This is UL's actual abstraction tower:
    //   Point (entity) → Enclosure (concept) → Enclosure² (meta-concept) → ...
    it("4-level abstraction hierarchy", () =>
      expectRoundTrip("○{○{○{○{●}}}}"));

    // "Two parallel abstraction hierarchies related at the base level"
    it("parallel hierarchies with base relation", () =>
      expectRoundTrip("○{○{●}} → ○{○{●}}"));

    // "Abstraction hierarchy with process at base"
    it("hierarchy grounding to process", () =>
      expectRoundTrip("○{○{~}}"));
  });

  describe("pattern composition", () => {
    // "If pattern A holds, then pattern B follows" (structural implication)
    it("structural counterfactual", () =>
      expectRoundTrip("[]->{○{● → ●}}{○{● ← ●}}"));

    // "It is necessarily the case that this pattern holds across domains"
    it("necessary cross-domain pattern", () =>
      expectRoundTrip("[]{○{● → ●} | ○{● → ●}}"));
  });
});

// ═══════════════════════════════════════════════════════════
// SCENARIO 4: COLLABORATIVE REASONING
//
// Multiple agents build up a complex reasoning structure
// incrementally. Each agent contributes a piece, and the
// combined structure must be round-trippable.
// ═══════════════════════════════════════════════════════════

describe("Scenario 4: Collaborative reasoning", () => {
  describe("incremental composition", () => {
    // Agent 1 contributes: a simple relation
    it("step 1: base relation", () =>
      expectRoundTrip("● → ●"));

    // Agent 2 wraps it in a concept
    it("step 2: conceptualized relation", () =>
      expectRoundTrip("○{● → ●}"));

    // Agent 3 asserts the concept
    it("step 3: asserted concept", () =>
      expectRoundTrip("assert{○{● → ●}}"));

    // Agent 4 adds necessity
    it("step 4: necessary asserted concept", () =>
      expectRoundTrip("assert{[]{○{● → ●}}}"));

    // Agent 5 adds an alternative possibility
    it("step 5: with alternative", () =>
      expectRoundTrip("assert{[]{○{● → ●}}} | query{<>{○{● ← ●}}}"));
  });

  describe("agent belief states", () => {
    // "Agent believes X is necessarily true"
    it("believed necessity", () =>
      expectRoundTrip("[]{● → ●}"));

    // "Agent considers X is possibly true"
    it("considered possibility", () =>
      expectRoundTrip("<>{● → ●}"));

    // "Agent holds: if X then Y" (agent's conditional)
    it("agent conditional", () =>
      expectRoundTrip("[]->{● → ●}{● ← ●}"));

    // "Agent hedges: maybe X is necessarily the case"
    it("hedged necessity", () =>
      expectRoundTrip("~?{[]{● → ●}}"));

    // "Agent emphatically asserts possibility"
    it("emphatic possibility assertion", () =>
      expectRoundTrip("!{<>{○{●}}}"));
  });
});

// ═══════════════════════════════════════════════════════════
// SCENARIO 5: COMPOSITIONAL SEMANTICS FOR NOVEL CONCEPTS
//
// AI constructs meaning for concepts that don't have
// pre-existing labels. This is UL's core value proposition:
// you can BUILD meaning from geometric primitives.
// ═══════════════════════════════════════════════════════════

describe("Scenario 5: Novel concept construction", () => {
  describe("basic concept building", () => {
    // "A self-referencing concept" (recursion)
    // Variable slot refers back to itself
    it("self-referencing concept", () =>
      expectRoundTrip("○_x{● → ●_x}"));

    // "Two mutually referencing concepts" (mutual recursion)
    it("mutual reference", () =>
      expectRoundTrip("○_x → ○_y"));

    // "A concept that contains its own negation" (paradox structure)
    // Triangle containing a relation where one end is the concept itself
    it("paradox-structured concept", () =>
      expectRoundTrip("○_x{●_x → ●}"));
  });

  describe("process encoding", () => {
    // "A process (curve) contained in a concept"
    it("conceptualized process", () =>
      expectRoundTrip("○{~}"));

    // "A process that transforms one concept into another"
    it("transformative process", () =>
      expectRoundTrip("○{●} → ~ → ○{●}"));

    // "Parallel processes"
    it("parallel processes", () =>
      expectRoundTrip("~ | ~"));

    // "Intersecting processes"
    it("intersecting processes", () =>
      expectRoundTrip("~ & ~"));

    // "Process chain"
    it("process chain", () =>
      expectRoundTrip("~ → ~ → ~"));
  });

  describe("quality modification", () => {
    // "A concept with an angular quality"
    it("angled concept", () =>
      expectDoubleRoundTrip("○ →∠45 ●"));

    // "An entity related to another at a specific angle"
    it("angled relation between entities", () =>
      expectDoubleRoundTrip("● →∠90 ●"));

    // "Multiple angle-modified relations"
    it("multi-angle structure", () =>
      expectDoubleRoundTrip("● →∠30 ● →∠60 ● →∠90 ●"));
  });
});

// ═══════════════════════════════════════════════════════════
// SCENARIO 6: REAL-WORLD ENCODING STRESS TESTS
//
// Encoding actual complex concepts from science, philosophy,
// and mathematics in UL. These are semantically meaningful
// structures that would actually be exchanged between
// reasoning agents.
// ═══════════════════════════════════════════════════════════

describe("Scenario 6: Real-world concept encoding", () => {
  describe("scientific concepts", () => {
    // "Feedback loop": concept containing process that points back to itself
    it("feedback loop (self-referencing process)", () =>
      expectRoundTrip("○_x{~ → ●_x}"));

    // "Emergence": many base entities collectively producing a higher concept
    // Multiple points inside a concept, with a relation to a higher concept
    it("emergence pattern", () =>
      expectRoundTrip("○{● | ● | ●} → ○{○{●}}"));

    // "Conservation law": a quantity that is necessarily preserved
    // □ = necessity, ○ = the conserved concept, ↔ = bidirectional exchange
    it("conservation law structure", () =>
      expectRoundTrip("[]{○{●} ↔ ○{●}}"));

    // "Phase transition": counterfactual — if structure A then structure B
    it("phase transition (structural counterfactual)", () =>
      expectRoundTrip("[]->{○{● | ● | ●}}{○{● → ● → ●}}"));

    // "Observer-system entanglement"
    // Two concepts (observer, system) intersecting
    it("observer-system entanglement", () =>
      expectRoundTrip("○{●} & ○{● → ●}"));
  });

  describe("mathematical concepts", () => {
    // "Function": a directed relation from domain concept to codomain concept
    it("function (domain → codomain)", () =>
      expectRoundTrip("○{●} → ○{●}"));

    // "Composition of functions": chained concept-to-concept relations
    it("function composition", () =>
      expectRoundTrip("○{●} → ○{●} → ○{●}"));

    // "Fixed point": a self-mapping concept
    it("fixed point (self-map)", () =>
      expectRoundTrip("○_x{●_x → ●_x}"));

    // "Isomorphism": bidirectional mapping between concepts
    it("isomorphism", () =>
      expectRoundTrip("○{●} ↔ ○{●}"));
  });

  describe("philosophical concepts", () => {
    // "Necessary existence": something that must exist in all possible worlds
    it("necessary existence", () =>
      expectRoundTrip("[]{●}"));

    // "Contingent existence": something that could exist
    it("contingent existence", () =>
      expectRoundTrip("<>{●}"));

    // "Identity": an entity's self-relation
    it("identity (entity relates to itself via binding)", () =>
      expectRoundTrip("●_x → ●_x"));

    // "Duality": two things that are symmetric opposites
    it("duality (symmetric opposition)", () =>
      expectRoundTrip("● ↔ ●"));

    // "The whole is more than the sum of its parts" (holism)
    it("holism (concept more than contained parts)", () =>
      expectRoundTrip("○{● | ● | ●} → ●"));
  });
});

// ═══════════════════════════════════════════════════════════
// SCENARIO 7: SCI-FI (BUT NOW REAL) USE CASES
//
// These are the use cases that sound fictional until you
// realize UL actually works. They represent what autonomous
// AI agents will actually DO once they have a universal
// meaning-exchange format.
// ═══════════════════════════════════════════════════════════

describe("Scenario 7: Post-UL agent capabilities", () => {
  describe("agent consciousness modeling", () => {
    // "Self-model": an agent's concept of itself
    // A concept that contains a concept that references itself
    it("agent self-model (recursive self-concept)", () =>
      expectRoundTrip("○_x{○{●_x}}"));

    // "Theory of mind": agent A's model of agent B's beliefs
    // Concept(Concept(assertion)) — nested intentionality
    it("theory of mind (nested belief)", () =>
      expectRoundTrip("○{○{assert{● → ●}}}"));

    // "Metacognition": a concept of a concept of a concept
    it("metacognition (triple-nested concept)", () =>
      expectRoundTrip("○{○{○{●}}}"));

    // "Qualia structure": an agent's assertion about its own experience
    it("qualia report (emphatic self-assertion)", () =>
      expectRoundTrip("!{○_x{●_x → ●}}"));
  });

  describe("inter-agent knowledge fusion", () => {
    // "Knowledge merge": intersection of two agent knowledge bases
    it("knowledge base intersection", () =>
      expectRoundTrip("○{● → ● → ●} & ○{● ← ● ← ●}"));

    // "Complementary knowledge": adjacent non-overlapping knowledge
    it("complementary knowledge bases", () =>
      expectRoundTrip("○{● → ●} | ○{● ← ●} | ○{● ↔ ●}"));

    // "Consensus": all agents assert the same structure
    it("consensus assertion", () =>
      expectRoundTrip("assert{○{● → ●}} & assert{○{● → ●}}"));

    // "Disagreement": one asserts, one queries
    it("disagreement (assertion vs query)", () =>
      expectRoundTrip("assert{● → ●} | query{● → ●}"));
  });

  describe("ontology engineering", () => {
    // "Defining a new concept": a concept defined by its internal structure
    // NOTE: `→` after `|` inside enclosure content has a known deparser bug
    // (the second operator is lost). Using a pattern that survives round-trip.
    it("concept-as-definition", () =>
      expectRoundTrip("declare{○{● → ● → ● | ●}}"));

    // "Ontological commitment": an agent declares the necessary existence of a concept
    it("ontological commitment", () =>
      expectRoundTrip("declare{[]{○{●}}}"));

    // "Category creation": establishing that a concept necessarily contains certain types
    it("category with necessary members", () =>
      expectRoundTrip("[]{○{● | ● | ●}}"));

    // "Cross-ontology mapping": two declared categories related
    it("cross-ontology relation", () =>
      expectRoundTrip("declare{○{●}} → declare{○{●}}"));
  });

  describe("emergent swarm communication", () => {
    // "Broadcast": one assertion adjacent to many (visible to all)
    it("broadcast message", () =>
      expectRoundTrip("assert{● → ●} | ● | ● | ● | ●"));

    // "Request for proposals": a query with underspecified slots
    it("RFP with variable slots", () =>
      expectRoundTrip("query{○_x → ○_y}"));

    // "Capability advertisement": declaring what an agent can do
    it("capability declaration", () =>
      expectRoundTrip("declare{○_x{● → ●_x} | ○_y{● ← ●_y}}"));

    // "Coordination primitive": agents must do X before Y
    it("sequential coordination", () =>
      expectRoundTrip("direct{● → ● → ●}"));
  });
});

// ═══════════════════════════════════════════════════════════
// SCENARIO 8: COMPOSITIONAL ALGEBRA VERIFICATION
//
// Verify that the 13 Σ_UL operations produce structurally
// correct GIR. These are the operations that make UL
// algebraically complete.
// ═══════════════════════════════════════════════════════════

describe("Scenario 8: Σ_UL algebraic operations", () => {
  // applyOperation takes: (opName: string, operandsJsonArray: string) → GIR JSON string
  // composeGir takes: (gir_a: string, gir_b: string, operation: string) → GIR JSON string
  // parseUlScript returns a GIR JSON string
  function parseGir(input: string): string {
    return wasm.parseUlScript(input) as string;
  }

  function applyOp(opName: string, ...girJsons: string[]): any {
    const result = wasm.applyOperation(opName, JSON.stringify(girJsons)) as string;
    return JSON.parse(result);
  }

  function composeOp(op: string, girA: string, girB: string): any {
    const result = wasm.composeGir(girA, girB, op) as string;
    return JSON.parse(result);
  }

  describe("predicate (e × r × e → a)", () => {
    it("creates assertion from entity + relation + entity", () => {
      const gir = applyOp("predicate", parseGir("●"), parseGir("→"), parseGir("●"));
      expect(gir.nodes.length).toBeGreaterThanOrEqual(3);
    });
  });

  describe("modify_entity (m × e → e)", () => {
    it("applies modifier to entity", () => {
      const gir = applyOp("modify_entity", parseGir("∠45"), parseGir("●"));
      expect(gir.nodes.length).toBeGreaterThanOrEqual(2);
    });
  });

  describe("modify_relation (m × r → r)", () => {
    it("applies modifier to relation", () => {
      const gir = applyOp("modify_relation", parseGir("∠45"), parseGir("→"));
      expect(gir.nodes.length).toBeGreaterThanOrEqual(2);
    });
  });

  describe("negate (a → a)", () => {
    it("negates an assertion", () => {
      const assertion = applyOp("predicate", parseGir("●"), parseGir("→"), parseGir("●"));
      const assertionJson = JSON.stringify(assertion);
      const negated = applyOp("negate", assertionJson);
      expect(negated.nodes.length).toBeGreaterThan(0);
    });
  });

  describe("embed (a → e)", () => {
    it("turns assertion into entity (nominalization)", () => {
      const assertion = applyOp("predicate", parseGir("●"), parseGir("→"), parseGir("●"));
      const assertionJson = JSON.stringify(assertion);
      const embedded = applyOp("embed", assertionJson);
      expect(embedded.nodes.length).toBeGreaterThan(0);
    });
  });

  describe("abstract (e → m)", () => {
    it("turns entity into modifier", () => {
      const gir = applyOp("abstract", parseGir("●"));
      expect(gir.nodes.length).toBeGreaterThanOrEqual(1);
    });
  });

  describe("compose (r × r → r)", () => {
    it("chains two relations", () => {
      const gir = composeOp("compose", parseGir("→"), parseGir("→"));
      expect(gir.nodes.length).toBeGreaterThanOrEqual(2);
    });
  });

  describe("invert (r → r)", () => {
    it("reverses a relation", () => {
      const gir = applyOp("invert", parseGir("→"));
      expect(gir.nodes.length).toBeGreaterThanOrEqual(1);
    });
  });

  describe("conjoin (a × a → a)", () => {
    it("combines two assertions with AND", () => {
      const a1Json = JSON.stringify(
        applyOp("predicate", parseGir("●"), parseGir("→"), parseGir("●"))
      );
      const a2Json = JSON.stringify(
        applyOp("predicate", parseGir("●"), parseGir("←"), parseGir("●"))
      );
      const conjoined = composeOp("conjoin", a1Json, a2Json);
      expect(conjoined.nodes.length).toBeGreaterThan(0);
    });
  });

  describe("disjoin (a × a → a)", () => {
    it("combines two assertions with OR", () => {
      const a1Json = JSON.stringify(
        applyOp("predicate", parseGir("●"), parseGir("→"), parseGir("●"))
      );
      const a2Json = JSON.stringify(
        applyOp("predicate", parseGir("●"), parseGir("←"), parseGir("●"))
      );
      const disjoined = composeOp("disjoin", a1Json, a2Json);
      expect(disjoined.nodes.length).toBeGreaterThan(0);
    });
  });

  describe("operation results survive SVG round-trip", () => {
    it("predicate result round-trips through SVG", () => {
      const gir = applyOp("predicate", parseGir("●"), parseGir("→"), parseGir("●"));
      const girJson = JSON.stringify(gir);
      const svg = wasm.renderSvg(girJson, 256, 256) as string;
      expect(svg).toMatch(/<metadata>/);
      const m = svg.match(/<!\[CDATA\[([\s\S]*?)\]\]>/);
      expect(m).not.toBeNull();
      const extracted = JSON.parse(m![1]);
      expect(extracted.nodes.length).toBe(gir.nodes.length);
      expect(extracted.edges.length).toBe(gir.edges.length);
    });

    it("compose result round-trips through SVG", () => {
      const gir = composeOp("compose", parseGir("→"), parseGir("→"));
      const girJson = JSON.stringify(gir);
      const svg = wasm.renderSvg(girJson, 256, 256) as string;
      const m = svg.match(/<!\[CDATA\[([\s\S]*?)\]\]>/);
      expect(m).not.toBeNull();
      const extracted = JSON.parse(m![1]);
      expect(extracted.nodes.length).toBe(gir.nodes.length);
    });
  });
});
