import { defineConfig } from "vitest/config";
import { resolve } from "path";

export default defineConfig({
  test: {
    alias: {
      // Intercept the dynamic import("../wasm/ul_wasm.js") used by wasm-bridge.ts
      // and replace it at test-time with our mock.
      "../wasm/ul_wasm.js": resolve(__dirname, "src/__tests__/__mocks__/ul_wasm.ts"),
    },
  },
});
