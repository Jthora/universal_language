//! Distinguished elements registry for Universal Language.
//!
//! Contains the 7 modal distinguished elements + 2 performative elements
//! defined in formal-foundations.md §7–§8. These are pre-built Gir values
//! that compose into modal and performative operators.

use crate::types::gir::Gir;
use crate::types::node::Node;
use crate::types::sort::Sort;
use std::collections::HashMap;

/// A named distinguished element: a pre-built Gir with documentation.
#[derive(Debug, Clone)]
pub struct DistinguishedElement {
    /// Short identifier (e.g. "w_current").
    pub name: &'static str,
    /// Human-readable description.
    pub description: &'static str,
    /// The element encoded as a single-node Gir.
    pub gir: Gir,
}

/// Registry of all distinguished elements in Σ_UL.
#[derive(Debug, Clone)]
pub struct DistinguishedRegistry {
    elements: HashMap<&'static str, DistinguishedElement>,
}

impl DistinguishedRegistry {
    /// Create a new registry pre-populated with all 9 distinguished elements.
    pub fn new() -> Self {
        let mut elements = HashMap::new();

        // ── Modal distinguished elements (formal-foundations.md §7) ──

        let w_current = make_entity("w_current", "The actual world — where assertions are evaluated by default");
        elements.insert("w_current", w_current);

        let r_satisfies = make_relation("r_satisfies", "The satisfaction relation — world makes assertion true (w ⊨ a)");
        elements.insert("r_satisfies", r_satisfies);

        let r_alethic = make_relation("r_alethic", "Alethic accessibility — logically/metaphysically possible worlds");
        elements.insert("r_alethic", r_alethic);

        let r_k_agent = make_relation("r_K_agent", "Epistemic accessibility — worlds consistent with what agent knows");
        elements.insert("r_K_agent", r_k_agent);

        let r_o = make_relation("r_O", "Deontic accessibility — worlds consistent with obligations");
        elements.insert("r_O", r_o);

        let r_ability = make_relation("r_ability_agent", "Ability accessibility — worlds the agent can bring about");
        elements.insert("r_ability_agent", r_ability);

        let r_closeness = make_relation("r_closeness", "Lewis closeness ordering — ranks world similarity for counterfactuals");
        elements.insert("r_closeness", r_closeness);

        // ── Performative distinguished elements (formal-foundations.md §8) ──

        let e_speaker = make_entity("e_speaker", "The speaker — discourse participant producing the utterance");
        elements.insert("e_speaker", e_speaker);

        let e_hearer = make_entity("e_hearer", "The hearer — discourse participant receiving the utterance");
        elements.insert("e_hearer", e_hearer);

        DistinguishedRegistry { elements }
    }

    /// Look up a distinguished element by name.
    pub fn get(&self, name: &str) -> Option<&DistinguishedElement> {
        self.elements.get(name)
    }

    /// Get the Gir for a distinguished element by name.
    pub fn gir(&self, name: &str) -> Option<&Gir> {
        self.elements.get(name).map(|e| &e.gir)
    }

    /// List all registered element names.
    pub fn all_names(&self) -> Vec<&'static str> {
        let mut names: Vec<_> = self.elements.keys().copied().collect();
        names.sort();
        names
    }

    /// List all registered elements.
    pub fn all(&self) -> Vec<&DistinguishedElement> {
        let mut elems: Vec<_> = self.elements.values().collect();
        elems.sort_by_key(|e| e.name);
        elems
    }
}

impl Default for DistinguishedRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience: create the default registry.
pub fn default_registry() -> DistinguishedRegistry {
    DistinguishedRegistry::new()
}

// ── Helpers ──

fn make_entity(name: &'static str, description: &'static str) -> DistinguishedElement {
    let node = Node::point(name).with_label(name).with_sort(Sort::Entity);
    let gir = Gir::new(name, vec![node], vec![]);
    DistinguishedElement { name, description, gir }
}

fn make_relation(name: &'static str, description: &'static str) -> DistinguishedElement {
    let node = Node::line(name, true).with_label(name).with_sort(Sort::Relation);
    let gir = Gir::new(name, vec![node], vec![]);
    DistinguishedElement { name, description, gir }
}

// ── Tests ──

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_has_9_elements() {
        let reg = DistinguishedRegistry::new();
        assert_eq!(reg.all_names().len(), 9);
    }

    #[test]
    fn lookup_w_current() {
        let reg = default_registry();
        let elem = reg.get("w_current").unwrap();
        assert_eq!(elem.name, "w_current");
        assert_eq!(elem.gir.nodes.len(), 1);
        assert_eq!(elem.gir.nodes[0].sort, Sort::Entity);
    }

    #[test]
    fn lookup_r_alethic() {
        let reg = default_registry();
        let elem = reg.get("r_alethic").unwrap();
        assert_eq!(elem.gir.nodes[0].sort, Sort::Relation);
    }

    #[test]
    fn lookup_e_speaker() {
        let reg = default_registry();
        let elem = reg.get("e_speaker").unwrap();
        assert_eq!(elem.gir.nodes[0].sort, Sort::Entity);
    }

    #[test]
    fn all_names_sorted() {
        let reg = default_registry();
        let names = reg.all_names();
        let mut sorted = names.clone();
        sorted.sort();
        assert_eq!(names, sorted);
    }

    #[test]
    fn gir_shorthand() {
        let reg = default_registry();
        assert!(reg.gir("r_satisfies").is_some());
        assert!(reg.gir("nonexistent").is_none());
    }

    #[test]
    fn all_elements_valid() {
        let reg = default_registry();
        for elem in reg.all() {
            assert!(!elem.gir.nodes.is_empty(), "Element {} has no nodes", elem.name);
            assert_eq!(elem.gir.root, elem.name, "Root ID mismatch for {}", elem.name);
        }
    }
}
