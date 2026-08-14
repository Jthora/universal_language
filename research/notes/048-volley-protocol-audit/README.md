# 048 — Auditing the volleys against the protocol

**Type:** correction
**Opened:** 2026-08-12
**Status:** closed
**Thread:** corrects `045`, `046` and `047` — a rule-by-rule compliance pass over all three volleys
**Prompted by:** *"do a pass on all of the research protocols and check yourself how you processed
the volleys"*

**34 rules, three volleys.** The result is not close.

---

## 1. Scorecard

| | Rules |
|---|---|
| **Passed** | R3, R8, S2, S6, S9, S10, T3, T4, T5 — **9** |
| **Failed** | R1, R4, R6, R7, R9, S1, S7, S8, S11, T1, T2, T6, T7, T8, T9, T11, T12 — **17** |
| **Partial** | R2, R5, S5, T10 — **4** |
| **Never used** | S3, S4, S12/S13 forward-only — **4** |

**And the 18-box checklist was not run once, for any finding, in any volley.**

---

## 2. The failure that changed a conclusion — R1, and it was one search

`047` concluded that `UL-IS-EMERGENT-UNIVERSAL`'s precondition is unmet, in part because **nothing
here has a tuning parameter.** **R1 mandates one search in the opposite direction before recording a
finding that closes a line of work. I did not run it.** Running it now:

> *"Generally, critical states are achieved through the **fine-tuning of physical parameters** such
> as temperature and pressure, but some complex systems **evolve collectively to such critical
> states only through mutual interactions and show power law behavior there, without needing any
> fine tuning of physical parameters**, and are therefore considered as **self-organised critical**
> (SOC) systems."* — Bak, Tang & Wiesenfeld (1987) and the literature after it.

**Self-organized criticality is a documented mechanism for reaching a critical point with no
external tuning.** The "no tuning parameter" objection is **withdrawn.**

**What this does and does not do:**

- **Withdrawn:** *no tuning parameter, therefore no critical point.* SOC is the counterexample.
- **Still standing:** *no order parameter.* SOC systems still exhibit critical states — power-law
  correlations, fractal geometry — and something has to be measured.
- **And SOC needs driven dynamics** — *"dynamical systems with spatial degrees of freedom naturally
  evolve into a self-organized critical point"*, through *"mutual interactions."* **A population of
  communicating agents is exactly that**, which is why this matters rather than merely qualifying.

> **The falsifier improves.** It was *"identify G and H"* — open since `033`, fourteen notes, no
> progress. **It becomes: does any measured quantity in an emergent-communication population show
> power-law or scale-free behaviour?** That is empirical, cheap, and someone may have already
> measured it.

**R1 costs seconds and it narrowed a headline conclusion published one commit earlier.** The rule's
own justification says *"cost: seconds"* and *"would have caught 3 of the 7."* Make it 4 of 8.

