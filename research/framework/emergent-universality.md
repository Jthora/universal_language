# The Emergent-Universality Framework — Full Construct Stack

**Date:** 2026-08-01
**Supersedes the framing in:** `research/notes/001-triangulation/README.md` §"UL has to be a bootstrapping
protocol" (too weak — see §0), and the "primitive count is an arbitrary design choice" conclusion
in `phase3-formal-necessity-rebuild.md` (wrong level of description — see §2).

---

## 0. The correction that made this possible

Three kinds of universality had been collapsed into two:

| | Claim | Status |
|---|---|---|
| **Logical necessity** | True in all possible worlds; derivable from the constituents | **Dead.** Zadrozny: compositionality constrains nothing without external conventions. |
| **Emergent universality** | Arises reliably across radically different substrates because of the *dynamics*, invisible in the microscopic description | **Never considered.** This is where the answer is. |
| **Convention** | We agreed on it | The retreat position. Too weak. |

The keystone finding, from statistical physics:

> **"The critical exponents are not visible in the microscopic description. A complete Hamiltonian
> for water does not contain them. They emerge only under the projection defined by renormalization
> group operations."**

**Universal structure can exist that is not derivable from the constituents.** Water and uniaxial
magnets — different atoms, different force laws — share *identical* critical exponents. Class
membership is set by **dimensionality and the symmetry of the order parameter**; everything else is
formally an *irrelevant operator*.

**Consequence:** Zadrozny is a microscopic-level result. It says formal compositionality doesn't
pin down a signature. It says **nothing** about what emerges under coarse-graining. Treating it as
a global impossibility was an error of level, not of rigour.

---

## 1. The keystone: symmetry *emerges*, it isn't installed

This is the finding that changes the project's status:

> *"Near a second order transition, long-distance properties are invariant under dilatations
> **even though microscopic models have typical scales.** This **emergent symmetry** is best
> described in the renormalization group framework."*
>
> *"**Conformal symmetry is an emergent property of many systems at their critical point.**"* — and
> the literature finds it *"mysterious that critical phenomena show **enhanced** conformal symmetry
> rather than mere scale invariance."*

**Symmetry at a fixed point can exceed the symmetry of the model you started from.** You do not have
to choose the right symmetry. If there is a semantic fixed point, its symmetry is **discovered**,
not designed.

**This is the rigorous form of "the primitives are clues."** The geometric primitives are
microscopic details. They hint at the symmetry class without being it. Whether there are five or six
is an *irrelevant operator* — not because the question is arbitrary, but because it sits below the
level where universality lives.

---

## 2. The construct stack

Revised, with each layer's status under the emergent framing.

| Layer | Construct | What it is | Status |
|---|---|---|---|
| **0** | **Distinction** | The mark; marked/unmarked (Spencer-Brown, *Laws of Form*). Bateson: "the difference that makes a difference." | **Pre-semantic floor.** Not chosen — required by *any* information-bearing system. Genuinely non-anthropocentric. |
| **1** | **Universal Symbology** | The primitive marks. Which distinctions are makeable. Geometric primitives live here. | **Microscopic detail / clue.** Symmetry data, not the universal itself. |
| **2** | **Universal Syntax** | How marks combine. Composition rules. | **Measurable and shown to converge.** See §3. |
| **3** | **Universal Grammar\*** | Which combinations are well-formed/meaningful. | *\*Name collision — see §5.* |
| **4** | **Renderings** | **UWS** (written/spatial), **Vocal Semantic Correspondence** (prosodic), others | Modality-specific instantiations. UWS is *one* rendering, not privileged. |
| **5** | **UL** | The emergent universal structure — **the universality class itself.** | **Not a code.** Characterized by order parameter + symmetry, not by a primitive inventory. |
| **⊥** | **UP — Universal Protocol** | Bootstrapping/coupling procedure by which two independent instantiations reach correspondence | **Still applies** — but as the *mechanism of mutual discovery*, not as a replacement for UL. |
| **⊤** | **UQPL** | Computation over the structure | Unchanged. Total, invariant-preserving. |

### New constructs the framing requires

The universality apparatus needs semantic analogues, and naming them is the actual research program:

- **Semantic Order Parameter** — the quantity that characterizes the phase. *What is it?* Candidate:
  a compositionality measure (§3). **This is the central open question.**
- **Semantic Coarse-Graining** — the projection under which universals appear. **Strong candidate
  already in the repo: the Erlangen hierarchy** (§4).
- **Relevance Classification** — which features are relevant vs. irrelevant operators. The RG-for-
  deep-learning literature has this formally, and it is *"equivalent to the notion of 'relevant'
  information defined in the Information Bottleneck formalism."*
- **Semantic Fixed Point** — a scale-invariant structure under repeated coarse-graining. **If UL
  exists, this is what it is.**

---

## 3. It is measurable — and it already converges

The universality hypothesis is operationally testable *today*, because emergent communication
research has the metrics and the results.

**Metrics (established, off-the-shelf):**
- **TopSim (topographic similarity)** — Spearman correlation between distances in input space and
  Levenshtein distances in message space. "Nearby inputs described with similar messages." Note this
  is essentially a **structure-preservation measure** — an empirical homomorphism condition.
- **Context independence** — statistical independence of a symbol's meaning across contexts
- **Positional disentanglement** / **bag-of-symbols disentanglement**
- **RSA** (representational similarity analysis, borrowed from computational neuroscience)
- **Zero-shot generalization accuracy**

