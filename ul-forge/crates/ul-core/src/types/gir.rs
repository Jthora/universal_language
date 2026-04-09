use serde::{Deserialize, Serialize};

use super::edge::Edge;
use super::node::{Node, NodeId, NodeType};

/// Top-level GIR document — the canonical representation of a UL glyph.
///
/// A GIR document is a typed graph: a flat pool of [`Node`]s connected by
/// typed [`Edge`]s. The tree spine is extracted from `contains` and `modified_by`
/// edges; cross-edges (`adjacent`, `intersects`, `connects`, `references`)
/// bridge different branches.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gir {
    /// GIR format version. Currently `"1.0"`.
    pub ul_gir: String,

    /// ID of the root node (the outermost element).
    pub root: NodeId,

    /// All nodes in the glyph.
    pub nodes: Vec<Node>,

    /// All edges (relationships) between nodes.
    pub edges: Vec<Edge>,

    /// Optional metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<GirMetadata>,

    /// Variable IDs currently in binding scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_scope: Option<Vec<String>>,

    /// Modal context — which nodes are world entities and which edges are accessibility relations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modal_context: Option<ModalContext>,
}

/// Modal context tracking world nodes and accessibility edges.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModalContext {
    /// Node IDs that represent possible worlds.
    pub world_nodes: Vec<NodeId>,
    /// Indices into the `edges` array that are accessibility relations.
    pub accessibility_edges: Vec<usize>,
}

/// Optional metadata attached to a GIR document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GirMetadata {
    /// Source file that produced this GIR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Tool that generated this GIR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_by: Option<String>,

    /// ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl Gir {
    /// Create a new GIR document with the given root, nodes, and edges.
    pub fn new(root: impl Into<String>, nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
        Gir {
            ul_gir: "1.0".to_string(),
            root: root.into(),
            nodes,
            edges,
            metadata: None,
            binding_scope: None,
            modal_context: None,
        }
    }

    /// Set metadata on this GIR document.
    pub fn with_metadata(mut self, metadata: GirMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Set binding scope on this GIR document.
    pub fn with_binding_scope(mut self, scope: Vec<String>) -> Self {
        self.binding_scope = Some(scope);
        self
    }

    /// Set modal context on this GIR document.
    pub fn with_modal_context(mut self, ctx: ModalContext) -> Self {
        self.modal_context = Some(ctx);
        self
    }

    /// Get all nodes that are variable slots (have `variable_id` set).
    pub fn bound_variables(&self) -> Vec<&Node> {
        self.nodes
            .iter()
            .filter(|n| n.variable_id.is_some() && n.node_type == NodeType::VariableSlot)
            .collect()
    }

    /// Look up a node by ID.
    pub fn node(&self, id: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.id == id)
    }

    /// Look up a node by ID (mutable).
    pub fn node_mut(&mut self, id: &str) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    /// Get all edges of a given type.
    pub fn edges_of_type(&self, edge_type: super::edge::EdgeType) -> Vec<&Edge> {
        self.edges
            .iter()
            .filter(|e| e.edge_type == edge_type)
            .collect()
    }

    /// Get all edges where the given node is the source.
    pub fn edges_from(&self, node_id: &str) -> Vec<&Edge> {
        self.edges.iter().filter(|e| e.source == node_id).collect()
    }

    /// Get all edges where the given node is the target.
    pub fn edges_to(&self, node_id: &str) -> Vec<&Edge> {
        self.edges.iter().filter(|e| e.target == node_id).collect()
    }

    /// Get the tree-spine edges (contains + modified_by).
    pub fn tree_spine_edges(&self) -> Vec<&Edge> {
        self.edges
            .iter()
            .filter(|e| e.edge_type.is_tree_spine())
            .collect()
    }

    /// Get the cross-edges (adjacent, intersects, connects, references).
    pub fn cross_edges(&self) -> Vec<&Edge> {
        self.edges
            .iter()
            .filter(|e| !e.edge_type.is_tree_spine())
            .collect()
    }

    /// Get IDs of all children of a node (via `contains` edges).
    pub fn children_of(&self, node_id: &str) -> Vec<&str> {
        self.edges
            .iter()
            .filter(|e| {
                e.source == node_id && matches!(e.edge_type, super::edge::EdgeType::Contains)
            })
            .map(|e| e.target.as_str())
            .collect()
    }

    /// Serialize to pretty-printed JSON.
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Serialize to compact JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Deserialize from JSON.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
