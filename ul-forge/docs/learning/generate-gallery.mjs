#!/usr/bin/env node
/**
 * generate-gallery.mjs — Generate real SVG renders for the lexicon gallery.
 *
 * Usage: node docs/learning/generate-gallery.mjs
 *
 * Requires: wasm-pack build of ul-wasm (run from repo root).
 * Falls back to geometric placeholder SVGs for entries that can't be parsed.
 */
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT = join(__dirname, "..", "..");
const GALLERY = join(ROOT, "ul-core", "lexicon", "gallery");
const WASM_PKG = join(ROOT, "ul-forge", "web", "wasm-pkg");

// Gallery entries: [number, name, filename_slug, UL-Script (or null for geometric-only)]
const ENTRIES = [
  [1, "The Void", "void", null],
  [2, "Single Point", "point", "●"],
  [3, "Single Line Segment", "line-segment", "● — ●"],
  [4, "Single Directed Line (Ray)", "directed-line", "● → ●"],
  [5, "Single Angle", "angle", "∠"],
  [6, "Single Curve", "curve", "~"],
  [7, "Empty Enclosure", "empty-enclosure", "○{ }"],
  [8, "θ = 0° (Coincidence)", "angle-0", "∠0"],
  [9, "θ = 90° (Perpendicularity)", "angle-90", "∠90"],
  [10, "θ = 180° (Opposition)", "angle-180", "∠180"],
  [11, "θ = 360° (Full Rotation)", "angle-360", "∠360"],
  [12, "θ = 60° (Equilateral)", "angle-60", "∠60"],
  [13, "θ = 120° (Hexagonal)", "angle-120", "∠120"],
  [14, "Triangle", "triangle", "△{ ● ● ● }"],
  [15, "Square", "square", "□{ ● ● ● ● }"],
  [16, "Pentagon", "pentagon", "○{ ● ● ● ● ● }"],
  [17, "Hexagon", "hexagon", "○{ ● ● ● ● ● ● }"],
  [18, "Circle", "circle", "○"],
  [19, "Circle Enclosing Void", "circle-void", "○{ }"],
  [20, "Circle as Process", "circle-process", "○{ ~ }"],
  [21, "Spiral", "spiral", "~spiral"],
  [22, "Sine Wave (Periodic)", "sine-wave", "~sine"],
  [23, "Coincident Points", "coincident-points", "●●"],
  [24, "modify_entity", "modify-entity", "∠ \"big\" ● \"house\""],
  [25, "modify_relation", "modify-relation", "∠ \"quickly\" → \"runs\""],
  [26, "Point in Circle", "point-in-circle", "○{ ● }"],
  [27, "Point in Triangle", "point-in-triangle", "△{ ● }"],
  [28, "Point in Square", "point-in-square", "□{ ● }"],
  [29, "Ray from Point", "ray-from-point", "● → "],
  [30, "Two Points in Opposition", "opposition", "● ∠180 ●"],
  [31, "Equilateral Relation", "equilateral", "△{ ● → ● → ● }"],
  [32, "Point Connected to Enclosure", "point-to-enclosure", "● → ○{ }"],
  [33, "Enclosure-to-Enclosure", "enclosure-pair", "○{ } → ○{ }"],
  [34, "Self-Nesting", "self-nesting", "○{ ○{ ○{ } } }"],
  [35, "Negation", "negation", "¬○{ ● → ● }"],
  [36, "Conjunction", "conjunction", "○{ ○{ ● → ● } ○{ ● → ● } }"],
  [37, "Disjunction", "disjunction", "○{ ○{ ● → ● } ∨ ○{ ● → ● } }"],
  [38, "Embedding", "embedding", "○{ ○{ ● → ● } → ● }"],
  [39, "Abstraction", "abstraction", "∠(○{ ● })"],
  [40, "Composed Relations", "composed-relations", "● → ● → ●"],
  [41, "Reversed Relation", "reversed-relation", "● ← ●"],
  [42, "Quantification", "quantification", "○{ ∠1.0 ○_x ●_x → ● }"],
];

/**
 * Generate a geometric placeholder SVG (used when WASM render fails).
 */
