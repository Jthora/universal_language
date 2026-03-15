/**
 * <ul-symbol> — Static UL glyph display element.
 *
 * Attributes:
 *   script  — UL-Script text to parse and render (e.g., "point(existence)")
 *   width   — SVG width in pixels (default: 200)
 *   height  — SVG height in pixels (default: 200)
 *   theme   — "light" | "dark" (default: "light")
 *
 * @example
 * <ul-symbol script="point(existence)" width="200" height="200"></ul-symbol>
 * <ul-symbol script="enclosure(circle, point(existence))"></ul-symbol>
 */

import { ensureWasmLoaded, getWasm, isWasmReady } from "./wasm-loader.js";
import { baseStyles } from "./styles.js";

export class ULSymbol extends HTMLElement {
  static observedAttributes = ["script", "width", "height", "theme"];

  private shadow: ShadowRoot;
  private container: HTMLDivElement;

  constructor() {
    super();
    this.shadow = this.attachShadow({ mode: "open" });
    this.shadow.innerHTML = `<style>${baseStyles}</style><div class="container"><span class="loading">Loading…</span></div>`;
    this.container = this.shadow.querySelector(".container")!;
  }

  async connectedCallback(): Promise<void> {
    await ensureWasmLoaded();
    this.render();
  }

  attributeChangedCallback(): void {
    if (isWasmReady()) this.render();
  }

  private render(): void {
    const script = this.getAttribute("script") || "";
    const width = parseFloat(this.getAttribute("width") || "200");
    const height = parseFloat(this.getAttribute("height") || "200");

    if (!script.trim()) {
      this.container.innerHTML = "";
      return;
    }

    try {
      const wasm = getWasm();
      const girJson = wasm.parseUlScript(script);
      const svg = wasm.renderSvg(girJson, width, height);
      this.container.innerHTML = svg;
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      this.container.innerHTML = `<span class="error">${this.escapeHtml(msg)}</span>`;
    }
  }

  private escapeHtml(text: string): string {
    const d = document.createElement("div");
    d.textContent = text;
    return d.innerHTML;
  }
}
