# 046 — Volley 2: the synonym sweep

**Type:** cycle
**Opened:** 2026-08-12
**Status:** closed
**Thread:** executes Volley 2 of `044`; completes the sweep `043` prescribed as **S9**
**Question:** what do the other four vocabularies for our fixed-point object already know?

`043` found we discovered *dessins* by accident thirty notes late. A scan then found **DCEL and
doubly-connected edge list had never appeared in this project at all.** Three queries, hard stop.

---

## 1. Before searching  ← written first (S2)

**The specific prediction, and it is uncomfortable:** `map.rs` reinvents the **DCEL** — the standard
computational-geometry structure for planar subdivisions, textbook since the 1970s.

**Sharper still:** I predict DCEL's standard treatment of disconnected subdivisions is a **face
record carrying an outer boundary plus a list of inner boundaries (holes)** — which would make our
`Nesting` type, derived painfully across notes `032`, `039` and `040`, **a reinvention of a
forty-year-old field in the data structure.**

**Expected secondary finds:** known pitfalls — unbounded-face handling, degeneracies,
floating-point robustness in the geometric predicates.

**What would genuinely surprise me:** rotation systems carrying *semantic* interpretations in any of
these fields. `034` concluded "no formalism uses rotation systems for meaning" **without searching
ribbon graph, fat graph or DCEL** — so a hit there would reopen the bridge test.

**Stop condition:** three queries. Coverage of vocabularies, not depth in any one.

## 2. Searches run

| # | Query | Outcome |
|---|---|---|
| 1 | DCEL / planar subdivision / face record / inner components / disconnected | **prediction confirmed**; escalated to a primary read (S8) |
| 2 | ribbon graph · fat graph · rotation system — semantics beyond topology | **predicted surprise did not occur** |
| 3 | combinatorial map — image analysis, spatial databases | convergence, on a narrower claim than it first appears |

**Primary read:** Mount, *CMSC 754* Lecture 10, "The Doubly-Connected Edge List" (Spring 2020) —
fetched and read in full rather than summarized.

---

## 3. The prediction was right, and the reinvention is exact

### 3a. The structural correspondence

| DCEL (Mount) | `map.rs` |
|---|---|
| `e.twin` — the oppositely-directed half-edge | `alpha(d) = d ^ 1` |
| `e.next` — next edge CCW about the incident face | `phi(d) = sigma[alpha(d)]` |
| rotation at a vertex — *implicit*, via `next`∘`twin` | **`sigma` — stored primitively** |
| `f.inc_edge` — an explicit face record | **derived**, as a `phi`-orbit |
| `v` stores **"its coordinates and identity"** | **coordinates absent** |

**The two structures carry the same information and disagree about which half is primitive.** DCEL
stores `next`/`prev` and recovers the rotation; we store the rotation and recover `next`. They are
inter-derivable, so this is not a difference in content.

**The one real difference is that DCEL vertices carry coordinates and ours do not.** That is not
originality — it is the ordinary observation that **our object is a known reduct of a known
structure**, and the field already treats the records as separable: *"In some applications, it is
not necessary to know either the face or vertex information (or both) at all, and if so these
records may be deleted."*

> **Recorded plainly: `map.rs` is a DCEL with the geometry deleted and the rotation promoted. The
> combinatorial content is not ours and never was.**

### 3b. And the `Nesting` derivation was a re-derivation — of the *second* standard answer

The field has **two** solutions where notes `032`, `039` and `040` found one.

**Solution 1 — inner-boundary lists.** de Berg *et al.*: a face has *"zero or more inner
boundaries."* **This is our `Nesting`**, arrived at independently and three notes late.

**Solution 2 — dummy edges.** Mount, verbatim:

> *"We will assume that the faces of complex do not have holes inside of them. (More formally, we
> say that the boundary of each face is **simply connected**.) **This assumption can be always
> satisfied by introducing some number of dummy edges joining each hole either to the outer boundary
> of the face, or to some other hole that has been connected to the outer boundary in this way.**
> With this assumption, we may assume that the edges bounding each face form a single cyclic list."*

**Solution 2 is the one we did not find, and it is pointed straight at F-025.** F-025 was the
failure of dropping *"connected"* from Heffter–Edmonds while quoting it. The reason we needed
`Nesting` at all **is** that missing precondition — and the field's standard move is to
**restore the precondition by construction** rather than to build a side structure around it.

### 3c. Checked, not asserted

I have proposed my own repairs unchecked before (**F-020**), so this one is machine-verified —
`map.rs::a_dummy_edge_replaces_the_nesting_structure`, passing:

- one dummy bridge across two disjoint triangles gives **3 faces from `faces()` alone**, no
  `Nesting` argument;
- **χ = 2** and `genus() = Some(0)` — the genus formula, inapplicable to the disconnected map,
  applies again;
- it **agrees exactly** with `faces_planar` on the same configuration;
- the cost is stated exactly: **E inflated by one per bridge** (c−1 in general), **V and F
  untouched**, so any edge-counting invariant must exclude dummy edges.

**And the load-bearing detail:** *where* the bridge dart is inserted into the rotation is what
selects the containing face.

> **The dummy edge puts the nesting information back into σ. `Nesting` keeps it outside.** Both are
> correct; only one needs a second data structure, and it is not ours.

**This is not being applied yet.** `Nesting` is tested and working, and E-inflation touches
`euler_characteristic`. It is recorded as the field's alternative with its cost measured.

### 3d. The pitfalls arrived as predicted

- **General position** is an explicit assumption: *"no two vertices share the same location, and no
  two edges are collinear."* We have never stated a degeneracy policy.
