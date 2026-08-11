# Phase 3 — Formal Necessity Rebuild

**Status:** Complete (first pass) — this document supersedes the retired Parts III–V of
`uws/formal-specification.md` (the Isomorphism Theorem, the Unique Grounding Theorem, and
Strengthened Universality — see `archive/superseded-2026-08/foundations/formal-foundations-FULL-ORIGINAL.md`
for the original text).
**Date:** 2026-08-01
**Goal (from `PLAN.md`):** Either (a) show a specific primitive/sort count is forced given an
*explicitly stated* set of representational constraints, or (b) conclude the count is a free
parameter and the real invariant is the underlying convergent structure.

**Result: mostly (b), with a real, substantive (a)-flavored partial result.** There is a genuine,
convergent, non-circular necessity argument available — but it argues for an *infinite, freely
generated hierarchy of derived categories* and a *specific small set of operations*, not for any
specific finite sort/primitive count. The finite count is where the notation's design choice
actually lives, and this document says so plainly rather than hiding it in "role property" language
built to match a predetermined answer.

---

## Part A: The convergent backbone

### A.1 The Curry–Howard–Lambek correspondence

Three formalisms, independently motivated by three different problems, provably coincide:

| Formalism | Motivating problem | Era/originator |
|---|---|---|
| Simply-typed lambda calculus | Foundations of computability | Church, 1930s |
| Intuitionistic natural deduction | Foundations of proof | Gentzen/Heyting, 1930s |
| Cartesian closed categories | Foundations of structure (category theory) | Eilenberg/Mac Lane, Lambek, 1960s–70s |

The correspondence: types ≅ propositions ≅ objects; terms ≅ proofs ≅ morphisms; function types
≅ implications ≅ exponential objects. This is a real, deep, well-established result (propositions-
as-types), and it is the strongest *mathematical-formalism* convergence available — three
communities, solving three different problems, discovering they'd built the same thing.

### A.2 Montague's minimality result

Montague (1970), building directly on Church's simple type theory, showed that a compositional
semantics for natural language can be built from **two base types**: `e` (entities) and `t` (truth
values), plus one type-formation rule (function types `σ → τ`). Every other semantic category used
in natural-language semantics — unary predicates (`e → t`), two-place relations (`e → (e → t)`),
generalized quantifiers (`(e → t) → t`), predicate modifiers (`(e → t) → (e → t)`), sentential
connectives (`t → t`, `t → (t → t)`) — is a **derived type**, not an additional stipulation.

**Honest caveat on "minimal":** 2 is the standard, well-precedented minimum for a *typed*
compositional semantics with the properties (decidability, strong normalization) that make such a
system usable. It is not an absolute lower bound across every conceivable foundation (untyped
systems exist, at the cost of those properties). "2 base types" should be read as "the
well-established minimal typed answer," not "the only conceivable answer at any cost."

### A.3 What A.1 + A.2 together actually force

If a system (1) has two base categories functioning as "individual" and "truth-bearing
proposition," and (2) is closed under function-type formation — i.e., is simply typed / a Cartesian
closed category — then it **automatically contains**, as *derivable constructions rather than
additional stipulations*, the entire infinite hierarchy above: predicates, relations, quantifiers,
modifiers, connectives, all of it. This is the actual discovered structure. It is not a specific
finite sort count — it is an infinitely-generated free hierarchy from a 2-generator base, and this
generativity is what's forced, not any particular cross-section of it.

### A.4 Correction (added after review): the clean A.1 correspondence is intuitionistic, not classical

