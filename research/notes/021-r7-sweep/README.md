# 021 — Phase 1 · D1: the R7 sweep

**Type:** cycle
**Opened:** 2026-08-01
**Status:** open
**Thread:** follows `018` (plan, item D1); implements **R7**
**Question:** every negative this project cites needs one adversarial search. Which are unchecked,
and do they survive?

---

## Inventory

| Negative | Mentions | Adversarial search? |
|---|---|---|
| Zadrozny — compositionality vacuous | 64 | ✅ `011` — **overturned as over-broad** |
| TopSim as compositionality measure | 38 | ✅ `012` — **this was the search; it demoted the claim** |
| Chomsky's UG / nativism | 26 | ✅ `004` — superseded via Christiansen & Chater |
| Newman's objection | 21 | ✅ `012`, partially answered `014` |
| Evans & Levinson — few universals | 17 | ✅ `004` — **inverted; it is a prediction** |
| Symbolic protocols fail cross-play | — | ✅ `013`, narrowed `019` |
| Euclid definitional ordering | — | ✅ `015`, corrected `016` |
| Double-negation elimination | — | ✅ `020` |
| **Löb's obstacle** | **32** | ❌ **never** |
| **Rice's theorem** | **7** | ❌ **never** |
| **Plump — confluence undecidable** | **10** | ❌ **never** |
| Hilbert projection / convexity | 10 | ~ effectively dissolved by AGM (`006`) |

**Three unchecked, and all three are load-bearing.** Löb forces an architecture decision; Rice bounds
the central engineering goal; Plump is the sole basis of the IR decision.

## 1. Before searching  ← written first (S2)

**Löb.** Expect the theorem to be solid and its *practical bite* to be contested — the obstacle
concerns proof-based self-trust, and probabilistic or heuristic self-trust may sidestep it entirely.
Expect more escape routes than I recorded, and possibly recent work claiming resolution.

**Rice — and here is the specific worry.** **Rice's theorem is about non-trivial semantic properties
of *programs* — index sets of partial recursive functions. GIR is a data structure, not a program.**
Unless GIR expressions denote computations, invoking Rice may be a **category error**, and I may have
imported an impossibility result that never applied. I predict this is at least partly right, and
that the honest scope is narrower than "SEMANTIC-EQUALITY is undecidable."

**Plump.** Expect it to hold. The scope is already narrowed to unrestricted cyclic rewriting, so the
risk here is lowest of the three.

**Would change the plan if:** Rice does not apply — then `SEMANTIC-EQUALITY` was never bounded from
above by it, and the abstract-interpretation route in Phase 5 is a *choice* rather than a
requirement.

**Objects mathematical?** All three. Find the theorem and its scope (T5, T8).

**Standing risk:** these three currently support conclusions I like — Löb lands on the side of the
theosis framing, Rice justifies a design decision already made. **T6 applies: check them as hard as
I checked TopSim.**

**Status:** closed

## 2. Searches run

All three adversarial (R1). **Counter-evidence easy to find in every case** (S7).

## 3. Findings

### 3.1 Rice — **overreached. I imported a theorem whose preconditions I never checked.**

The §1 worry was right:

> *"Rice's theorem states that any nontrivial **extensional** property of **partial computable
> functions** has an undecidable index set… classical Rice's theorem focuses on the extensional
> content of **programs**, namely, on the partial recursive functions that programs compute."*
>
> *"**Intensional, or syntactic, properties of program codes are DECIDABLE** precisely because they
> concern the program's structure rather than its behavior, thereby sharpening understanding of the
> true scope of Rice's Theorem."*

**Rice applies to extensional properties of programs, indexed by their code. GIR is a typed graph —
a data structure.** Unless GIR expressions denote partial computable functions, **Rice does not
apply at all**, and structural properties of it are decidable precisely because they are structural.

