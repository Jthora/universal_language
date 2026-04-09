//! Pest grammar → AST bridge.
//!
//! Uses `pest_derive` to generate a parser from `ul_script.pest`,
//! then walks the pest parse tree to build our typed AST.

use pest::Parser;
use pest_derive::Parser;

use crate::error::UlError;

use super::ast::*;

/// Extract the next pair from an iterator, returning a parse error if absent.
fn expect_next<'a>(
    pairs: &mut pest::iterators::Pairs<'a, Rule>,
    context: &str,
) -> Result<pest::iterators::Pair<'a, Rule>, UlError> {
    pairs.next().ok_or_else(|| UlError::Parse {
        line: 0,
        column: 0,
        message: format!("expected {context}"),
    })
}

/// Extract the next inner pair from a pair, returning a parse error if absent.
fn expect_inner<'a>(
    pair: pest::iterators::Pair<'a, Rule>,
    context: &str,
) -> Result<pest::iterators::Pair<'a, Rule>, UlError> {
    pair.into_inner().next().ok_or_else(|| UlError::Parse {
        line: 0,
        column: 0,
        message: format!("expected {context}"),
    })
}

#[derive(Parser)]
#[grammar = "parser/ul_script.pest"]
struct UlScriptParser;

/// Parse UL-Script text into an AST document.
pub fn parse_to_ast(input: &str) -> Result<AstDocument, UlError> {
    let pairs = UlScriptParser::parse(Rule::Document, input).map_err(|e| {
        let (line, col) = match e.line_col {
            pest::error::LineColLocation::Pos((l, c)) => (l, c),
            pest::error::LineColLocation::Span((l, c), _) => (l, c),
        };
        UlError::Parse {
            line,
            column: col,
            message: e.variant.message().to_string(),
        }
    })?;

    let mut glyphs = Vec::new();

    for pair in pairs {
        if pair.as_rule() == Rule::Document {
            for inner in pair.into_inner() {
                match inner.as_rule() {
                    Rule::Glyph => {
                        if let Some(glyph) = parse_glyph(inner)? {
                            glyphs.push(glyph);
                        }
                    }
                    Rule::EOI => {}
                    _ => {}
                }
            }
        }
    }

    Ok(AstDocument { glyphs })
}

fn parse_glyph(pair: pest::iterators::Pair<Rule>) -> Result<Option<AstGlyph>, UlError> {
    let inner = expect_inner(pair, "glyph content")?;
    match inner.as_rule() {
        Rule::Comment => {
            let text = inner.as_str().trim_start_matches('#').trim().to_string();
            Ok(Some(AstGlyph::Comment(text)))
        }
        Rule::Composition => {
            let comp = parse_composition(inner)?;
            Ok(Some(AstGlyph::Composition(comp)))
        }
        _ => Ok(None),
    }
}

fn parse_composition(pair: pest::iterators::Pair<Rule>) -> Result<AstComposition, UlError> {
    let mut inner = pair.into_inner();

    let head_pair = expect_next(&mut inner, "composition head term")?;
    let head = Box::new(parse_term(head_pair)?);

    let mut tail = Vec::new();
    while let Some(op_pair) = inner.next() {
        let operator = parse_operator(op_pair)?;
        let term_pair = expect_next(&mut inner, "term after operator")?;
        let term = parse_term(term_pair)?;
        tail.push((operator, term));
    }

    Ok(AstComposition { head, tail })
}

fn parse_term(pair: pest::iterators::Pair<Rule>) -> Result<AstTerm, UlError> {
    let inner = expect_inner(pair, "term content")?;
    match inner.as_rule() {
        Rule::Mark => parse_mark(inner),
        Rule::Group => {
            let comp_pair = expect_inner(inner, "group composition")?;
            let comp = parse_composition(comp_pair)?;
            Ok(AstTerm::Group(comp))
        }
        rule => Err(UlError::Parse {
            line: 0,
            column: 0,
            message: format!("unexpected term rule: {rule:?}"),
        }),
    }
}

