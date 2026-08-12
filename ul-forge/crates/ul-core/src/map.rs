//! Combinatorial maps — the coarse-graining fixed point.
//!
//! Seven independent lines of research converged on this object (see `STATE-OF-PLAY.md` §10):
//! it is what survives coarse-graining, it unifies separation with incidence, and grounding
//! regions in its faces makes spatial reasoning decidable.
//!
//! A combinatorial map is a triple `(D, σ, α)`:
//!
//! - `D` — **darts**, the directed half-edges. Each undirected edge contributes two.
//! - `σ` — the **vertex permutation**: the cyclic order of darts around each vertex.
//! - `α` — the **edge involution**: pairs each dart with its opposite.
//!
//! Then everything else is an orbit:
//!
//! | Object | Orbits of |
//! |---|---|
//! | vertices | `σ` |
//! | edges | `α` |
//! | **faces** | `φ = σ ∘ α` |
//!
//! By the Heffter–Edmonds principle the rotation system determines the embedding up to
//! homeomorphism, which is why this is the fixed point rather than merely a convenient encoding.
//!
//! # What GIR does not currently carry
//!
//! `Gir` stores edges as an unordered `Vec<Edge>`. **A rotation system is strictly more information
//! than that** — it is the cyclic order of edges at each vertex, and GIR has no field for it.
//! [`CombinatorialMap::from_gir`] therefore uses edge insertion order as the rotation, which is
//! **arbitrary**. Any face structure it reports is the face structure *of that arbitrary choice*.
//! Callers with real geometry should use [`CombinatorialMap::from_rotations`] and supply the
//! angular order.

use crate::types::gir::Gir;
use crate::types::node::NodeId;
use std::collections::HashMap;

/// A directed half-edge. Darts `2i` and `2i+1` are the two halves of edge `i`.
pub type Dart = usize;

/// A graph embedded on an orientable surface, given combinatorially.
#[derive(Debug, Clone, PartialEq)]
pub struct CombinatorialMap {
    /// `sigma[d]` — the next dart counter-clockwise around the same vertex.
    sigma: Vec<Dart>,
    /// Vertex label for each dart's origin, in insertion order.
    dart_origin: Vec<NodeId>,
    /// Number of darts (always even: two per edge).
    n_darts: usize,
}

impl CombinatorialMap {
    /// The edge involution: swaps the two darts of an edge.
    #[inline]
    fn alpha(d: Dart) -> Dart {
        d ^ 1
    }

    /// The face permutation `φ = σ ∘ α`.
    #[inline]
    fn phi(&self, d: Dart) -> Dart {
        self.sigma[Self::alpha(d)]
    }

    /// Build from explicit rotations: for each vertex, its incident darts in cyclic order.
    ///
    /// This is the honest constructor — a rotation system is data, and this asks for it.
    pub fn from_rotations(rotations: Vec<(NodeId, Vec<Dart>)>, n_darts: usize) -> Self {
        let mut sigma = vec![0; n_darts];
        let mut dart_origin = vec![String::new(); n_darts];
        for (vertex, darts) in &rotations {
            for (i, &d) in darts.iter().enumerate() {
                sigma[d] = darts[(i + 1) % darts.len()];
                dart_origin[d] = vertex.clone();
            }
        }
        CombinatorialMap { sigma, dart_origin, n_darts }
    }

    /// Build from a [`Gir`], using **edge insertion order** as the rotation.
    ///
    /// The rotation is therefore **arbitrary** — see the module note. This is useful for
    /// structural checks (degree, connectivity) that do not depend on the rotation, and
    /// misleading for face structure, which does.
    pub fn from_gir(gir: &Gir) -> Self {
        let mut incident: HashMap<NodeId, Vec<Dart>> = HashMap::new();
        let mut order: Vec<NodeId> = Vec::new();
        for (i, e) in gir.edges.iter().enumerate() {
            let (d0, d1) = (2 * i, 2 * i + 1);
            for (v, d) in [(&e.source, d0), (&e.target, d1)] {
                if !incident.contains_key(v) {
                    order.push(v.clone());
                }
                incident.entry(v.clone()).or_default().push(d);
            }
        }
        let rotations = order
            .into_iter()
            .map(|v| {
                let darts = incident.get(&v).cloned().unwrap_or_default();
                (v, darts)
            })
            .collect();
        Self::from_rotations(rotations, gir.edges.len() * 2)
    }

