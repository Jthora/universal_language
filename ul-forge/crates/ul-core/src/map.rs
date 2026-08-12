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

/// Where a component sits, for a **disconnected** configuration.
///
/// # Why this exists
///
/// A rotation system determines an embedding **only for a connected graph** (Heffter–Edmonds).
/// For a disconnected configuration it does not, because the *relative placement* of components —
/// which one lies inside which face of another — is not recoverable from the rotations alone.
///
/// Without it, [`CombinatorialMap::faces`] treats each component as embedded on its own sphere:
/// two disjoint triangles trace **four** faces rather than the **three** they bound in the plane,
/// and `χ = 2c` rather than the correct `1 + c`. The result is *decidable and wrong*, which is
/// worse than undecidable because nothing signals it.
///
/// # The data
///
/// For each component: one of its own darts on its **outer** face, and — unless the component is at
/// top level — a dart on the face of the container it sits inside. Choosing an outer face is
/// choosing a point at infinity; combinatorially, no face is distinguished, so this is genuinely
/// extra information rather than something derivable.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Nesting {
    /// `(dart on this component's outer face, dart on the containing face)`.
    /// `None` container means the component sits in the unbounded region.
    placements: Vec<(Dart, Option<Dart>)>,
}

impl Nesting {
    /// All components at top level — the common case of several separate strokes.
    pub fn all_top_level(outer_darts: Vec<Dart>) -> Self {
        Nesting { placements: outer_darts.into_iter().map(|d| (d, None)).collect() }
    }

    /// Place a component (identified by a dart on its outer face) inside a given face.
    pub fn place(mut self, outer: Dart, inside: Dart) -> Self {
        self.placements.push((outer, Some(inside)));
        self
    }
}

impl CombinatorialMap {
    /// Canonical id of the face containing `dart` — the smallest dart in its `φ`-orbit.
    fn face_id(&self, dart: Dart) -> Dart {
        let mut min = dart;
        let mut d = self.phi(dart);
        while d != dart {
            min = min.min(d);
            d = self.phi(d);
        }
        min
    }

    /// Faces of a possibly-disconnected configuration, given its nesting.
    ///
    /// Traces faces per component, then **identifies each component's outer face with the face it
    /// sits in**. Top-level components share the unbounded face. This is the planar face set, and
    /// it satisfies `V − E + F = 1 + c`.
    pub fn faces_planar(&self, nesting: &Nesting) -> Vec<Vec<Dart>> {
        let raw = self.faces();
        // union-find over face ids
        let mut parent: std::collections::HashMap<Dart, Dart> =
            raw.iter().map(|f| { let id = *f.iter().min().unwrap(); (id, id) }).collect();
        fn find(p: &mut std::collections::HashMap<Dart, Dart>, x: Dart) -> Dart {
            let mut r = x;
            while p[&r] != r { r = p[&r]; }
            let mut c = x;
            while p[&c] != c { let n = p[&c]; p.insert(c, r); c = n; }
            r
        }

        // every top-level component shares one unbounded face
        let mut unbounded: Option<Dart> = None;
        for (outer, container) in &nesting.placements {
            let a = self.face_id(*outer);
            let target = match container {
                Some(c) => self.face_id(*c),
                None => match unbounded { Some(u) => u, None => { unbounded = Some(a); a } },
            };
            let (ra, rt) = (find(&mut parent, a), find(&mut parent, target));
            if ra != rt { parent.insert(ra, rt); }
        }

        let mut merged: std::collections::HashMap<Dart, Vec<Dart>> = std::collections::HashMap::new();
        for f in raw {
            let root = find(&mut parent, *f.iter().min().unwrap());
            merged.entry(root).or_default().extend(f);
        }
        let mut out: Vec<Vec<Dart>> = merged.into_values().collect();
        out.iter_mut().for_each(|f| f.sort_unstable());
        out.sort();
        out
    }

    /// Euler characteristic using the planar face set: `V − E + F`, which is `1 + c`.
    pub fn euler_characteristic_planar(&self, nesting: &Nesting) -> i64 {
        self.vertices().len() as i64 - self.edge_count() as i64
            + self.faces_planar(nesting).len() as i64
    }
}

impl CombinatorialMap {
    /// The mirror image: every rotation reversed.
    ///
    /// Reflection is the *only* arbitrary choice a rotation system leaves open — a receiver who does
    /// not share the sender's orientation convention sees either this map or its mirror, and nothing
    /// else. That is the ℤ/2 in `ROTATION-MINIMIZES-CONVENTION`.
    pub fn mirror(&self) -> CombinatorialMap {
        // reversing sigma: sigma_rev[sigma[d]] = d
        let mut sigma = vec![0; self.n_darts];
        for d in 0..self.n_darts {
            sigma[self.sigma[d]] = d;
        }
        CombinatorialMap { sigma, dart_origin: self.dart_origin.clone(), n_darts: self.n_darts }
    }

