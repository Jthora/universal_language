# RESEARCH-PROTOCOL.md — How To Research This Correctly

> **This file is how not to get fooled.**
>
> `AGENTS.md` tells you where things are. `PRIMER.md` tells you what is true. **This file is
> method** — the technique for doing research in a domain where the obvious reading of the
> literature is reliably the wrong one.
>
> Read it before searching. It is short, and it will save you from closing a line of work that was
> never actually closed.

---

## 0. The one-paragraph version

**Results that appear to settle this question negatively usually don't**, and the reason is
mechanical rather than mysterious: impossibility results have scope conditions, and the scope is
what gets dropped in transmission. A theorem about microscopic compositionality gets read as a
verdict on emergence. A theorem requiring convexity gets read as a verdict on repair. A theorem
about Turing-expressive systems gets applied to a total language. **In every documented case here,
the counter-evidence was one search away and the finding was still live.** The rules below make
that check automatic instead of leaving it to judgment — because the judgment call is the part that
fails, and it fails while producing confident, rigorous-sounding output.

---

## 1. The evidence this is a real effect

The project owner has worked on this since 2015 and reports the same pattern for over a decade,
across human and machine collaborators alike: **research is found that appears to settle the
question negatively, and it takes sustained pushback to discover that it didn't.**

Seven cases from a single session, with what each one actually supported:

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
| **T8** | apply an impossibility result | **check that the target is inside its scope.** Rice needs Turing-expressiveness. Convexity is metric-only |
| **T9** | cite a theorem for a conclusion | **check the theorem proves the conclusion.** A sound theorem can support an unsound reading — Zadrozny proves an encoding exists, *not* that compositionality is substantively empty. The gap was a condition his construction quietly drops |
| **T12** | introduce a classification and sort things by it | **a classification is a claim.** State what would show the object does *not* fit, *before* sorting. A framework introduced and applied in one motion has never been checked — and a framing determines what every later claim means |
| **T11** | contrast two results as though they were arms of one experiment | **check whether any study actually compared them.** Different papers, tasks, scales and setups do not form a controlled comparison. "X succeeds where Y fails" needs one study varying X and Y — otherwise it is your inference, not a finding |
| **T10** | accept a null result | **ask what was measured.** A theorem states its scope; a null result does not — its scope *is* its operationalization |

**The master signature: your output sounds rigorous and it ends the conversation.** That combination
is the alarm.

---

## 3. Negative-result discipline (R1–R6)

Full text: `research/method/negative-results.md`. **Normative.**

- **R1 — Adversarial search is mandatory on every negative.** Before recording any finding that
  closes a line of work, run **at least one** search in the opposite direction. Not "when warranted"
  — that judgment is the compromised one. Cost: seconds. Would have caught **4 of the 8**.
  - **The opposite of a graveyard is a survivor.** `045` surveyed why ontology projects die and
    derived a design rule from the causes of death alone. The opposite-direction query — *which ones
    survived?* — was never run. Two volleys later a primary read found the **Gene Ontology**: sixty
    monthly releases, monotonous growth, no decay, **manually curated throughout.** The survivor
    falsified the rule. **A graveyard contains only the dead; the failure record tells you what
    kills, and only the survivors tell you what saves.**
  - **The opposite of an obstruction is a workaround.** `047` recorded that universality needs a
    tuning parameter and we have none. One query returns **self-organized criticality** — criticality
    reached *without* tuning, a 1987 result. **The conclusion was published before the search.**
  - **This clause was briefly written as a separate rule, S13.** It is not separate; R1 already
    required it. Folded back deliberately — **a thirty-fourth rule makes the checklist longer and
    less likely to be run, which is how seventeen rules failed at once.** See `048`.
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
- **R9 — A framing is a claim. Register it as one.** Load-bearing assertions buried in a
  `notes:` field get no tier, no evidence check and no falsifier — and a *framing* silently
  determines what every claim under it means. **"UWS is kind A" sat in a notes field and
  contradicted eleven notes of derivation.** See F-027.
- **R8 — Adversarially check your own proposals, not only inherited claims.** A test you designed
  is a claim about what that test would show, and carries the same failure modes. **The
  Euclid/Aristotle dependency test was promoted for a dozen turns as "cheap and decisive" and was
  ill-posed; one query showed it.** R1 governed claims from the literature; nothing governed my own.
  See F-020.
- **R7 — Apply every new rule retroactively before applying it forward.** A rule that governs only
  new work grandfathers in the foundational decisions — which predate the discipline and carry the
  most downstream leverage. **Zadrozny was the most load-bearing negative in this project and never
  received the adversarial search R1 mandates, because R1 was written after it.** One query
  overturned it. See F-019.

---

## 4. Source-independence protocol (S1–S7)

Full text: `research/method/source-independence.md`. **Normative.**

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
- **S8 — Read one primary before registering a priority-0 claim.** Four primary reads in ~40 searches,
  and **all four materially changed a claim.** A summary finds a source; it never carries one.
- **S9 — Synonym sweep before concluding novelty or absence.** One object, many vocabularies —
  *rotation system · combinatorial map · ribbon graph · fat graph · dessin d'enfant · **DCEL***. We
  found the second-to-last by accident thirty notes late, and the last only by running the sweep
  deliberately. **Search three vocabularies before claiming something is absent or ours.**
