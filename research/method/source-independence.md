# Source-Independence Protocol — Research Under an Untrusted Literature

**Date:** 2026-08-01
**Question:** how do we structure research to arrive at the truth about UL even if the literature is
systematically corrupted — planted studies, promoted misdirection, real work suppressed?
**Status:** normative. Companion to `research/method/negative-results.md`.
**Framing:** this document takes no position on whether corruption is occurring. **It does not need
to.** Every rule below is good practice under an honest literature and load-bearing under a
corrupted one, so the premise never has to be settled to act on it.

---

## 1. The problem is real at scales nobody disputes

Systematic literature corruption is documented: tobacco and sugar industry-funded research,
suppressed pharmaceutical trial data, publication bias, the file-drawer effect, citation cartels,
and a replication crisis in which large fractions of published findings fail to reproduce.

**So the mechanism exists.** Scale and intent are the contested part. **The countermeasures are
identical either way**, which is what makes this tractable rather than a matter of belief.

---

## 2. Attribution: what the record actually shows

You've offered a hypothesis for the observed loop — I find "proof it's impossible," you correct me.
F-017 offers a different one: my own asymmetric scrutiny. **Both could be operating. They are
distinguishable, and we should instrument the difference rather than assume either.**

**The discriminator:** if the literature were seeded, counter-evidence should be *systematically
hard to find* — obscure, low-quality, poorly indexed, or absent. If the loop is my bias,
counter-evidence is sitting in plain sight and I simply didn't look.

**On the seven documented instances, counter-evidence was trivially findable every time.**
Christiansen & Chater is in *Behavioral and Brain Sciences* and heavily cited. AGM is a cornerstone
of belief revision with textbook status. Landau theory is undergraduate physics. **One search each.**

That is evidence for the internal-bias hypothesis **on these seven data points**. It does not touch
the broader claim — a corrupted literature could be corrupted elsewhere, or at a level these
questions don't reach. **But it does mean the loop you've been breaking was, so far, mine.** The
honest reading is that your tenacity has been correcting a defect in me, and R1–R6 target that
defect directly.

**S7 below keeps measuring this**, so the question stays open to evidence instead of assumption.

---

## 3. The structural insight

The corruption problem and the drift problem are **the same problem**, and they have the same
solution.

The Cure can't measure drift against a maintained baseline, because the baseline drifts too (the IPK
pathology). It needs an anchor in something invariant that can be checked directly.

**Research can't measure truth against literature consensus, because consensus can drift or be
pushed. It needs the same kind of anchor.**

> **Anchor to what you can verify yourself.**

That is why the turn toward proof this session matters far beyond its original motivation. **A
theorem does not care who funded it.** Gentzen's consistency proof can be checked line by line
without trusting Gentzen, the journal, the institution, or the century. Mathematics is the one
domain where the *source* is irrelevant because the *verification* is available to anyone who does
the work.

**The project was already moving to a corruption-immune epistemic base for unrelated reasons.**

---

## 4. The rules

### S1 — Rank evidence by verifiability, not by citation weight

| Tier | Kind | Why it resists corruption |
|---|---|---|
| **1** | **Proof we can check ourselves** | Source-independent by construction |
| **2** | **Primary source read directly** | No summarizer between us and the claim |
| **3** | **Convergence across mutually hostile / non-contacting traditions** | No single actor seeds all of them |
| **4** | **Reproduction we ran ourselves** | Our data, our instrument |
| **5** | Single citation, read via summary | **May not be load-bearing** |

**Rule: no foundational claim rests on tier 5.** The existing `claims.yaml` tiers rank *how well
established* a claim is; this ranks *how much trust the evidence requires*. They are different axes
and both matter.

### S2 — Preregister expectations before searching

Write down, before running searches: **what we expect to find, and what would change the plan.**

This is the direct instrument against the observed loop. Had I recorded *"I expect Chomsky's UG to
be contestable"* before searching, the drift to "cede the term" would have been visible as a
departure rather than arriving as a conclusion. **Literature-pull is only detectable against a
recorded prior.**

### S3 — Load-bearing negatives require independent rederivation

