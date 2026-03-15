use serde::{Deserialize, Serialize};
use std::fmt;

/// The four sorts of Σ_UL.
///
/// Every node in a GIR document has a sort that determines which operations
/// it can participate in. Sorts are the type system of Universal Language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Sort {
    /// Things that exist — points, enclosures, nominalized assertions.
    Entity,
    /// Directed connections — lines, curves, composed relations.
    Relation,
    /// Qualities that modify entities or relations — angles.
    Modifier,
    /// Complete statements — predicates, negations, conjunctions.
    Assertion,
}

impl Sort {
    /// Short symbol used in compact representations.
    pub fn symbol(self) -> &'static str {
        match self {
            Sort::Entity => "e",
            Sort::Relation => "r",
            Sort::Modifier => "m",
            Sort::Assertion => "a",
        }
    }
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sort::Entity => write!(f, "entity"),
            Sort::Relation => write!(f, "relation"),
            Sort::Modifier => write!(f, "modifier"),
            Sort::Assertion => write!(f, "assertion"),
        }
    }
}
