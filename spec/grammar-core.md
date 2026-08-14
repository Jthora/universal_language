# GRAMMAR CORE — the derived grammar of UWS

**This directory describes what exists.** Every rule below is backed by running code and a passing
test; every rule carries its status — **derived** (forced by the substrate), **content** (a choice
that carries meaning), or **ledger** (a convention, with its cost). Nothing here is aspiration.
Derivation record: `research/notes/056`. Code: `ul-forge/crates/ul-core/src/map.rs`.

---

## 0. The object

An expression's fixed-point form is a **combinatorial map** — darts `D`, rotation `σ`, edge
involution `α` — with vertices as σ-orbits, edges as α-orbits, faces as φ-orbits (φ = σ∘α), plus a
**Nesting** relation for the planar reading of disconnected configurations. *(Status: derived —
notes `014`–`046`; the map is prior art, `MAP-IS-A-DCEL-REDUCT`.)*

## 1. Well-formedness

| Rule | Statement | Status | Check |
|---|---|---|---|
| **W1** | σ is a vertex-preserving permutation of the darts | **derived** — definitional: this is what it takes to be a map | `validate()`; test `w1_w2_validity_holds…` |
| **W2** | the dart count is even (α is total) | **derived** — an edge *is* two darts | same |
| **W3** | a configuration with more than one component denotes planar regions only together with its Nesting | **derived** — Heffter–Edmonds is stated for connected graphs (F-025); without nesting, region structure is objectively indeterminate, machine-shown by the 4-vs-3 face result | `faces_planar`; corpus `003` vs `004` |

## 2. Composition operations

| Op | Signature | What its choices are | Status | Check |
|---|---|---|---|---|
| **O1 place** | `disjoint_union` + `Nesting::place` | *which face* the new component sits in — **content**: containment vs separation is exactly this choice (corpus `005`/`006` differ in nothing else) | derived op; content-bearing argument | `o1_juxtaposition…`; corpus `005`, `006` |
| **O2 connect** | `connect(da, db)` — one new edge | *where in each rotation* the edge inserts — **content**: insertion points select the faces the edge splits (notes/046) | derived op; content-bearing arguments | `o2_connection_restores…` |
| **O3 subdivide** | `subdivide(d)` — a degree-2 vertex on an edge | none — **semantically inert**: faces, genus, components, free ends all unchanged | **derived, and inert by theorem**: metric detail dies before the fixed point | `o3_subdividing_a_triangle_gives_a_square`; `…open_strokes_too` |

**E1 (equivalence).** Expressions related by subdivision denote the same configuration — a triangle
with a subdivided edge carries exactly a square's invariants, machine-checked. *(The inverse
operation `smooth` is definable and not yet implemented; equivalence is currently generated in the
subdivision direction only — honestly labeled.)*

**Derived composites.** *Enclosure* is not a primitive: `enclose(inner, d, n)` = a cycle + O1 + a
placement into the bounded face. "Circling something means encapsulation" is a composite of the
core operations, and the meaning is checked, not stipulated *(test
`enclosure_is_a_derived_composite_not_a_primitive`)*.

## 3. The ledger for this layer

| Residue | Cost | Status |
|---|---|---|
| Global orientation | **ℤ/2** | proved and measured (`convention_ambiguity`); the layer's only convention |
| Point-at-infinity (which face is unbounded) | one designation per planar expression | **content of the planar reading**, carried explicitly by Nesting (notes/032) — a drawing determines it; the formalism refuses to guess it |
| Dart numbering, vertex names | none | presentation, not structure (`label_ambiguity` measures what *labels-as-content* would cost; here they carry nothing) |

## 4. Lexicon V1 — five meanings whose structural facts are theorems

Corpus entries `005`–`009`, each machine-verified by the harness on every CI run:

| Entry | Meaning | The theorem-checked fact |
|---|---|---|
| `005` | **containment** | segment co-facial with the ring's bounded region, not the unbounded one (Jordan) |
| `006` | **separation** | the minimal pair: identical everything, opposite co-facality |
| `007` | **adjacency** | a shared edge's two darts lie in two distinct bounded regions (RCC-8's EC, structurally) |
| `008` | **junction + free end** | degree 3 at the meet, degree 1 at each terminus — the derived symbol inventory in use |
| `009` | **multiplicity** | component count = 3; V − E + F = 1 + c |

**Scope, stated plainly:** what is VERIFIED is the *structural* side — the regional and degree facts.
That these structures *mean* containment, adjacency, count **to a reader** is the M1 exemplification
route (`SEMANTIC-STACK-M1-M2-M3`, CONJECTURED), whose own keystone is `READING-INVARIANCE-TARGET`.
The tier labels travel with the content, here as everywhere.

## 5. Known limits

- **Isolated points are not representable.** A degree-0 vertex has no darts, so the dart formalism
  cannot see it — the legacy `Point` primitive at degree 0 currently has no fixed-point form.
  Enriching the object (an explicit vertex set alongside `D`) is open, and flagged rather than
  papered over.
- `smooth` (E1's inverse) and a full isomorphism check (beyond invariant equality) are not yet
  implemented.
- This is the grammar's **core**: the operations and validity of configurations. The assertion
  layer (truth-valued expressions), quantification, and the M2 execution semantics are not part of
  this document because they do not yet exist.