    /// A cheap invariant: the degree sequence together with the sorted multiset of face sizes.
    ///
    /// **Not a complete isomorphism invariant.** Two non-isomorphic maps can share a signature, so
    /// [`Self::convention_ambiguity`] may report `1` (achiral) for a map that is in fact chiral.
    /// That direction *understates* ambiguity, which is the safe direction for the bound but makes
    /// the achiral detection approximate. A complete test needs canonical-form computation.
    fn signature(&self) -> (Vec<usize>, Vec<usize>) {
        let mut faces: Vec<usize> = self.faces().iter().map(|f| f.len()).collect();
        faces.sort_unstable();
        (self.degree_sequence(), faces)
    }

    /// How many distinct readings a receiver faces who shares **no** orientation convention.
    ///
    /// `1` when the map is achiral under [`Self::signature`], otherwise `2`.
    ///
    /// **The bound of 2 is structural, not empirical:** a rotation system has exactly two
    /// orientations — keep σ or reverse it — so reflection is the only free choice. The ℤ/2
    /// structure is what `mirror_is_an_involution_and_preserves_degree` actually tests; this
    /// function is a consequence of it rather than independent evidence for it.
    pub fn convention_ambiguity(&self) -> usize {
        if self.signature() == self.mirror().signature() { 1 } else { 2 }
    }
}