fn parse_mark(pair: pest::iterators::Pair<Rule>) -> Result<AstTerm, UlError> {
    let mut inner = pair.into_inner();

    let first = expect_next(&mut inner, "mark content")?;

    // ModalOperator branch: []{...}, <>{...}, []->{...}{...}
    if first.as_rule() == Rule::ModalOperator {
        let mod_inner = expect_inner(first, "modal operator type")?;
        match mod_inner.as_rule() {
            Rule::Necessity => {
                let content_pair = expect_inner(mod_inner, "necessity content")?;
                let comp_pair = expect_inner(content_pair, "content composition")?;
                let content = parse_composition(comp_pair)?;
                return Ok(AstTerm::ModalUnary {
                    kind: AstModalKind::Necessity,
                    content,
                });
            }
            Rule::Possibility => {
                let content_pair = expect_inner(mod_inner, "possibility content")?;
                let comp_pair = expect_inner(content_pair, "content composition")?;
                let content = parse_composition(comp_pair)?;
                return Ok(AstTerm::ModalUnary {
                    kind: AstModalKind::Possibility,
                    content,
                });
            }
            Rule::Counterfactual => {
                let mut cf_inner = mod_inner.into_inner();
                let ante_content = expect_next(&mut cf_inner, "counterfactual antecedent")?;
                let ante_comp = expect_inner(ante_content, "antecedent composition")?;
                let antecedent = parse_composition(ante_comp)?;
                let cons_content = expect_next(&mut cf_inner, "counterfactual consequent")?;
                let cons_comp = expect_inner(cons_content, "consequent composition")?;
                let consequent = parse_composition(cons_comp)?;
                return Ok(AstTerm::ModalCounterfactual {
                    antecedent,
                    consequent,
                });
            }
            rule => {
                return Err(UlError::Parse {
                    line: 0,
                    column: 0,
                    message: format!("unexpected modal operator: {rule:?}"),
                });
            }
        }
    }

    // ForceAnnotation branch: assert{...}, query{...}, etc.
    if first.as_rule() == Rule::ForceAnnotation {
        let mut fa_inner = first.into_inner();
        let token_pair = expect_next(&mut fa_inner, "force token")?;
        let force = match token_pair.as_str() {
            "assert" => AstForceKind::Assert,
            "query" => AstForceKind::Query,
            "direct" => AstForceKind::Direct,
            "commit" => AstForceKind::Commit,
            "express" => AstForceKind::Express,
            "declare" => AstForceKind::Declare,
            other => {
                return Err(UlError::Parse {
                    line: 0,
                    column: 0,
                    message: format!("unknown force token: {other}"),
                });
            }
        };
        let content_pair = expect_next(&mut fa_inner, "force content")?;
        let comp_pair = expect_inner(content_pair, "content composition")?;
        let content = parse_composition(comp_pair)?;
        return Ok(AstTerm::ForceAnnotation { force, content });
    }

    // AssertionModifier branch: ?{...}, !{...}, ~?{...}
    if first.as_rule() == Rule::AssertionModifier {
        let mod_inner = expect_inner(first, "assertion modifier type")?;
        let (kind, mod_pair) = match mod_inner.as_rule() {
            Rule::EvidentialMark => (AstAssertionModifierKind::Evidential, mod_inner),
            Rule::EmphaticMark => (AstAssertionModifierKind::Emphatic, mod_inner),
            Rule::HedgedMark => (AstAssertionModifierKind::Hedged, mod_inner),
            rule => {
                return Err(UlError::Parse {
                    line: 0,
                    column: 0,
                    message: format!("unexpected assertion modifier: {rule:?}"),
                })
            }
        };
        // Content is the first inner child of the specific modifier rule
        let content_pair = expect_inner(mod_pair, "assertion modifier content")?;
        let comp_pair = expect_inner(content_pair, "content composition")?;
        let content = parse_composition(comp_pair)?;
        return Ok(AstTerm::AssertionModifier { kind, content });
    }

    // Primitive branch
    let primitive = parse_primitive(first)?;

    let content = if let Some(content_pair) = inner.next() {
        let comp_pair = expect_inner(content_pair, "content composition")?;
        Some(parse_composition(comp_pair)?)
    } else {
        None
    };

    Ok(AstTerm::Mark(AstMark { primitive, content }))
}