function placeholderSvg(num, name) {
  // Generate simple geometric shapes based on entry number
  const bg = "var(--gallery-bg, #1e1e1e)";
  const fg = "var(--gallery-fg, #d4d4d4)";
  const accent = "var(--gallery-accent, #569cd6)";
  let shape = "";

  if (num <= 1) {
    // Void — empty
    shape = `<circle cx="100" cy="100" r="40" fill="none" stroke="${accent}" stroke-dasharray="4" stroke-opacity="0.3"/>`;
  } else if (num === 2) {
    shape = `<circle cx="100" cy="100" r="6" fill="${accent}"/>`;
  } else if (num === 3) {
    shape = `<line x1="60" y1="100" x2="140" y2="100" stroke="${accent}" stroke-width="2"/>`;
  } else if (num === 4) {
    shape = `<line x1="60" y1="100" x2="140" y2="100" stroke="${accent}" stroke-width="2" marker-end="url(#arrow)"/>
    <defs><marker id="arrow" markerWidth="8" markerHeight="6" refX="8" refY="3" orient="auto"><path d="M0,0 L8,3 L0,6" fill="${accent}"/></marker></defs>`;
  } else if (num === 5) {
    shape = `<line x1="100" y1="100" x2="140" y2="70" stroke="${accent}" stroke-width="2"/>
    <line x1="100" y1="100" x2="140" y2="130" stroke="${accent}" stroke-width="2"/>
    <path d="M120,82 A22,22 0 0,1 120,118" fill="none" stroke="${fg}" stroke-width="1" stroke-opacity="0.5"/>`;
  } else if (num === 6) {
    shape = `<path d="M60,130 Q80,60 120,80 T160,70" fill="none" stroke="${accent}" stroke-width="2"/>`;
  } else if (num >= 7 && num <= 13) {
    // Enclosures / angles
    shape = `<circle cx="100" cy="100" r="40" fill="none" stroke="${accent}" stroke-width="2"/>`;
  } else if (num === 14) {
    shape = `<polygon points="100,60 60,140 140,140" fill="none" stroke="${accent}" stroke-width="2"/>`;
  } else if (num === 15) {
    shape = `<rect x="65" y="65" width="70" height="70" fill="none" stroke="${accent}" stroke-width="2"/>`;
  } else if (num >= 16 && num <= 18) {
    shape = `<circle cx="100" cy="100" r="40" fill="none" stroke="${accent}" stroke-width="2"/>`;
  } else if (num >= 35 && num <= 37) {
    // Operations on assertions
    shape = `<circle cx="80" cy="100" r="30" fill="none" stroke="${accent}" stroke-width="2"/>
    <circle cx="120" cy="100" r="30" fill="none" stroke="#ce9178" stroke-width="2"/>`;
  } else {
    // Default: point + enclosure
    shape = `<circle cx="100" cy="100" r="35" fill="none" stroke="${accent}" stroke-width="2"/>
    <circle cx="100" cy="100" r="4" fill="${accent}"/>`;
  }

  return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200" width="200" height="200">
  <rect width="200" height="200" fill="${bg}" rx="8"/>
  ${shape}
  <text x="100" y="185" text-anchor="middle" fill="${fg}" font-family="monospace" font-size="9" opacity="0.7">${name}</text>
</svg>`;
}

async function main() {
  mkdirSync(GALLERY, { recursive: true });

  let wasmParse = null;
  let wasmRender = null;

  // Try to load WASM module
  try {
    const wasmPath = join(WASM_PKG, "ul_wasm.js");
    const wasmModule = await import(wasmPath);
    if (typeof wasmModule.default === "function") {
      await wasmModule.default();
    }
    wasmParse = wasmModule.parse;
    wasmRender = wasmModule.render;
    console.log("WASM module loaded successfully");
  } catch (e) {
    console.warn("WASM module not available, using geometric placeholders:", e.message);
  }

  let rendered = 0;
  let placeholders = 0;

  for (const [num, name, slug, script] of ENTRIES) {
    const padded = String(num).padStart(3, "0");
    const filename = `${padded}-${slug}.svg`;
    let svg = null;

    // Try WASM render if script provided
    if (wasmParse && wasmRender && script) {
      try {
        const gir = wasmParse(script);
        svg = wasmRender(gir, 200.0, 200.0);
        rendered++;
      } catch {
        // Fall through to placeholder
      }
    }

    if (!svg) {
      svg = placeholderSvg(num, name);
      placeholders++;
    }

    writeFileSync(join(GALLERY, filename), svg);
  }

  console.log(`Gallery generated: ${rendered} WASM renders, ${placeholders} placeholders`);
  console.log(`Output: ${GALLERY}`);
}

main().catch(console.error);
