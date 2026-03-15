/**
 * Tier 6: Worker message protocol tests.
 *
 * Tests the WASM worker's message handling (init, pipeline, deparse)
 * using mocked WASM bindings in jsdom environment.
 */

import { describe, it, expect, beforeEach, vi } from "vitest";

vi.mock("ul-wasm", async () => {
  const mod = await import("./helpers/wasm-mock");
  return mod.installWasmMock();
});

// Import the mock to verify calls
import { wasmMock, resetWasmMock } from "./helpers/wasm-mock";

// Simulate the worker's onmessage handler inline.
// We can't directly import a Web Worker in vitest/jsdom, so we
// replicate the handler logic and test its message protocol.

let initialized = false;
const messages: any[] = [];

function postMessage(msg: any) {
  messages.push(msg);
}

async function handleMessage(msg: any) {
  if (msg.type === "init") {
    if (!initialized) {
      await wasmMock.default();
      wasmMock.init();
      initialized = true;
    }
    postMessage({ type: "ready" });
    return;
  }

  if (msg.type === "pipeline") {
    const { id, source } = msg;
    try {
      const girJson = wasmMock.parseUlScript(source) as string;
      const gir = JSON.parse(girJson);
      const validation = wasmMock.validateGir(girJson, false);
      const svg = wasmMock.renderSvg(girJson, 256, 256);
      postMessage({ type: "result", id, gir, validation, svg, error: null });
    } catch (err: unknown) {
      const error = err instanceof Error ? err.message : String(err);
      postMessage({
        type: "result",
        id,
        gir: null,
        validation: null,
        svg: null,
        error,
      });
    }
  }

  if (msg.type === "deparse") {
    const { id, gir } = msg;
    try {
      const girJson = JSON.stringify(gir);
      const text = wasmMock.deparseGir(girJson);
      postMessage({ type: "deparse-result", id, text, error: null });
    } catch (err: unknown) {
      const error = err instanceof Error ? err.message : String(err);
      postMessage({ type: "deparse-result", id, text: null, error });
    }
  }
}

beforeEach(() => {
  resetWasmMock();
  initialized = false;
  messages.length = 0;
});

describe("worker init protocol", () => {
  it("sends ready after init", async () => {
    await handleMessage({ type: "init" });
    expect(messages).toEqual([{ type: "ready" }]);
    expect(wasmMock.default).toHaveBeenCalledTimes(1);
    expect(wasmMock.init).toHaveBeenCalledTimes(1);
  });

  it("is idempotent (second init is no-op)", async () => {
    await handleMessage({ type: "init" });
    await handleMessage({ type: "init" });
    expect(messages).toHaveLength(2);
    expect(wasmMock.default).toHaveBeenCalledTimes(1);
  });
});

describe("worker pipeline protocol", () => {
  it("returns result for valid source", async () => {
    await handleMessage({ type: "init" });
    messages.length = 0;

    await handleMessage({ type: "pipeline", id: "test-1", source: "●" });
    expect(messages).toHaveLength(1);
    const msg = messages[0];
    expect(msg.type).toBe("result");
    expect(msg.id).toBe("test-1");
    expect(msg.gir).toBeDefined();
    expect(msg.validation).toBeDefined();
    expect(msg.svg).toBeDefined();
    expect(msg.error).toBeNull();
  });

  it("returns error for parse failure", async () => {
    await handleMessage({ type: "init" });
    messages.length = 0;

    wasmMock.parseUlScript.mockImplementationOnce(() => {
      throw new Error("syntax error");
    });

    await handleMessage({ type: "pipeline", id: "test-2", source: "{{{{" });
    const msg = messages[0];
    expect(msg.type).toBe("result");
    expect(msg.id).toBe("test-2");
    expect(msg.error).toBe("syntax error");
    expect(msg.gir).toBeNull();
  });

  it("preserves message id through pipeline", async () => {
    await handleMessage({ type: "init" });
    messages.length = 0;

    await handleMessage({ type: "pipeline", id: "msg-42", source: "●" });
    expect(messages[0].id).toBe("msg-42");
  });
});

describe("worker deparse protocol", () => {
  it("returns deparsed text", async () => {
    await handleMessage({ type: "init" });
    messages.length = 0;

    const gir = { ul_gir: "0.2", root: "p", nodes: [], edges: [] };
    await handleMessage({ type: "deparse", id: "dep-1", gir });
    const msg = messages[0];
    expect(msg.type).toBe("deparse-result");
    expect(msg.id).toBe("dep-1");
    expect(msg.text).toBe("●");
    expect(msg.error).toBeNull();
  });

  it("returns error on deparse failure", async () => {
    await handleMessage({ type: "init" });
    messages.length = 0;

    wasmMock.deparseGir.mockImplementationOnce(() => {
      throw new Error("deparse failed");
    });

    await handleMessage({ type: "deparse", id: "dep-2", gir: {} });
    const msg = messages[0];
    expect(msg.type).toBe("deparse-result");
    expect(msg.error).toBe("deparse failed");
    expect(msg.text).toBeNull();
  });
});
