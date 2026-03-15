use serde::{Deserialize, Serialize};
use std::fmt;

use super::node::NodeId;

/// The 6 edge types representing spatial and semantic relationships between nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeType {
    /// Source enclosure spatially contains target. Tree-spine edge.
    Contains,
    /// Target modifier applies to source. Tree-spine edge.
    ModifiedBy,
    /// Source and target share a boundary (touching).
    Adjacent,
    /// Source and target spatially overlap.
    Intersects,
    /// A line/curve links source and target (directed or undirected).
    Connects,
    /// Semantic cross-reference (may be non-spatial).
    References,
}

impl EdgeType {
    /// Whether this edge type participates in the tree spine.
    pub fn is_tree_spine(self) -> bool {
        matches!(self, EdgeType::Contains | EdgeType::ModifiedBy)
    }

    /// Whether cycles are allowed for this edge type.
    pub fn allows_cycles(self) -> bool {
        !self.is_tree_spine()
    }
}

impl fmt::Display for EdgeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EdgeType::Contains => write!(f, "contains"),
            EdgeType::ModifiedBy => write!(f, "modified_by"),
            EdgeType::Adjacent => write!(f, "adjacent"),
            EdgeType::Intersects => write!(f, "intersects"),
            EdgeType::Connects => write!(f, "connects"),
            EdgeType::References => write!(f, "references"),
        }
    }
}

/// A typed edge between two nodes in the GIR graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    /// Source node ID.
    pub source: NodeId,

    /// Target node ID.
    pub target: NodeId,

    /// Relationship type.
    #[serde(rename = "type")]
    pub edge_type: EdgeType,
}

impl Edge {
    /// Create a new edge.
    pub fn new(source: impl Into<String>, target: impl Into<String>, edge_type: EdgeType) -> Self {
        Edge {
            source: source.into(),
            target: target.into(),
            edge_type,
        }
    }

    /// Create a containment edge (source contains target).
    pub fn contains(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self::new(source, target, EdgeType::Contains)
    }

    /// Create a modification edge (source is modified by target).
    pub fn modified_by(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self::new(source, target, EdgeType::ModifiedBy)
    }

    /// Create a connection edge (line/curve connects source to target).
    pub fn connects(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self::new(source, target, EdgeType::Connects)
    }

    /// Create an adjacency edge.
    pub fn adjacent(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self::new(source, target, EdgeType::Adjacent)
    }

    /// Create an intersection edge.
    pub fn intersects(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self::new(source, target, EdgeType::Intersects)
    }

    /// Create a reference edge.
    pub fn references(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self::new(source, target, EdgeType::References)
    }
}
