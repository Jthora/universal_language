import { defineConfig } from "vitest/config";
import react from "@vitejs/plugin-react";
import path from "path";

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      "@ul-forge/core": path.resolve(__dirname, "src/core/index.ts"),
      "ul-wasm": path.resolve(__dirname, "wasm-pkg/ul_wasm.js"),
    },
  },
  test: {
    environment: "jsdom",
    globals: true,
    setupFiles: ["./src/__tests__/setup.ts"],
    environmentMatchGlobs: [
      ["src/__tests__/wasm-integration.test.ts", "node"],
      ["src/__tests__/composer.test.ts", "node"],
      ["src/__tests__/teaching.test.ts", "node"],
      ["src/__tests__/rendering.test.ts", "node"],
    ],
  },
});
