//! Puzzle difficulty progression and selection.
//!
//! Maps the 42 lexicon entries to a difficulty ladder that the
//! teaching system uses to select appropriate next puzzles.

use serde::{Deserialize, Serialize};

/// A puzzle definition for the teaching system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Puzzle {
    /// Puzzle identifier (maps to lexicon entry name or custom).
    pub id: String,
    /// Difficulty rating (1–10).
    pub difficulty: u8,
    /// Constructive level from the lexicon (0–7).
    pub level: u8,
    /// Required operations the student must demonstrate.
    pub required_operations: Vec<String>,
    /// Description of what to construct.
    pub description: String,
    /// The target GIR as JSON (what the student should produce).
    pub target_gir_json: String,
}

/// Get the next appropriate puzzle given the student's proficiency map.
///
/// `proficiency`: map of operation name → proficiency score (0.0–1.0).
/// Returns the easiest puzzle that the student hasn't mastered yet.
pub fn get_next_puzzle(proficiency: &std::collections::HashMap<String, f64>) -> Puzzle {
    let curriculum = build_curriculum();

    // Find first puzzle where the student lacks proficiency in at least one required operation,
    // or has no proficiency data at all for introductory puzzles
    for puzzle in &curriculum {
        let mastered = if puzzle.required_operations.is_empty() {
            // Introductory puzzle (no ops required): mastered if student has any proficiency at all
            !proficiency.is_empty()
        } else {
            puzzle.required_operations.iter().all(|op| {
                proficiency.get(op).copied().unwrap_or(0.0) >= 0.8
            })
        };
        if !mastered {
            return puzzle.clone();
        }
    }

    // All mastered — return the hardest puzzle for practice
    curriculum.last().unwrap().clone()
}

/// Build the default curriculum of puzzles ordered by difficulty.
fn build_curriculum() -> Vec<Puzzle> {
    vec![
        Puzzle {
            id: "single_point".into(),
            difficulty: 1,
            level: 1,
            required_operations: vec![],
            description: "Place a single point — the simplest act of meaning.".into(),
            target_gir_json: r#"{"ul_gir":"0.2","root":"p","nodes":[{"id":"p","node_type":"point","sort":"entity"}],"edges":[]}"#.into(),
        },
        Puzzle {
            id: "single_line".into(),
            difficulty: 2,
            level: 1,
            required_operations: vec!["predicate".into()],
            description: "Connect two points with a line — create a basic relation.".into(),
            target_gir_json: r#"{"ul_gir":"0.2","root":"l","nodes":[{"id":"p1","node_type":"point","sort":"entity"},{"id":"l","node_type":"line","sort":"relation","directed":true},{"id":"p2","node_type":"point","sort":"entity"}],"edges":[{"source":"l","target":"p1","edge_type":"connects"},{"source":"l","target":"p2","edge_type":"connects"}]}"#.into(),
        },
        Puzzle {
            id: "enclosure".into(),
            difficulty: 3,
            level: 1,
            required_operations: vec!["embed".into()],
            description: "Create an enclosure containing a point — form a concept.".into(),
            target_gir_json: r#"{"ul_gir":"0.2","root":"enc","nodes":[{"id":"enc","node_type":"enclosure","sort":"assertion","shape":"circle"},{"id":"p","node_type":"point","sort":"entity"}],"edges":[{"source":"enc","target":"p","edge_type":"contains"}]}"#.into(),
        },
        Puzzle {
            id: "modified_entity".into(),
            difficulty: 4,
            level: 2,
            required_operations: vec!["modify_entity".into()],
            description: "Modify an entity with an angle — add quality to existence.".into(),
            target_gir_json: r#"{"ul_gir":"0.2","root":"e","nodes":[{"id":"e","node_type":"point","sort":"entity"},{"id":"m","node_type":"angle","sort":"modifier","measure":45.0}],"edges":[{"source":"e","target":"m","edge_type":"modified_by"}]}"#.into(),
        },
        Puzzle {
            id: "negation".into(),
            difficulty: 5,
            level: 2,
            required_operations: vec!["negate".into()],
            description: "Negate an assertion — express 'not'.".into(),
            target_gir_json: "{}".into(),
        },
        Puzzle {
            id: "conjunction".into(),
            difficulty: 5,
            level: 2,
            required_operations: vec!["conjoin".into()],
            description: "Combine two assertions with conjunction — express 'and'.".into(),
            target_gir_json: "{}".into(),
        },
        Puzzle {
            id: "composition".into(),
            difficulty: 6,
            level: 2,
            required_operations: vec!["compose".into()],
            description: "Compose two relations — create a process through transitivity.".into(),
            target_gir_json: "{}".into(),
        },
        Puzzle {
            id: "inversion".into(),
            difficulty: 6,
            level: 2,
            required_operations: vec!["invert".into()],
            description: "Invert a relation — reverse the direction of causation.".into(),
            target_gir_json: "{}".into(),
        },
        Puzzle {
            id: "nested_enclosure".into(),
            difficulty: 7,
            level: 3,
            required_operations: vec!["embed".into()],
            description: "Create nested enclosures — build abstraction layers.".into(),
            target_gir_json: "{}".into(),
        },
        Puzzle {
            id: "full_predicate".into(),
            difficulty: 8,
            level: 4,
            required_operations: vec!["predicate".into(), "modify_relation".into()],
            description: "Build a full argument structure: entity + modified relation + entity → assertion.".into(),
            target_gir_json: "{}".into(),
        },
        Puzzle {
            id: "semantic_field".into(),
            difficulty: 9,
            level: 4,
            required_operations: vec!["embed".into(), "conjoin".into()],
            description: "Create a semantic field — a concept containing multiple related assertions.".into(),
            target_gir_json: "{}".into(),
        },
        Puzzle {
            id: "narrative_arc".into(),
            difficulty: 10,
            level: 5,
            required_operations: vec!["compose".into(), "embed".into(), "modify_relation".into()],
            description: "Construct a narrative arc — chain processes within an embedded context.".into(),
            target_gir_json: "{}".into(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn beginner_gets_first_puzzle() {
        let proficiency = HashMap::new();
        let puzzle = get_next_puzzle(&proficiency);
        assert_eq!(puzzle.id, "single_point");
        assert_eq!(puzzle.difficulty, 1);
    }

    #[test]
    fn advanced_student_skips_mastered() {
        let mut proficiency = HashMap::new();
        // Master all operations that appear in early puzzles
        proficiency.insert("predicate".into(), 0.9);
        proficiency.insert("embed".into(), 0.9);
        proficiency.insert("modify_entity".into(), 0.9);
        let puzzle = get_next_puzzle(&proficiency);
        // Should skip to a puzzle requiring an unmastered operation
        assert!(puzzle.difficulty >= 5);
    }

    #[test]
    fn fully_mastered_gets_hardest() {
        let mut proficiency = HashMap::new();
        for op in &["predicate", "modify_entity", "modify_relation", "negate",
                     "conjoin", "disjoin", "embed", "abstract", "compose",
                     "invert", "quantify"] {
            proficiency.insert((*op).to_string(), 1.0);
        }
        let puzzle = get_next_puzzle(&proficiency);
        assert_eq!(puzzle.id, "narrative_arc");
    }
}
