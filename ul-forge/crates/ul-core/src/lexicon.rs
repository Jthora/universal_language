//! Lexicon index: 42 canonical UL definitions as a searchable in-memory index.
//!
//! Each entry is a `LexiconEntry` with name, tier, constructive level,
//! symbol, algebraic expression, and natural-language labels.

use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

/// A single lexicon entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LexiconEntry {
    /// Entry number (1–42).
    pub id: u32,
    /// Constructive level (0–7).
    pub level: u8,
    /// Canonical name.
    pub name: String,
    /// Tier classification: "T1", "T2", or "T3".
    pub tier: String,
    /// Visual symbol representation.
    pub symbol: String,
    /// Σ_UL algebraic expression (or "—" if geometry-only).
    pub sigma_ul: String,
    /// Natural-language labels / semantic associations.
    pub labels: Vec<String>,
}

/// Build the static canon of 42 entries.
fn build_canon() -> Vec<LexiconEntry> {
    let raw: &[(&str, u8, &str, &str, &str, &[&str])] = &[
        // Level 0
        ("The Void", 0, "T1", "Ø", "—", &["Nothing", "Silence", "Absence"]),
        // Level 1 — atomic carriers
        ("Single Point", 1, "T1", "●", "e", &["Existence", "Being", "Something"]),
        ("Single Line", 1, "T1", "●──●", "predicate(e, r, e)", &["Relation", "Connection"]),
        ("Single Directed Line", 1, "T2", "●──→●", "predicate(e, r, e)", &["Action", "Causation"]),
        ("Single Angle", 1, "T1", "∠", "modify_relation(m, r)", &["Quality", "Character", "Manner"]),
        ("Single Curve", 1, "T1", "~", "compose(r, r)", &["Process", "Change", "Becoming"]),
        ("Single Enclosure", 1, "T1", "○", "embed(a)", &["Concept", "Category", "Boundary"]),
        // Level 2 — distinguished parameters & single operations
        ("Zero Angle", 2, "T1", "∠0°", "modify_relation(m₀, r)", &["Identity", "Sameness", "Equivalence"]),
        ("Right Angle", 2, "T1", "∠90°", "modify_relation(m₉₀, r)", &["Independence", "Orthogonality"]),
        ("Straight Angle", 2, "T1", "∠180°", "negate(a) / invert(r)", &["Opposition", "Negation", "Reversal"]),
        ("Full Angle", 2, "T1", "∠360°", "—", &["Completion", "Cycle", "Return"]),
        ("Circle", 2, "T1", "○", "embed(a)", &["Totality", "Wholeness", "Containment"]),
        ("Triangle", 2, "T2", "△", "embed(a)", &["Structure", "Stability", "Triad"]),
        ("Square", 2, "T2", "□", "embed(a)", &["Order", "Balance", "Foundation"]),
        ("Negation", 2, "T1", "¬a", "negate(a)", &["Not", "Denial", "Contradiction"]),
        ("Conjunction", 2, "T1", "a ∩ a", "conjoin(a, a)", &["And", "Both", "Together"]),
        ("Disjunction", 2, "T1", "a | a", "disjoin(a, a)", &["Or", "Either", "Alternative"]),
        ("Embedding", 2, "T1", "a → e", "embed(a)", &["Nominalization", "Reification"]),
        ("Abstraction", 2, "T1", "e → m", "abstract(e)", &["Adjectivalization", "Property extraction"]),
        ("Composition", 2, "T1", "r ∘ r", "compose(r, r)", &["Transitivity", "Chaining"]),
        ("Inversion", 2, "T1", "r⁻¹", "invert(r)", &["Passive", "Reversal", "Dual"]),
        ("Quantification", 2, "T1", "∀/∃", "quantify(m, e)", &["All", "Some", "Quantity"]),
        ("Open Curve", 2, "T2", "⌒", "compose(r, r)", &["Journey", "Trajectory", "Progress"]),
        ("Closed Curve", 2, "T2", "∞", "compose(r, r)", &["Cycle", "Recursion", "Orbit"]),
        ("Self-Reference", 2, "T1", "●→●", "embed(predicate(e,r,e))", &["Reflexivity", "Self", "Identity loop"]),
        ("Modifier Chain", 2, "T2", "m(m(e))", "modify_entity(m, modify_entity(m, e))", &["Intensification", "Degree"]),
        // Level 3 — two-primitive combinations
        ("Modified Predicate", 3, "T1", "m(e r e)", "modify_entity(m, predicate(...))", &["Qualified assertion", "Contextualized claim"]),
        ("Nested Enclosure", 3, "T1", "○(○)", "embed(embed(a))", &["Metacognition", "Abstraction layer"]),
        ("Parallel Relations", 3, "T2", "r ∥ r", "conjoin(predicate(...), predicate(...))", &["Analogy", "Correspondence"]),
        ("Branching", 3, "T2", "Y", "disjoin(predicate(...), predicate(...))", &["Choice", "Fork", "Decision"]),
        ("Feedback Loop", 3, "T2", "↻", "compose(r, invert(r))", &["Equilibrium", "Homeostasis"]),
        ("Modified Composition", 3, "T1", "m(r ∘ r)", "modify_relation(m, compose(r, r))", &["Regulated process", "Constrained change"]),
        ("Quantified Group", 3, "T1", "∀(○)", "quantify(m, embed(a))", &["Universal claim", "Category-wide truth"]),
        ("Asymmetric Junction", 3, "T2", "⊥", "conjoin(a, negate(a))", &["Paradox", "Contradiction"]),
        // Level 4 — multi-primitive
        ("Argument Structure", 4, "T1", "e r(m) e → a", "predicate(e, modify_relation(m, r), e)", &["Predication", "Full statement"]),
        ("Semantic Field", 4, "T2", "○{...}", "embed(conjoin(a₁, ..., aₙ))", &["Domain", "Knowledge cluster"]),
        // Level 5+ — higher compositions
        ("Narrative Arc", 5, "T2", "~→~→~", "compose(compose(r, r), r)", &["Story", "Temporal sequence"]),
        ("Hierarchical System", 5, "T2", "○(○(○))", "embed(embed(embed(a)))", &["Taxonomy", "Hierarchy"]),
        ("Dialectic", 5, "T3", "a ∩ ¬a → a'", "embed(conjoin(a, negate(a)))", &["Synthesis", "Resolution"]),
        ("Democracy", 5, "T3", "⬡", "quantify(m, embed(conjoin(...)))", &["Collective decision", "Distributed authority"]),
        ("Ecosystem", 6, "T3", "○{↻↻↻}", "embed(conjoin(compose(r₁,r₁⁻¹), ...))", &["Interdependence", "Complex adaptive system"]),
        ("Consciousness", 7, "T3", "○(●→○(●→○(...)))", "embed(predicate(e, r, embed(...)))", &["Self-awareness", "Recursive self-model"]),
    ];

    raw.iter()
        .enumerate()
        .map(|(i, &(name, level, tier, symbol, sigma, labels))| LexiconEntry {
            id: (i + 1) as u32,
            level,
            name: name.to_string(),
            tier: tier.to_string(),
            symbol: symbol.to_string(),
            sigma_ul: sigma.to_string(),
            labels: labels.iter().map(|l| l.to_string()).collect(),
        })
        .collect()
}

