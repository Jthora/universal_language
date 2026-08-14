# 056 — Construction items 1 and 2: the grammar core, and the first derived-meaning lexicon

**Type:** cycle
**Opened:** 2026-08-12
**Status:** closed
**Thread:** executes Construction items 1–2 of `055`
**Question:** can the grammar be *derived* rather than designed — and can lexicon entries exist
whose meanings are checked rather than stipulated?

**Yes to both, and both are now code.** First entry in `spec/` (`grammar-core.md`), seven new
grammar tests, five lexicon entries in the corpus — all green, all on CI.

---

## 1. The derivation, compressed

**Well-formedness fell out of the object.** W1/W2 are definitional — what it takes to *be* a map —
and W3 (nesting required for disconnected planar readings) is F-025's connected-precondition
finding, now a grammar rule instead of a scar.

**The composition inventory is three operations, and the discovery is where the meaning sits:**

> **Every choice an operation takes is content, not convention.** O1's choice — *which face* — is
> the difference between containment and separation. O2's choice — *where in the rotation* — selects
> which regions the new edge splits (exactly `046`'s dummy-edge finding, promoted from remark to
> API). O3 takes no choice at all and is **semantically inert** — the machine-checked form of
> "metric detail dies before the fixed point": `subdivide(triangle)` carries a square's invariants,
> asserted in a test.

**The ledger for the layer closes at ℤ/2** — the already-proved orientation residue. Nothing new
went on it. **That is the grammar-side of literal universality holding**, at least for the core.

**And the legacy placement grammar was re-derived on the way:** `uws/`'s "spatial placement" is O1's
content-bearing argument — the Nesting relation. What the designed notation asserted, the substrate
now produces. (Module S's re-grounding just became an editing job rather than a research job.)

**Enclosure is a composite, not a primitive.** `enclose = cycle + O1 + bounded-face placement` —
the owner's *"circling something has clear meaning of encapsulation"* is now a derived operation
whose meaning is a checked co-facality fact.

## 2. Lexicon V1 — the entries and the honest scope

Five meanings: **containment, separation, adjacency, junction-with-free-ends, multiplicity**
(corpus `005`–`009`). The pair `005`/`006` is the jewel: **identical components, identical degree
sequence, identical face count — the entire meaning difference is which region the segment
shares.** A minimal pair in the linguist's sense, and the contrast is machine-verified.

**Scope, per the spec:** VERIFIED covers the *structural* side. That the structures mean these
things *to a reader* is M1 (CONJECTURED), resting on reading-invariance. The claim registry says
exactly this and no more.

## 3. Findings that came out of the build

- **Isolated points are unrepresentable** — a degree-0 vertex has no darts. The legacy `Point`
  primitive at degree 0 has **no fixed-point form** in the current object. Options when it
  matters: enrich the object with an explicit vertex set (standard in the literature), or
  register degree-0 as outside the core. Flagged in the spec; not decided here.
- **The harness needed `placements` support** (containment requires placing one component inside
  another's face) — added, with the co-facality checks (`same_face` / `different_face`) that give
  the corpus regional semantics.
- `path(n)` joined `cycle(n)` as a public constructor — the open stroke with its **degree-1 free
  ends**, so the unnamed primitive of `024` is now a first-class citizen of the API.

## 4. What changed

- `map.rs`: `validate`, `components`, `cycle`, `path`, `disjoint_union`, `connect`, `subdivide`,
  `enclose` + seven tests. Workspace: **362 tests.**
- `corpus/`: entries `005`–`009`; harness extended (components, placements, co-facality).
- `spec/grammar-core.md`: **the first file in `spec/`** — every rule with
  derived/content/ledger status and a test pointer.
- `claims.yaml`: `GRAMMAR-CORE-IS-DERIVED` (ARGUED), `SUBDIVISION-IS-INERT` (VERIFIED),
  `LEXICON-V1-FIVE-MEANINGS` (VERIFIED, scope-split per M1).
- **Open:** `smooth` (E1's inverse); isomorphism beyond invariants; the degree-0 decision; the
  assertion layer — which is where Construction item 3 (the M2 engine) picks up.
