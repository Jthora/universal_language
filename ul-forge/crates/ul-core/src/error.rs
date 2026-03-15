use thiserror::Error;

use crate::types::edge::EdgeType;
use crate::types::node::NodeId;
use crate::types::sort::Sort;

/// Convenience type alias.
pub type UlResult<T> = Result<T, UlError>;

/// All error types for UL Forge operations.
#[derive(Debug, Error)]
pub enum UlError {
    // -- Parse errors --
    #[error("parse error at line {line}, column {column}: {message}")]
    Parse {
        line: usize,
        column: usize,
        message: String,
    },

    #[error("unexpected token '{found}' at position {position}, expected one of: {expected}")]
    UnexpectedToken {
        position: usize,
        found: String,
        expected: String,
    },

    // -- Validation errors --
    #[error("missing required field '{field}' on node '{node_id}'")]
    MissingField { node_id: NodeId, field: String },

    #[error("duplicate node ID: '{node_id}'")]
    DuplicateNodeId { node_id: NodeId },

    #[error("dangling edge reference: node '{node_id}' does not exist")]
    DanglingEdgeRef { node_id: NodeId },

    #[error("sort violation: {edge_type} edge from '{from_node}' ({from_sort}) to '{to_node}' ({to_sort}) — {reason}")]
    SortViolation {
        edge_type: EdgeType,
        from_node: NodeId,
        from_sort: Sort,
        to_node: NodeId,
        to_sort: Sort,
        reason: String,
    },

    #[error("containment cycle detected involving nodes: {node_ids:?}")]
    ContainmentCycle { node_ids: Vec<NodeId> },

    #[error("no root node found (every node has an incoming 'contains' edge)")]
    NoRoot,

    #[error("multiple root nodes found: {node_ids:?}")]
    MultipleRoots { node_ids: Vec<NodeId> },

    #[error("dangling modifier: node '{node_id}' has sort 'modifier' but no 'modified_by' edge")]
    DanglingModifier { node_id: NodeId },

    #[error("invalid angle arity: angle node '{node_id}' must reference exactly 2 lines/curves, found {found}")]
    InvalidAngleArity { node_id: NodeId, found: usize },

    // -- Render errors --
    #[error("render error: {message}")]
    Render { message: String },

    // -- Serialization --
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    // -- I/O --
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
