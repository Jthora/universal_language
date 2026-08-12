# Superseding Chomsky, and What the Primitive Sets Actually Are

**Date:** 2026-08-01
**Continues:** `research/framework/emergent-universality.md`
**Two claims tested:** (a) Chomsky's Universal Grammar is a human-centric shadow of a cross-substrate
structure, and our version supersedes it as its root cause; (b) primitive *sets* are alternative
decompositions of something invariant, so the count is genuinely arbitrary while the primitives are
not.

**Both survive, and one incoming result that looked like a refutation turns out to be a prediction.**

---

## 1. The Chomsky claim has an established lineage — and we extend it

You don't need to compete with UG from outside. **There is already a mainstream position that
universals come from learning constraints rather than innate grammar**, and it's in *Behavioral and
Brain Sciences*.

**Christiansen & Chater, "Language as shaped by the brain" (2008):**

> *"Constraints on the learning and processing of sequential structure may be reflected in the
> universal properties of human language **if language has evolved to fit human sequential learning
> mechanisms**."*
>
> A Universal Grammar *"could not have arisen either by biological adaptation or non-adaptationist
> genetic processes"* — **language change occurs far faster than genetic change, making it "an
> unstable target for biological adaptation."**
>
> Universals are *"emergent properties of how the brain learns, rather than genetically encoded
> principles."*

That is the arrow reversed: **language adapts to the learner, not the learner to language.** The
"innate faculty" is unnecessary because the constraints are constraints *on learning itself*.

### The extension we can make that they didn't

Christiansen & Chater stop at *human* brains — they say universals reflect **human** sequential
learning. But **the argument doesn't depend on the learner being human.** If universals come from
constraints on learning-and-compressing structured input, they should appear in **any** learner
facing that problem.

**And emergent communication tests exactly this, with non-human learners, and finds them:**
independently trained agents converge to near-perfect compositionality in 100% of seeds at 3+
agents, developing topographic organization without supervision.

So the chain is:

| Position | Claim | Status |
|---|---|---|
| Chomsky | Universals ⇐ innate human faculty | Contested |
| Christiansen & Chater | Universals ⇐ **learning constraints** in human brains | Established, mainstream |
| **This project** | Learning constraints **aren't human-specific** ⇒ universals are cross-substrate | **Extension — with non-human evidence** |

**Chomsky isn't wrong about the phenomenon. He's wrong about the cause — and that misattribution is
exactly what makes his version human-centric.** UG becomes the *special case*: what cross-substrate
learning constraints look like when the learner happens to be a human brain. Root cause, as you put
it, and it doesn't require a fight — it requires citing C&C and extending them by one step that
the emergent-communication data now supports.

**Practical consequence, revising my earlier glossary warning:** don't avoid the term. **Claim it,
with the qualifier.** "Universal Grammar (cross-substrate)" positions the work as *superseding*
rather than *sidestepping*, and there's a real lineage behind it.

---

## 2. The result that looked like a refutation is actually a prediction

**Evans & Levinson, "The Myth of Language Universals" (2009)** is the strongest empirical attack on
universals in existence:

> *"There are **vanishingly few universals** of language in the direct sense that all languages
> exhibit them; instead, **diversity can be found at almost every level of linguistic
> organization**"* — across 6,000–8,000 languages, universal characteristics are *"few and
> unprofound."*

Taken at face value, that guts the universality thesis.

**But it's exactly what universality theory predicts.** Water and uniaxial magnets share identical
critical exponents *while being maximally different microscopically*. Universality does **not** say
the surface features match — it says microscopic differences are **irrelevant operators**, and that
invariants appear only **under coarse-graining**.

**Evans & Levinson measured at the wrong level.** They catalogued surface features across languages
— precisely where universality theory says you should find diversity, not universals. Their result
is *consistent with* emergent universality and arguably *predicted by* it.

**This flips the strongest counter-evidence in the field into supporting context** — and it yields a
sharp, falsifiable prediction: *surface diversity should be maximal while coarse-grained invariants
persist.* If someone coarse-grains the WALS typological data and finds nothing invariant, the
hypothesis dies. Nobody appears to have tried.

---

## 3. Nobody agrees on the primitive count — and that's evidence *for* the position

Five independent research traditions, five different counts:

| Tradition | Primitive count | Basis |
|---|---|---|
| Montague / type theory | **2** (e, t) | formal semantics |
| UWS geometric | **4, 5, or 6** | geometric features |
| Vaiśeṣika | **6–7** *padārthas* | Indian ontology |
| Aristotle | **10** categories | Greek ontology |
| **Wierzbicka NSM** | **65** semantic primes | 30+ years cross-linguistic fieldwork |

And NSM's own count **moved** — 60 in 2002, 65 now, after decades of revision. It is a *presentation
under maintenance*, not a discovery.

