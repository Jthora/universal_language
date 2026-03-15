//! Shared type definitions for the UL game engine.
//!
//! Grounded in the real Universal Language formal system (Σ_UL):
//! - 5 geometric primitives: Point, Line, Angle, Curve, Enclosure
//! - 4 sorts: Entity, Relation, Modifier, Assertion
//! - 11 operations: predicate, modify_entity, modify_relation, negate,
//!   conjoin, disjoin, embed, abstract, compose, invert, quantify

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Σ_UL operations ────────────────────────────────────────────

/// The 11 operations of the Σ_UL algebraic signature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    /// e × r × e → a: combine subject, relation, object into statement.
    Predicate,
    /// m × e → e: apply modifier to an entity.
    ModifyEntity,
    /// m × r → r: apply modifier to a relation.
    ModifyRelation,
    /// a → a: negate a statement (vertical reflection).
    Negate,
    /// a × a → a: combine statements with AND (overlapping frames).
    Conjoin,
    /// a × a → a: combine statements with OR (adjacent frames).
    Disjoin,
    /// a → e: turn statement into entity (nominalization via enclosure).
    Embed,
    /// e → m: turn entity into modifier (adjectivalization).
    Abstract,
    /// r × r → r: chain two relations (transitivity).
    Compose,
    /// r → r: reverse a relation direction.
    Invert,
    /// m × e → a: apply quantifier-modifier to entity.
    Quantify,
}

/// Tier classification from the UL lexicon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tier {
    /// T1: geometrically forced — exactly one construction with these properties.
    Forced,
    /// T2: structurally distinguished — extremal or maximally symmetric.
    Distinguished,
    /// T3: conventional — reasonable but not unique choice.
    Conventional,
}

// ── Pattern matching ───────────────────────────────────────────

/// A pattern to match against GIR nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternNode {
    /// A binding name for this node in the pattern (e.g. "root", "child1").
    pub id: String,
    /// If set, node_type must match (point, line, angle, curve, enclosure).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// If set, sort must match (entity, relation, modifier, assertion).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// If set, label must match (exact string).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// A pattern to match against GIR edges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternEdge {
    /// Source pattern node id.
    pub source: String,
    /// Target pattern node id.
    pub target: String,
    /// If set, edge_type must match (contains, modified_by, connects, adjacent, intersects).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_type: Option<String>,
}

/// A structural pattern to match against a GIR subgraph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GirPattern {
    pub nodes: Vec<PatternNode>,
    #[serde(default)]
    pub edges: Vec<PatternEdge>,
}

// ── Composition rules ──────────────────────────────────────────

/// A composition rule grounded in real UL structure.
///
/// Rules check for correct use of geometric primitives, sorts, and operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionRule {
    pub name: String,
    /// Which Σ_UL operation this rule tests (if specific to one).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
    /// Tier classification (T1 forced, T2 distinguished, T3 conventional).
    #[serde(default = "default_tier")]
    pub tier: Tier,
    /// GIR pattern that must be present for this rule to be satisfied.
    pub required_pattern: GirPattern,
    /// GIR pattern that must NOT be present (optional).
    #[serde(default)]
    pub forbidden_pattern: Option<GirPattern>,
    /// Importance weight (0.0–1.0) for scoring.
    #[serde(default = "default_weight")]
    pub weight: f64,
    /// Difficulty level (1–5), used for proficiency scaling.
    #[serde(default = "default_difficulty")]
    pub difficulty: u8,
}

fn default_weight() -> f64 {
    1.0
}
fn default_tier() -> Tier {
    Tier::Conventional
}
fn default_difficulty() -> u8 {
    1
}

// ── Scoring ────────────────────────────────────────────────────

/// Evaluation grade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Grade {
    /// 0.95–1.0: Perfect or near-perfect match.
    Exact,
    /// 0.60–0.95: Structurally similar, minor differences.
    Close,
    /// 0.20–0.60: Partially correct.
    Partial,
    /// 0.0–0.20: Structurally unrelated.
    Unrelated,
}

impl Grade {
    pub fn from_score(score: f64) -> Self {
        match score {
            s if s >= 0.95 => Self::Exact,
            s if s >= 0.60 => Self::Close,
            s if s >= 0.20 => Self::Partial,
            _ => Self::Unrelated,
        }
    }
}

