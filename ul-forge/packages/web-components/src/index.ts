/**
 * @ul-forge/components — Universal Language web components.
 *
 * Custom elements for embedding UL glyphs in any HTML page:
 *   <ul-symbol>     — Display a static glyph
 *   <ul-composer>   — Interactive editor with live preview
 *   <ul-dictionary> — Searchable lexicon browser
 *
 * @example
 * <!-- CDN (no build system needed) -->
 * <script src="https://cdn.jsdelivr.net/npm/@ul-forge/components/dist/ul-components.iife.js"></script>
 *
 * <ul-symbol script="point(existence)" width="200" height="200"></ul-symbol>
 * <ul-composer width="400" height="300"></ul-composer>
 * <ul-dictionary columns="3"></ul-dictionary>
 */

import { ULSymbol } from "./ul-symbol.js";
import { ULComposer } from "./ul-composer.js";
import { ULDictionary } from "./ul-dictionary.js";

// Register custom elements
if (!customElements.get("ul-symbol")) {
  customElements.define("ul-symbol", ULSymbol);
}
if (!customElements.get("ul-composer")) {
  customElements.define("ul-composer", ULComposer);
}
if (!customElements.get("ul-dictionary")) {
  customElements.define("ul-dictionary", ULDictionary);
}

export { ULSymbol, ULComposer, ULDictionary };
