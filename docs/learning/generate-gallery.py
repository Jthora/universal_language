#!/usr/bin/env python3
"""
Generate SVG gallery from the 42 canonical lexicon entries.

Usage:
    cd ul-forge && maturin develop -m crates/bindings/python/Cargo.toml
    python docs/learning/generate-gallery.py

Outputs: ul-core/lexicon/gallery/*.svg + gallery/index.md
"""

import json
import os
import sys

# Try to import ul_forge; fall back to stub SVGs if unavailable
try:
    import ul_forge
    HAS_FORGE = True
except ImportError:
    HAS_FORGE = False
    print("Warning: ul_forge not available. Generating placeholder SVGs.", file=sys.stderr)


# 42 canonical lexicon entries: (number, slug, name, ul_script)
ENTRIES = [
    (1,  "void",              "The Void",                     ""),
    (2,  "point",             "Single Point",                 "●"),
    (3,  "line-segment",      "Single Line Segment",          "● — ●"),
    (4,  "directed-line",     "Single Directed Line (Ray)",   "● → ●"),
    (5,  "angle",             "Single Angle",                 "@45"),
    (6,  "curve",             "Single Curve",                 "● ~ ●"),
    (7,  "empty-enclosure",   "Empty Enclosure",              "○{ }"),
    (8,  "angle-0",           "θ = 0° (Coincidence)",         "@0"),
    (9,  "angle-90",          "θ = 90° (Perpendicularity)",   "@90"),
    (10, "angle-180",         "θ = 180° (Opposition)",        "@180"),
    (11, "angle-360",         "θ = 360° (Full Rotation)",     "@360"),
    (12, "angle-60",          "θ = 60° (Equilateral)",        "@60"),
    (13, "angle-120",         "θ = 120° (Hexagonal)",         "@120"),
    (14, "triangle",          "Triangle",                     "△{ }"),
    (15, "square",            "Square",                       "□{ }"),
    (16, "pentagon",          "Pentagon",                     "⬠{ }"),
    (17, "hexagon",           "Hexagon",                     "⬡{ }"),
    (18, "circle",            "Circle",                       "○{ }"),
    (19, "circle-void",       "Circle Enclosing Void",        "○{ Ø }"),
    (20, "circle-process",    "Circle as Process",            "○"),
    (21, "spiral",            "Spiral",                       "● ~↑ ●"),
    (22, "sine-wave",         "Sine Wave (Periodic)",         "● ~ ●"),
    (23, "coincident-points", "Coincident Points",            "● = ●"),
    (24, "modify-entity",     "modify_entity",                "@45 ●"),
    (25, "modify-relation",   "modify_relation",              "@45 →"),
    (26, "point-in-circle",   "Point in Circle",              "○{ ● }"),
    (27, "point-in-triangle", "Point in Triangle",            "△{ ● }"),
    (28, "point-in-square",   "Point in Square",              "□{ ● }"),
    (29, "ray-from-point",    "Ray from Point",               "● →"),
    (30, "opposition",        "Two Points in Opposition",     "● @180 ●"),
    (31, "equilateral",       "Equilateral Relation",         "△{ ● ● ● }"),
    (32, "point-to-enclosure","Point Connected to Enclosure", "● → ○{ }"),
    (33, "enclosure-pair",    "Enclosure-to-Enclosure",       "○{ } → ○{ }"),
    (34, "self-nesting",      "Self-Nesting",                 "○{ ○{ ○{ } } }"),
    (35, "negation",          "Negation",                     "¬○{ ● → ● }"),
    (36, "conjunction",       "Conjunction",                  "○{ ● → ● } ∧ ○{ ● → ● }"),
    (37, "disjunction",       "Disjunction",                 "○{ ● → ● } ∨ ○{ ● → ● }"),
    (38, "embedding",         "Embedding",                    "○{ ○{ ● → ● } }"),
    (39, "abstraction",       "Abstraction",                  "@○{ ● }"),
    (40, "composed-relations","Composed Relations",           "● → ● → ●"),
    (41, "reversed-relation", "Reversed Relation",            "● ← ●"),
    (42, "quantification",    "Quantification",               "@0.5 ○{ ● }"),
]


def placeholder_svg(name: str, num: int) -> str:
    """Generate a placeholder SVG with the entry name."""
    return f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200" width="200" height="200">
  <rect width="200" height="200" fill="#1e1e1e" rx="8"/>
  <text x="100" y="90" text-anchor="middle" fill="#569cd6" font-family="monospace" font-size="14">#{num:02d}</text>
  <text x="100" y="120" text-anchor="middle" fill="#d4d4d4" font-family="monospace" font-size="11">{name}</text>
  <text x="100" y="150" text-anchor="middle" fill="#888" font-family="monospace" font-size="9">[render pending]</text>
</svg>'''


def render_entry(num: int, slug: str, name: str, ul_script: str) -> str:
    """Render a lexicon entry to SVG."""
    if not ul_script or not HAS_FORGE:
        return placeholder_svg(name, num)
    try:
        gir = ul_forge.parse(ul_script)
        svg = ul_forge.render(json.dumps(gir))
        return svg
    except Exception as e:
        print(f"  Warning: failed to render #{num} ({name}): {e}", file=sys.stderr)
        return placeholder_svg(name, num)


def main():
    gallery_dir = os.path.join(os.path.dirname(__file__), "../../ul-core/lexicon/gallery")
    os.makedirs(gallery_dir, exist_ok=True)

    # Generate SVGs
    for num, slug, name, ul_script in ENTRIES:
        svg = render_entry(num, slug, name, ul_script)
        filename = f"{num:03d}-{slug}.svg"
        filepath = os.path.join(gallery_dir, filename)
        with open(filepath, "w") as f:
            f.write(svg)
        print(f"  ✓ {filename}")

    # Generate index
    index_path = os.path.join(gallery_dir, "index.md")
    with open(index_path, "w") as f:
        f.write("# Lexicon Gallery — 42 Canonical Entries\n\n")
        f.write("> Auto-generated by `docs/learning/generate-gallery.py`\n\n")
        f.write("| # | Name | Glyph | Tier |\n")
        f.write("|---|------|-------|------|\n")
        for num, slug, name, ul_script in ENTRIES:
            filename = f"{num:03d}-{slug}.svg"
            f.write(f"| {num} | {name} | ![{name}]({filename}) | — |\n")

    print(f"\n✓ Generated {len(ENTRIES)} SVGs + index.md in {gallery_dir}")


if __name__ == "__main__":
    main()
