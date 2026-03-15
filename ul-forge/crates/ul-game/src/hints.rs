//! Hint generation: provide contextual guidance based on GIR analysis.
//!
//! Compares the student's current GIR against a target and produces
//! actionable hints without revealing the answer.

use crate::analysis;
use serde::{Deserialize, Serialize};
use ul_core::types::gir::Gir;

/// A teaching hint with severity and category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hint {
    /// Severity: "info", "suggestion", "warning".
    pub severity: String,
    /// Category: "sort", "structure", "operation", "primitive", "general".
    pub category: String,
    /// Human-readable hint text.
    pub message: String,
}

/// Generate hints by comparing a student's GIR to a target GIR.
pub fn generate_hints(attempt: &Gir, target: &Gir) -> Vec<Hint> {
    let mut hints = Vec::new();

    let attempt_report = analysis::analyze_structure(attempt);
    let target_report = analysis::analyze_structure(target);

    // Check missing primitives
    for (prim, &count) in &target_report.primitive_counts {
        let attempt_count = attempt_report.primitive_counts.get(prim).copied().unwrap_or(0);
        if attempt_count == 0 && count > 0 {
            hints.push(Hint {
                severity: "suggestion".into(),
                category: "primitive".into(),
                message: format!("Your composition doesn't use any {prim} primitives. The target requires at least one."),
            });
        } else if attempt_count < count {
            hints.push(Hint {
                severity: "info".into(),
                category: "primitive".into(),
                message: format!("Your composition has fewer {prim} primitives than expected."),
            });
        }
    }

    // Check missing sorts
    for (sort, &count) in &target_report.sort_distribution {
        let attempt_count = attempt_report.sort_distribution.get(sort).copied().unwrap_or(0);
        if attempt_count == 0 && count > 0 {
            hints.push(Hint {
                severity: "suggestion".into(),
                category: "sort".into(),
                message: format!("Your composition is missing the {sort} sort. Consider adding one."),
            });
        }
    }

    // Check missing operations
    let attempt_ops = analysis::detect_operations_list(attempt);
    let target_ops = analysis::detect_operations_list(target);
    for op in &target_ops {
        if !attempt_ops.contains(op) {
            hints.push(Hint {
                severity: "suggestion".into(),
                category: "operation".into(),
                message: format!("The target uses the {:?} operation, which your composition doesn't express yet.", op),
            });
        }
    }

    // Check depth
    if attempt_report.depth < target_report.depth {
        hints.push(Hint {
            severity: "info".into(),
            category: "structure".into(),
            message: "Your composition could benefit from more nesting depth.".into(),
        });
    }

    // Check node count
    if attempt_report.node_count < target_report.node_count / 2 {
        hints.push(Hint {
            severity: "warning".into(),
            category: "structure".into(),
            message: "Your composition seems significantly smaller than the target. Keep building!".into(),
        });
    }

    // If no specific hints, give encouragement
    if hints.is_empty() {
        if attempt_report.complexity_score >= target_report.complexity_score * 0.9 {
            hints.push(Hint {
                severity: "info".into(),
                category: "general".into(),
                message: "Your composition looks structurally similar to the target. Fine-tune the details!".into(),
            });
        } else {
            hints.push(Hint {
                severity: "info".into(),
                category: "general".into(),
                message: "You're making progress. Keep adding structural elements.".into(),
            });
        }
    }

    hints
}

/// Generate self-standing hints for a GIR (no target comparison).
/// Useful for open-ended exploration and validation feedback.
pub fn analyze_hints(gir: &Gir) -> Vec<Hint> {
    let mut hints = Vec::new();
    let report = analysis::analyze_structure(gir);

    // Check if only one sort is used
    if report.sort_distribution.len() == 1 {
        hints.push(Hint {
            severity: "suggestion".into(),
            category: "sort".into(),
            message: "Your composition uses only one sort. UL has 4 sorts — try incorporating more variety.".into(),
        });
    }

    // Check if no operations detected
    if report.detected_operations.is_empty() && report.node_count > 1 {
        hints.push(Hint {
            severity: "info".into(),
            category: "operation".into(),
            message: "No Σ_UL operations detected. Try connecting primitives with predicate, modify, or embed.".into(),
        });
    }

    // Very flat structure
    if report.depth <= 1 && report.node_count > 3 {
        hints.push(Hint {
            severity: "suggestion".into(),
            category: "structure".into(),
            message: "Your structure is very flat. Consider using enclosures to create containment hierarchy.".into(),
        });
    }

    // Single node
    if report.node_count == 1 {
        hints.push(Hint {
            severity: "info".into(),
            category: "general".into(),
            message: "You have a single primitive. Add more nodes and connect them to build meaning.".into(),
        });
    }

    hints
}

#[cfg(test)]
mod tests {
    use super::*;
    use ul_core::types::edge::Edge;
    use ul_core::types::node::{EnclosureShape, Node};

    #[test]
    fn hints_for_empty_vs_complex() {
        let attempt = Gir::new("p", vec![Node::point("p")], vec![]);
        let target = Gir::new(
            "root",
            vec![
                Node::enclosure("root", EnclosureShape::Circle),
                Node::point("p1"),
                Node::line("l1", true),
            ],
            vec![
                Edge::contains("root", "p1"),
                Edge::contains("root", "l1"),
            ],
        );
        let hints = generate_hints(&attempt, &target);
        assert!(!hints.is_empty());
        // Should mention missing primitives or structure
        assert!(hints.iter().any(|h| h.category == "primitive" || h.category == "structure"));
    }

    #[test]
    fn analyze_single_point() {
        let gir = Gir::new("p", vec![Node::point("p")], vec![]);
        let hints = analyze_hints(&gir);
        assert!(hints.iter().any(|h| h.message.contains("single primitive")));
    }

    #[test]
    fn analyze_flat_structure() {
        let gir = Gir::new(
            "root",
            vec![
                Node::enclosure("root", EnclosureShape::Circle),
                Node::point("p1"),
                Node::point("p2"),
                Node::point("p3"),
                Node::point("p4"),
            ],
            vec![
                Edge::contains("root", "p1"),
                Edge::contains("root", "p2"),
                Edge::contains("root", "p3"),
                Edge::contains("root", "p4"),
            ],
        );
        let hints = analyze_hints(&gir);
        // Should suggest more variety since only 2 sorts (Entity + Assertion for enclosure)
        // and no operations detected
        assert!(!hints.is_empty());
    }
}
