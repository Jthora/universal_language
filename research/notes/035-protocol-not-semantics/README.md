# 035 — If the map is not semantic, is it protocol-optimal?

**Type:** cycle
**Opened:** 2026-08-12
**Status:** closed
**Thread:** follows `034`, which found the map is not semantic structure
**Superseded by:** `038` *(partly — the justification, not the claim)*  ← *the one permitted edit to a closed note (`../README.md`)*

---

## 0. The retreat risk, named before starting

`034` tested whether the combinatorial map appears in semantic formalisms. It does not. **This note
asks whether it has a different role instead — and that is structurally the move that produces
unfalsifiable retreats.** The D2 failure mode is: theory fails → relocate theory → declare success.

**The guard I am binding myself to:** the relocated claim must be **falsifiable and make a
prediction the original did not.** If I cannot state what would kill it, this is a retreat and should
be recorded as one rather than pursued.

**And a second guard:** `034`'s evidence is about *semantic* formalisms. A claim about *protocol* is
genuinely a different claim — but only if it is not simply "wherever the evidence isn't." It has to
have independent support.

---

## 1. Before searching  ← written first (S2)

**The candidate reframe.** `034` §3.3 found that rotation and labels differ in *what must be shared*:

- To read a **labelled** graph, two parties must share a **label alphabet and its semantics** — a
  dictionary.
- To read a **rotation** system, they must share only an **orientation convention** — clockwise
  versus counter-clockwise. **One bit.**

**So the claim would be: cyclic order minimizes the shared prior needed to interoperate.** That is
not a claim about meaning; it is a claim about **bootstrapping cost** — and the glossary already has
a layer for exactly that: **UP, the Universal Protocol.**

**Expected to find:** that "minimize arbitrary convention" is already a formalized objective. My
strong suspicion is **Other-Play** (Hu et al.) and the zero-shot coordination line, which exists
precisely to strip out symmetry-dependent arbitrary conventions so independently trained agents can
coordinate. If so, the technical statement is about **equivariance**: a rotation system is
equivariant under vertex relabelling; a label alphabet is not.

**Would change the plan if:** the shared-prior difference turns out to be illusory — e.g. if
orientation conventions are as costly to establish as alphabets, or if the equivariance claim does
not hold.

**The falsifier I am committing to now, so the reframe is not free:**

> If rotation-based encodings do **not** reduce the shared prior required for zero-shot
> interoperation relative to labelled encodings, the reframe fails and the map is spatial overhead
> after all.

**Objects mathematical?** Partly — equivariance and shared-prior cost are formalizable. The empirical
half is the ablation `019` already identified.

## 2. Searches run

| Query | Direction | Result |
|---|---|---|
| Other-Play, zero-shot coordination, equivariance, arbitrary conventions | supporting | **The formalism exists and is exactly this problem** |

## 3. Findings

### 3.1 "Minimize arbitrary convention" is a named objective with a literature

> **Other-Play** is *"a method of **preventing conventions that rely on arbitrarily breaking the
> symmetries of the setting**."*
>
> *"A common failure mode is symmetry breaking, when agents **arbitrarily converge on one out of many
> equivalent but mutually incompatible policies**."*
>
> *"**Equivariant policies** are such that symmetric changes to their observation cause a
> corresponding change to their output, **fundamentally preventing the agent from breaking
> symmetries**."*

**"Equivalent but mutually incompatible" is precisely `034`'s idioglossia**, arrived at from
multi-agent RL rather than from notation.

### 3.2 The claim, made precise

**A label alphabet is a symmetry-breaking choice.** Calling a role `ARG0` rather than `ARG1` is
arbitrary; any permutation of the alphabet yields an equivalent, incompatible system.

**A rotation system is invariant under relabelling** — cyclic order does not depend on what the
vertices or edges are called. But it is not convention-free: it still fixes an **orientation**.

| Encoding | Arbitrary symmetry broken | Size |
|---|---|---|
| **Labels** (alphabet of size n) | **Sₙ** | n! — 6, 24, 720, 40 320 … |
| **Rotation** | **ℤ/2** (orientation only) | **2, for all n** |

> **Rotation breaks strictly less arbitrary symmetry than labelling, and the gap grows factorially.**

That is the quantified form of `034` §3.3's *"one bit versus a dictionary."*

### 3.3 And it addresses a stated open problem in that field

> *"Current symmetry-based methods **assume a priori access to these symmetries**, but automatically
> discovering them can be computationally infeasible."*

**A rotation system carries its symmetry group explicitly** — the cyclic group at each vertex. It
does not require symmetry *discovery*; the structure hands it over. That is a genuine point of
contact rather than an analogy, and it is a contribution *to* that literature rather than a
borrowing from it.

### 3.4 Where this puts the combinatorial map

| Construct | The map's role |
|---|---|
| **UWS** | Its coarse-graining fixed point — established (`024`, scoped in `032`) |
| **UL** | **Not established, and `034` is evidence against** |
| **UP** — Universal Protocol | **A defensible optimum: minimal arbitrary convention** |

**The session's largest result lands on UP, which was the least-developed construct in the glossary.**
UP was defined as *"the bootstrapping/coupling procedure by which two independent instantiations reach
correspondence"* — and minimal shared prior is exactly what a bootstrapping procedure needs.

## 4. Checking this against the guard I set

§0 bound this note to two conditions.

**(a) Does the relocated claim make a prediction the original did not?**

| | Prediction | Status |
|---|---|---|
| Original — map is UL's fixed point | semantic formalisms converge on rotation | **Falsified** (`034`) |
| **New — map minimizes arbitrary convention** | **rotation-based encodings coordinate zero-shot better than label-based ones** | **Untested, and testable** |

**Different predictions. Passes.**

**(b) Does it have independent support, or is it just "wherever the evidence isn't"?**

It has a literature that developed without reference to notation, a quantified statement (ℤ/2 versus
Sₙ), and it addresses a stated open problem in that field. **Passes.**

**The falsifier committed to in §1 stands unchanged:** if rotation-based encodings do not reduce the
shared prior required for zero-shot interoperation, the reframe fails and the map is spatial
overhead.

**Tier honesty: ARGUED, not verified.** The symmetry-group comparison is an argument. The experiment
remains the symbolic-versus-rotation ablation at matched population size that `019` identified —
**now motivated by two independent routes**, which is the strongest reason yet to run it.

## 5. What changed

- `claims.yaml`: `ROTATION-MINIMIZES-CONVENTION` added; `FIXED-POINT-IS-COMBINATORIAL-MAP` gains its
  UP role alongside the UL doubt.
- **UP is no longer the empty construct.** It has a candidate answer with a formal home.
