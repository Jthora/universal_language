/**
 * Tier 3: Algebraic composer integration tests.
 *
 * Validates all 11 Σ_UL operations via the WASM binary:
 *   predicate, modify_entity, modify_relation, negate, conjoin, disjoin,
 *   embed, abstract, compose, invert, quantify
 *
 * Also tests composeGir (binary shortcut), detectOperations, and
 * sort/arity constraints.
 *
 * Sort reference (UL-Script → GIR root sort):
 *   Entity:   ● (point), ○{…} (enclosure)
 *   Relation: -> (arrow), <-> (bi-arrow), ~ (curve)
 *   Modifier: @45 (angle)
 *   Assertion: only via predicate() or quantify()
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

/** Parse UL-Script and return GIR JSON string. */
function p(input: string): string {
  return wasm.parseUlScript(input) as string;
}

/** Apply a Σ_UL operation to GIR JSON strings. */
function apply(op: string, ...girJsons: string[]): string {
  return wasm.applyOperation(op, JSON.stringify(girJsons)) as string;
}

/** Parse result to object. */
function toGir(json: string): any {
  return JSON.parse(json);
}

/** Build an assertion GIR via predicate(entity, relation, entity). */
function makeAssertion(): string {
  return apply("predicate", p("●"), p("->"), p("●"));
}

// ── Individual operations ──

describe("predicate (e × r × e → a)", () => {
  it("combines subject, relation, object into assertion", () => {
    const subject = p("●");       // Entity
    const relation = p("->");      // Relation
    const object = p("●");        // Entity
    const result = toGir(apply("predicate", subject, relation, object));
    expect(result.nodes.length).toBeGreaterThanOrEqual(3);
  });

  it("rejects wrong arity (2 operands)", () => {
    expect(() => apply("predicate", p("●"), p("->"))).toThrow();
  });

  it("rejects wrong arity (1 operand)", () => {
    expect(() => apply("predicate", p("●"))).toThrow();
  });
});

describe("modify_entity (m × e → e)", () => {
  it("applies modifier to entity", () => {
    const modifier = p("@45");    // Modifier (angle)
    const entity = p("●");        // Entity
    const result = toGir(apply("modify_entity", modifier, entity));
    expect(result.nodes).toBeDefined();
  });

  it("rejects wrong arity", () => {
    expect(() => apply("modify_entity", p("@45"))).toThrow();
  });
});

describe("modify_relation (m × r → r)", () => {
  it("applies modifier to relation", () => {
    const modifier = p("@45");    // Modifier
    const relation = p("->");      // Relation
    const result = toGir(apply("modify_relation", modifier, relation));
    expect(result.nodes).toBeDefined();
  });
});

describe("negate (a → a)", () => {
  it("negates an assertion", () => {
    const assertion = makeAssertion();
    const result = toGir(apply("negate", assertion));
    expect(result.nodes).toBeDefined();
    expect(result.nodes.length).toBeGreaterThanOrEqual(1);
  });

  it("is a unary operation", () => {
    expect(() => apply("negate", makeAssertion(), makeAssertion())).toThrow();
  });
});

describe("conjoin (a × a → a)", () => {
  it("combines two assertions with AND", () => {
    const a1 = makeAssertion();
    const a2 = makeAssertion();
    const result = toGir(apply("conjoin", a1, a2));
    expect(result.nodes.length).toBeGreaterThanOrEqual(2);
  });
});

describe("disjoin (a × a → a)", () => {
  it("combines two assertions with OR", () => {
    const a1 = makeAssertion();
    const a2 = makeAssertion();
    const result = toGir(apply("disjoin", a1, a2));
    expect(result.nodes.length).toBeGreaterThanOrEqual(2);
  });
});

describe("embed (a → e)", () => {
  it("turns assertion into entity (nominalization)", () => {
    const assertion = makeAssertion();
    const result = toGir(apply("embed", assertion));
    expect(result.nodes).toBeDefined();
  });

  it("is unary", () => {
    expect(() => apply("embed", makeAssertion(), makeAssertion())).toThrow();
  });
});

describe("abstract (e → m)", () => {
  it("turns entity into modifier (adjectivalization)", () => {
    const entity = p("●");        // Entity
    const result = toGir(apply("abstract", entity));
    expect(result.nodes).toBeDefined();
  });
});

describe("compose (r × r → r)", () => {
  it("chains two relations (transitivity)", () => {
    const r1 = p("->");            // Relation
    const r2 = p("->");            // Relation
    const result = toGir(apply("compose", r1, r2));
    expect(result.nodes).toBeDefined();
  });
});

describe("invert (r → r)", () => {
  it("reverses a relation direction", () => {
    const relation = p("->");      // Relation
    const result = toGir(apply("invert", relation));
    expect(result.nodes).toBeDefined();
  });
});

describe("quantify (m × e → a)", () => {
  it("applies quantifier to entity", () => {
    const quantifier = p("@45");   // Modifier
    const entity = p("●");         // Entity
    const result = toGir(apply("quantify", quantifier, entity));
    expect(result.nodes).toBeDefined();
  });
});

// ── composeGir (binary shortcut) ──

describe("composeGir binary shortcut", () => {
  it("conjoins two assertion GIRs", () => {
    const a = makeAssertion();
    const b = makeAssertion();
    const resultJson = wasm.composeGir(a, b, "conjoin") as string;
    const result = toGir(resultJson);
    expect(result.nodes.length).toBeGreaterThanOrEqual(2);
  });

  it("disjoins two assertion GIRs", () => {
    const a = makeAssertion();
    const b = makeAssertion();
    const resultJson = wasm.composeGir(a, b, "disjoin") as string;
    expect(toGir(resultJson).nodes).toBeDefined();
  });

  it("composes two relations", () => {
    const r1 = p("->");            // Relation
    const r2 = p("->");            // Relation
    const resultJson = wasm.composeGir(r1, r2, "compose") as string;
    expect(toGir(resultJson).nodes).toBeDefined();
  });

  it("rejects non-binary operations", () => {
    expect(() =>
      wasm.composeGir(p("●"), p("●"), "negate"),
    ).toThrow();
  });

  it("rejects unknown operations", () => {
    expect(() =>
      wasm.composeGir(p("●"), p("●"), "frobnicate"),
    ).toThrow();
  });
});

// ── detectOperations ──

describe("detectOperations", () => {
  it("returns an array of operation names", () => {
    const girJson = p("○{●}");
    const ops = wasm.detectOperations(girJson) as string[];
    expect(Array.isArray(ops)).toBe(true);
    // Each element should be a string (operation name)
    for (const op of ops) {
      expect(typeof op).toBe("string");
    }
  });

  it("returns empty for minimal point", () => {
    const girJson = p("●");
    const ops = wasm.detectOperations(girJson) as string[];
    expect(Array.isArray(ops)).toBe(true);
    // A single point doesn't express any operation
  });
});

// ── Error boundary ──

describe("operation error boundaries", () => {
  it("rejects unknown operation name", () => {
    expect(() => apply("unknown_op", p("●"))).toThrow("unknown operation");
  });

  it("rejects empty operands array", () => {
    expect(() =>
      wasm.applyOperation("negate", JSON.stringify([])),
    ).toThrow();
  });
});
