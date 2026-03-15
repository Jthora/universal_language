//! UL-Script parser: tokenizer, PEG grammar, AST, and AST→GIR transformation.
//!
//! Pipeline: UL-Script text → tokenize → parse (pest) → AST → GIR

mod ast;
mod deparser;
mod grammar;
mod transform;

use crate::error::UlResult;
use crate::types::gir::Gir;

pub use ast::*;

/// Result of parsing UL-Script, including possible warnings.
#[derive(Debug)]
pub struct ParseResult {
    pub gir: Gir,
    pub warnings: Vec<String>,
}

/// Parse UL-Script text into a GIR document.
///
/// This is the main entry point for the parser pipeline:
/// `text → pest parse → AST → GIR`
pub fn parse(input: &str) -> UlResult<Gir> {
    let ast = grammar::parse_to_ast(input)?;
    let gir = transform::ast_to_gir(&ast)?;
    Ok(gir)
}

/// Parse with full diagnostics (warnings + GIR).
pub fn parse_with_diagnostics(input: &str) -> UlResult<ParseResult> {
    let ast = grammar::parse_to_ast(input)?;
    let gir = transform::ast_to_gir(&ast)?;
    Ok(ParseResult {
        gir,
        warnings: Vec::new(),
    })
}

/// Convert a GIR document back to UL-Script text (deparse).
pub fn deparse(gir: &Gir) -> UlResult<String> {
    Ok(deparser::deparse(gir))
}
