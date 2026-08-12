# 034 — The bridge test: does the fixed point appear outside UWS?

**Type:** cycle
**Opened:** 2026-08-01
**Status:** closed
**Thread:** follows `033`, which supplied the criterion that reorders everything
**Superseded by:** `038`  ← *the one permitted edit to a closed note (`../README.md`)*

---

## 0. The backlog, re-sorted by `033`'s criterion

**Test: a UL result should survive UWS being wrong.**

| Item | Survives UWS being wrong? | Priority |
|---|---|---|
| **Does the combinatorial map appear outside UWS?** | ✅ — it *is* the question | **1** |
| Identify **G** and **H** for the order parameter | ✅ | 2 |
| Coarse-grain independently emerged protocols | ✅ | 3 |
| Connectivity + orientation axes (topological) | ✅ | 4 |
| Derive topological axes from invariant theory | ✅ | 5 |
| — | | |
| Re-derive curvature on rotation number | ❌ microscopic | low |
| Refine the singularity axis | ❌ microscopic | low |
| Phase 5 code, semantic equality | *engineering, not UL* | parallel track |

**The curvature repair from `033` drops in priority the moment its own criterion is applied.** It
fixes a real defect in one notation's microscopic layer, and by universality that layer cannot
generalize. Recording that explicitly because it is the first time the criterion has cost something
I had just finished arguing for.

---

## 1. Before searching  ← written first (S2)

**The question:** the fixed point is a **combinatorial map** — a graph plus a *cyclic ordering of
branches at each vertex*. If that is UL's fixed point rather than a fact about drawing in a plane,
it should appear in **independently developed semantic formalisms** that owe nothing to UWS.

**Candidates:** Conceptual Graphs, AMR, semantic networks, dependency grammar, RDF, frame semantics.
All are graph-based and were developed independently of this project.

**What I expect, and it goes against us.** Semantic formalisms almost certainly use **unordered
graphs with labelled edges** — argument order is carried by *role labels* (agent, patient, ARG0,
ARG1), not by *cyclic position*. Labelled roles and cyclic rotation are **different structures**.

**So I predict the rotation system does NOT appear**, and that the honest reading would be:

> **The combinatorial map is a fact about marks in a plane, not about meaning. The fixed point would
> then be UWS's fixed point, not UL's — and the `meaning → map` gap from `028` would be not merely
> unbuilt but structural.**

**Would change the plan if:** any independently developed semantic formalism carries cyclic order at
nodes as load-bearing structure. That would be genuine convergent evidence and would make the bridge
a research target rather than a suspected impossibility.

**Recording the asymmetry deliberately (T6):** this is the first test in a while whose expected
outcome damages the position. If it comes back favourable I should scrutinize it *harder* than if it
comes back against, because favourable-and-surprising is the combination most likely to be
motivated reading.

## 2. Searches run

| Query | Direction | Result |
|---|---|---|
| Graph-based semantic formalisms, argument order, labelled roles | **adversarial (R1)** — testing my own position | **Labels, not order** |
| Does node ordering carry meaning in semantic graphs? | supporting | Ordering matters for **comprehension**, i.e. secondary notation |

**Counter-evidence: easy to find.** (S7)

## 3. Findings — the prediction held, and it goes against us

### 3.1 Semantic formalisms use labelled edges, and explicitly discard order

> AMR represents sentences as *"rooted, directed acyclic graphs where vertices represent concepts and
> **labelled arcs** are used to represent semantic relationships."* Arguments come from PropBank's
> *"frame-specific core argument roles (named **ARG0, ARG1**)."*
>
> And decisively: *"**AMR annotation… abstracts away from elements of surface syntactic structure
> such as word order.**"*

**Independently developed semantic formalisms distinguish a vertex's edges by *label*, not by
position — and AMR discards order deliberately.** Conceptual Graphs are the same family.

**The rotation system does not appear as semantic structure anywhere outside UWS.**

### 3.2 Where spatial order does appear, it is secondary notation

Node ordering matters *"for generating legible and informative tree layouts,"* with *"similarity,
readability, and stability"* the criteria, and improves *"visual saliency of clusters."*

**Those are comprehension criteria, not semantic ones.** In the diagrammatic-reasoning vocabulary
this is **secondary notation** — layout that aids reading without being part of the formal content.
**In UWS, spatial arrangement is claimed to be primary.** No other formalism makes that claim.

### 3.3 The steelman, per R6 — and it survives, weakened

**Are labelled edges and rotation systems inter-translatable?** Partly.

- **Rotation → labels:** trivial. Number the edges by their cyclic position.
- **Labels → rotation:** **requires extra data** — you must choose an order on the label alphabet.
  Nothing in ARG0/ARG1/ARG2 supplies a *cyclic* successor.

> **So rotation is strictly more structure than labelled distinguishability.** The two are not
> equivalent; one refines the other.

**Which sharpens the question rather than settling it:**

> **Either UWS's extra structure — cyclic order — is semantically real and every other formalism is
> missing it, or it is spatial overhead that carries no meaning.**

And there is a genuine argument for the first, from `013`/`019`: **cyclic order is intrinsic, labels
require a shared vocabulary.** Two systems can agree on "clockwise from here" without agreeing on a
label alphabet — which is exactly what cross-population intelligibility needs. **That is an argument,
not evidence.**

## 4. What this costs

**The convergent-evidence form of the bridge fails.** Nobody else arrived at rotation systems, so the
map cannot be presented as something independent traditions converged on. `DESSINS-PRECEDENT` shows
combinatorial maps carry deep *mathematical* content; this shows they carry no *semantic* content
anyone else found.

**Honest consequence for `FIXED-POINT-IS-COMBINATORIAL-MAP`:** it stands as UWS's fixed point. Calling
it **UL's** fixed point now requires the argument in §3.3 to be made and defended, and that argument
is currently unsupported.

**The `meaning → map` gap from `028` is not merely unbuilt — this is the first positive evidence that
it may be structural.** Not proof. But the cheapest available check ran, and it did not go our way.

## 5. What changed

- `claims.yaml`: `SEMANTIC-FORMALISMS-USE-LABELS-NOT-ORDER` added;
  `FIXED-POINT-IS-COMBINATORIAL-MAP` scoped — UWS's fixed point, not established as UL's;
  `UL-WORK-IS-FIXED-POINT-WORK` gains the caveat that fixed-point work is UL-relevant *only if* the
  fixed point is UL's.
- **The live question, now well-posed:** is cyclic order semantically load-bearing, or spatial
  overhead? The intrinsic-vs-shared-vocabulary argument is the only thing standing on that side, and
  it needs testing rather than repeating.
