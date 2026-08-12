# 029 — Phase 4: spatial semantics, the Löb decision, and the entrenchment ordering

**Type:** decision
**Opened:** 2026-08-01
**Status:** closed
**Thread:** follows `028`; executes Phase 4 items B, E2, E1

---

## 1. Before searching  ← written first (S2)

**B — mereotopology / RCC.** Expect RCC-8: eight jointly exhaustive, pairwise disjoint relations over
regions, with region and a connection predicate as primitives, and known complexity results.

**The suspicion I am committing to before checking:** `024` established the fixed point is a
combinatorial map, which **already determines its faces** by face tracing. If RCC relations between
faces are *derivable from the map*, then **RCC is not a semantics to adopt — it is already implied**,
and B closes as "we have it" rather than as a design choice.

**The tension to watch:** RCC is **point-free** (Whitehead lineage, `016`), while a combinatorial map
has **vertices as primitive**. Those are opposite primitivity choices. Either they are complementary
— vertices as incidence, faces as regions — or adopting RCC means reversing `024`.

**E2 — the Löb decision.** Not research; a decision to be recorded with alternatives. Expect the
answer to be shaped by `021`: *"soundness involves semantic maps beyond the logic's a priori
control."*

**E1 — the entrenchment ordering.** AGM guarantees a rational repair operator *given* a faithful
preorder. **Prediction: the ordering should not be stipulated but derived** — the coarse-graining
survival order from `014`/`022` is already a total-ish ranking of structural robustness, and using it
would make entrenchment a consequence of the geometry rather than a free parameter.

**Objects mathematical?** B partly. E1 and E2 are decisions constrained by theorems already in hand.

## 2. B — RCC is not adopted. It is derived, and that buys decidability.

**RCC-8**, confirmed: primitive is a binary `C(x,y)` — *"x connects with y"* — over non-null regions,
yielding eight jointly exhaustive, pairwise disjoint relations (DC, EC, PO, EQ, TPP, NTPP, TPPi,
NTPPi).

**The complexity is bad:**

> Consistency checking for RCC-8 is **NP-complete**; entailment **co-NP-complete**. And *"while the
> **entire RCC framework is undecidable**, subsets can be defined that are decidable."*

### 2a. The suspicion from §1 was right

`024` established the fixed point is a combinatorial map, whose **faces are recoverable by face
tracing**. Faces of a plane graph are the connected components of ℝ² \ K — **open, disjoint, and
exhaustive.**

**So take regions to be unions of faces.** Then:

- The region domain is the **finite Boolean algebra 2^F** over the face set F.
- Every RCC-8 relation between two such regions is a **finite check**.
- **Undecidability and NP-completeness both come from arbitrary regions in continuous space. A finite
  domain has neither.**

> **RCC is not a semantics to adopt. It is already implied by the IR, and grounding it in the map's
> faces makes it decidable.**

### 2b. And the point-free tension dissolves

§1 flagged that RCC is point-free (Whitehead lineage) while a combinatorial map has vertices as
primitive — apparently opposite choices.

**They are complementary, not competing.** One object supplies both: **vertices give incidence,
faces give regions.** We get region-based spatial semantics *without* abandoning points, because the
map carries both layers. `016`'s two traditions are unified by the same object that unified
separation and incidence in `024`.

**Decision: regions are unions of faces of the combinatorial map. No new primitives. RCC-8 as the
relation vocabulary, decidable by construction.**

## 3. E2 — the Löb decision: the Cure is a comparator, not a self-prover

The options in `018` were: external-and-stronger, an accepted descending strength chain, or abandon
proof-based self-trust.

**The decision refines the third**, on `021`'s finding:

> *"A logical agent cannot make a priori claims about the soundness of its own reasoning because
> **soundness involves semantic maps beyond the logic's a priori control**."*

Combined with `007`'s split:

| Question the Cure asks | Löb |
|---|---|
| *"Is my representation self-consistent?"* | **bites** |
| *"Does my representation match the fixed structure?"* | **never fires** |

> **DECISION: the Cure's trust derives from the external anchor, not from self-proof. It checks
> representations against a fixed mathematical structure it did not author. It is a comparator.**

**Consequences, and they are binding:**

1. **The Cure must not be given the job of proving its own repair policy sound in general.** That is
   self-referential and Löb blocks it. It verifies *individual repairs against the anchor*.
2. **Policy-level soundness is external by construction** — human review, or a separate stronger
   system. This is not a gap to close later; it is the architecture.
3. **This is why the anchor work was load-bearing rather than philosophical.** Without a fixed
   external structure there is no comparator, and without a comparator the only remaining option is
   self-proof, which Löb forecloses.

## 4. E1 — the entrenchment ordering, derived rather than stipulated

AGM guarantees a rational operator **given** a faithful preorder (`AGM-PREORDER-REPAIR`). Which one
has been the Cure's remaining blocker.

**Prediction from §1 holds: it should not be stipulated.**

> **DECISION: entrenchment is the coarse-graining survival order.**

| Entrenchment | Level | Invariants |
|---|---|---|
| **Highest** | topological | the combinatorial map — incidence, separation, junction degree |
| | projective | incidence, cross-ratio |
| | affine | parallelism, midpoints |
| **Lowest** | metric | lengths, angles, curvature |

**When the Cure must give something up, it surrenders metric detail first and topological structure
last.**

### 4a. Why this is a derivation

1. **The ordering is computed, not chosen** — `014` and `022` derived it, and `022` explained *why*
   the axes die in that order (metric → differential → topological).
2. **It coincides with the anchor.** `007` says anchor to the most invariant available; the most
   entrenched thing is the anchor. **Two independently derived orderings agree**, which is the
   check that matters.
3. **It is a total preorder, as AGM requires.** Erlangen levels are totally ordered, and each
   invariant has a coarsest level at which it survives, inducing a total preorder on invariants.

**So the Cure's last free parameter is not free.** It falls out of the geometry, and by
Katsuno-Mendelzon the resulting operator is rational by construction.

### 4b. What this does and does not settle

**Settles:** the *shape* of the ordering, and that it is derived. The Cure's blocker as a research
question is closed.

**Does not settle:** ties within a level need a secondary criterion, and no semantic invariant has
yet been written as a checkable predicate — so this is a specification, not an implementation.
**Phase 5 is where that is tested.**

## 5. What changed

- `claims.yaml`: `REGIONS-ARE-FACE-UNIONS` added (B); `CURE-IS-COMPARATOR` added (E2);
  `ENTRENCHMENT-IS-SURVIVAL-ORDER` added (E1); `AGM-PREORDER-REPAIR` no longer blocked.
- **Phase 4 closed.** All three items are DESIGN-CHOICE entries with derived rather than stipulated
  rationale.
