use serde::{Deserialize, Serialize};
use std::fmt;

use super::sort::Sort;

/// A unique identifier for a node within a GIR document.
pub type NodeId = String;

/// The geometric type of a node — one of the 5 UL primitives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeType {
    /// `•` — Existence, identity, position.
    Point,
    /// `→` — Directed or undirected connection.
    Line,
    /// `∠` — Measure between two relations.
    Angle,
    /// `~` — Parameterized path, process.
    Curve,
    /// `◯ △ □` — Bounded region, concept.
    Enclosure,
}

impl NodeType {
    /// The default sort for this geometric type, per Σ_UL grounding.
    pub fn default_sort(self) -> Sort {
        match self {
            NodeType::Point => Sort::Entity,
            NodeType::Line => Sort::Relation,
            NodeType::Angle => Sort::Modifier,
            NodeType::Curve => Sort::Relation,
            NodeType::Enclosure => Sort::Entity,
        }
    }
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeType::Point => write!(f, "point"),
            NodeType::Line => write!(f, "line"),
            NodeType::Angle => write!(f, "angle"),
            NodeType::Curve => write!(f, "curve"),
            NodeType::Enclosure => write!(f, "enclosure"),
        }
    }
}

/// Shape of an enclosure node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EnclosureShape {
    Circle,
    Triangle,
    Square,
    Ellipse,
    Polygon,
    Freeform,
}

impl fmt::Display for EnclosureShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnclosureShape::Circle => write!(f, "circle"),
            EnclosureShape::Triangle => write!(f, "triangle"),
            EnclosureShape::Square => write!(f, "square"),
            EnclosureShape::Ellipse => write!(f, "ellipse"),
            EnclosureShape::Polygon => write!(f, "polygon"),
            EnclosureShape::Freeform => write!(f, "freeform"),
        }
    }
}

/// A node in the GIR graph — one geometric primitive instance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    /// Unique identifier within the document.
    pub id: NodeId,

    /// Geometric primitive type.
    #[serde(rename = "type")]
    pub node_type: NodeType,

    /// Σ_UL sort. Defaults to the type's natural sort if omitted during deserialization.
    #[serde(default = "Sort::default_from_context")]
    pub sort: Sort,

    /// Human-readable label (e.g., "concept", "existence").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Shape — only meaningful for enclosure nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<EnclosureShape>,

    /// Direction vector `[dx, dy]` — only meaningful for line nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<[f64; 2]>,

    /// Whether the line is directed (arrow) or undirected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directed: Option<bool>,

    /// Angle measure in degrees `[0, 360)` — only meaningful for angle nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure: Option<f64>,

    /// Curvature (non-negative) — only meaningful for curve nodes.
    /// When `curvature_profile` is absent, this is the constant curvature κ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curvature: Option<f64>,

    /// Piecewise curvature profile κ(s) — sampled at uniform arc-length intervals.
    /// When present, overrides the scalar `curvature` field for variable-curvature curves.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curvature_profile: Option<Vec<f64>>,

    /// Number of vertices — only meaningful for polygon enclosures.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertices: Option<u32>,
}

// We can't use serde's default with a function that takes context,
// so we handle sort defaulting in a custom impl.
impl Sort {
    /// Placeholder for serde default — returns Entity.
    /// Actual defaulting based on NodeType happens in Node::new().
    fn default_from_context() -> Sort {
        Sort::Entity
    }
}

impl Node {
    /// Create a new node with the given id and type. Sort is inferred from the type.
    pub fn new(id: impl Into<String>, node_type: NodeType) -> Self {
        Node {
            id: id.into(),
            sort: node_type.default_sort(),
            node_type,
            label: None,
            shape: None,
            direction: None,
            directed: None,
            measure: None,
            curvature: None,
            curvature_profile: None,
            vertices: None,
        }
    }

    /// Create a point node.
    pub fn point(id: impl Into<String>) -> Self {
        Self::new(id, NodeType::Point)
    }

    /// Create a line node.
    pub fn line(id: impl Into<String>, directed: bool) -> Self {
        let mut node = Self::new(id, NodeType::Line);
        node.directed = Some(directed);
        node
    }

    /// Create an angle node with the given measure in degrees.
    pub fn angle(id: impl Into<String>, degrees: f64) -> Self {
        let mut node = Self::new(id, NodeType::Angle);
        node.measure = Some(degrees);
        node
    }

    /// Create a curve node with the given curvature.
    pub fn curve(id: impl Into<String>, curvature: f64) -> Self {
        let mut node = Self::new(id, NodeType::Curve);
        node.curvature = Some(curvature);
        node
    }

    /// Create an enclosure node with the given shape.
    pub fn enclosure(id: impl Into<String>, shape: EnclosureShape) -> Self {
        let mut node = Self::new(id, NodeType::Enclosure);
        node.shape = Some(shape);
        node
    }

    /// Set a label on this node.
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Override the sort (for transformed roles like embed/abstract).
    pub fn with_sort(mut self, sort: Sort) -> Self {
        self.sort = sort;
        self
    }
}