- **S12 — Every technical term you have already written is a search term you owe.** *"half-edge"*
  sat in our own module doc comment for sixteen notes while the DCEL literature — which had already
  solved our disconnected-components problem **two different ways** — went unread. **S9 fires when
  you make a novelty claim; S12 fires the moment a term enters the repo.** Grep your own source for
  vocabulary before searching the world for it: the cheapest query you will ever run is the one whose
  keyword you typed yourself.
- **S10 — Failure-first survey, at the start.** Find who tried and why they stopped, *before*
  building. The Leibniz/Wilkins/Solresol/Blissymbolics survey was the most informative crawl about
  UWS's prospects and it ran at note 37 of 43. **A field's graveyard is better documented than its
  frontier.** **Then run R1's survivor clause on it** — that survey has still never had one, and
  `037` already half-records a survivor it did not pursue.
- **S11 — Scope-before-cite.** One query on a source's own limitations *before* using it as evidence.
  AMR was cited as universality evidence twice, the second time after the repo had recorded it as
  English-only.
- **S7 — Keep the corruption hypothesis falsifiable and instrumented.** Log, per negative, whether
  counter-evidence was easy or hard to find. **A hypothesis that explains away every disconfirmation
  is structurally identical to the failure mode this project already documented** (a score driven
  32%→100% by patching theory after each disconfirmation).

---

## 5. Empirical evidence — the asymmetry, and the guard on it

**A theorem carries its scope in its statement. A null result does not.** *"We looked and did not
find it"* is only as strong as whether the measurement matched the phenomenon's level — and the
choice of measurement encodes a theory. Measure at the wrong level and the null is an artifact of
the instrument, reported as a fact about the world.

**Three of the seven documented errors in §1 are exactly this**, which is why it earns a rule:

| Null result | What it measured | What we needed |
|---|---|---|
| Evans & Levinson: few universals | **surface** features across languages | coarse-grained invariants |
| ISO pictogram comprehension | **referential** iconicity | structural iconicity |
| Music-notation reform failure | displacing an **entrenched incumbent** | creation in an unserved niche |

So: **for existence claims about structure, proof is decisive and empirical nulls are weak.** For
**membership** claims — does this real system instantiate the structure — empirical evidence is
*necessary* and proof cannot reach.

### The guard — read this before using the rule

**This is not a license to discount inconvenient data, and it will be misused that way if the
asymmetry is stated one-sidedly.** The failure mode on the other side is this project's older and
better-documented disease: a completeness score driven 32% → 100% by explaining away each
disconfirmation.

| Kind of evidence | Weight |
|---|---|
| Empirical **null** offered against an existence claim | Check the operationalization first (T10) |
| Empirical **positive** finding | **Full weight.** No operationalization escape |
| Data disconfirming **our own** stated prediction | **Full weight, no exceptions.** `FAILURES.md` is append-only precisely here |

**Test for honest use:** if the operationalization objection is one you would *not* have raised had
the result gone your way, you are not applying T10 — you are rationalizing. Every use of T10 must
name the specific level mismatch, in advance of knowing the result's direction where possible.

---

## 6. The anchor principle — why proof is the center of gravity

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

## 7. Two-minute checklist

Before recording any finding that closes a line of work:

- [ ] **R1** Ran ≥1 adversarial search against it
- [ ] **R2** Wrote its scope conditions
- [ ] **R3** Named what failed — the claim, or one formalization of it?
- [ ] **R4** Wrote a revival condition
- [ ] **R6** Stated the steelman
- [ ] **R8** If this is my own proposal — searched against it too
- [ ] **T11** If contrasting two results — did one study actually compare them?
- [ ] **T12** If classifying — stated what would show the object doesn't fit?
- [ ] **R9** If this is a framing — registered as a claim, not left in a notes field?
- [ ] **S8** If priority-0 — read one primary, not a summary
- [ ] **S9** If claiming absence/novelty — searched three vocabularies
- [ ] **S11** If citing a source — checked its stated scope first
- [ ] Ran `ruby tools/check.rb --strict`
- [ ] **T5** Checked whether the objects are mathematical
- [ ] **T6** Gave it the same scrutiny as the last positive finding
- [ ] **T9** Checked the theorem proves the conclusion, not just something adjacent
- [ ] **T10** If it's a null result — named the operationalization, and would have raised the
      objection regardless of direction
- [ ] **S7** Logged whether counter-evidence was easy or hard to find

Eighteen boxes. If you can't tick them, you haven't finished the finding.

---

## 8. What this does not fix

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

## 9. Enforcement

- `claims.yaml` requires `scope`, `formalization`, `revival_condition`, and `refutation_tier` on
  every `RETIRED` entry (R3/R4/R5). CI-checkable.
- `FAILURES.md` is **append-only and immutable.** F-012a, F-013a, F-014, F-015, F-016, F-017 are
  the record behind this file. Read them; they are the primary source for §1.
- **The meta-rule:** when a correction is written twice for the same pattern, stop patching
  instances and fix the generator. **Two of one shape is the signal.**
