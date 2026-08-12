# 047 — Volley 3: primary reads on the priority-0 claims

**Type:** cycle
**Opened:** 2026-08-12
**Status:** closed
**Thread:** executes Volley 3 of `044`; applies **S8**
**Question:** which priority-0 claims rest on summaries, and what happens when we read the source?

`043` measured this at **four primary reads in ~40 searches, and all four materially changed a
claim.** Eleven claims sit at priority 0.

> **Result: four reads, four claims changed. The running score is 8 for 8, and this volley includes
> the falsification of a claim registered one day earlier.**

---

## 1. The audit  ← before any reading

| Claim | Tier | External quotations | Read a primary? |
|---|---|---|---|
| `CURE-MUST-BE-AUTOMATIC` | ARGUED | **7** | **no — all from `045`'s search summaries** |
| `ROTATION-MINIMIZES-CONVENTION` | ARGUED | 7 | **no** |
| `FIXED-POINT-IS-COMBINATORIAL-MAP` | ARGUED | 8 | partly — `046` read Mount on DCEL, **not** Heffter–Edmonds |
| `CURE-IS-COMPARATOR` | DESIGN-CHOICE | 3 | **no** |
| `SEMANTIC-APARTNESS` | CONJECTURED | 2 | **no** |
| `UL-IS-EMERGENT-UNIVERSAL` | CONJECTURED | 1 | **no** |
| `EXPRESSIVENESS-CHAIN` | ARGUED | 1 | **no** |
| `REGIONS-ARE-FACE-UNIONS` | DESIGN-CHOICE | 1 | **no** |
| `NOTATION-LOGIC-IS-INTUITIONISTIC` | DESIGN-CHOICE | 0 | n/a — internal |
| `ENTRENCHMENT-IS-SURVIVAL-ORDER` | DESIGN-CHOICE | 0 | n/a — internal |
| `CONNECTIVITY-AXIS-MISSING` | ARGUED | 0 | n/a — derived, machine-checked |

### 1a. The audit's first finding is about this session

**`CURE-MUST-BE-AUTOMATIC` was registered at priority 0 in `045`, on seven search-summary
quotations, with no primary read** — in the volley immediately after `044` promoted S8 as the
highest expected yield per query in the plan.

> **S8 was violated one volley after being promoted. Not in old work — here.**

---

## 2. `UL-IS-EMERGENT-UNIVERSAL` — the precondition is unmet, and two fixed points have been running together

**Preregistered:** the physics will check out and the precondition will not; nothing here has
identified a critical point, an order parameter, or a diverging correlation length.

**Confirmed, and the source is more specific than the prediction.** Ansanelli, *Critical Exponents
and the Renormalization Group* (2019), §4, read in full:

> **"The most important step to connect all the theory of the Renormalization Group with the phase
> transitions in statistical mechanics is to identify the critical point with the fixed point of the
> RG flow."**

And the block-spin picture, stated three ways:

> *Above* T_c: *"the configuration looks more randomized."* *Below* T_c: *"the system would choose
> one orientation and would go closer and closer to the configuration with all the spins aligned."*
> **At T = T_c:** *"the configuration does not suffer substantial changes… they are statistically
> the same… the system has **scale invariance** … So, **the correlation length diverges at this
> point.**"*

> **Coarse-graining has fixed points everywhere. Away from criticality they are trivial — all-random
> or all-aligned — and carry no structure. A *non-trivial* fixed point requires tuning to a critical
> point.**

And dimension is a precondition, not a parameter: *"for d = 1 it fails completely: **there is no
phase transition**."*

### 2a. The finding: "fixed point" has been doing double duty across two unrelated formalisms

| | Where it comes from | What it needs |
|---|---|---|
| **Erlangen fixed point** — what survives increasingly coarse transformation groups | invariant theory; Jordan, Whitney–Graustein, Heffter–Edmonds | **nothing** — it is a filtration of invariants under a group tower |
| **RG fixed point** — a universality class | statistical mechanics | **a critical point, an order parameter, a diverging correlation length, a tuning parameter** |

**The project has been using the second to justify claims about the first.** `033`'s
`UL-WORK-IS-FIXED-POINT-WORK` argues that microscopic detail *cannot generalize* **because** UL is a
universality class — then applies that criterion to the *Erlangen* fixed point, which is a different
object arrived at by a different route.

**This is F-027's shape at the top of the claim tree**: one word covering two things.

### 2b. What survives, stated precisely

