# 012 — Roster A: fastest paths to falsification

**Type:** cycle
**Opened:** 2026-08-01
**Status:** closed
**Thread:** follows `010` (rosters), `011` (Zadrozny scope)
**Question:** do the load-bearing legs of the current position survive adversarial search?

---

## 1. Before searching  ← written first (S2)

**Expected to find:**

- **Newman's objection** is real and, I predict, **the same shape as Zadrozny**: "any set of the
  right cardinality can be given any structure, therefore structural claims are trivial." If that
  prediction holds, the standard response will also be the same shape — **privilege some relations
  over others** — and our Erlangen fixed point would be a principled way of doing exactly that.
  *If Newman turns out to be a different shape entirely, that prediction is wrong and I should say
  so plainly.*
- **Emergent-communication critique exists and is substantive.** I expect TopSim to be contested as
  a measure, and I expect at least some finding that emergent protocols are degenerate or fail to
  generalize. **This is the one that can actually hurt** — it is our strongest empirical leg.
- **RG outside physics: mixed.** Real work in machine learning; loose analogies elsewhere criticized.
- **Universality in discrete/finite systems:** I expect finite-size effects to be a genuine
  limitation and possibly a serious one.

**Would change the plan if:**

- TopSim is shown invalid, or the 100%-convergence result is an artifact → the empirical leg goes,
  and the position rests on proof alone until a better measure exists.
- Newman's objection applies *and* has no principled response → the generating-set /
  same-structure-different-presentation framing is vacuous, and `011`'s fixed-point table with it.
- RG transfer outside physics is shown to be systematically invalid → the whole universality frame
  needs rebuilding on something else.

**Objects mathematical?** Newman and universality-class applicability are **mathematical** — the
burden there is a theorem, not a study (T5). The emergent-communication critique is **empirical**,
so R2a applies: I must name the operationalization before accepting any null as decisive, and I must
be willing to raise the same objection had the result gone the other way.

**Standing risk on this cycle:** the failure mode being guarded against is *asymmetric scrutiny*
(T6). These searches are designed to hurt. If I come back reporting that everything survived
comfortably, that is itself a signal to re-examine — the point of Roster A is to find damage.

---

## 2. Searches run

All four adversarial (R1) — this roster has no supporting queries by design.

| Query | Result |
|---|---|
| Newman objection, structural realism, cardinality triviality | Objection is real; standard response is "privilege natural relations" |
| Emergent communication, TopSim critique | **Substantive, multiple independent criticisms** |
| RG analogy outside physics, criticism | Real limitation, and it names our exact problem |
| Universality class, finite-size and discrete systems | Genuine caveats; not fatal |

**Counter-evidence: easy to find in every case.** One query each. (S7 — consistent with
`LOOP-ATTRIBUTION-IS-INTERNAL-SO-FAR`; nothing here was obscure or suppressed.)

## 3. Findings

### 3.1 Newman's objection — prediction confirmed, and it is Zadrozny's shape

> *"If our knowledge of the unobservable world were strictly limited to its structure, our knowledge
> turns out to be trivial, amounting to nothing more than knowledge of the **cardinality** of the
> world."* — *"as long as there are enough objects in the relevant domain, one can always obtain a
> structure suitable for that domain."*

**Identical form to Zadrozny:** *you can always construct it that way, therefore the claim is empty.*
Two of the three deepest objections this project faces are the same move.

And the standard response is the one predicted: **privilege some relations over others.** Melia &
Saatsi (2006) apply *"naturalness and causal significance"*; others return to Carnap's *"founded
relations, interpreted as real, experienceable, physical relations."*

**Where we stand better than the standard response, and where we do not.** Those replies *stipulate*
that natural relations exist. **We derive ours**: Erlangen privileges relations by transformation
group, and the survival computation is a calculation rather than an appeal to experience. That is a
stronger answer.

**But it does not come free.** Newman survives as a real constraint: **we must be able to say why the
privileging group tower is not itself arbitrary.** Left open, honestly — this is now the sharpest
philosophical obligation on the framework.

### 3.2 TopSim — this is damage, and it lands on our strongest empirical leg

Four independent criticisms:

- **Cannot distinguish different compositional structures.** *"An emergent language transparently
  concatenating symbols in a fixed order and one mixing deletion and insertion operations on
  free-ordered symbols can have the same topographic similarity."*
- **Does not track generalization.** *"Gains of generalization do not correlate with gains of
  topographic similarity… does not capture agents' language structure in image based settings."*
