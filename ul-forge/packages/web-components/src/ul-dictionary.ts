/**
 * <ul-dictionary> — Searchable reference browser for UL lexicon entries.
 *
 * Attributes:
 *   columns — Number of grid columns (default: 3)
 *   query   — Initial search query
 *
 * @example
 * <ul-dictionary columns="3"></ul-dictionary>
 * <ul-dictionary query="existence"></ul-dictionary>
 */

import { ensureWasmLoaded, getWasm, isWasmReady } from "./wasm-loader.js";
import { dictionaryStyles } from "./styles.js";

interface LexiconEntry {
  id: number;
  level: number;
  name: string;
  tier: string;
  symbol: string;
  sigma_ul: string;
  labels: string[];
}

export class ULDictionary extends HTMLElement {
  static observedAttributes = ["columns", "query"];

  private shadow: ShadowRoot;
  private searchInput!: HTMLInputElement;
  private grid!: HTMLDivElement;
  private debounceTimer: ReturnType<typeof setTimeout> | null = null;

  constructor() {
    super();
    this.shadow = this.attachShadow({ mode: "open" });
  }

  async connectedCallback(): Promise<void> {
    const query = this.getAttribute("query") || "";

    this.shadow.innerHTML = `
      <style>${dictionaryStyles}</style>
      <div class="dictionary">
        <input class="search-input" type="text" placeholder="Search UL lexicon…" value="${this.escapeHtml(query)}" />
        <div class="entries-grid"><span class="loading">Loading…</span></div>
      </div>
    `;

    this.searchInput = this.shadow.querySelector(".search-input")!;
    this.grid = this.shadow.querySelector(".entries-grid")!;

    this.searchInput.addEventListener("input", () => {
      if (this.debounceTimer) clearTimeout(this.debounceTimer);
      this.debounceTimer = setTimeout(() => this.search(), 200);
    });

    this.updateGridColumns();

    await ensureWasmLoaded();
    this.search();
  }

  attributeChangedCallback(name: string): void {
    if (name === "columns") this.updateGridColumns();
    if (name === "query" && this.searchInput && isWasmReady()) {
      this.searchInput.value = this.getAttribute("query") || "";
      this.search();
    }
  }

  private updateGridColumns(): void {
    if (!this.grid) return;
    const cols = parseInt(this.getAttribute("columns") || "3", 10);
    this.grid.style.gridTemplateColumns = `repeat(${Math.max(1, Math.min(cols, 6))}, 1fr)`;
  }

  private search(): void {
    const query = this.searchInput.value.trim();
    const wasm = getWasm();

    try {
      // Empty query → show all entries (query with empty string)
      const entries = wasm.queryLexicon(query || "") as LexiconEntry[];
      this.renderEntries(entries);
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      this.grid.innerHTML = `<span class="error">${this.escapeHtml(msg)}</span>`;
    }
  }

  private renderEntries(entries: LexiconEntry[]): void {
    if (entries.length === 0) {
      this.grid.innerHTML = `<div class="empty-state">No entries found</div>`;
      return;
    }

    const wasm = getWasm();
    const cards = entries.map((entry) => {
      let previewHtml = "";
      try {
        // Parse the symbol text and render a compact preview
        const girJson = wasm.parseUlScript(entry.symbol);
        previewHtml = wasm.renderGlyphPreview(girJson);
      } catch {
        previewHtml = `<span style="font-size:24px;opacity:0.3">?</span>`;
      }

      const labelsHtml = entry.labels
        .map((l) => `<span class="label-tag">${this.escapeHtml(l)}</span>`)
        .join("");

      return `
        <div class="entry-card">
          <div class="entry-preview">${previewHtml}</div>
          <div class="entry-info">
            <div class="entry-name">${this.escapeHtml(entry.name)}</div>
            <div class="entry-tier">${this.escapeHtml(entry.tier)}</div>
            <div class="entry-sigma">${this.escapeHtml(entry.sigma_ul)}</div>
            ${labelsHtml ? `<div class="entry-labels">${labelsHtml}</div>` : ""}
          </div>
        </div>
      `;
    });

    this.grid.innerHTML = cards.join("");
  }

  private escapeHtml(text: string): string {
    const d = document.createElement("div");
    d.textContent = text;
    return d.innerHTML;
  }
}
