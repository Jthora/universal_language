# 028 — Phase 3: constructive foundations and the expressiveness bound

**Type:** cycle
**Opened:** 2026-08-01
**Status:** open
**Thread:** follows `027`; executes Phase 3 items A4, A1, A2 in order

---

## 1. Before reading  ← written first (S2)

**A4 — the quantifier-free constructive geometry paper.** Expect: points primitive, construction
*functions* rather than existence axioms, and — given `020`'s finding that constructive geometry
declines decidable equality — **an apartness relation somewhere.** If apartness appears, that is a
direct connection to `SEMANTIC-APARTNESS` and the paper matters more than as a formal target.

**A1 — the constructibility ladder.** Expect standard Galois theory: constructible numbers are those
in towers of quadratic extensions; trisection fails because cos 20° has degree 3; origami reaches
cubics.

**The prediction I am committing to before doing A1:** *A1 as originally scoped cannot close.*
Constructibility bounds what **figures** can be built. UWS expressiveness is about what **meanings**
can be expressed. `018` flagged that bridge as not free; nothing since has built it, and I expect the
honest output of A1 to be **a precise statement of where the chain breaks**, not a bound on the
notation.

**A2 — Huzita–Hatori.** Expect seven axioms, strictly stronger than straightedge-compass. **The
base-6 correspondence I expect to dismiss:** fold axioms and mark primitives are different kinds of
object, and `017` recorded retrofitting such matches as the documented failure mode. I am recording
the expected dismissal in advance so that finding a match would be surprising rather than convenient.

**Objects mathematical?** A1 and A2 yes. A4 is reading.

**Status:** closed

## 2. A4 — and my apartness prediction was wrong

The paper builds on *"Suppes' quantifier-free axioms for constructive affine plane geometry"* and
**Beeson's constructive version of Tarski's geometry**, using *"quantifier-free first order logic
with all statements referring to finite objects only"*, and avoiding the Fifth Postulate as
introducing *"non-feasible constructions."*

**Then the correction.** §1 predicted apartness would appear. It does not:

> *"The law of trichotomy of order is replaced by the **stability of equality and betweenness**.
> **Apartness is not needed in constructive geometry.**"*

**Constructive geometry gets by with *stable equality* rather than apartness.** Stability is
`¬¬(a = b) → (a = b)` — double-negation elimination **restricted to equality**, which is far weaker
than the general DNE `020` retired.

**This narrows `SEMANTIC-APARTNESS` rather than refuting it.** The precedent I expected — that the
notation's home framework uses apartness — does not exist. The actual precedent is *stable
equality*, which suggests the right target may be **semantic equality plus a stability assumption**
rather than apartness. That is a different and probably easier ask, and it came from a prediction
failing.

**Two further findings:**

- Beeson *"modifies the axioms so that the points they assert to exist are **unique and depend
  continuously on parameters**."* **Continuity is the driving constraint** — the same reason `020`
  found for declining decidable equality. Constructive geometry is organized around constructions
  being continuous functions.
- **In Tarski's geometry, only points are primitive** — betweenness and equidistance are *relations*,
  not objects. **That is a fourth position** on the primitivity question from `016`, and it is the
  most austere: Euclid-constructive is point-first, Hilbert is point-and-line co-primitive,
  **Tarski is point-only**, Whitehead is region-only.

## 3. A1 — the bound, and exactly where the chain breaks

**Prediction from §1 confirmed: A1 as originally scoped cannot close.** But it now fails *precisely*,
which is the useful form.

The full chain is:

```
meaning  →  combinatorial map  →  drawing  →  constructible figure
   ???            Mnëv +              Galois theory
              nonstretchability
```

| Link | Status |
|---|---|
| **drawing ↔ constructible figure** | **Characterized.** Straightedge-compass reaches quadratic towers; origami reaches cubics and quartics |
| **map ↔ drawing** | **Characterized** (`027`). Not surjective — nonstretchable maps have empty realization space; fibres arbitrarily complex — Mnëv |
| **meaning ↔ map** | **Unbuilt. This is the gap.** |

**So the lower two links of the notation are completely characterized by theorems, and the top link
does not exist.** The constructibility ladder bounds the *figure* layer and says nothing about
semantic expressiveness, because nothing connects meaning to the map.

**This is a better outcome than a bound would have been**, because it locates the missing work
exactly rather than producing a number that quietly presupposed the bridge.

### 3a. The actionable consequence

The ladder does yield one concrete requirement:

> **A straightedge-and-compass notation reaches only quadratic towers. Reaching cube roots requires a
> neusis or fold operation.**

So *if* the notation ever needs to express content of algebraic degree 3, **it needs a fold-like
primitive operation** — Huzita–Hatori axiom 6, the Beloch fold. **Conditional on the unbuilt bridge**,
but it is a genuine operation-level requirement derived from a theorem rather than from taste.

## 4. A2 — Huzita–Hatori, and the base-6 coincidence dismissed

> The axioms are *"**not a minimal set** of axioms but rather **the complete set of possible single
> folds**."* Justin found all seven in 1986; **Huzita rediscovered only 1–6 in 1991; Hatori
> rediscovered the seventh in 2001.**
>
> *"Axiom 6 allows so-called **neusis** constructions, which is impossible in straightedge and
> compass."* The **Beloch fold** (1936) solves general cubics; origami reaches *"any cubic equation
> or quartic equation."*

**The base-6 correspondence is dismissed, and more firmly than expected.** Two independent reasons:

1. **Different objects.** Seven fold *operations* is not a basis of *marks*. Predicted in §1.
2. **The 6 is a historical artifact.** All seven were found in 1986; Huzita's list of six was simply
   **incomplete**, and the seventh was recovered fifteen years later. **The number six was never
   structural — it was an enumeration error**, which is a stronger dismissal than "different kinds of
   thing."

**Recording that I predicted this dismissal in advance** (§1), so the outcome is not a convenient
one found afterwards.

**What survives from A2:** origami is a genuinely stronger construction basis than straightedge-
compass, and it is *complete* for single folds. If the notation ever needs the fold operation of
§3a, Huzita–Hatori is the specification of what folding can do.

## 5. What changed

- `claims.yaml`: `SEMANTIC-APARTNESS` narrowed — stable equality is the constructive-geometry
  precedent, not apartness; `CONSTRUCTIVE-PRIORITY-OF-POINT` gains Tarski as a fourth position;
  `EXPRESSIVENESS-CHAIN` added, stating the two characterized links and the unbuilt one.
- **Phase 3 closed.** Its honest output is a located gap rather than a bound.