fn parse_primitive(pair: pest::iterators::Pair<Rule>) -> Result<AstPrimitive, UlError> {
    let inner = expect_inner(pair, "primitive type")?;
    match inner.as_rule() {
        Rule::VariableSlot => {
            let id_pair = expect_inner(inner, "variable slot identifier")?;
            Ok(AstPrimitive::VariableSlot(id_pair.as_str().to_string()))
        }
        Rule::BoundRef => {
            let id_pair = expect_inner(inner, "bound reference identifier")?;
            Ok(AstPrimitive::BoundRef(id_pair.as_str().to_string()))
        }
        Rule::Point | Rule::FilledPoint => Ok(AstPrimitive::Point),
        Rule::Enclosure => {
            let shape = expect_inner(inner, "enclosure shape")?;
            match shape.as_rule() {
                Rule::Circle => Ok(AstPrimitive::Circle),
                Rule::Triangle => Ok(AstPrimitive::Triangle),
                Rule::Square => Ok(AstPrimitive::Square),
                Rule::Pentagon => Ok(AstPrimitive::Pentagon),
                Rule::Hexagon => Ok(AstPrimitive::Hexagon),
                rule => Err(UlError::Parse {
                    line: 0,
                    column: 0,
                    message: format!("unexpected enclosure shape: {rule:?}"),
                }),
            }
        }
        Rule::Arrow => parse_arrow(inner),
        Rule::Curve => Ok(AstPrimitive::Curve),
        Rule::AngleMod => {
            let num_pair = expect_inner(inner, "angle value")?;
            let num = num_pair.as_str().parse::<f64>().map_err(|e| UlError::Parse {
                line: 0,
                column: 0,
                message: format!("invalid angle number: {e}"),
            })?;
            Ok(AstPrimitive::Angle(num))
        }
        rule => Err(UlError::Parse {
            line: 0,
            column: 0,
            message: format!("unexpected primitive: {rule:?}"),
        }),
    }
}

fn parse_arrow(pair: pest::iterators::Pair<Rule>) -> Result<AstPrimitive, UlError> {
    let inner = expect_inner(pair, "arrow direction")?;
    match inner.as_rule() {
        Rule::RightArrow => Ok(AstPrimitive::RightArrow),
        Rule::LeftArrow => Ok(AstPrimitive::LeftArrow),
        Rule::BiArrow => Ok(AstPrimitive::BiArrow),
        rule => Err(UlError::Parse {
            line: 0,
            column: 0,
            message: format!("unexpected arrow type: {rule:?}"),
        }),
    }
}

fn parse_operator(pair: pest::iterators::Pair<Rule>) -> Result<AstOperator, UlError> {
    let inner = expect_inner(pair, "operator type")?;
    match inner.as_rule() {
        Rule::Connection => {
            let mut conn_inner = inner.into_inner();
            let arrow_pair = expect_next(&mut conn_inner, "connection arrow")?;
            let arrow_inner = expect_inner(arrow_pair, "arrow direction")?;
            let direction = match arrow_inner.as_rule() {
                Rule::RightArrow => AstDirection::Right,
                Rule::LeftArrow => AstDirection::Left,
                Rule::BiArrow => AstDirection::Both,
                rule => {
                    return Err(UlError::Parse {
                        line: 0,
                        column: 0,
                        message: format!("unexpected arrow direction: {rule:?}"),
                    })
                }
            };

            let angle = if let Some(angle_pair) = conn_inner.next() {
                let num_pair = expect_inner(angle_pair, "angle modifier value")?;
                let num = num_pair.as_str().parse::<f64>().map_err(|e| UlError::Parse {
                    line: 0,
                    column: 0,
                    message: format!("invalid angle number: {e}"),
                })?;
                Some(num)
            } else {
                None
            };

            Ok(AstOperator::Connection { direction, angle })
        }
        Rule::Adjacency => Ok(AstOperator::Adjacency),
        Rule::Intersection => Ok(AstOperator::Intersection),
        rule => Err(UlError::Parse {
            line: 0,
            column: 0,
            message: format!("unexpected operator: {rule:?}"),
        }),
    }
}