**The result that matters:**

> Scaling to 3+ agents, **100% of seeds converge to near-perfect compositionality.** Four agents:
> **100% across 80 seeds**, versus 54% for two. Agents develop **topographic organization** —
> specific message positions encoding specific semantic properties — **without explicit
> supervision.**

Independently trained agents, different seeds, no shared design, no supervision, non-human —
**converging on compositional structure with 100% reliability.** Not logical necessity. Not
convention. **Emergence, measured.**

And "position encodes property" is *placement carrying meaning* — UWS's spatial grammar, arising
spontaneously and unprompted.

**This is the first genuine empirical evidence the project has ever had for its core claim.**

---

## 4. The Erlangen hierarchy may be the coarse-graining flow

The repo already contains a progressive-forgetting sequence, currently archived as
Σ_UL-era material:

```
Euclidean → Similarity → Affine → Projective → Topological
```

Each step enlarges the transformation group and forgets more structure. **RG does the same thing:**
integrate out short-range detail, flow, look for fixed points.

And the directions agree in a specific, checkable way: **coarser Erlangen levels have larger
symmetry groups, and RG fixed points have *enhanced* symmetry.** Both say *coarse-graining increases
symmetry.*

**Status: structural hypothesis, not established.** RG is a flow in coupling space with fixed points;
Erlangen is a lattice of groups. They are analogous, not identical. But this is a *testable*
correspondence, and if it holds, a substantial body of archived work becomes load-bearing again in
a new role.

**Also relevant:** RG↔deep-learning is an active, real research area — not my analogy. Both are
coarse-graining procedures over information flow; RG "relevance" is formally equivalent to
Information Bottleneck "relevant information"; and 2025 work develops RG frameworks for universality
in learning and scaling laws. **The wiki's Universal Semantic Manifold page already invoked
Information Bottleneck** — closer to the mark than I credited at the time.

---

## 5. Terminology warning: "Universal Grammar"

**Do not use this name unqualified.** Chomsky's Universal Grammar is a claim about a
**species-specific innate human language faculty**. What this framework needs is nearly the
opposite: **cross-substrate emergent constraints, demonstrated in non-human agents.**

Using the term unmarked will (a) import a contested nativist commitment nobody here holds, and
(b) invite dismissal by linguists on grounds that have nothing to do with the actual claim.

**Recommend:** `Universal Compositional Constraints` or `Emergent Syntax Constraints`, with an
explicit GLOSSARY entry distinguishing it from Chomskyan UG. Same discipline as the
graph-invariant / semantic-invariant split.

---

## 6. The transitory path, and where it needs work

The argued chain is: *geometric primitives are mathematically fundamental → symbols built from them
are universal → writing systems attribute to languages → therefore a path from symbology to UL.*

**What's solid:** distinction is genuinely pre-semantic and substrate-independent (Layer 0). The
symmetry structure of the primitives is real mathematics, not taste. And script analysis genuinely
does inform language structure.

**Where the inference needs a bridge, stated honestly:** "geometrically fundamental" ≠ "semantically
universal." Geometry's objects being universal does not by itself make a *geometric notation for
meaning* universal — that requires the semantic order parameter to actually have the symmetry the
geometry suggests. **That bridge is exactly the universality-class question**, which is now
empirically approachable rather than a matter of assertion. The chain is plausible and testable
rather than proven — which is a far better position than either "it's obvious" or "it's arbitrary."

---

## 7. What this opens — brainstorm, prioritized

**Immediately testable (the emergent-communication apparatus exists):**
1. **Do independently emerged protocols share invariants under coarse-graining?** Run agents to
   convergence, apply progressive coarse-graining, look for a fixed point. *This is the direct test
   of the UL hypothesis, and nobody appears to have run it.*
2. **What is the semantic order parameter?** Test TopSim, context independence, and positional
   disentanglement as candidates. Which one shows critical behavior?
3. **Do emergent protocols fall into discrete classes?** Universality predicts a small number of
   classes, not a continuum. Cluster protocols across many runs and see.

**Theoretical:**
4. Test the **Erlangen ≅ RG-flow** correspondence formally.
5. Identify **relevant vs irrelevant operators** for semantic systems — is primitive count provably
   irrelevant, as predicted?
6. Does the symmetry of the emergent structure match the octahedral/three-axis structure the
   geometric work suggests — or something else entirely? **Either answer is informative.**

**Framework:**
7. Rewrite the wiki around emergent universality rather than derived necessity. The geometric
   material survives — *reclassified as symmetry evidence rather than foundation*.
8. Recover the archived Erlangen material into the new role.
9. Add the Layer 0 (Distinction) grounding — Spencer-Brown and Bateson — which the project has never
   cited and which supplies its least anthropocentric footing.

---

## 8. What this costs, honestly

- **UL is not a code and not a dictionary.** It is a universality class. There is no universal
  symbol table to publish.
- **The specific primitives are demoted** — from foundation to evidence. That is a promotion in
  rigour and a demotion in status.
- **UWS becomes one rendering**, valuable and ours, not privileged.
- **And the hypothesis can now fail.** If independently emerged protocols show no shared invariants
  under coarse-graining, UL-as-emergent-universal dies — with an actual experiment, not an argument.

That last point is the gain. For the first time the central claim is **falsifiable by experiment
rather than adjudicated by philosophy.**
