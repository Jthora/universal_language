# 015 — The dependency-order test is ill-posed

**Type:** correction
**Opened:** 2026-08-01
**Status:** closed
**Thread:** corrects `004`, which proposed this test, and every note since that promoted it as
priority-0 and as "the non-circular replacement for the retired grounding argument."
**Question:** is Euclid's definitional dependency order isomorphic to the ontological dependency
orders of Aristotle and Vaiśeṣika?

**Answer: the question is malformed, and one search would have shown it before I promoted it for a
dozen turns.**

---

## 1. What was held

That the test was *cheap, non-circular and decisive either way* — because both sides are fixed
independently: **"Euclid's definitional dependency graph is a fact about a text written for
geometry, and the Aristotelian and Vaiśeṣika orderings are facts about texts written for ontology.
Neither was constructed to match the other."**

Priority-0. Repeatedly named as the first move of the reconstruction.

## 2. What broke it

### 2.1 Euclid's foundational definitions do no mathematical work

> *"In all the books of the Elements, **Euclid never refers to the first nine definitions**, or to
> any other descriptive definitions, to justify steps in his proofs."*

The first nine definitions are **point, line, extremities of a line, straight line, surface,
extremities of a surface, plane surface** — precisely the ones the test depends on. They are
**descriptive prefatory material, never load-bearing in the mathematics.**

So the claim that their ordering "falls out of what the objects are" is wrong. **Their ordering is
expository.** Euclid chose a presentation sequence; he did not derive a dependency structure, and
nothing in the Elements rests on it.

### 2.2 Definitional and ontological dependence are different relations

Kit Fine's essentialist account gives the bridge — and shows exactly why it fails here:

> One entity ontologically depends on a second *"just in case the latter is a constituent in a
> proposition that expresses a **real definition** of the former. Unlike a **nominal definition**,
> which states what a competent speaker of the language understands, a **real definition** states
> what the defined object is."*

**The bridge requires real definitions.** Euclid's first nine, never used in proof, function as
nominal ones. So the link between his ordering and Aristotle's ontological priority is not available.

### 2.3 Aristotle himself distinguishes the two

Aristotle treats **definitional priority** (Categories 12, Metaphysics 5.11) as a *distinct* sense of
priority from ontological priority. Collapsing them does not just risk error — **it misreads the
source I was proposing to use as one side of the comparison.**

And Vaiśeṣika differs structurally again: *samavaya* (inherence) is elevated to a **category in its
own right**, so the dependency relation is itself an entity in the ontology. There is no
corresponding move in Aristotle or Euclid.

### 2.4 Even granting all that, the orders are not isomorphic

Euclid's sequence gives a **chain** — point ≺ line ≺ angle, and point ≺ line ≺ boundary ≺ figure ≺
circle, depth ≥ 4. Aristotle gives a **fan** — substance ≺ each of the other nine, which are mutually
incomparable, depth 2. Vaiśeṣika likewise: *dravya* ≺ {*guṇa*, *karma*}, depth 2.

**A chain of depth 4 is not isomorphic to a fan of depth 2.** What they share is only that each has a
**unique minimal element** — point, substance, *dravya*. That is a weak property common to very many
orders, and it proves almost nothing.

## 3. What replaced it

**The test is withdrawn as specified.** Recorded with the full negative discipline:

- **Scope (R2):** what fails is *this* comparison — Euclid's Book I definitional sequence against
  Aristotelian/Vaiśeṣika ontological priority. It does **not** show that geometry and ontology have
  unrelated structure, nor that cross-civilizational corroboration (S4) is unavailable generally.
- **Formalization, not claim (R3):** what failed is **Euclid's Elements as the geometric side**, not
  the idea of comparing geometric and ontological dependency.
- **Revival condition (R4):** run the geometric side from a source whose definitions are **real and
  load-bearing** — a modern axiomatization such as **Hilbert's *Grundlagen*** or **Tarski's axioms**,
  where primitives and defined terms are explicitly separated and definitions are used in proof.
  Fine's essentialist criterion then supplies a principled bridge to ontological dependence.
- **Steelman (R6):** the test would have worked if Euclid's definitions were real definitions doing
  inferential work. The scholarship says they are not.
- **Refutation tier (R5):** ARGUED — a documented fact about the Elements plus a standard
  distinction in the dependence literature.

### The finding worth keeping

Attempting the revival immediately produces something: **in Hilbert and Tarski, point and line are
*both primitive and undefined*.** There is no "point ≺ line" ordering — **they are co-primitive.**

So Euclid's point-first sequence is expository, and modern axiomatics does not reproduce it. **The
"everything descends from the point" picture is not a mathematical result.**

And that converges with `014`: the Erlangen derivation found **Point and Enclosure as
co-survivors** at the fixed point — neither prior to the other. Two independent routes agree that
the right structure at the base is **not a chain from a single primitive but a small co-primitive
set.** That is a real result extracted from a failed test, and it sharpens
`FIXED-POINT-TABLE-IS-CANONICAL`.

## 4. Why it survived as long as it did

**R1 mandates adversarial search on negatives. Nothing mandated it on my own positive proposals.**

This test was *my* construction, promoted for a dozen turns as cheap and decisive, and never
subjected to the discipline applied to every claim from the literature. The check that killed it was
a single query about Euclid scholarship — the same cost as every other adversarial search in this
project.

**Generalized (F-020): adversarially check your own proposed tests and methods before promoting
them, not only the claims you inherit.** A proposal is a claim about what a test would show, and it
carries the same failure modes — I had assumed the comparison was well-formed, which is a
scope error of exactly the family documented in F-017.

**Second-order:** this is the third consecutive note where the discipline cost the project a result
it wanted (`012` TopSim, `014` four corrections, `015` here). That is the machinery working. The
failure mode to watch now is the opposite one — treating attrition as evidence of rigour, when what
matters is only whether each specific finding was handled correctly.

## 5. What changed

- `claims.yaml`: `DEFINITIONAL-DEPENDENCY-ISOMORPHISM` → RETIRED with revival condition;
  `FIXED-POINT-TABLE-IS-CANONICAL` gains the co-primitive convergence.
- `FAILURES.md`: F-020.
- `RESEARCH-PROTOCOL.md` / `method/negative-results.md`: **R8**.
- **Left open:** the Hilbert/Tarski version of the test, which is now the live form and is *not*
  cheap — it requires real work on both sides.