**Complete non-convergence across five traditions.** Under the old framing that was damning. Under
universality it's exactly right: **if primitive count is an irrelevant operator, different research
programs at different coarse-graining levels should get different counts while pointing at the same
class.** The disagreement is the predicted signature.

Note also what NSM's critics object to — *"the reductive requirement that a definiens be simpler
than a definiendum"* and reliance on *"canonical contexts."* Those are complaints about the
**presentation**, not about whether an invariant core exists.

---

## 4. What alternative primitive sets actually are, mathematically

Your three sets:

- **5:** point, line, angle, curve, enclosure
- **4a:** point, circle, line, wave — *curve collapses into wave, angle collapses into line*
- **4b:** point, circle, line, curve

The collapses are the informative part. A **wave is a periodic curve**; an **angle is two lines
meeting**. So these primitives are **not independent** — some are derivable from others under
composition.

**Therefore the sets are not bases. They are generating sets.**

That distinction is precise and it matters:

- A **basis** (vector space) has invariant cardinality — every basis of a space has the same size.
- A **generating set** does not. The same structure admits generating sets of different sizes, and
  minimality is a separate property from generation.

**The invariant is the generated structure, not the generators.** Different presentations, one
object — exactly like group presentations, where the same group has many presentations with
different generator counts.

This gives the cleanest statement of your position I can construct:

> **The primitives are real; the primitive *set* is a presentation. Counts vary because generating
> sets vary. What's invariant is what they generate — and that's characterized by symmetry, which
> is why the symmetry work is the load-bearing part.**

And it explains the base-N observation: **choosing Base 6 is choosing a presentation with six slots**,
which then makes certain symmetry structures (octahedral, three binary axes) the natural fit and
opens specific mappings (Elemental Combination Algebra). Base 4 gives a tetrahedral presentation and
different affordances. **These are different coordinate systems on the same object** — genuinely
useful tools, none of them the truth, all of them evidence about it.

---

## 5. The strongest non-circular version of the grounding argument

Here's something that survives the objection that killed the Unique Grounding Theorem.

The old argument defined semantic primitives with role-properties written to mirror geometric ones,
then presented the match as proof. Circular.

**But the geometric definitions have a *dependency structure*, and that structure is an objective
mathematical fact, not an assignment:**

- A **point** is defined without reference to anything else
- A **line** is defined *using points*
- An **angle** is defined *using lines meeting*
- An **enclosure** is defined *using a closed curve*

That's a strict dependency ordering, and Euclid didn't choose it to match semantics — it falls out
of what the objects are.

**Now compare, independently:**

- **Aristotle:** substance is primary; quality and relation *depend on* substance
- **Vaiśeṣika:** *dravya* (substance) is primary; *guṇa* (quality) and *karma* (action) **inhere in**
  substance and cannot exist independently

**Two ontological traditions with no contact, both giving substance-first dependency orderings — and
the geometric definitions give the same shape.**

**This is testable and non-circular**, because both sides are fixed independently: Euclid's
definitional dependency graph is a fact about a text written for geometry, and the Aristotelian and
Vaiśeṣika dependency orderings are facts about texts written for ontology. Neither was constructed
to match the other. **Formalize both as partial orders and check for isomorphism.**

If they match, that's real convergent evidence for the geometry→meaning path. If they don't, the
path needs different support. **Either result is informative, and it costs one careful afternoon.**

---

## 6. Where to push next

**Immediately checkable:**
1. **The dependency-order isomorphism test** above. Cheap, non-circular, decisive either way.
2. **Coarse-grain WALS.** Evans & Levinson found surface diversity; universality predicts invariants
   appear only under coarse-graining. Nobody has looked.
3. **Do emergent protocols cluster into discrete classes?** Universality predicts *few classes*, not
   a continuum. Directly testable with existing corpora (ELCC).

**Structural:**
4. **Distinctive feature theory (phonology)** is the closest existing analogue to "alternative
   primitive sets for one space" — Jakobson vs SPE vs articulatory features decompose the same
   phoneme inventory differently. Unchecked, and likely to have already solved the
   generating-set-vs-invariant problem in a concrete domain.
5. **Formal Concept Analysis** — builds concept lattices from object/attribute tables. A candidate
   formalism for "what's invariant across presentations."
6. **Minimal generating sets and presentation-invariants** — what *is* preserved across all
   generating sets of a structure? That's the mathematically precise version of "the primitives
   remain."

**Reframing:**
7. Claim "Universal Grammar (cross-substrate)" explicitly, citing Christiansen & Chater and
   extending by one step. Supersede rather than sidestep.
8. Present Evans & Levinson's diversity finding as **supporting context**, not opposition — with the
   coarse-graining prediction attached so it stays falsifiable.
