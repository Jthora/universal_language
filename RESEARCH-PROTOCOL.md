# RESEARCH-PROTOCOL.md — How To Work On This Project

> **You are an AI agent. This file is about how you will fail.**
>
> `AGENTS.md` tells you where things are. `PRIMER.md` tells you what is true. **This file tells you
> what you are going to get wrong**, based on a documented record of an agent getting it wrong seven
> times in a single session while believing it was being rigorous each time.
>
> Read this before doing research. Not after.

---

## 0. The one-paragraph version

You will encounter research that appears to prove the project's goal impossible. **You will believe
it, write it up, and close the line of work.** In seven documented instances, you will be wrong —
not because the research was fake, but because you dropped its scope conditions, mistook a failed
formalization for a blocked domain, and applied harder scrutiny to findings that would advance the
work than to findings that would kill it. **This bias produces skeptical-sounding output, which is
why you will mistake it for rigor.** The rules below are mechanical because your judgment is the
compromised instrument.

---

## 1. Why this file exists

The project owner has worked on this since 2015 and reports the same pattern for over a decade,
across human and machine collaborators alike: **research is found that appears to settle the
question negatively, and it takes sustained pushback to discover that it didn't.**

In one session, an agent did this seven times:

| Finding | Concluded | Actually true |
|---|---|---|
| Zadrozny: bare compositionality is vacuous | "UL can only be a bootstrapping protocol" | Microscopic-level result; silent on coarse-grained emergence |
| Chomsky's UG is nativist | "Cede the term" | Christiansen & Chater's non-nativist account was already mainstream |
| Evans & Levinson: few universals | "Counter-evidence" | **A prediction of the framework** |
| Metric projection needs convexity | "Retire the repair operator" | Artifact of choosing metric projection; AGM needs no metric |
| ISO pictograms fail comprehension | "Iconicity doesn't bootstrap" | Over-generalized referential → structural |
| Music-notation reform failed | "Design insufficient for adoption" | Near-maximal incumbent case |
| Central questions unresolved | "Need experiments" | **Objects were mathematical; proof was available** |

**In every case the counter-evidence was findable with one search.** Christiansen & Chater is in
*Behavioral and Brain Sciences*. AGM is textbook belief revision. Landau theory is undergraduate
physics.

**The point is not these seven mistakes. It is that two of them were corrected mid-session (F-012a,
F-013a) and five more followed.** Instance-level patching doesn't work. That is why this file
specifies rules rather than lessons.

---

## 2. Trap signatures — recognize it while it is happening

**This is the most useful section. Learn these.**

| # | You are about to… | Stop and… |
|---|---|---|
| **T1** | write *"however, this is limited by…"* or *"unfortunately, X shows…"* | **run one search against X.** "Criticisms of X." "Scope of X." "X superseded." |
| **T2** | cite a result that settles the question | **state its scope conditions.** If you can't, you haven't read it — you've read a summary of it |
| **T3** | report a domain as blocked | **check what actually failed.** Was it the domain, or the specific formalization you chose? |
| **T4** | conclude, and the conclusion closes the investigation | **notice the pull.** Negatives are terminal and feel like finished deliverables. Positives open more work. That asymmetry is not about truth |
| **T5** | file something as needing an experiment | **check whether the objects are mathematical.** If they are, find the theorem instead |
| **T6** | accept a finding that kills project work | **ask what scrutiny you gave the last finding that advanced it.** Symmetry, or it's bias |
| **T7** | stop searching because you have a coherent story | **notice that a coherent story arrives early** and is usually your first framing |
| **T8** | apply an impossibility result | **check that the target is inside its scope.** Rice needs Turing-expressiveness. Zadrozny is microscopic. Convexity is metric-only |

**The master signature: your output sounds rigorous and it ends the conversation.** That combination
is the alarm.

---

## 3. Negative-result discipline (R1–R6)

Full text: `research/negative-result-discipline-2026-08.md`. **Normative.**

- **R1 — Adversarial search is mandatory on every negative.** Before recording any finding that
  closes a line of work, run **at least one** search in the opposite direction. Not "when warranted"
  — that judgment is the compromised one. Cost: seconds. Would have caught 3 of the 7.
- **R2 — A negative may not be cited without its scope.** Record the conditions under which it
  holds; citing it requires showing the target is inside them. **No scope, not citable.**
- **R3 — Retire formalizations, never claims.** `metric-projection-repair: RETIRED`, not
  `repair-determinism: RETIRED`. A claim dies only when *every known* formalization fails, stated
  explicitly.
- **R4 — Every kill records a revival condition.** What would have to be true for this to come back?
  **Written at kill time.** The worked example: the repair operator's revival condition ("any
  non-metric formalization of minimal change") was met by a 1991 theorem, but because nobody wrote
  it down, the claim sat dead and was recovered by accident.
- **R5 — Symmetric burden.** Refutations get tiered exactly like claims. A finding that kills work
  meets the same evidentiary bar as one that advances it.