static CANON: LazyLock<Vec<LexiconEntry>> = LazyLock::new(build_canon);

/// Get all 42 canonical lexicon entries.
pub fn all_entries() -> &'static [LexiconEntry] {
    &CANON
}

/// Look up a lexicon entry by exact name (case-insensitive).
pub fn lookup(name: &str) -> Option<&'static LexiconEntry> {
    let lower = name.to_lowercase();
    CANON.iter().find(|e| e.name.to_lowercase() == lower)
}

/// Search the lexicon by substring match on name, labels, or sigma_ul.
/// Returns entries where any field contains the query (case-insensitive).
pub fn search(query: &str) -> Vec<&'static LexiconEntry> {
    let lower = query.to_lowercase();
    CANON
        .iter()
        .filter(|e| {
            e.name.to_lowercase().contains(&lower)
                || e.sigma_ul.to_lowercase().contains(&lower)
                || e.labels.iter().any(|l| l.to_lowercase().contains(&lower))
                || e.symbol.contains(&lower)
        })
        .collect()
}

/// Get all entries at a specific tier ("T1", "T2", or "T3").
pub fn by_tier(tier: &str) -> Vec<&'static LexiconEntry> {
    CANON.iter().filter(|e| e.tier == tier).collect()
}

/// Get all entries at a specific constructive level.
pub fn by_level(level: u8) -> Vec<&'static LexiconEntry> {
    CANON.iter().filter(|e| e.level == level).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canon_has_42_entries() {
        assert_eq!(all_entries().len(), 42);
    }

    #[test]
    fn lookup_by_name() {
        let entry = lookup("The Void").unwrap();
        assert_eq!(entry.id, 1);
        assert_eq!(entry.tier, "T1");
        assert_eq!(entry.level, 0);
    }

    #[test]
    fn lookup_case_insensitive() {
        assert!(lookup("single point").is_some());
        assert!(lookup("SINGLE POINT").is_some());
    }

    #[test]
    fn search_by_label() {
        let results = search("existence");
        assert!(!results.is_empty());
        assert!(results.iter().any(|e| e.name == "Single Point"));
    }

    #[test]
    fn search_by_operation() {
        let results = search("negate");
        assert!(!results.is_empty());
        assert!(results.iter().any(|e| e.name == "Negation"));
    }

    #[test]
    fn tier_counts() {
        let t1 = by_tier("T1");
        let t2 = by_tier("T2");
        let t3 = by_tier("T3");
        assert_eq!(t1.len() + t2.len() + t3.len(), 42);
        assert!(t1.len() > t2.len()); // T1 is largest
        assert!(t2.len() > t3.len()); // T3 is smallest
    }

    #[test]
    fn level_0_has_one_entry() {
        assert_eq!(by_level(0).len(), 1);
    }
}