/// Breakdown of a partial-credit score based on real UL dimensions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialCredit {
    /// How well the topology matches (node types, edge structure) (0.0–1.0).
    pub structural_match: f64,
    /// How well sorts are assigned (entity/relation/modifier/assertion) (0.0–1.0).
    pub sort_correctness: f64,
    /// How well the Σ_UL operations are applied (0.0–1.0).
    pub operation_correctness: f64,
    /// How well the construction order is maintained (0.0–1.0).
    pub sequence_order: f64,
}

impl PartialCredit {
    /// Weighted composite score.
    pub fn composite(&self) -> f64 {
        const W_STRUCT: f64 = 0.35;
        const W_SORT: f64 = 0.25;
        const W_OP: f64 = 0.25;
        const W_SEQ: f64 = 0.15;
        self.structural_match * W_STRUCT
            + self.sort_correctness * W_SORT
            + self.operation_correctness * W_OP
            + self.sequence_order * W_SEQ
    }
}

// ── Evaluation results ─────────────────────────────────────────

/// Result of evaluating a GIR against composition rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationResult {
    pub score: f64,
    pub grade: Grade,
    pub matched_rules: Vec<String>,
    pub violated_rules: Vec<String>,
    pub feedback: Vec<String>,
}

/// Result of scoring a composition against a specific puzzle target.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreResult {
    pub score: f64,
    pub grade: Grade,
    pub partial_credit: PartialCredit,
    pub feedback: Vec<String>,
}

/// Result of evaluating Jane's learning attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JaneResult {
    pub score: f64,
    pub grade: Grade,
    pub improvements: Vec<String>,
    pub proficiency_delta: HashMap<String, f64>,
}

// ── Puzzle targets ─────────────────────────────────────────────

/// A puzzle target that defines what the player should construct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PuzzleTarget {
    /// The expected GIR (as JSON; parsed when needed).
    pub expected_gir_json: String,
    /// Σ_UL operations that should be demonstrated.
    #[serde(default)]
    pub required_operations: Vec<Operation>,
    /// Ordering constraints (node-id sequences).
    #[serde(default)]
    pub sequence_constraints: Vec<Vec<String>>,
    /// Per-component weights override.
    #[serde(default)]
    pub weights: Option<ScoringWeights>,
}

/// Optional weight override for puzzle scoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringWeights {
    pub structural: f64,
    pub sort: f64,
    pub operation: f64,
    pub sequence: f64,
}

// ── Sequence validation ────────────────────────────────────────

/// Result of validating a sequence of GIR structures.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceResult {
    pub valid: bool,
    pub errors: Vec<String>,
    /// Per-pair ordering scores.
    pub pair_scores: Vec<f64>,
}

// ── Animation ──────────────────────────────────────────────────

/// Easing function for animation interpolation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}

/// A single animation keyframe for a node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationKeyframe {
    pub node_id: String,
    /// Timestamp in milliseconds from animation start.
    pub timestamp_ms: u32,
    pub x: f64,
    pub y: f64,
    pub scale: f64,
    pub rotation: f64,
    pub opacity: f64,
    pub easing: Easing,
}

/// A complete animation sequence for a glyph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationSequence {
    pub keyframes: Vec<AnimationKeyframe>,
    pub total_duration_ms: u32,
}

// ── Modding ────────────────────────────────────────────────────

/// Result of loading custom rule definitions at runtime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadResult {
    pub loaded: usize,
    pub errors: Vec<String>,
}

// ── Game config ────────────────────────────────────────────────

/// Configuration for creating a new game context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    /// Initial composition rules (as JSON array).
    #[serde(default)]
    pub rules_json: String,
    /// Session identifier.
    #[serde(default)]
    pub session_id: String,
}

// ── Structure analysis ──────────────────────────────────────────