- **Face construction is the hard part, and the warning is specific:** *"it is generally not
  possible to know the face structure at the moment that the sweep is advancing, without looking
  'into the future' of the sweep to see whether regions will merge."* Our faces are traced from a
  finished map, so this does not bite yet — **it bites the moment anything builds a map
  incrementally.**

**Volley 2 predicted it would cost originality and save engineering. It did both.**

---

## 4. The surprise did not occur — and one word nearly caused an import

**No formalism in any of the three vocabularies uses rotation systems to carry meaning.** What the
cyclic order buys is stated the same way everywhere: *"the cyclic order of edges at a vertex
corresponds to the orientation of the plane"*, and the ribbon construction *"associates an oriented
surface with boundary"* by pasting rectangles to disks. **Orientation, not semantics.**

**The near-miss, recorded because it is exactly the shape we keep falling for:** the GIS literature
returned *"Feature semantic/spatial aggregation"*. Checking the source's scope before citing it
(**S11**) — "semantic" there means **feature attributes and level-of-detail aggregation**, not
compositional meaning. **A word match, not a hit.** That is the card that looks like a brick, and it
would have been a false positive on the one question this volley was meant to test.

> **`034`'s negative survives the synonym sweep.** All five vocabularies have now been searched —
> rotation system, combinatorial map, ribbon graph, fat graph, dessin d'enfant, plus DCEL as a
> sixth. **It has been upgraded from an unchecked negative to a checked one**, which is the whole
> point of S9 and is worth more than the confirmation feels like.

---

## 5. Two things found sideways

### 5a. The orientation axis splits in two — and the first draft of this section was wrong

`032` flagged an orientation axis and **never derived it**; `024` recorded that the map is
determined *"up to reflection unless orientation is fixed."*

> Ribbon graphs are *"equivalent in power to **signed rotation systems**"* — a rotation system
> **plus a sign on each edge**.

**I first wrote that the reflection ambiguity simply *is* that sign, and that Volley 5's third open
question had collapsed into a literature lookup. That is false, and it is F-023's shape exactly** —
importing a formalism without checking what its parts do.

| | What it is | What it is for |
|---|---|---|
| **Global reflection** | reverse **every** rotation → the mirror map | which of the plane's **two orientations** the sender used; the ℤ/2 in `convention_ambiguity()` |
| **Local edge signs** | a sign per edge: does traversing it **reverse local orientation**? | embeddings in **non-orientable** surfaces |

**Setting every sign to minus is not the same operation as reversing every rotation.** The two
coincide nowhere in particular, and UWS is drawn on a plane — which is orientable — so the local
signs may have **no content here at all.**

> **The finding is the split, not an identification.** There are two candidates for the orientation
> axis where the project has been carrying one, and which is the right one is undetermined. **Volley
> 5's third question is better posed and no closer to answered.**

**How it was caught, which is the part worth keeping:** `check-propagation.rb` refused the new claim
until I recorded a review against `FIXED-POINT-IS-COMBINATORIAL-MAP`, whose LIMITS paragraph states
the reflection ambiguity precisely. **Reading the hub scope because a checker demanded it is what
exposed the conflation.** The rule was built to stop *stale* scopes propagating into new claims; it
caught a defect *in the new claim* instead. **That is a second use it was not designed for, and it
is the second time a checker has outperformed a careful reading of my own text.**

### 5b. Convergence — but only on the claim that was already argued

Combinatorial maps are load-bearing in **computational geometry** (DCEL), **3D GIS and spatial
databases**, **image segmentation** — where they compute *"Betti numbers, which are used to control
the number of cavities or tunnels"* — **moduli spaces of Riemann surfaces**, and **QFT**.

**Five unrelated fields, one object.** That is real, and it is *not* the finding it looks like:

> **It supports "the map is the canonical representation of a topological partition of space" —
> which was already ARGUED and is now better supported. It does not support "the map is where
> meaning lives."** Those are two claims and collapsing them is the **F-027** move exactly.

---

## 6. The process failure — and it is worse than "we never searched"

`044` recorded that DCEL *"never appeared in this project at all."* **That was wrong in the
direction that hurts.** `map.rs:9` and `map.rs:37` both say **half-edge**, in our own doc comments,
and `030` says it too:

> *"darts as directed **half-edges**"*

**The synonym was already written in our own source, and was never used as a search term.** S9 asks
for a sweep before claiming novelty; this is narrower and sharper — **a term we had already typed
went unsearched for sixteen notes.** Filed as **F-029**, and it earns **S12**.

---

## 7. What changed

- `claims.yaml`: `FIXED-POINT-IS-COMBINATORIAL-MAP` gains DCEL prior art and drops any implicit
  novelty; `MAP-IS-A-DCEL-REDUCT` added (VERIFIED); `NESTING-HAS-A-STANDARD-ALTERNATIVE` added
  (VERIFIED); `ORIENTATION-SPLITS-INTO-TWO-CANDIDATES` added (ARGUED, and corrected before
  registration); `034`'s negative marked **checked across six vocabularies**.
- `FAILURES.md`: **F-029** — a synonym present in our own source went unsearched.
- `RESEARCH-PROTOCOL.md`: **S12** — every technical term we have written is a search term we owe.
- `map.rs`: `a_dummy_edge_replaces_the_nesting_structure` — the field's alternative, verified.
- **Open:** whether to migrate `faces_planar` to dummy edges. Measured, not decided — E-inflation
  touches `euler_characteristic`, and `Nesting` currently works.
- **Open:** no degeneracy policy exists. Harmless now; required before anything builds maps
  incrementally.
