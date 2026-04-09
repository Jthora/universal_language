//! # UL Core
//!
//! Core library for Universal Language: types, parser, validator, and renderer.
//!
//! The central data structure is [`Gir`] — the Glyph Intermediate Representation,
//! a typed graph with a tree spine that encodes meaning through 5 geometric primitives.

pub mod composer;
pub mod distinguished;
pub mod error;
pub mod lexicon;
pub mod modal;
pub mod parser;
pub mod performative;
pub mod pragmatic;
pub mod renderer;
pub mod types;
pub mod validator;

pub use error::{UlError, UlResult};
pub use types::edge::{Edge, EdgeType};
pub use types::gir::Gir;
pub use types::node::{AssertionModifierKind, EnclosureShape, Node, NodeType, PerformativeForce};
pub use types::sort::Sort;
pub use distinguished::{DistinguishedRegistry, default_registry};

// Layout geometry (P2: exposed for ul-game and downstream consumers)
pub use renderer::{compute_layout, Connection, PositionedElement, PositionedGlyph, Shape};