/// Report produced by structural analysis of a GIR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureReport {
    pub node_count: u32,
    pub edge_count: u32,
    /// Counts per geometric primitive (point, line, angle, curve, enclosure).
    pub primitive_counts: HashMap<String, u32>,
    /// Counts per sort (entity, relation, modifier, assertion).
    pub sort_distribution: HashMap<String, u32>,
    /// Names of Σ_UL operations detected in the structure.
    pub detected_operations: Vec<String>,
    /// Maximum containment depth.
    pub depth: u32,
    /// Maximum breadth (most children at any single node).
    pub breadth: u32,
    /// Composite complexity score (0.0–1.0).
    pub complexity_score: f64,
    /// Symmetry group of the root node (Erlangen classification).
    pub symmetry_group: SymmetryGroup,
    /// Part of speech derived from the root node's symmetry.
    pub part_of_speech: PartOfSpeech,
    /// Per-node symmetry breakdown: node_id → symmetry group.
    pub node_symmetries: HashMap<String, SymmetryGroup>,
}

// ── Symmetry classification ─────────────────────────────────────

/// Symmetry group of a geometric object (Erlangen Program).
///
/// Determines the grammatical role of a symbol in Universal Language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymmetryGroup {
    /// SO(2): full continuous rotational symmetry (circle).
    So2,
    /// D₃: 3-fold dihedral symmetry (triangle).
    D3,
    /// D₄: 4-fold dihedral symmetry (square).
    D4,
    /// D₅: 5-fold dihedral symmetry (pentagon).
    D5,
    /// D₆: 6-fold dihedral symmetry (hexagon).
    D6,
    /// Bilateral: single mirror axis only (angle, undirected line, ellipse).
    Bilateral,
    /// None: no symmetry (directed line, point, freeform, curve).
    None,
}

/// Part of speech derived from symmetry group classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartOfSpeech {
    /// Full SO(2) symmetry → applies universally.
    Determiner,
    /// High rotational symmetry (D₃–D₆) → context-independent concept.
    Noun,
    /// Low/no rotational symmetry, directional → action/process.
    Verb,
    /// Bilateral symmetry only → one dimension of comparison.
    Adjective,
    /// No symmetry, unique → refers to exactly one thing.
    ProperNoun,
}

// ── Erlangen equivalence ────────────────────────────────────────

/// Erlangen hierarchy level for comparing GIR structures.
///
/// From strictest to loosest invariant preservation:
/// Euclidean ⊂ Similarity ⊂ Affine ⊂ Projective ⊂ Topological.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErlangenLevel {
    /// E(n): identical — same types, shapes, angles, sizes, labels.
    Euclidean,
    /// Sim(n): synonym — same structure, ignore scale differences.
    Similarity,
    /// Aff(n): paraphrase — same containment/parallel structure, ignore proportions.
    Affine,
    /// PGL(n): structural analog — same incidence structure, ignore which is parallel.
    Projective,
    /// Homeo: translation — same connectivity (graph isomorphism of types).
    Topological,
}

/// Result of comparing two GIRs at a given Erlangen level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquivalenceResult {
    /// The Erlangen level tested.
    pub level: ErlangenLevel,
    /// Overall equivalence score (0.0–1.0).
    pub score: f64,
    /// Whether the structures are equivalent at this level (score ≥ 0.95).
    pub equivalent: bool,
    /// Scores for each sub-dimension checked.
    pub dimensions: EquivalenceDimensions,
}

/// Sub-dimension scores for Erlangen equivalence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquivalenceDimensions {
    /// Topology: same node count, edge count, containment graph shape.
    pub topology: f64,
    /// Type match: same node types (point, line, angle, curve, enclosure).
    pub types: f64,
    /// Sort match: same sort assignments.
    pub sorts: f64,
    /// Shape match: same enclosure shapes (circle, triangle, etc.).
    pub shapes: f64,
    /// Metric match: same angles, curvatures, sizes.
    pub metrics: f64,
}

// ── Validation DTO ─────────────────────────────────────────────

/// Serializable validation result (mirrors ul_core::validator::ValidationResult).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationDto {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    /// Errors classified by validation layer.
    pub layers: ValidationLayers,
}

/// Per-layer error classification for the 4-layer validation pipeline.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidationLayers {
    /// L1 Schema: duplicate IDs, dangling refs, missing fields.
    pub schema: Vec<String>,
    /// L2 Sort: sort constraint violations.
    pub sort: Vec<String>,
    /// L3 Invariant: containment cycles, root violations, arity checks.
    pub invariant: Vec<String>,
    /// L4 Geometry: geometric satisfiability errors.
    pub geometry: Vec<String>,
}
