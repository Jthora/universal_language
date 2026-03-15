/**
 * <ul-composer> — Interactive UL glyph editor with live preview.
 *
 * Attributes:
 *   width   — Preview width (default: 400)
 *   height  — Preview height (default: 300)
 *   value   — Initial UL-Script text
 *
 * Events:
 *   ul-change — Fired on input change: { detail: { script, gir, valid } }
 *
 * @example
 * <ul-composer width="400" height="300"></ul-composer>
 */

import { ensureWasmLoaded, getWasm, isWasmReady } from "./wasm-loader.js";
import { composerStyles } from "./styles.js";

const PRIMITIVES = [
  { label: "● Point", insert: "point(existence)" },
  { label: "— Line", insert: "line(relation)" },
  { label: "∠ Angle", insert: "angle(quality)" },
  { label: "↝ Curve", insert: "curve(process)" },
  { label: "○ Circle", insert: "enclosure(circle, )" },
  { label: "△ Triangle", insert: "enclosure(triangle, )" },
  { label: "□ Square", insert: "enclosure(square, )" },
];

export class ULComposer extends HTMLElement {
  static observedAttributes = ["width", "height", "value"];

  private shadow: ShadowRoot;
  private textarea!: HTMLTextAreaElement;
  private preview!: HTMLDivElement;
  private statusDot!: HTMLDivElement;
  private statusText!: HTMLSpanElement;
  private debounceTimer: ReturnType<typeof setTimeout> | null = null;

  constructor() {
    super();
    this.shadow = this.attachShadow({ mode: "open" });
  }

  async connectedCallback(): Promise<void> {
    const width = this.getAttribute("width") || "400";
    const height = this.getAttribute("height") || "300";
    const value = this.getAttribute("value") || "";

    this.shadow.innerHTML = `
      <style>${composerStyles}</style>
      <div class="composer">
        <div class="input-area">
          <textarea class="script-input" placeholder="Type UL-Script… e.g. point(existence)" rows="2">${this.escapeHtml(value)}</textarea>
          <div class="palette"></div>
        </div>
        <div class="preview"><span class="loading">Loading WASM…</span></div>
        <div style="display:flex;justify-content:space-between;align-items:center">
          <div class="status">
            <div class="status-dot"></div>
            <span class="status-text"></span>
          </div>
          <div class="toolbar">
            <button class="export-btn" title="Download SVG">Export SVG</button>
          </div>
        </div>
      </div>
    `;

    this.textarea = this.shadow.querySelector(".script-input")!;
    this.preview = this.shadow.querySelector(".preview")!;
    this.statusDot = this.shadow.querySelector(".status-dot")!;
    this.statusText = this.shadow.querySelector(".status-text")!;

    // Primitive palette buttons
    const palette = this.shadow.querySelector(".palette")!;
    for (const p of PRIMITIVES) {
      const btn = document.createElement("button");
      btn.textContent = p.label;
      btn.addEventListener("click", () => this.insertPrimitive(p.insert));
      palette.appendChild(btn);
    }

    // Input handling with debounce
    this.textarea.addEventListener("input", () => {
      if (this.debounceTimer) clearTimeout(this.debounceTimer);
      this.debounceTimer = setTimeout(() => this.renderPreview(), 150);
    });

    // Export button
    this.shadow.querySelector(".export-btn")!.addEventListener("click", () => this.exportSvg());

    // Initialize WASM and render initial value
    await ensureWasmLoaded();
    this.preview.innerHTML = "";
    if (value) this.renderPreview();
  }

  attributeChangedCallback(name: string, _old: string | null, value: string | null): void {
    if (name === "value" && this.textarea && value !== null) {
      this.textarea.value = value;
      if (isWasmReady()) this.renderPreview();
    }
  }

  private insertPrimitive(text: string): void {
    const start = this.textarea.selectionStart;
    const end = this.textarea.selectionEnd;
    const current = this.textarea.value;
    this.textarea.value = current.slice(0, start) + text + current.slice(end);
    this.textarea.selectionStart = this.textarea.selectionEnd = start + text.length;
    this.textarea.focus();
    this.renderPreview();
  }

  private renderPreview(): void {
    const script = this.textarea.value.trim();
    const w = parseFloat(this.getAttribute("width") || "400");
    const h = parseFloat(this.getAttribute("height") || "300");

    if (!script) {
      this.preview.innerHTML = "";
      this.statusDot.className = "status-dot";
      this.statusText.textContent = "";
      return;
    }

    try {
      const wasm = getWasm();
      const girJson = wasm.parseUlScript(script);
      const validation = wasm.validateGir(girJson, false) as {
        valid: boolean;
        errors: string[];
        warnings: string[];
      };

      if (validation.valid) {
        const svg = wasm.renderSvg(girJson, w, h);
        this.preview.innerHTML = svg;
        this.statusDot.className = "status-dot valid";
        this.statusText.textContent = validation.warnings.length
          ? `Valid (${validation.warnings.length} warning${validation.warnings.length > 1 ? "s" : ""})`
          : "Valid";
      } else {
        this.preview.innerHTML = `<span class="error">${this.escapeHtml(validation.errors[0] || "Invalid")}</span>`;
        this.statusDot.className = "status-dot invalid";
        this.statusText.textContent = `${validation.errors.length} error${validation.errors.length > 1 ? "s" : ""}`;
      }

      this.dispatchEvent(
        new CustomEvent("ul-change", {
          detail: { script, gir: girJson, valid: validation.valid },
          bubbles: true,
        }),
      );
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      this.preview.innerHTML = `<span class="error">${this.escapeHtml(msg)}</span>`;
      this.statusDot.className = "status-dot invalid";
      this.statusText.textContent = "Parse error";
    }
  }

  private exportSvg(): void {
    const svgEl = this.preview.querySelector("svg");
    if (!svgEl) return;
    const blob = new Blob([svgEl.outerHTML], { type: "image/svg+xml" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "ul-glyph.svg";
    a.click();
    URL.revokeObjectURL(url);
  }

  private escapeHtml(text: string): string {
    const d = document.createElement("div");
    d.textContent = text;
    return d.innerHTML;
  }
}
