# The formalism — everything you need to derive the invariants yourself

A figure is encoded as a **combinatorial map**: a set of `n_darts` darts, numbered `0 .. n_darts-1`.

- **`alpha`, the edge involution:** `alpha(d) = d XOR 1`. So darts pair up `(0,1)`, `(2,3)`, `(4,5)`, …
  and each pair is one undirected **edge**. Therefore `edges = n_darts / 2`.
- **`sigma`, the rotation:** `rotations` gives, for each vertex, its incident darts **in cyclic
  order**. So if a vertex lists `[a, b, c]` then `sigma(a)=b`, `sigma(b)=c`, `sigma(c)=a`.
- **Vertices** are the orbits of `sigma`. A vertex's **degree** is the size of its orbit.
  `degree_sequence` is the list of degrees, **sorted ascending**.
- **Faces** are the orbits of `phi = sigma ∘ alpha`, i.e. `phi(d) = sigma(alpha(d))`. Tracing these
  orbits is face tracing. `faces` is the number of such orbits.
- **Components:** two darts are in the same component if some sequence of `sigma` and `alpha` steps
  connects them. `components` is the number of such classes.
- **Euler characteristic:** `chi = vertices - edges + faces`.
- **Genus:** for a **connected** map, `chi = 2 - 2g`, so `g = (2 - chi) / 2`. The formula applies
  **only** to connected maps. For a disconnected map the genus is not defined by this formula.
- **`same_face` / `different_face`:** pairs of darts asserted to lie in the same / different face
  orbit.
- **Nesting and planar faces:** when a figure has several separate components drawn on one page,
  their unbounded outer faces are in reality one region. `nesting.top_level_outer_darts` names one
  dart on the outer face of each component that nothing contains; `planar_faces` is the face count
  after merging those, and `euler_planar` is `vertices - edges + planar_faces`. Where an entry has
  a `nesting` block, `same_face` / `different_face` refer to the **merged** faces.

That is the whole method. Everything in `entries/` is derivable from the `rotations` and
`n_darts` of each entry using only the above.
