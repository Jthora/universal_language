/**
 * WASM worker — runs parse/validate/render off the main thread.
 *
 * Messages:
 *   → { type: "init" }               — initialize WASM
 *   ← { type: "ready" }
 *   → { type: "pipeline", id, source } — run full pipeline
 *   ← { type: "result", id, gir, validation, svg, error }
 */

import init, { parse, validate, render, deparse } from "ul-wasm";

let initialized = false;

self.onmessage = async (e: MessageEvent) => {
  const msg = e.data;

  if (msg.type === "init") {
    if (!initialized) {
      await init();
      initialized = true;
    }
    self.postMessage({ type: "ready" });
    return;
  }

  if (msg.type === "pipeline") {
    const { id, source } = msg;
    try {
      const gir = parse(source);
      const girJson = JSON.stringify(gir);
      const validation = validate(girJson);
      const svg = render(girJson);
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
      const text = deparse(girJson);
      self.postMessage({ type: "deparse-result", id, text, error: null });
    } catch (err: unknown) {
      const error = err instanceof Error ? err.message : String(err);
      self.postMessage({ type: "deparse-result", id, text: null, error });
    }
  }
};
