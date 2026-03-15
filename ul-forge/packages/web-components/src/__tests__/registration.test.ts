import { describe, it, expect, vi } from "vitest";

// Mock the WASM loader — jsdom cannot load actual WASM
vi.mock("../wasm-loader.js", () => ({
  ensureWasmLoaded: vi.fn().mockResolvedValue({}),
  getWasm: vi.fn().mockReturnValue({
    parseUlScript: () => '{"nodes":[],"edges":[]}',
    renderSvg: () => "<svg></svg>",
    renderGlyphPreview: () => "<svg></svg>",
    validateGir: () => ({ valid: true, errors: [] }),
    queryLexicon: () => [],
    lookupLexiconEntry: () => null,
  }),
  isWasmReady: vi.fn().mockReturnValue(true),
}));

describe("Custom element registration", () => {
  it("ULSymbol class extends HTMLElement", async () => {
    const { ULSymbol } = await import("../ul-symbol.js");
    expect(ULSymbol.prototype).toBeInstanceOf(HTMLElement);
  });

  it("ULComposer class extends HTMLElement", async () => {
    const { ULComposer } = await import("../ul-composer.js");
    expect(ULComposer.prototype).toBeInstanceOf(HTMLElement);
  });

  it("ULDictionary class extends HTMLElement", async () => {
    const { ULDictionary } = await import("../ul-dictionary.js");
    expect(ULDictionary.prototype).toBeInstanceOf(HTMLElement);
  });

  it("index.ts registers all three custom elements", async () => {
    await import("../index.js");
    expect(customElements.get("ul-symbol")).toBeDefined();
    expect(customElements.get("ul-composer")).toBeDefined();
    expect(customElements.get("ul-dictionary")).toBeDefined();
  });

  it("ULSymbol has expected observed attributes", async () => {
    const { ULSymbol } = await import("../ul-symbol.js");
    expect(ULSymbol.observedAttributes).toContain("script");
    expect(ULSymbol.observedAttributes).toContain("width");
    expect(ULSymbol.observedAttributes).toContain("height");
  });
});