    /// Orbits of a permutation, as sorted dart sets.
    fn orbits(&self, next: impl Fn(&Self, Dart) -> Dart) -> Vec<Vec<Dart>> {
        let mut seen = vec![false; self.n_darts];
        let mut out = Vec::new();
        for start in 0..self.n_darts {
            if seen[start] {
                continue;
            }
            let mut orbit = Vec::new();
            let mut d = start;
            loop {
                seen[d] = true;
                orbit.push(d);
                d = next(self, d);
                if d == start {
                    break;
                }
            }
            out.push(orbit);
        }
        out
    }

    /// Vertices, as orbits of `σ`.
    pub fn vertices(&self) -> Vec<Vec<Dart>> {
        self.orbits(|m, d| m.sigma[d])
    }

    /// Faces, as orbits of `φ = σ ∘ α`. This is face tracing.
    pub fn faces(&self) -> Vec<Vec<Dart>> {
        self.orbits(|m, d| m.phi(d))
    }

    /// Number of undirected edges.
    pub fn edge_count(&self) -> usize {
        self.n_darts / 2
    }

    /// Local degree of the vertex containing `dart` — the junction-axis coordinate.
    ///
    /// Degree is a **topological** invariant (see `research/notes/024-junction-axis/`), so this
    /// is well defined regardless of how the rotation was chosen.
    pub fn degree(&self, dart: Dart) -> usize {
        let mut n = 0;
        let mut d = dart;
        loop {
            n += 1;
            d = self.sigma[d];
            if d == dart {
                break;
            }
        }
        n
    }

    /// The vertex label a dart originates from.
    pub fn origin(&self, dart: Dart) -> &str {
        &self.dart_origin[dart]
    }

    /// The degree sequence, sorted ascending. A rotation-independent invariant.
    pub fn degree_sequence(&self) -> Vec<usize> {
        let mut degrees: Vec<usize> =
            self.vertices().iter().map(|orbit| orbit.len()).collect();
        degrees.sort_unstable();
        degrees
    }

    /// Euler characteristic `V − E + F`.
    ///
    /// Equals 2 for a connected map on the sphere. Lower values indicate higher genus,
    /// or a disconnected map.
    pub fn euler_characteristic(&self) -> i64 {
        self.vertices().len() as i64 - self.edge_count() as i64 + self.faces().len() as i64
    }