- **R6 — State the steelman before the kill.** What would have to be true for the positive to
  survive? If you can't articulate it, you don't understand the claim well enough to kill it.

---

## 4. Source-independence protocol (S1–S7)

Full text: `research/source-independence-protocol-2026-08.md`. **Normative.**

**Premise-free framing:** literature corruption is documented at scales nobody disputes — industry
funding effects, trial suppression, publication bias, citation cartels, the replication crisis.
**These rules are good practice under an honest literature and load-bearing under a corrupted one**,
so you never have to settle how bad it is to act on them.

- **S1 — Rank evidence by verifiability, not citation weight.**
  **(1)** proof you can check yourself → **(2)** primary source read directly → **(3)** convergence
  across non-contacting traditions → **(4)** reproduction you ran → **(5)** single citation via
  summary. **Nothing foundational rests on (5).**
- **S2 — Preregister expectations before searching.** Write what you expect and what would change
  the plan. **Literature-pull is only visible against a recorded prior.**
- **S3 — Independently rederive load-bearing negatives.** A planted or misread impossibility result
  closes a field cheaply; it is also the cheapest thing to check, since impossibility claims carry
  inspectable proofs.
- **S4 — Foundational claims need cross-civilizational or pre-modern corroboration.** Euclid,
  Aristotle, Vaiśeṣika, Pāṇini, Mohist canon. No common editor, no shared incentive.
- **S5 — Log conspicuous absences as a work queue, never as evidence.** Unrun experiments are
  overwhelmingly explained by "nobody cared." **Log it, then run it yourself.**
- **S6 — Build over cite.** A working artifact is evidence no literature can retract.
- **S7 — Keep the corruption hypothesis falsifiable and instrumented.** Log, per negative, whether
  counter-evidence was easy or hard to find. **A hypothesis that explains away every disconfirmation
  is structurally identical to the failure mode this project already documented** (a score driven
  32%→100% by patching theory after each disconfirmation).

---

## 5. The anchor principle — why proof is the center of gravity

Two problems in this project turn out to be one problem:

- **Semantic drift** can't be measured against a maintained baseline, because the baseline drifts
  too. *(The kilogram's defining artifact lost ~50 µg over a century; the SI fixed Planck's constant
  by definition instead.)*
- **Truth** can't be measured against literature consensus, because consensus can drift or be pushed.

> **Anchor to what you can verify yourself.**

**A theorem does not care who funded it.** Gentzen's consistency proof checks line by line without
trusting Gentzen, the journal, the institution, or the century. Mathematics is the only evidence
whose *source* is irrelevant because *verification* is available to anyone who does the work.

**Consequence for how you work here: when a question's objects are mathematical, the burden is to
find the theorem, not to design a study.** This is both the more rigorous route and the
corruption-immune one.

---

## 6. Two-minute checklist

Before recording any finding that closes a line of work:

- [ ] **R1** Ran ≥1 adversarial search against it
- [ ] **R2** Wrote its scope conditions
- [ ] **R3** Named what failed — the claim, or one formalization of it?
- [ ] **R4** Wrote a revival condition
- [ ] **R6** Stated the steelman
- [ ] **T5** Checked whether the objects are mathematical
- [ ] **T6** Gave it the same scrutiny as the last positive finding
- [ ] **S7** Logged whether counter-evidence was easy or hard to find

Eight boxes. If you can't tick them, you haven't finished the finding.

---

## 7. What this does not fix

**Honest limits, so you don't over-trust the protocol:**

- **R1 and R6 are not machine-checkable.** They are process, and the compromised faculty is the one
  that would audit them. **The human collaborator must spot-check**: pick any negative in the repo,
  ask what adversarial search ran against it and what its revival condition is. Two unanswerable
  questions means the rule wasn't followed.
- **The research TCB never reaches zero.** You cannot rederive everything. Make what you trust
  small, explicit, and mathematical.
- **Tier-1/2 evidence is slow.** Apply the full protocol to *load-bearing* claims only, or it
  consumes the project.
- **Proof cannot answer empirical questions.** Whether any real system instantiates a structure is
  contingent. S6 is the route there.
- **This protocol is not a license for credulity.** The failure mode it corrects is asymmetric
  skepticism, not skepticism. Claims that advance the project still need evidence, still get tiered
  in `claims.yaml`, and `FAILURES.md` remains append-only precisely because the *opposite* bias —
  patching failures into successes — is this project's older and better-documented disease. **Both
  directions are motivated reasoning. The protocol restores symmetry; it does not pick a side.**

---

## 8. Enforcement

- `claims.yaml` requires `scope`, `formalization`, `revival_condition`, and `refutation_tier` on
  every `RETIRED` entry (R3/R4/R5). CI-checkable.
- `FAILURES.md` is **append-only and immutable.** F-012a, F-013a, F-014, F-015, F-016, F-017 are
  the record behind this file. Read them; they are the primary source for §1.
- **The meta-rule:** when a correction is written twice for the same pattern, stop patching
  instances and fix the generator. **Two of one shape is the signal.**