Never accept *"X was proven impossible"* as an endpoint. **Re-derive it, or establish its scope
conditions from the primary source.** A negative that closes a research direction gets the same
treatment as a security-critical dependency.

This is R2/R3 from the negative-result discipline, promoted: under a corrupted-literature
hypothesis, **planted negatives are the highest-value attack**, because one fake impossibility result
closes a field for a generation at almost no cost to the attacker. It is also the cheapest thing to
check, since impossibility claims carry proofs that can be inspected.

### S4 — Foundational claims need cross-civilizational or pre-modern corroboration

Sources outside the reach of any plausible modern seeding operation: **Euclid, Aristotle, Vaiśeṣika,
Pāṇini, the Mohist canon, Islamic geometry.** Independent civilizations, no shared incentive, no
common editor.

**The `DEFINITIONAL-DEPENDENCY-ISOMORPHISM` test already has this property and it was not designed
for it** — Euclid vs. Aristotle vs. Vaiśeṣika spans three civilizations and two millennia. Under S4
that test is worth more than its already-high priority, because a match across those three cannot be
an artifact of recent literature.

### S5 — Log conspicuous absences as a work queue

Cheap, obvious experiments that nobody has run are worth cataloguing. This session found several:
coarse-graining WALS, clustering emergent protocols into universality classes, the direct
coarse-graining test for shared invariants.

**Discipline on this rule:** an absence is **a work item, never evidence.** Unrun experiments are
overwhelmingly explained by "nobody cared" rather than "it was suppressed," and treating gaps as
proof of suppression is unfalsifiable reasoning. **Log it, then run it ourselves** — which converts
the question from speculation into tier-4 evidence regardless of why the gap existed.

### S6 — Build over cite

Where a question can be settled by constructing the artifact, **construct it.** A working system is
tier-1 evidence that no literature can retract. This is the strongest available answer to the entire
problem: **if we build a functioning UL and it works, the state of the literature becomes
irrelevant.**

### S7 — Keep the corruption hypothesis falsifiable, and instrument it

**Track, per load-bearing negative, whether counter-evidence was easy or hard to find.** Record it.
If counter-evidence is consistently trivial to locate, the loop is internal and R1–R6 are the fix. If
it is consistently obscure, absent, or of anomalously low quality **in a field where the positive
results should be equally cheap to produce**, that is a signal worth taking seriously.

**This is required for the protocol to work at all.** A corruption hypothesis that explains away
every disconfirming finding as planted is unfalsifiable by construction — and that is structurally
identical to the D2 failure this project already documented, where the theory was patched after each
disconfirmation until it scored 100%. **The same discipline that protects against my bias protects
against this one.** Making it measurable keeps it a hypothesis rather than a frame.

---

## 5. What this costs, honestly

- **Tier-1 and tier-2 evidence is slower.** Reading primary sources and re-deriving results is
  expensive compared to accepting a summary. The protocol trades speed for independence, and that is
  the right trade for foundational claims and the wrong one for everything else. **Apply it to
  load-bearing claims only**, or it will consume the project.
- **We cannot rederive everything.** A trusted base exists in research exactly as it does in
  verification — the TCB never reaches zero. The goal is to **make it small, explicit, and
  mathematical**, since mathematical trust is the only kind that can be discharged by anyone
  independently.
- **Proof cannot answer empirical questions.** Whether any real system instantiates the class remains
  contingent, and S6 is the route there — build it and measure.

---

## 6. What changes immediately

1. **The proof turn is now doubly motivated.** It was adopted because the objects are mathematical
   (F-015). It is retained additionally because **mathematics is the only source-independent evidence
   there is.**
2. **`DEFINITIONAL-DEPENDENCY-ISOMORPHISM` gains priority under S4** — three civilizations, two
   millennia, no common editor. Run it early.
3. **The conspicuous-absence list becomes a work queue**, not a grievance: WALS coarse-graining,
   protocol clustering, the direct invariant test.
4. **Preregistration (S2) starts now** — expectations recorded before searches, so literature-pull
   is visible as deviation.
5. **S7 logging starts now**, so the attribution question in §2 accumulates evidence instead of
   remaining a standoff.