    /// Genus, assuming the map is connected: `g = (2 − χ) / 2`.
    ///
    /// Returns `None` when `2 − χ` is odd, which indicates the map is not connected and the
    /// formula does not apply — rather than silently returning a wrong genus.
    pub fn genus(&self) -> Option<u32> {
        let d = 2 - self.euler_characteristic();
        if d < 0 || d % 2 != 0 {
            return None;
        }
        Some((d / 2) as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a cycle graph on `n` vertices directly as a rotation system.
    fn cycle(n: usize) -> CombinatorialMap {
        // Edge i joins vertex i to vertex (i+1) % n, with darts 2i (at i) and 2i+1 at (i+1).
        let mut rotations: Vec<(NodeId, Vec<Dart>)> = Vec::new();
        for v in 0..n {
            let incoming = 2 * ((v + n - 1) % n) + 1;
            let outgoing = 2 * v;
            rotations.push((format!("v{v}"), vec![outgoing, incoming]));
        }
        CombinatorialMap::from_rotations(rotations, 2 * n)
    }

    #[test]
    fn triangle_has_two_faces() {
        let m = cycle(3);
        assert_eq!(m.vertices().len(), 3, "V");
        assert_eq!(m.edge_count(), 3, "E");
        // Jordan: a simple closed curve separates the plane into exactly two components.
        assert_eq!(m.faces().len(), 2, "F — inside and outside");
    }

    #[test]
    fn cycles_satisfy_euler_and_are_genus_zero() {
        for n in 3..8 {
            let m = cycle(n);
            assert_eq!(m.euler_characteristic(), 2, "V-E+F for {n}-cycle");
            assert_eq!(m.genus(), Some(0), "{n}-cycle is planar");
        }
    }

    #[test]
    fn every_vertex_of_a_cycle_has_degree_two() {
        let m = cycle(5);
        assert_eq!(m.degree_sequence(), vec![2, 2, 2, 2, 2]);
    }

    #[test]
    fn jordan_separation_holds_for_every_simple_closed_curve() {
        // ENCLOSURE-IS-DISTINCTION, as an executable check rather than an assertion.
        for n in 3..12 {
            assert_eq!(
                cycle(n).faces().len(),
                2,
                "a simple closed curve must yield exactly two faces"
            );
        }
    }

    /// Theta graph: two vertices joined by three parallel edges.
    ///
    /// `second` is the rotation at vertex `b`. Traversing `b` in the *opposite* sense to `a`
    /// gives the planar embedding; traversing it the same way embeds on the torus.
    fn theta(second: Vec<Dart>) -> CombinatorialMap {
        CombinatorialMap::from_rotations(
            vec![("a".to_string(), vec![0, 2, 4]), ("b".to_string(), second)],
            6,
        )
    }

    #[test]
    fn theta_graph_planar_embedding_has_three_faces() {
        // Degree sequence [3,3] — degree-3 junctions, the cell the single-curve
        // classification could not see (note 023).
        let m = theta(vec![1, 5, 3]); // opposite orientation at b
        assert_eq!(m.degree_sequence(), vec![3, 3], "two degree-3 junctions");
        assert_eq!(m.vertices().len(), 2);
        assert_eq!(m.edge_count(), 3);
        assert_eq!(m.faces().len(), 3, "theta graph bounds three faces in the plane");
        assert_eq!(m.euler_characteristic(), 2);
        assert_eq!(m.genus(), Some(0));
    }

    #[test]
    fn the_same_graph_on_a_torus_has_one_face() {
        // The rotation *is* the embedding. Traversing both vertices in the same sense
        // takes the identical graph onto the torus — same vertices, same edges, same
        // degree sequence, different surface.
        //
        // This test exists because the planar test above was originally written with this
        // rotation and failed. The code was right; the expectation was wrong.
        let m = theta(vec![1, 3, 5]); // same orientation at b
        assert_eq!(m.degree_sequence(), vec![3, 3], "degree is rotation-independent");
        assert_eq!(m.faces().len(), 1);
        assert_eq!(m.euler_characteristic(), 0);
        assert_eq!(m.genus(), Some(1), "genus 1 — the torus");
    }

    #[test]
    fn origin_labels_survive_construction() {
        let m = theta(vec![1, 5, 3]);
        assert_eq!(m.origin(0), "a");
        assert_eq!(m.origin(1), "b");
    }

    #[test]
    fn rotation_changes_faces_but_not_degree() {
        // The point of the module note: degree is rotation-independent, faces are not.
        let a = CombinatorialMap::from_rotations(
            vec![("a".into(), vec![0, 2, 4]), ("b".into(), vec![1, 3, 5])],
            6,
        );
        let b = CombinatorialMap::from_rotations(
            vec![("a".into(), vec![0, 4, 2]), ("b".into(), vec![1, 3, 5])],
            6,
        );
        assert_eq!(a.degree_sequence(), b.degree_sequence(), "degree is topological");
        assert_ne!(
            a.faces().len(),
            b.faces().len(),
            "reversing a rotation changes the embedding, hence the faces"
        );
    }
}
