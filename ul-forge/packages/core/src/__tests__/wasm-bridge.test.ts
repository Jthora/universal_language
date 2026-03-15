import { describe, it, expect, beforeEach } from "vitest";

// The WASM module is replaced by __mocks__/ul_wasm.ts via vitest.config.ts alias.
// No vi.mock needed — the alias intercepts the dynamic import("../wasm/ul_wasm.js").

import {
  initialize,
  _resetForTesting,
  clearCaches,
  parse,
  validate,
  render,
  renderPreview,
  deparse,
  parseAndRender,
  parseValidateRender,
  createContext,
  evaluate,
  layout,
  analyzeStructure,
  queryLexicon,
  lookupLexiconEntry,
} from "../wasm-bridge.js";

const sampleGir = {
  nodes: [{ id: "n1", node_type: "point" as const, label: "existence" }],
  edges: [],
  root: "n1",
} as any;

beforeEach(() => {
  _resetForTesting();
  clearCaches();
});

describe("initialize", () => {
  it("completes without error", async () => {
    await expect(initialize()).resolves.toBeUndefined();
  });

  it("is idempotent — second call is a no-op", async () => {
    await initialize();
    await expect(initialize()).resolves.toBeUndefined();
  });
});

describe("pre-init guard", () => {
  it("parse throws before initialize", () => {
    expect(() => parse("●")).toThrow("WASM not initialized");
  });

  it("validate throws before initialize", () => {
    expect(() => validate(sampleGir)).toThrow("WASM not initialized");
  });

  it("render throws before initialize", () => {
    expect(() => render(sampleGir)).toThrow("WASM not initialized");
  });

  it("deparse throws before initialize", () => {
    expect(() => deparse(sampleGir)).toThrow("WASM not initialized");
  });

  it("createContext throws before initialize", () => {
    expect(() => createContext()).toThrow("WASM not initialized");
  });
});

describe("parse", () => {
  it("returns parsed GIR object with nodes", async () => {
    await initialize();
    const gir = parse("●");
    expect(gir.nodes).toHaveLength(1);
    expect(gir.nodes[0].node_type).toBe("point");
  });
});

describe("validate", () => {
  it("returns validation result with valid flag", async () => {
    await initialize();
    const result = validate(sampleGir, true);
    expect(result).toHaveProperty("valid");
    expect(result.valid).toBe(true);
  });
});

describe("render", () => {
  it("returns SVG string", async () => {
    await initialize();
    const svg = render(sampleGir, 200, 200);
    expect(svg).toContain("<svg");
  });

  it("caches result — same output for same input", async () => {
    await initialize();
    const svg1 = render(sampleGir, 200, 200);
    const svg2 = render(sampleGir, 200, 200);
    expect(svg1).toBe(svg2);
  });
});

describe("renderPreview", () => {
  it("returns compact SVG", async () => {
    await initialize();
    const svg = renderPreview(sampleGir);
    expect(svg).toContain("<svg");
  });
});

describe("deparse", () => {
  it("returns UL-Script string", async () => {
    await initialize();
    const text = deparse(sampleGir);
    expect(typeof text).toBe("string");
    expect(text.length).toBeGreaterThan(0);
  });
});

describe("parseAndRender", () => {
  it("combines parse + render into SVG", async () => {
    await initialize();
    const svg = parseAndRender("●", 128, 128);
    expect(svg).toContain("<svg");
  });
});

describe("parseValidateRender", () => {
  it("returns gir, validation, and svg when valid", async () => {
    await initialize();
    const result = parseValidateRender("●");
    expect(result.gir).toBeDefined();
    expect(result.gir.nodes).toBeDefined();
    expect(result.validation.valid).toBe(true);
    expect(result.svg).toContain("<svg");
  });
});

describe("layout", () => {
  it("returns positioned glyph data", async () => {
    await initialize();
    const result = layout(sampleGir, 800, 600);
    expect(result).toBeDefined();
    expect(result).toHaveProperty("elements");
  });

  it("caches layout results", async () => {
    await initialize();
    const result1 = layout(sampleGir, 800, 600);
    const result2 = layout(sampleGir, 800, 600);
    expect(result1).toBe(result2); // same reference from cache
  });
});

describe("evaluate", () => {
  it("returns evaluation result", async () => {
    await initialize();
    const ctx = createContext();
    const result = evaluate(ctx, sampleGir);
    expect(result).toHaveProperty("grade");
  });
});

describe("analyzeStructure", () => {
  it("returns structure report", async () => {
    await initialize();
    const result = analyzeStructure(sampleGir);
    expect(result).toBeDefined();
  });

  it("caches structure analysis", async () => {
    await initialize();
    const result1 = analyzeStructure(sampleGir);
    const result2 = analyzeStructure(sampleGir);
    expect(result1).toBe(result2); // same reference
  });
});

describe("clearCaches", () => {
  it("does not throw", async () => {
    await initialize();
    render(sampleGir, 200, 200);
    expect(() => clearCaches()).not.toThrow();
  });
});

describe("lexicon", () => {
  it("queryLexicon returns array", async () => {
    await initialize();
    const result = queryLexicon("point");
    expect(Array.isArray(result)).toBe(true);
  });

  it("lookupLexiconEntry returns null for unknown", async () => {
    await initialize();
    const result = lookupLexiconEntry("nonexistent");
    expect(result).toBeNull();
  });
});

describe("_resetForTesting", () => {
  it("makes functions throw again until re-init", async () => {
    await initialize();
    parse("●"); // should work
    _resetForTesting();
    expect(() => parse("●")).toThrow("WASM not initialized");
    await initialize();
    expect(() => parse("●")).not.toThrow();
  });
});