**What this does and does not mean.** It does *not* show semantic equality is decidable. It shows
**Rice never established that it wasn't.** Any undecidability would have to come from the semantics
actually assigned — and the real constraint is the one we already had: **confluence and
normalization**, per Plump.

**Consequences, and they reach into Phase 5.** Abstract interpretation was recorded as *forced by
Rice*. It is not forced; it is a **design choice**. And the "infinitely many false positives" cost
was inherited from the same misapplication. On an acyclic, strongly normalizing core,
**equality-by-normal-form may simply work** — which was the Knuth-Bendix plan before Rice was
invoked over it.

This also removes an objection to `SEMANTIC-APARTNESS` (`020`) before it was raised.

### 3.2 Löb — holds, with more escape routes than recorded, and one reading that *strengthens* us

> *"Technical means can **partially resolve** the Löbian obstacle to the extent of obtaining some
> desiderata, **but not yet all**, nor yet by fundamental rather than technical means."*

Named partial solutions I had not recorded: **Marcello's Waterfall** and **Model Polymorphism**.

**The reading worth keeping:**

> *"A logical agent cannot make a priori claims about the soundness of its own reasoning because
> **soundness involves semantic maps beyond the logic's a priori control**."*

**That is exactly the argument in `007`, arrived at independently.** The obstacle is not a mysterious
barrier — it is the observation that soundness is a *semantic* notion and a system cannot fix its
own semantics. Which is precisely why an **external semantic anchor** works:
`GEOMETRIC-ANCHOR-ESCAPES-LOB` is the right shape, and this is independent support for it.

**Net: the obstacle stands** ("not yet all"), the architecture decision it forces stands, and the
anchor argument is better supported than before.

### 3.3 Plump — confirmed, correctly scoped, one refinement

> *"In contrast to **term graph rewriting**, confluence of terminating graph rewrite systems turns
> out to be undecidable in general… but is **decidable under the termination assumption for term
> graph rewriting** specifically."*
>
> *"For DPO rewriting (without interfaces) confluence is undecidable, but **for DPOI rewriting the
> Knuth–Bendix property is saved**: confluence of a terminating DPOI system can be decided by
> checking whether its critical pairs are joinable."*

`IR-NORMALIZATION-STRATEGY` recorded both escapes — acyclic term graphs, and DPO-with-interfaces —
and both are confirmed. **No correction needed**, which is worth stating: the sweep is not only for
finding errors.

**One refinement to record:** joinability alone is insufficient in the graph setting. *"The mere
existence of common reducts for all critical pairs does not imply local confluence"* — the Critical
Pair Lemma requires **joining derivations that are compatible** (strong joinability). That is a
stricter condition than the term-rewriting version, and it will matter when the rewrite system is
actually built.

## 4. What changed

- `claims.yaml`: `RICE-BOUNDS-SEMANTIC-EQUALITY` **scope corrected** — it does not bound GIR;
  `SEMANTIC-EQUALITY` un-blocked from Rice; `LOBIAN-OBSTACLE` gains the named escape routes and the
  semantic-map reading; `IR-NORMALIZATION-STRATEGY` gains the strong-joinability condition.
- `FAILURES.md`: F-023.
- **Phase 1 D1: complete. Phase 1 closed.**

### Sweep scorecard

| Negative | Outcome |
|---|---|
| Zadrozny (`011`) | **Overturned** — necessity route reopened |
| TopSim (`012`) | **Overturned** — claim downgraded |
| Dependency test (`015`) | **Overturned** — my own, retired |
| Rice (here) | **Overturned** — never applied |
| DNE (`020`) | **Overturned** — spec was wrong, not the code |
| Newman (`012`) | Real; partially answered |
| Löb (here) | Holds; better escape routes; anchor strengthened |
| Plump (here) | **Confirmed**, one refinement |
| Evans & Levinson (`004`) | **Inverted** — it is a prediction |

**Five of nine load-bearing negatives were over-broad as this project had recorded them.** That is
the strongest available argument for R7 being a permanent rule rather than a one-off sweep.