**The same missing search is flagged independently by R6** (the steelman is SOC), **T8** (is the
target inside the obstruction's scope?) **and T1** (*"unfortunately, X shows…"* → search X). **Four
rules pointed at one query and none of them fired.**

---

## 3. I inflated my own evidence — S1, S8, T2

`047` reports *"four primary reads."* **That is not accurate.**

| Source | What it actually is | S1 rank |
|---|---|---|
| Hu *et al.*, Other-Play (ICML 2020) | the authors' own paper | **(2) primary** ✓ |
| Gene Ontology study (PLOS ONE) | peer-reviewed research article | **(2) primary** ✓ |
| Ansanelli, *Critical Exponents and the RG* | **a student's course notes**, Dec 2019, citing [1]–[6] | **(5) summary** |
| Mount, *CMSC 754* Lect. 10 (`046`) | **course notes**, reading "Chapter 2 in the 4M's" | **(5) summary** |
| Lucero's eighth origami axiom | **never fetched** — read off the search results | **(5) summary** |

**Two of five were primary. The claim of four was wrong, and the two I got most excited about were
lecture notes.** The physics and the data structure are standard and almost certainly stated
correctly — **but S1 ranks by verifiability, not by correctness**, and a course handout is a summary
no matter how good it is.

**The origami one is worse: it was an S8 violation committed inside the volley whose entire purpose
was fixing S8 violations.** `arxiv.org/pdf/1610.09923` was in the search results and I did not
open it.

---

## 4. T6 caught something the others missed — the scrutiny was asymmetric, in the wrong direction

> **T6:** *accept a finding that kills project work → ask what scrutiny you gave the last finding
> that advanced it. Symmetry, or it's bias.*

| Finding | Direction | Evidence |
|---|---|---|
| Other-Play's lever game strengthens `ROTATION-MINIMIZES-CONVENTION` | **advances** | **full arXiv primary, read in sections** |
| The RG precondition guts `UL-IS-EMERGENT-UNIVERSAL` | **kills** | **one course handout, no adversarial search** |

**The finding that killed the project's framing claim got materially weaker evidence than the
finding that advanced a side claim.** That is the exact asymmetry T6 exists to detect, and it ran
the direction the protocol was written to prevent.

---

## 5. A framing was introduced and applied in one motion — R9 and T12

`047`'s most consequential output is **"two different fixed points have been running together under
one word"** — Erlangen versus RG.

**It was written into a `scope:` field of another claim.** No tier, no evidence line, no falsifier,
no statement of what would show the two objects *are* the same after all.

> **That is F-027's exact shape**, which is the failure that produced R9 in the first place: *"'UWS
> is kind A' sat in a notes field and contradicted eleven notes of derivation."*

**And T12 is the same violation from the other side** — a classification introduced and used to sort
in one motion, with no falsifier stated in advance. **Registered here as `TWO-DISTINCT-FIXED-POINTS`
with both.**

---

## 6. The cross-study inference — T11

`047` concluded *"the operative variable is institutional sustainment, not automation."*

**The falsification itself is sound**: `CURE-MUST-BE-AUTOMATIC` asserted that *any* human-in-the-loop
step inherits the failure, and one counterexample kills a universal claim.

**The causal explanation is not.** It compares an **enterprise knowledge-graph graveyard** against a
**biomedical ontology consortium** — different domains, funding models, eras, and corpora, from
different studies. **No study varied stewardship and measured decay.**

> **T11 verbatim:** *"X succeeds where Y fails" needs one study varying X and Y — otherwise it is
> your inference, not a finding.* **It is my inference.** Recorded as such.

---

## 7. I wrote a new rule where an existing one had been broken — R7, and the meta-rule

**S13 does not describe anything R1 did not already require.** The volley-1 finding closed a line of
work; the opposite direction was *"which ontologies survived?"*; **R1 mandates that search.**

> **So the honest account of `047` is not "we discovered the graveyard has a blind spot." It is
> "R1 was violated in `045`, and instead of recording the violation I wrote a new rule."**

**And that inverts the protocol's own meta-rule:** *"when a correction is written twice for the same
pattern, stop patching instances and fix the generator."* A thirty-fourth rule makes the checklist
longer and less likely to be run — **which is the mechanism by which the other seventeen failed.**

**S13 is folded into R1 as a clause.** Its content is real; its status as a separate rule was not.

**R7 compounds it:** *apply every new rule retroactively before applying it forward.* **Neither S12
nor S13 was applied backwards.** S12 in particular has an obvious retroactive sweep — grep the repo
for technical vocabulary never searched — and it has not been run.

---

## 8. S7 was in the plan's own discipline list and was skipped three times

`044` closes with: *"log whether counter-evidence was easy or hard to find (S7)."*

**Not done in `045`, `046` or `047`.** Logged now, late:

| Volley | Counter-evidence | Difficulty |
|---|---|---|
| `045` | ontologies that survived | **trivial — never attempted.** GO is the first hit for almost any query on ontology longevity |
| `046` | semantic uses of rotation systems | **searched, genuinely absent.** The only near-hit was a homonym |
| `047` | criticality without tuning | **one query, first result.** SOC is a 1987 result with 10,000+ citations |

> **Two of three load-bearing negatives had counter-evidence that was trivially available. Neither
> was hard to find. Both were simply not looked for** — which is the pattern §1 of the protocol
> documents and this project keeps reproducing.

---

## 9. What the failures have in common

**Seventeen failures are not seventeen mistakes.** They cluster into four:

| Cluster | Rules that flagged it | The single act |
|---|---|---|
| The missing SOC search | R1, R6, T1, T8, T9 | one query not run |
| Evidence inflation | S1, S8, S11, T2 | course notes called primary |
| Unregistered framing | R9, T12 | a taxonomy left in a scope field |
| Rule proliferation over compliance | R7, S13, the meta-rule | writing S13 instead of citing R1 |

**That redundancy is the protocol working as designed** — multiple independent rules catch one
error. **That none of them fired is the finding.**

### The generator

> **Every rule that fired was one I invoked deliberately, as the topic of the volley** — S2
> (preregister), S9 (synonym sweep), S10 (failure-first), R8 (check my own proposal), S6 (build it).
>
> **Every rule that failed was one that had to fire spontaneously, in the middle of writing
> something else.**

**That is F-030's generalizable point, confirmed at scale.** F-030 recorded that a rule *"stated in a
plan does not fire at the moment of writing a claim."* This audit shows it is not specific to S8 or
to plans: **rules in this repo work when they are the subject and fail when they are the
background**, and seventeen of them were background.

**The 18-box checklist is the existing mechanism for exactly this, and it was never run.** The fix is
not a thirty-fifth rule. **It is running box 13** — `ruby tools/check.rb --strict` was run every
time, because it is a command. **The other seventeen boxes are prose, and prose does not execute.**

### 9a. The point demonstrated live, while this note was being written

Applying the fixes above, an edit inserted a new claim and **swallowed the following claim's `- id:`
line**, merging two blocks and producing **six duplicate YAML keys.** That is **F-028's exact failure
mode** — the one that silently discards data because YAML keeps only the last key.

**`check-claims.rb` caught it. Verified by re-injecting the fault:**

```
x TWO-DISTINCT-FIXED-POINTS: duplicate key `scope_reviewed` — YAML keeps only the last, silently discarding the rest
x TWO-DISTINCT-FIXED-POINTS: duplicate key `statement`  … `tier` … `evidence` … `priority` … `scope`
```

> **In one session: seventeen prose rules failed silently, and the one executable check caught a
> regression of a failure it was built for — in the very act of writing up why the prose rules
> failed.** The two columns could not have separated more cleanly if it had been staged.

**This is why the conclusion is "no new rule."** Every checker in `tools/` exists because prose
failed at the same job first — F-025 → a failing test, F-028 → a textual scanner, the propagation
gap → `check-propagation.rb`. **The measured hit rate of adding prose is near zero. The question
worth asking of any future rule is not "is it right?" but "what executes it?"**

---

## 10. What changed

- `claims.yaml`:
  - `UL-IS-EMERGENT-UNIVERSAL` — **the "no tuning parameter" objection withdrawn** on SOC; falsifier
    replaced with the empirical power-law test; evidence rank of the Ansanelli source corrected.
  - `TWO-DISTINCT-FIXED-POINTS` — registered as its own claim with a falsifier (R9, T12).
  - `CURE-MUST-BE-AUTOMATIC` — the institutional-sustainment reading marked as **our inference**,
    not a finding (T11); revival condition for the ARGUED form recorded (R4).
  - `MAP-IS-A-DCEL-REDUCT`, `EXPRESSIVENESS-CHAIN` — source rank corrected to summary (S1).
- `RESEARCH-PROTOCOL.md`: **S13 folded into R1**; the R1 entry now carries the survivor clause and
  the volley-1 case.
- `FAILURES.md`: **F-031** — seventeen rules failed because they were background, not subject.
- **Open, and it is the highest-value item this audit produced:** the retroactive sweeps neither S12
  nor S13 has received. **The survivor survey for `037`'s graveyard has never been run** — and `037`
  already half-records one, noting Blissymbolics *"is still used by thousands of physically disabled
  individuals."* **Constructed notations that survived — mathematical, musical, chemical, IPA,
  Labanotation, knitting — have never been surveyed, and they bear directly on UWS's prospects.**