**This matters and was missed in the first pass.** The clean Curry–Howard–Lambek triple in A.1 is,
in its standard form, a correspondence for **intuitionistic** logic — types-as-propositions,
terms-as-proofs, in a system without excluded middle. The notation's actual `negate` operation is
**classical**: an involution between exactly two states (⊕/⊖, boundary inversion), and Montague's
own semantics is bivalent/classical too (truth values, excluded middle as tautology, confirmed by
direct check rather than assumed). Classical logic does **not** get the same clean three-way
correspondence — it requires Griffin's 1990 result that control operators (`call/cc`) correspond to
classical tautologies like Peirce's law, later formalized as Parigot's λμ-calculus. That's real,
legitimate mathematics, but it is a specific technical extension motivated by one problem (typing
Scheme's control flow), not an independent three-community convergence the way A.1 is.

**Consequence for this document's argument:** Part A's hierarchy-generativity claim (A.3) is on
solid ground for the *intuitionistic* fragment (predicate combination, quantification, and
non-classical connectives all fall out cleanly). The classical, two-valued, involutive `negate`
operation specifically does **not** inherit its justification from the clean A.1 convergence — it
needs its own support. It has some: Part B's Aristotle/Nyāya evidence is itself about classical,
bivalent logical traditions, so negation's convergent-operation status in Part B stands on its own
and does not depend on A.1 covering the classical case. But the document should not imply, as the
first version did, that the type-theoretic backbone hands classical negation over for free — it
doesn't, and conflating the two was a real error, not a stylistic imprecision.

---

## Part B: The convergent operational core

Phase 1 and Phase 2 add a second, independent kind of evidence — not about the abstract type
hierarchy, but about *which specific operations recur* across traditions that never derived them
from Montague grammar at all:

| Operation | Independent convergent support | Source |
|---|---|---|
| Subject-predicate combination | Aristotelian syllogistic + Nyāya inference theory, developed "rather independently of each other" | Phase 1 |
| Negation | Same two traditions; also the strongest single non-human data point in this investigation (Pepperberg's parrot Alex generalizing "none" to novel cases without training) | Phase 1, Phase 2 |
| Quantification (universal/particular) | Aristotelian categorical propositions + Nyāya's comparable structure | Phase 1 |
| Substance/entity as foundational, other categories dependent on it | Aristotle's ten categories + Vaiśeṣika's *padārthas*, independently, despite "total separation in geography and intellectual lineage" | Phase 1 |
| Modification/gradation | Weaker, more diffuse support: Vaiśeṣika's *guṇa* (quality) category as dependent-on-substance; graded/parametric geometric encoding in non-human signaling (bee dance, cephalopod chromatophores) as a distant structural echo, not a semantic one | Phase 1, Phase 2 |
| Bounded/hierarchical compositional generativity | Broadly available in life (Bengalese finch song), though bounded, not unbounded, outside humans | Phase 2 |
| Unbounded compositional generativity | Confirmed reliably reachable in humans given only minimal social conditions, no inherited model (Nicaraguan Sign Language, independently across child cohorts); not confirmed in any non-human case | Phase 2 |

**What this table does not contain:** independent convergent support for `embed` (nominalization),
`bind` (variable binding/co-reference), `compose`/`invert` (relation chaining/reversal), or
`modify_assertion` as specifically-necessary operations. These are real, useful, well-motivated
constructions *within* the Part A hierarchy (Montague-style semantics handles nominalization and
binding routinely), but this investigation found no independent, non-Western, non-modern evidence
that they are *necessary* rather than *convenient*. Flagged honestly as unresolved, not asserted
either way.

---

## Part C: The flattening step — where the notation's design choice actually lives

The notation (Σ, in `uws/formal-specification.md`) does not expose the Part A hierarchy as
nested function types (`(e → t) → t`, etc.) — it **flattens** specific cross-sections of that
hierarchy into fixed, named, freely-drawable categories: Entity, Relation, Modifier, Assertion. This
is a real, useful design decision (nested arrow types are not drawable as a single bounded 2D glyph
the way a flat category can be) — but it is a *representational* choice about which cross-section
to name, not a mathematical necessity.

**This section originally asserted three "equally valid" flattenings without checking the claim. On
review, that was exactly the kind of unearned assertion this whole investigation exists to catch —
just pointed in the opposite direction (toward "everything's arbitrary" instead of toward "5 is
forced"). Here is the actual worked check, done properly.**

**First, an observation about the notation's own 4-category choice that the first draft missed
entirely:** "Modifier" in the actual signature (`uws/formal-specification.md` §1.5) is
already **not one type-shape** — it's used as `e → e` in `modify_entity`, as `r → r` in
`modify_relation`, as a determiner-like device in `quantify` (closer to `(e → t) → t`), and as
`a → a` in `modify_assertion`. Four distinct underlying function shapes, one shared glyph. So the
notation's own "4 sorts" already isn't a clean 1-shape-per-sort mapping — it's silently
disambiguating four things by *context of use* (which operation the modifier appears in), not by
distinct sorts. That's the first real evidence for the flattening argument, found by actually
checking the specification instead of describing it from memory.

**Now, the 3-category attempt: {Individual, Function, Proposition}.** Walking all 13 operations
through this scheme: `predicate` needs Function tagged as a binary individual-relation
(`e → (e → t)`); `modify_entity` needs Function tagged `e → e`; `modify_relation` needs Function
tagged `r → r` (itself already a Function-typed argument and result — a function *on* functions);
`quantify` needs Function tagged as a determiner (`(e→t) → t`-ish); `modify_assertion` needs
Function tagged `a → a`. That's **five distinct required tags** on the single merged "Function"
sort. Without those tags, the merge is unsound — nothing would stop a `modify_entity`-shaped value
from being passed where `modify_relation` expects one, producing nonsense. With the tags, the system
is sound again, but the distinguishing information the 4-sort version carries *as separate glyphs*
now has to be carried *as tags on one glyph* instead. **This is the actual, checked result: the
3-category flattening is expressively equivalent, but only by relocating the same disambiguating
information from sort-identity to explicit tags — it does not eliminate the information, it moves
it.** That is a real, demonstrated finding, not an assertion.

**The 6-category direction (splitting further, e.g. giving `modify_entity`'s shape,
`modify_relation`'s shape, and `quantify`'s determiner shape three separate sorts instead of one
shared "Modifier") was not run through the same check.** It's plausible by the same logic — sort
count and tag burden trade off against each other in principle — but this document should say
plainly that only the 3-category direction was actually verified here. The 6-category comparison
(and its resemblance to the wiki's octahedral system) remains **illustrative, not verified**, and
should be labeled as such rather than presented with the same confidence as the 3-category result.

**Revised conclusion:** the flattening step is real and is where a design choice lives, but the
honest finding is narrower than the first draft claimed: sort-count and per-glyph tag-complexity
trade off against each other (demonstrated for the 3-category case), which is *consistent with*
there being no single forced count — but "no single forced count" is not the same strength of claim
as "all flattenings are equally valid," and this document should not claim more than what was
actually checked.

---

## Part D: The replacement claim, stated formally

**Claim (Convergent Operational Core — replaces the retired Unique Grounding Theorem), revised
after review.**

The *intuitionistic* fragment of compositional relational semantics — a system built over base
categories for "individual" and "truth-bearing proposition," closed under function-type formation —
contains, as derivable structure rather than additional stipulation, the hierarchy of predicates,
relations, quantifiers, and (non-classical) modifiers (Part A, with the A.4 correction). This part
rests on genuine three-way mathematical convergence (Curry–Howard–Lambek) and is on solid footing.

The **classical** operations the notation actually uses — bivalent `negate` as involution, in
particular — do not inherit their justification from that same convergence (A.4); they instead rest
on Part B's independent evidence: Aristotelian and Nyāya logic, developed without contact,
converging on subject-predicate combination, negation, and quantification as classical, bivalent
operations, plus the strongest single non-human data point found (Alex the parrot's productive
"none"). This is real, separately-grounded support — it just isn't the same three-way mathematical
convergence as the intuitionistic fragment, and the two should not be blurred together.

The further step of **flattening the underlying hierarchy into a fixed finite number of named,
drawable categories is a representational choice** (Part C). This document *demonstrated* (not just
asserted) that a 3-category flattening is expressively equivalent to the notation's 4-category
choice, at the cost of relocating disambiguating information from sort-identity to explicit tags —
a real, checked result. Whether a 6-category flattening is equally equivalent was not checked with
the same rigor and is flagged as illustrative only, not verified.

**What this replaces, precisely, and how much more carefully than the first draft:** the old Unique
Grounding Theorem claimed a *unique, forced* bijection between 5 geometric and 5 semantic
primitives, derived from role properties that were, on inspection, constructed to match. This
document replaces that with something narrower, and now more precisely scoped than its own first
draft was: a genuine mathematical convergence for the intuitionistic fragment (Part A), a
separately-grounded (not mathematically-derived) convergent operational core for the classical
operations the notation actually uses (Part B), and one demonstrated (not merely claimed)
equivalence result for an alternative flattening (Part C) — with the sort/primitive count still
underdetermined by everything found, but now for a reason that was actually shown rather than
asserted.

---

## Part E: What remains open

- **Not machine-checked.** A rigorous version of Part A's claims (the hierarchy is fully generated,
  nothing is smuggled in) should be formalized in a proof assistant (Lean/Coq/Agda) per the original
  Phase 1 aspiration. That requires tooling beyond this session's solo/literature-based scope —
  flagged for whenever compute/collaborator resources are available, not silently dropped.
- **`embed`, `bind`, `compose`, `invert`, `modify_assertion`** have no independent convergent
  evidence found either for or against necessity — genuinely unresolved, not quietly assumed
  necessary the way the retired theorem did.
- **The specific choice of 4 (vs. 3, 5, 6, ...)** remains, honestly, arbitrary relative to
  everything found so far. If a future phase finds a principled reason to prefer one flattening
  over the others (e.g., a minimum-description-length argument, or a cognitive/perceptual
  constraint on what's learnable/drawable), that would upgrade part of Part C from "design choice"
  to "forced given stated constraints" — but no such argument has been found yet, and this document
  does not manufacture one to close the gap.
- **Corrections made after initial review (recorded for transparency, not scrubbed):** the first
  version of this document (i) treated the Curry–Howard–Lambek convergence as if it directly covered
  the notation's classical, bivalent `negate` operation, when the clean correspondence is
  intuitionistic and classical logic requires the separate Griffin/Parigot control-operator
  extension — corrected in A.4; (ii) asserted that three alternative flattenings (3/4/6-category)
  were "equally valid" without checking, when only the 3-category case was actually worked through
  — corrected in Part C, with the 6-category claim now explicitly marked unverified rather than
  presented with unearned confidence. Both errors were the same failure mode this investigation
  exists to catch, just running in the "everything's arbitrary" direction instead of the "5 is
  forced" direction — which is exactly why they needed to be caught and fixed rather than left in
  place because the overall conclusion still pointed the same way.
- **The 6-category flattening check remains genuinely undone.** This is the most concrete
  next-smallest piece of unfinished work in Phase 3, not a hypothetical — someone should actually
  walk the wiki's octahedral 6-symbol system through the same 13-operation check that Part C ran for
  the 3-category case, rather than continuing to describe the comparison illustratively.

## Where this leaves `uws/formal-specification.md`

The "PART III–V: RETIRED" stub in that document should be updated to point here as the actual
replacement content, rather than only pointing at the open question — see next edit.
