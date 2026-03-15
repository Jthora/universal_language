//! SVG and TikZ output generation from positioned glyphs.

use std::fmt::Write;

use crate::types::gir::Gir;

use super::layout::{Connection, PositionedElement, PositionedGlyph, Shape};
use super::RenderOptions;

/// Escape a string for safe embedding in SVG/XML attributes and text content.
fn escape_svg(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Generate SVG string from a positioned glyph.
pub fn generate_svg(
    glyph: &PositionedGlyph,
    gir: &Gir,
    options: &RenderOptions,
    gir_json: Option<&str>,
) -> String {
    let w = options.width;
    let h = options.height;
    let label = escape_svg(
        gir.node(&gir.root)
            .and_then(|n| n.label.as_deref())
            .unwrap_or("UL glyph"),
    );

    let mut svg = String::with_capacity(4096);

    // Header
    // Note: all write!/writeln! calls target a String buffer, which is infallible.
    writeln!(svg, r#"<?xml version="1.0" encoding="UTF-8"?>"#)
        .expect("String write is infallible");
    writeln!(
        svg,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" width="{w}" height="{h}">"#
    )
    .expect("String write is infallible");

    // Metadata
    if let Some(json) = gir_json {
        writeln!(svg, "  <metadata>").expect("String write is infallible");
        writeln!(svg, "    <![CDATA[{}]]>", json).expect("String write is infallible");
        writeln!(svg, "  </metadata>").expect("String write is infallible");
    }

    // Defs: styles and arrowhead marker
    writeln!(svg, "  <defs>").expect("String write is infallible");
    writeln!(svg, "    <style>").expect("String write is infallible");
    writeln!(
        svg,
        "      .ul-mark {{ stroke: black; stroke-width: 2; fill: none; }}"
    )
    .expect("String write is infallible");
    writeln!(svg, "      .ul-point {{ fill: black; stroke: none; }}")
        .expect("String write is infallible");
    writeln!(
        svg,
        "      .ul-edge {{ stroke: black; stroke-width: 1.5; fill: none; }}"
    )
    .expect("String write is infallible");
    writeln!(
        svg,
        "      .ul-edge-dashed {{ stroke: black; stroke-width: 1.5; fill: none; stroke-dasharray: 5,3; }}"
    )
    .expect("String write is infallible");
    writeln!(svg, "    </style>").expect("String write is infallible");
    writeln!(
        svg,
        r#"    <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="10" refY="3.5" orient="auto">"#
    )
    .expect("String write is infallible");
    writeln!(
        svg,
        r#"      <polygon points="0 0, 10 3.5, 0 7" fill="black" />"#
    )
    .expect("String write is infallible");
    writeln!(svg, "    </marker>").expect("String write is infallible");
    writeln!(svg, "  </defs>").expect("String write is infallible");

    // Glyph group
    writeln!(
        svg,
        r#"  <g class="ul-glyph" role="img" aria-label="{label}" data-ul-root="{root}">"#,
        root = gir.root
    )
    .expect("String write is infallible");
    writeln!(svg, "    <title>{label}</title>").expect("String write is infallible");

    // Z-order: connections first (behind), then enclosures, lines, angles, points on top
    // Draw connections
    for conn in &glyph.connections {
        render_connection(&mut svg, conn);
    }

    // Draw elements (sorted by z-order)
    let mut sorted: Vec<&PositionedElement> = glyph.elements.iter().collect();
    sorted.sort_by_key(|e| z_order(&e.shape));

    for elem in sorted {
        render_element(&mut svg, elem);
    }

    writeln!(svg, "  </g>").expect("String write is infallible");
    writeln!(svg, "</svg>").expect("String write is infallible");

    svg
}

fn z_order(shape: &Shape) -> u8 {
    match shape {
        Shape::Circle { .. } | Shape::Triangle { .. } | Shape::Square { .. } => 1,
        Shape::Line { .. } | Shape::Curve { .. } => 2,
        Shape::Angle { .. } => 3,
        Shape::Point { .. } => 4,
    }
}

fn render_element(svg: &mut String, elem: &PositionedElement) {
    let id = &elem.node_id;
    match &elem.shape {
        Shape::Point { radius } => {
            writeln!(
                svg,
                r#"    <circle cx="{:.1}" cy="{:.1}" r="{:.1}" class="ul-point" data-ul-node="{id}" />"#,
                elem.x, elem.y, radius
            )
            .expect("String write is infallible");
        }
        Shape::Circle { radius } => {
            writeln!(
                svg,
                r#"    <circle cx="{:.1}" cy="{:.1}" r="{:.1}" class="ul-mark" data-ul-node="{id}" />"#,
                elem.x, elem.y, radius
            )
            .expect("String write is infallible");
        }
        Shape::Triangle { size } => {
            let half = size / 2.0;
            let h = size * 0.866; // sqrt(3)/2
            let x = elem.x;
            let y = elem.y;
            // Equilateral triangle centered at (x, y)
            let top_y = y - h * 2.0 / 3.0;
            let bot_y = y + h / 3.0;
            writeln!(
                svg,
                r#"    <polygon points="{:.1},{:.1} {:.1},{:.1} {:.1},{:.1}" class="ul-mark" data-ul-node="{id}" />"#,
                x, top_y,
                x - half, bot_y,
                x + half, bot_y
            )
            .expect("String write is infallible");
        }
        Shape::Square { size } => {
            let half = size / 2.0;
            writeln!(
                svg,
                r#"    <rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" class="ul-mark" data-ul-node="{id}" />"#,
                elem.x - half,
                elem.y - half,
                size,
                size
            )
            .expect("String write is infallible");
        }
        Shape::Line {
            x1,
            y1,
            x2,
            y2,
            directed,
        } => {
            let marker = if *directed {
                r#" marker-end="url(#arrowhead)""#
            } else {
                ""
            };
            writeln!(
                svg,
                r#"    <line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" class="ul-edge"{marker} data-ul-node="{id}" />"#,
                x1, y1, x2, y2
            )
            .expect("String write is infallible");
        }
        Shape::Angle { radius, degrees } => {
            // Draw an arc from 0° to the angle measure
            let r = *radius;
            let rad = degrees.to_radians();
            let ex = elem.x + r * rad.cos();
            let ey = elem.y - r * rad.sin();
            let large_arc = if *degrees > 180.0 { 1 } else { 0 };
            writeln!(
                svg,
                r#"    <path d="M {:.1},{:.1} A {:.1},{:.1} 0 {large_arc},0 {:.1},{:.1}" class="ul-edge" data-ul-node="{id}" />"#,
                elem.x + r,
                elem.y,
                r, r,
                ex, ey
            )
            .expect("String write is infallible");
            // Degree label
            let label_x = elem.x + r * 1.2 * (rad / 2.0).cos();
            let label_y = elem.y - r * 1.2 * (rad / 2.0).sin();
            writeln!(
                svg,
                r#"    <text x="{:.1}" y="{:.1}" font-size="10" text-anchor="middle">{:.0}°</text>"#,
                label_x, label_y, degrees
            )
            .expect("String write is infallible");
        }
        Shape::Curve {
            x1,
            y1,
            x2,
            y2,
            curvature,
        } => {
            let mid_x = (x1 + x2) / 2.0;
            let dy = (x2 - x1).abs() * curvature;
            writeln!(
                svg,
                r#"    <path d="M {:.1},{:.1} Q {:.1},{:.1} {:.1},{:.1}" class="ul-edge" data-ul-node="{id}" />"#,
                x1, y1,
                mid_x, y1 - dy,
                x2, y2
            )
            .expect("String write is infallible");
        }
    }
}

fn render_connection(svg: &mut String, conn: &Connection) {
    let class = if conn.dashed {
        "ul-edge-dashed"
    } else {
        "ul-edge"
    };
    let marker = if conn.directed {
        r#" marker-end="url(#arrowhead)""#
    } else {
        ""
    };
    writeln!(
        svg,
        r#"    <line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" class="{class}"{marker} />"#,
        conn.x1, conn.y1, conn.x2, conn.y2
    )
    .expect("String write is infallible");
}

/// Generate TikZ output from a positioned glyph.
pub fn generate_tikz(glyph: &PositionedGlyph, _gir: &Gir) -> String {
    let mut out = String::with_capacity(1024);
    writeln!(out, "\\begin{{tikzpicture}}").expect("String write is infallible");

    // Scale: SVG units → TikZ cm (1 SVG unit = 0.02 cm roughly)
    let scale = 0.02;

    for elem in &glyph.elements {
        let x = elem.x * scale;
        let y = -elem.y * scale; // Flip y for TikZ (math coords)
        match &elem.shape {
            Shape::Point { radius } => {
                writeln!(
                    out,
                    "  \\fill ({:.2},{:.2}) circle ({:.3});",
                    x,
                    y,
                    radius * scale
                )
                .expect("String write is infallible");
            }
            Shape::Circle { radius } => {
                writeln!(
                    out,
                    "  \\draw ({:.2},{:.2}) circle ({:.2});",
                    x,
                    y,
                    radius * scale
                )
                .expect("String write is infallible");
            }
            Shape::Triangle { size } => {
                let half = size / 2.0 * scale;
                let h = size * 0.866 * scale;
                writeln!(
                    out,
                    "  \\draw ({:.2},{:.2}) -- ({:.2},{:.2}) -- ({:.2},{:.2}) -- cycle;",
                    x,
                    y + h * 2.0 / 3.0,
                    x - half,
                    y - h / 3.0,
                    x + half,
                    y - h / 3.0
                )
                .expect("String write is infallible");
            }
            Shape::Square { size } => {
                let half = size / 2.0 * scale;
                writeln!(
                    out,
                    "  \\draw ({:.2},{:.2}) rectangle ({:.2},{:.2});",
                    x - half,
                    y - half,
                    x + half,
                    y + half
                )
                .expect("String write is infallible");
            }
            Shape::Line {
                x1,
                y1,
                x2,
                y2,
                directed,
            } => {
                let arrow = if *directed { "->" } else { "-" };
                writeln!(
                    out,
                    "  \\draw[{arrow}] ({:.2},{:.2}) -- ({:.2},{:.2});",
                    x1 * scale,
                    -y1 * scale,
                    x2 * scale,
                    -y2 * scale
                )
                .expect("String write is infallible");
            }
            Shape::Curve { x1, y1, x2, y2, .. } => {
                writeln!(
                    out,
                    "  \\draw ({:.2},{:.2}) .. controls ({:.2},{:.2}) .. ({:.2},{:.2});",
                    x1 * scale,
                    -y1 * scale,
                    (x1 + x2) / 2.0 * scale,
                    (-y1 - 0.5) * scale,
                    x2 * scale,
                    -y2 * scale
                )
                .expect("String write is infallible");
            }
            Shape::Angle { radius, degrees } => {
                writeln!(
                    out,
                    "  \\draw ({:.2},{:.2}) arc (0:{:.0}:{:.2});",
                    x + radius * scale,
                    y,
                    degrees,
                    radius * scale
                )
                .expect("String write is infallible");
            }
        }
    }

    for conn in &glyph.connections {
        let arrow = if conn.directed { "->" } else { "-" };
        writeln!(
            out,
            "  \\draw[{arrow}] ({:.2},{:.2}) -- ({:.2},{:.2});",
            conn.x1 * scale,
            -conn.y1 * scale,
            conn.x2 * scale,
            -conn.y2 * scale
        )
        .expect("String write is infallible");
    }

    writeln!(out, "\\end{{tikzpicture}}").expect("String write is infallible");
    out
}
