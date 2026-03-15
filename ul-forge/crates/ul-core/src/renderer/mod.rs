//! GIR renderer: template matching, hierarchical layout, SVG/TikZ generation.
//!
//! Pipeline:
//! 1. Template lookup → known glyph? return pre-computed SVG
//! 2. Layout computation → position all nodes
//! 3. SVG generation → positioned nodes → SVG string

mod layout;
mod svg;
mod templates;

use crate::error::UlResult;
use crate::types::gir::Gir;

pub use layout::Layout;
pub use layout::{compute_layout, Connection, PositionedElement, PositionedGlyph, Shape};

/// Options for rendering.
#[derive(Debug, Clone)]
pub struct RenderOptions {
    /// Output format.
    pub format: OutputFormat,
    /// Canvas width in SVG units.
    pub width: f64,
    /// Canvas height in SVG units.
    pub height: f64,
    /// Whether to embed GIR JSON in SVG metadata.
    pub embed_gir: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        RenderOptions {
            format: OutputFormat::Svg,
            width: 400.0,
            height: 400.0,
            embed_gir: true,
        }
    }
}

/// Supported output formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Svg,
    TikZ,
}

/// Render a GIR document to a visual output string (SVG or TikZ).
pub fn render(gir: &Gir, options: &RenderOptions) -> UlResult<String> {
    // Step 1: Compute layout (template lookup or hierarchical constraint)
    let positioned = layout::compute_layout(gir, options.width, options.height);

    // Step 2: Generate output
    match options.format {
        OutputFormat::Svg => {
            let gir_json = if options.embed_gir {
                gir.to_json().ok()
            } else {
                None
            };
            Ok(svg::generate_svg(
                &positioned,
                gir,
                options,
                gir_json.as_deref(),
            ))
        }
        OutputFormat::TikZ => Ok(svg::generate_tikz(&positioned, gir)),
    }
}