- **The Erlangen derivation is untouched.** It rests on theorems, needs no critical point, and has a
  genuinely non-trivial limit. **Nothing in `014`, `022`, `024`, `032` or `046` depends on the
  physics.** That is the good news and it is the larger part of the work.
- **`UL-IS-EMERGENT-UNIVERSAL` now has a named, unmet precondition.** It stays CONJECTURED — which
  is the right tier and always was — but its `falsified_by` becomes concrete: **no order parameter,
  no critical point, no universality class.** `033`'s "identify G and H" has been open for fourteen
  notes; we now know it is not a refinement but **the hypothesis of the claim.**
- **`UL-WORK-IS-FIXED-POINT-WORK` loses one of its two legs.** "Curvature dies before the fixed
  point" stands on Erlangen alone — it is a theorem. **"Therefore it cannot generalize to UL" used
  universality**, and that step is now unsupported.

> **Consequence, recorded because it is the third time this item has moved:** the curvature-axis
> repair (`033`) was demoted twice by a criterion that has now lost the leg that did the demoting.
> **It is no longer excluded on principle.** It is merely not a priority.

---

## 3. `CURE-MUST-BE-AUTOMATIC` — falsified as derived, one day after registration

**Preregistered:** the vivid figures are vendor-sourced or differently scoped.

**Something better happened.** The Gene Ontology longitudinal study (Ansanelli's counterpart in this
volley — PLOS ONE, open access, **sixty monthly releases, January 2008 to December 2012**):

> *"number of classes and relations increased monotonously"* — classes **+50%**, relations **+85%**.
> BP *"complexity increased"*; CC *"refined with the addition of leaves providing a finer level"*;
> MF *"complexity remained stable"* with *"uniform modifications."*
> Sustained by **"active curation"** and **"continuous evolution"** — **manual curation**, curators
> making monthly release decisions.

**The Gene Ontology is a human-in-the-loop process that did not fail.** Five years of monotonous
growth, no staleness, no decay.

### 3a. The inference was invalid, and this is exactly what S8 exists to catch

`045` derived the claim as: *"any human-in-the-loop step inherits the documented failure."*

> **That does not follow, and GO is the counterexample. The failure literature shows that *absence
> of a sustaining institution* kills an ontology. It does not show that *human involvement* does.
> The operative variable is institutional sustainment, not automation.**

**What survives is a design rationale, not a derivation:** the Cure should not *require* an
institution that most deployments will never have. **That is a reason to prefer automation. It is
not a proof that automation is necessary** — and the difference is the whole tier.

`CURE-MUST-BE-AUTOMATIC` is **demoted from ARGUED to DESIGN-CHOICE** and rewritten.

### 3b. And the graveyard survey has a blind spot — this earns S13

`045` ran S10, the failure-first survey, and it was the most productive query in that volley. **But a
graveyard contains only the dead.** GO is a survivor, and survivors are not in the graveyard.

> **S13 — pair every graveyard survey with a survivor survey.** The failure record tells you what
> kills. **Only the survivors tell you what saves**, and a design derived from failures alone will
> optimize against the causes of death without ever learning the causes of life.

**S10 produced the error and S8 caught it, two volleys apart.** That is the plan's sequencing
working, and it is an argument for running volleys in the designed order rather than by appetite.

---

## 4. `ROTATION-MINIMIZES-CONVENTION` — wrong in both directions

**Preregistered:** Other-Play is analogically apt and not the "formal home"; its symmetry group is
over the environment, not a label alphabet, so the S_n-versus-ℤ/2 count is ours.

**Half right.** Hu, Lerer, Peysakhovich & Foerster (ICML 2020), §4, read in full:

> *"The first key concept we introduce is the class of **equivalence mappings**, Φ, for a given
> Dec-POMDP. Each element of Φ is a bijection of each of **S, O, and A** onto itself, such that it
> leaves the Dec-POMDP unchanged."*

**So Φ is over the environment.** The S_n-versus-ℤ/2 quantification is ours, and "formal home"
overreaches — that phrasing is corrected.

**But the other half of the prediction was wrong, and it matters more.** The paper grounds those
symmetries in labelling *itself*:

> *"These symmetries are the 'payoff irrelevant' parts of the Dec-POMDP. **They come from the fact
> that the actions and states in the Dec-POMDP do not come with labels**"* … *"Note that **OP does
> not use any action labels.**"*

And the zero-shot setting is defined by their absence:

