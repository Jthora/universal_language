import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import wasm from "vite-plugin-wasm";
import path from "path";

export default defineConfig({
  plugins: [react(), wasm()],
  resolve: {
    alias: {
      "@ul-forge/core": path.resolve(__dirname, "src/core/index.ts"),
      "ul-wasm": path.resolve(__dirname, "wasm-pkg/ul_wasm.js"),
    },
  },
  optimizeDeps: {
    exclude: ["ul-wasm"],
  },
  server: {
    fs: {
      allow: [".", "./wasm-pkg"],
    },
  },
});