/// Readings a receiver faces for a **label-based** encoding with `k` distinct labels and no shared
/// alphabet: every permutation of the alphabet is a consistent reading.
///
/// This is the Sₙ side of the comparison. It is a function rather than a method because it does not
/// depend on the map at all — which is itself the point: label cost is set by the alphabet, not by
/// the structure.
pub fn label_ambiguity(k: usize) -> u128 {
    (1..=k as u128).product()
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

    /// Two disjoint triangles, as one map.
    fn two_triangles() -> CombinatorialMap {
        // Edges 0,1,2 form triangle A on v0..v2; edges 3,4,5 form triangle B on v3..v5.
        let mut rotations: Vec<(NodeId, Vec<Dart>)> = Vec::new();
        for (base, off) in [(0usize, 0usize), (3, 3)] {
            for i in 0..3 {
                let v = base + i;
                let outgoing = 2 * (off + i);
                let incoming = 2 * (off + (i + 2) % 3) + 1;
                rotations.push((format!("v{v}"), vec![outgoing, incoming]));
            }
        }
        CombinatorialMap::from_rotations(rotations, 12)
    }

    #[test]
    fn degree_sequence_cannot_see_connectivity() {
        // A hexagon and two disjoint triangles have the SAME degree sequence.
        // Connectivity is therefore an axis the junction axis does not capture —
        // see research/notes/032-axis-audit/.
        let hex = cycle(6);
        let tris = two_triangles();
        assert_eq!(
            hex.degree_sequence(),
            tris.degree_sequence(),
            "same degree sequence: [2;6]"
        );
        assert_eq!(hex.vertices().len(), tris.vertices().len());
        assert_eq!(hex.edge_count(), tris.edge_count());

        // The face counts do distinguish them — but NOT in the way a planar reading expects.
        assert_eq!(hex.faces().len(), 2, "hexagon: inside and outside");
        assert_eq!(
            tris.faces().len(),
            4,
            "two triangles trace FOUR faces, not three: the map treats each component as \
             embedded on its own sphere, so the two 'outside' faces are never identified"
        );
        assert_eq!(hex.euler_characteristic(), 2, "connected planar: chi = 2");
        assert_eq!(
            tris.euler_characteristic(),
            4,
            "chi = 2c for c components — two separate spheres, not one plane"
        );
        // Heffter-Edmonds is stated for CONNECTED graphs. For a disconnected configuration the
        // rotation system does not determine a single-surface embedding: the relative nesting of
        // components is information the map does not carry. See FAILURES.md F-025.
        assert_eq!(hex.genus(), Some(0));
        assert_eq!(tris.genus(), None, "genus formula does not apply to a disconnected map");
    }


    #[test]
    fn nesting_recovers_the_correct_planar_face_count() {
        // Two disjoint triangles bound THREE regions in the plane: two insides and one
        // shared outside. Without nesting the map traces four (notes 032, 039).
        let m = two_triangles();
        assert_eq!(m.faces().len(), 4, "without nesting: each component on its own sphere");

        // Outer face of triangle A contains dart 1; of triangle B, dart 7. Both at top level.
        let n = Nesting::all_top_level(vec![1, 7]);
        assert_eq!(m.faces_planar(&n).len(), 3, "with nesting: the two outer faces are identified");

        // Euler for a planar graph with c components is 1 + c.
        assert_eq!(m.euler_characteristic_planar(&n), 3, "V - E + F = 1 + c = 3");
    }

    /// Two triangles joined by one **dummy edge** — the computational-geometry alternative.
    ///
    /// Mount, *CMSC 754* Lect. 10: the no-holes assumption "can be always satisfied by introducing
    /// some number of *dummy edges* joining each hole either to the outer boundary of the face, or
    /// to some other hole that has been connected to the outer boundary in this way."
    fn two_triangles_bridged() -> CombinatorialMap {
        let mut rotations: Vec<(NodeId, Vec<Dart>)> = Vec::new();
        for (base, off) in [(0usize, 0usize), (3, 3)] {
            for i in 0..3 {
                let v = base + i;
                let outgoing = 2 * (off + i);
                let incoming = 2 * (off + (i + 2) % 3) + 1;
                let mut darts = vec![outgoing, incoming];
                // Edge 6 (darts 12, 13) bridges v0 to v3. WHERE it is inserted in the rotation is
                // what selects the containing face — the nesting information, carried in sigma.
                if v == 0 {
                    darts.push(12);
                } else if v == 3 {
                    darts.push(13);
                }
                rotations.push((format!("v{v}"), darts));
            }
        }
        CombinatorialMap::from_rotations(rotations, 14)
    }

    #[test]
    fn a_dummy_edge_replaces_the_nesting_structure() {
        let bridged = two_triangles_bridged();

        // The bridge restores CONNECTEDNESS, which is Heffter-Edmonds' precondition (F-025).
        // So plain face tracing now works with no side structure at all.
        assert_eq!(
            bridged.faces().len(),
            3,
            "the dummy edge yields the correct planar face count from faces() alone"
        );
        assert_eq!(bridged.euler_characteristic(), 2, "connected planar: chi = 2");
        assert_eq!(bridged.genus(), Some(0), "the genus formula applies again");

        // It agrees with the Nesting route on the same configuration.
        let plain = two_triangles();
        let n = Nesting::all_top_level(vec![1, 7]);
        assert_eq!(bridged.faces().len(), plain.faces_planar(&n).len());

        // The cost, stated exactly: E is inflated by one per bridge (c - 1 in general), so any
        // edge-counting invariant must exclude dummy edges. V and F are untouched.
        assert_eq!(plain.edge_count(), 6);
        assert_eq!(bridged.edge_count(), 7, "one dummy edge for two components");
        assert_eq!(bridged.vertices().len(), plain.vertices().len(), "V unchanged");
    }

    #[test]
    fn nesting_distinguishes_side_by_side_from_contained() {
        // Same graph, same degree sequence, same face COUNT — different structure.
        // This difference is exactly what RCC-8 needs and what the rotation system alone loses.
        let m = two_triangles();
        let side_by_side = Nesting::all_top_level(vec![1, 7]);
        let contained = Nesting::all_top_level(vec![1]).place(7, 0); // B inside A's other face

        assert_eq!(m.faces_planar(&side_by_side).len(), 3);
        assert_eq!(m.faces_planar(&contained).len(), 3, "same count");
        assert_ne!(
            m.faces_planar(&side_by_side),
            m.faces_planar(&contained),
            "different face STRUCTURE — containment is visible where the count is not"
        );
    }

    #[test]
    fn nesting_is_a_no_op_for_connected_maps() {
        // A connected map already has its correct planar faces; nesting must not change them.
        let hex = cycle(6);
        let n = Nesting::all_top_level(vec![1]);
        assert_eq!(hex.faces_planar(&n).len(), hex.faces().len(), "2");
        assert_eq!(hex.euler_characteristic_planar(&n), 2);
    }


    #[test]
    fn rotation_ambiguity_never_exceeds_two() {
        // The core of ROTATION-MINIMIZES-CONVENTION: a receiver sharing no orientation convention
        // faces at most two readings, regardless of how large the structure is.
        for n in 3..14 {
            assert!(
                cycle(n).convention_ambiguity() <= 2,
                "{n}-cycle: ambiguity must not exceed 2"
            );
        }
        assert!(theta(vec![1, 5, 3]).convention_ambiguity() <= 2);
        assert!(two_triangles().convention_ambiguity() <= 2);
    }

    #[test]
    fn symmetric_maps_are_achiral_and_cost_nothing() {
        // A cycle is its own mirror, so orientation carries no information at all.
        for n in 3..10 {
            assert_eq!(cycle(n).convention_ambiguity(), 1, "{n}-cycle is achiral");
        }
    }

    #[test]
    fn label_cost_grows_factorially_where_rotation_stays_flat() {
        // The comparison the claim rests on, made concrete.
        let rotation = cycle(8).convention_ambiguity() as u128; // <= 2
        for k in [3usize, 4, 6, 8] {
            let labels = label_ambiguity(k);
            assert!(
                labels > rotation,
                "k={k}: labels {labels} should exceed rotation {rotation}"
            );
        }
        assert_eq!(label_ambiguity(8), 40_320);
        assert!(rotation <= 2, "rotation cost is flat in structure size");
    }

    #[test]
    fn mirror_is_an_involution_and_preserves_degree() {
        // Reflection is a ℤ/2 action: applying it twice returns the original.
        let m = theta(vec![1, 5, 3]);
        assert_eq!(m.mirror().mirror().signature(), m.signature(), "mirror is involutive");
        assert_eq!(m.mirror().degree_sequence(), m.degree_sequence(), "degree is reflection-invariant");
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
