# 069 — Aut(map): the bridge object, and it bounds rather than computes

**Type:** cycle
**Opened:** 2026-10-14
**Status:** closed
**Thread:** builds the object `068`'s analysis and the owner's go-ahead identified as the gate on
everything — the symbol↔formal bridge. Corrects three things I told the owner in conversation.
**Prompted by:** the finding that the repo's corpus and the imported Codex catalog shared exactly
one field, and that the obvious object connecting them — the automorphism group of the
combinatorial map — appeared twice in this repo and not at all in theirs.

---

## 1. What was built

`CombinatorialMap::automorphisms()` in `map.rs`, with `MapAutomorphisms` carrying both cosets as
explicit dart permutations:

- **Aut⁺** — `φσ = σφ` and `φα = αφ`. The combinatorial rotations.
- **Aut⁻** — `φσ = σ⁻¹φ` and `φα = αφ`. The mirrors.

Constraint-propagating backtracking: assigning `φ(d)` forces its whole connected component
through σ and α, so **disconnected maps work naturally** — the next free dart opens the next
component. A step budget makes a pathological input degrade visibly (`truncated`) instead of
hanging. `planar_symmetry(&Nesting)` restricts to automorphisms fixing the unbounded region, since
no rotation of the page moves the unbounded face into a bounded one.

**Verified against group theory rather than against its own output** — 7 tests, 377 workspace tests
passing, clippy clean: a bare edge (order 2), the triangle and square maps (D3 order 6, D4 order 8),
the 3-star (3 rotations and 3 *distinct* mirrors, order 6), two disjoint edges (2·2·2 = 8).

## 2. A bug the probing caught, and its shape is worth keeping

The first version **subtracted the Aut⁺/Aut⁻ overlap** to avoid "double counting" the group order.
That reported a **single edge as chiral** — plainly false, an edge is its own mirror.

The two sets are **cosets**: disjoint when `σ ≠ σ⁻¹`, and *identical* when σ is an involution
(every vertex of degree ≤ 2), because the reversing condition then *is* the preserving condition.
Chirality is Aut⁻ being **empty**, and subtracting the overlap destroys emptiness exactly in the
cases where the map is most obviously amphichiral. `order()` now takes the union; the regression has
a named test.

The Codex's own rule covers this: *"detectors get seeded both ways before they run at scale — every
detector in this repo over-fired the first time, one by 100×. A large flag count means suspect the
checker first."* **Mine under-fired, which is the harder direction**: a false negative presents as a
clean pass. The triangle and the 3-star both have order 6 and are structurally different, which is
why `mirror_coincides_with_rotation()` exists as a separate signal.

## 3. The bridge bounds. It does not compute. (Correction 1)

I told the owner *"Aut(map) computes exactly those."* **That is wrong.**

| figure | Codex measured | map computes |
|---|---|---|
| `cross(Line,Line)` — the "+" , orbit 91 | symmetry **4**, mirror_axes **4** | Aut⁺ **4**, Aut⁻ **4** — **equal** |
| `close(Angle)` — isoceles triangle, orbit 24 | symmetry **1**, mirror_axes **1** | Aut⁺ **6**, Aut⁻ **6** — **strictly greater** |

A combinatorial map carries **no metric.** It cannot distinguish an isoceles triangle from an
equilateral one, so its automorphism group is the symmetry of the *most symmetric realization*. The
relation is therefore:

> **map symmetry ≥ drawn symmetry**, with equality exactly when the drawing realizes the
> combinatorial symmetry.

The "+" is the equality case; the triangle is strict. **This is better than the identity I claimed,
not worse** — an inequality with a stated equality condition is checkable in both directions, and it
explains the mismatches instead of being embarrassed by them. Registered as
`MAP-SYMMETRY-BOUNDS-DRAWN-SYMMETRY`, VERIFIED with both cases as tests.

## 4. The one shared field is a false friend (Correction 2)

I reported the two corpora sharing exactly one field, `components`, and treated it as the bridge's
toehold. **It is not a toehold. The two `components` mean different things.**

The Codex's `+` records `strokes: 2, components: 2, crossings: 1` — two strokes that cross. The
formal object is **one connected graph with a degree-4 vertex**, components 1. Their count is of ink
strokes before crossings are resolved; the repo's is of graph components after.

So the vocabularies share **zero fields with matching semantics**, not one. The gap measured in
`068` was understated.

## 5. The formalism cannot express a crossing that is not a vertex (Correction 3)

This is the finding that actually matters, and it was not in my analysis at all.

The Codex's model admits ink that **overlaps without joining** — `components: 2, crossings: 1` is
precisely that. A combinatorial map is a graph embedding: every place two edges meet **is** a
vertex. There is no object in the formalism for an un-joined crossing.

So bridging requires a decision that the mathematics does not make for us:

> **Is a crossing a vertex?**

Answer yes and the "+" is one connected map, the strokes lose their identity, and a figure drawn as
two independent marks becomes one object. Answer no and the combinatorial map is the wrong
formalism — you need a knot-diagram-like structure with over/under or at least with crossings
distinguished from vertices.

**Either answer is a convention, and under `LEDGER-CLOSURE-IS-UNIVERSE-SCOPE` a convention on the
load-bearing path is a ledger line.** The bridge is not free. Registered as
`CROSSING-IS-A-LEDGER-LINE` so the cost is visible before anyone builds on top of it — this is
exactly the error note `049` found recurring in every capped notation: hard-coding a discretization
and not recording that you did.

My own recommendation to the owner was that Aut(map) *"isn't a design question where getting it
wrong bakes in a convention: the automorphism group of a combinatorial map is not a choice."* The
group is not a choice. **Which figures get encoded as which maps is.** The convention sits one level
below where I looked for it.

## 6. What this leaves

1. **Rule on the crossing question** — owner's call, because it is a ledger entry rather than an
   implementation detail. My read: answer **yes** for now (a crossing is a vertex), record the cost,
   and note that it makes `components` and `strokes` unbridgeable rather than merely mismatched.
2. **Encode figures in both vocabularies.** Still zero exist. §3's two cases were encoded *for this
   note* from the Codex's measurements; they are not corpus entries. The real test needs a set
   authored in both, and the equality condition in §3 is what it would test.
3. **A chiral witness.** Every map probed here has Aut⁻ non-empty, so `is_chiral()` has no positive
   test. The Codex derives exactly one chiral mark (the Wave) and attributes handedness to
   *placement* rather than to the marks — `A-beside-B` and `B-beside-A` as enantiomorphs. Encoding
   that and finding Aut⁻ empty would be the first genuine chirality result on this side.
4. **`symmetry: "inf"` is permanently out of reach.** Point and Circle carry continuous symmetry; a
   map is discrete. The bridge is defined only on figures with finite symmetry, and that is a scope
   condition rather than a gap to close.