- **Does not track task success**, and *"does not provide a mapping between emergent communication
  atoms and natural language concepts, and thus cannot directly assess compositionality."*
- *"Small tree reconstruction error is not a sufficient condition for topographic similarity."*

**R2a / T10 do not shield this.** These are criticisms *of the measure*, not null results whose
operationalization I can question. The honesty test applies in reverse: had TopSim been vindicated I
would have banked it as a solid measure. It wasn't. **This is straightforward damage and must be
recorded as such.**

**What survives:** that independently trained agents converge on *something* consistent, and that
**topographic organization** — position encoding property — is observed structurally rather than
via TopSim. Zero-shot generalization is a separate, behavioural measure. **What does not survive:**
"near-perfect compositionality, 100% of seeds" as a load-bearing quantitative claim, and TopSim as a
candidate semantic order parameter.

### 3.3 RG outside physics — the criticism names our exact gap

> *"Unlike in physics, there is **no precise or canonical notion of what defines 'scale'** in data or
> models, and how coarse-graining should be implemented."*
>
> *"RG is commonly applied to physical systems with **many symmetries**, in contrast to deep learning
> which is often applied to data with limited structure."*

**That is a direct hit on `ERLANGEN-AS-COARSE-GRAINING`** — the missing canonical coarse-graining is
precisely what we claimed Erlangen supplies.

It cuts both ways and I want to be careful not to take only the flattering half. Against us: RG
transfer outside physics is **not** generally legitimate, so the burden is now to **prove** the
Erlangen↔RG correspondence rather than assert it as a structural analogy. For us: the criticism
identifies the *absence* of a canonical coarse-graining as the defect, and Erlangen is an actual
group-theoretic candidate — which is more than the ML literature has. **Net: the hypothesis is not
refuted, its burden of proof is raised, and it moves from "attractive analogy" to "must be
demonstrated."**

### 3.4 Finite and discrete systems — real caveats, not fatal

- *"In discrete variable systems… negative moments probe **lattice-scale discreteness** rather than
  genuine critical fluctuations"* — discreteness produces artifacts.
- *"In anisotropic systems… finite-size scaling functions are shown to be **nonuniversal**."*
- But: *"the finite-size scaling framework is **universally applicable** to second-order phase
  transitions,"* and the class is set by *"dimensionality of the system and **symmetry of the order
  parameter**."*

Semantics is discrete and finite, so the caveats bite. They do not invalidate the frame — and note
the last quote reinforces that **symmetry of the order parameter is what defines the class**, which
is the `G/H` route.

## 4. Negatives recorded

**TOPSIM-INSUFFICIENT** — scope, per R2:

- **Scope (R2):** criticisms target TopSim *as a measure of compositional structure*. They do not
  show that agent protocols fail to converge, nor that compositionality is absent — only that this
  metric cannot establish it. Do not cite as "emergent communication shows no structure."
- **Formalization, not claim (R3):** what failed is *TopSim as the operationalization*. The claim
  that independently emerged protocols share structure is untouched and still open.
- **Revival condition (R4):** a measure that distinguishes derivational structure rather than
  string distance. **Candidate already in the literature: Concept-Best-Matching** (arXiv 2403.14705).
  Also live: zero-shot cross-play compatibility, which is behavioural and avoids the metric problem.
- **Steelman before the kill (R6):** TopSim would be adequate if message-space edit distance
  reliably tracked semantic composition. It does not, because different derivations can produce
  identical edit-distance profiles.
- **Refutation tier (R5):** ARGUED — multiple independent papers, consistent finding.

## 5. What changed

- `claims.yaml`: `COMPOSITIONALITY-CONVERGES-EMPIRICALLY` scoped down; `ERLANGEN-AS-COARSE-GRAINING`
  burden raised; `NEWMAN-OBJECTION` added; `ORDER-PARAMETER-FROM-SYMMETRY-BREAKING` strengthened
  relative to metric search.
- **Left open:** why the privileging group tower is not arbitrary (§3.1) — the sharpest
  philosophical obligation now standing.

### The structural observation

**All four hits landed on the empirical side. The proof-side results — κ-stratification, the
Erlangen survival table, Jordan ≅ distinction — are untouched**, because theorems do not depend on
a contested metric.

I flag this as *convenient*, and it should be treated with suspicion rather than satisfaction: it is
exactly what the asymmetry written into R2a predicts, which makes it the kind of result most likely
to be over-read. The defensible version is narrow — **this cycle raised the relative weight of the
proof program because the empirical leg lost a measure, not because proof gained anything.**
