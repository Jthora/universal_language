import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import { resolve } from "path";

export default defineConfig({
  plugins: [wasm()],
  build: {
    lib: {
      entry: resolve(__dirname, "src/index.ts"),
      name: "ULComponents",
      formats: ["es", "iife"],
      fileName: (format) =>
        format === "es" ? "ul-components.esm.js" : "ul-components.iife.js",
    },
    rollupOptions: {
      // WASM is inlined via vite-plugin-wasm
      output: {
        inlineDynamicImports: true,
      },
    },
    target: "es2022",
    minify: "esbuild",
  },
  resolve: {
    alias: {
      "ul-wasm": resolve(__dirname, "../../web/wasm-pkg/ul_wasm.js"),
    },
  },
});
