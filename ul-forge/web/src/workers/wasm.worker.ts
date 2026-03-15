/**
 * WASM worker — runs parse/validate/render off the main thread.
 *
 * Messages:
 *   → { type: "init" }               — initialize WASM
 *   ← { type: "ready" }
 *   → { type: "pipeline", id, source } — run full pipeline
 *   ← { type: "result", id, gir, validation, svg, error }
 *   → { type: "deparse", id, gir }   — deparse GIR → text
 *   ← { type: "deparse-result", id, text, error }
 */

import wasmInit, {
  init as wasmModuleInit,
  parseUlScript,
  validateGir,
  renderSvg,
  deparseGir,
} from "ul-wasm";

let initialized = false;

self.onmessage = async (e: MessageEvent) => {
  const msg = e.data;

  if (msg.type === "init") {
    if (!initialized) {
      await wasmInit();
      wasmModuleInit();
      initialized = true;
    }
    self.postMessage({ type: "ready" });
    return;
  }

  if (msg.type === "pipeline") {
    const { id, source } = msg;
    try {
      const girJson = parseUlScript(source) as string;
      const gir = JSON.parse(girJson);
      const validation = validateGir(girJson, false);
      const svg = renderSvg(girJson, 256, 256) as string;
      self.postMessage({ type: "result", id, gir, validation, svg, error: null });
    } catch (err: unknown) {
      const error = err instanceof Error ? err.message : String(err);
      self.postMessage({ type: "result", id, gir: null, validation: null, svg: null, error });
    }
  }

  if (msg.type === "deparse") {
    const { id, gir } = msg;
    try {
      const girJson = JSON.stringify(gir);
      const text = deparseGir(girJson) as string;
      self.postMessage({ type: "deparse-result", id, text, error: null });
    } catch (err: unknown) {
      const error = err instanceof Error ? err.message : String(err);
      self.postMessage({ type: "deparse-result", id, text: null, error });
    }
  }
};