> focal points *"are grounded in **exogenous** features, **action labels**, that are meaningful due
> to a prior shared context. The zero-shot coordination setting thus is a special form of the tacit
> coordination problem in which **there are no shared exogenous features between the different
> agents**."*

**The link to labelling is not our analogy. It is the paper's own stated basis for the symmetry
group.** I predicted the connection was ours and it is theirs.

### 4a. And the lever game supplies the quantified mechanism we asserted

Ten levers: nine paying **1.0**, symmetric and interchangeable; one paying **0.9**, uniquely
identifiable.

> *"picking one of the 1.0 levers leads to **0.11 expected return**. By contrast, OP suggests the
> choice of the **0.9** lever."*

**Choosing among n symmetric options pays 1/n. Taking a worse but uniquely identifiable option pays
0.9. An eight-fold gap, from arbitrary symmetry-breaking alone.**

> **That is `ROTATION-MINIMIZES-CONVENTION`'s mechanism demonstrated numerically in an unrelated
> field — and it is why a rotation system beats a label alphabet: not because it is better, but
> because it is not one of n! equivalent choices.**

**Net: the claim's substance is stronger than recorded and its citation was weaker.** Both are fixed.

**Confirmed as already recorded:** OP requires the symmetries to be **known in advance**, which the
claim's contribution framing already turns on. That framing survives, and is now sharper — geometry
*supplies* the group where OP must be handed it.

---

## 5. `EXPRESSIVENESS-CHAIN` — the completeness is a theorem over a stated domain, and the count moved

**Preregistered:** the quotation has a stated precondition about what counts as a fold, and
completeness is a theorem with hypotheses rather than a definition.

**Confirmed, with a detail I did not predict.** The seven Huzita–Justin axioms — *"not a minimal set
of axioms but rather the complete set of possible single folds"* — were **not complete**:

> **Lucero (2017) added an eighth**, found *"after enumerating all possible incidences between
> constructible points and lines on a plane."* *"Exhaustive analysis of all possible incidences
> reveals a set of **eight** elementary operations."*

The precondition is exactly the domain: **incidences among constructible points and lines, in the
plane.** And the eighth is peculiar — it *"does not create a new line"* but is *"needed in actual
paper folding when it is required to fold a layer of paper along a line marked on the layer
immediately below."*

### 5a. Why this is worth more than the correction

**This project retired a primitive count as "legacy numbers" precisely because it was chosen rather
than derived.** Origami is the case where a count *was* derived — by exhaustive enumeration over an
explicitly stated domain — **and it still moved, 7 → 8, when the domain was re-examined.**

> **A derived count is not a permanent count. It is a count plus a domain specification, and it is
> exactly as stable as that specification.** That is the standard `FIXED-POINT-TABLE-IS-CANONICAL`
> should be held to.

**And the eighth axiom has our shape.** It is real in the physical practice and null in the
construction algebra — which is precisely the status of the **degree-1 free end** that `024` derived
and found unnamed in the inventory. **An independent field enumerated its operations and found the
same kind of gap in the same place.** That is a reason to register the free end, not merely a
curiosity.

---

## 6. What changed

- `claims.yaml`:
  - `UL-IS-EMERGENT-UNIVERSAL` — precondition made explicit; `falsified_by` sharpened to the missing
    order parameter; the two senses of "fixed point" separated.
  - `UL-WORK-IS-FIXED-POINT-WORK` — the universality leg withdrawn; the Erlangen leg retained.
  - `CURE-MUST-BE-AUTOMATIC` — **ARGUED → DESIGN-CHOICE**, rewritten; GO recorded as counterexample.
  - `ROTATION-MINIMIZES-CONVENTION` — "formal home" corrected; the label grounding attributed to the
    source; the lever game added as the quantified mechanism.
  - `EXPRESSIVENESS-CHAIN` — the 7 → 8 revision and the domain precondition recorded.
- `RESEARCH-PROTOCOL.md`: **S13** — pair every graveyard survey with a survivor survey.
- `FAILURES.md`: **F-030** — S8 violated one volley after being promoted, and the claim it produced
  was false.
- **Open:** the curvature-axis repair is no longer excluded on principle. Not a priority; no longer
  ruled out.
- **Open, and now the project's sharpest question:** is there an order parameter at all? If not,
  `UL-IS-EMERGENT-UNIVERSAL` is not a conjecture awaiting evidence — it is a category error, and the
  Erlangen work stands alone and unaffected.
