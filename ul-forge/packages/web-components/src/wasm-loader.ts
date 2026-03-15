/**
 * Lazy WASM loader — ensures the WASM module is initialized exactly once,
 * shared across all web component instances on the page.
 */

type WasmModule = {
  default: (input?: RequestInfo | URL) => Promise<unknown>;
  init: () => void;
  parseUlScript: (input: string) => string;
  validateGir: (gir: string, checkGeometry: boolean) => unknown;
  renderSvg: (gir: string, w: number, h: number) => string;
  renderGlyphPreview: (gir: string) => string;
  queryLexicon: (query: string) => unknown;
  lookupLexiconEntry: (name: string) => unknown;
};

let wasmModule: WasmModule | null = null;
let initPromise: Promise<WasmModule> | null = null;

/**
 * Load and initialize the WASM module. Returns the same promise on
 * subsequent calls (singleton pattern).
 */
export function ensureWasmLoaded(): Promise<WasmModule> {
  if (wasmModule) return Promise.resolve(wasmModule);
  if (initPromise) return initPromise;

  initPromise = (async () => {
    const mod = await import("ul-wasm") as unknown as WasmModule;
    await mod.default();
    mod.init();
    wasmModule = mod;
    return mod;
  })();

  return initPromise;
}

/** Get the loaded WASM module (throws if not yet initialized). */
export function getWasm(): WasmModule {
  if (!wasmModule) throw new Error("WASM not loaded. Await ensureWasmLoaded() first.");
  return wasmModule;
}

/** Check if WASM is ready (non-throwing). */
export function isWasmReady(): boolean {
  return wasmModule !== null;
}
