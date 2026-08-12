# 013 — Roster G: emergent communication, properly

**Type:** cycle
**Opened:** 2026-08-01
**Status:** open
**Thread:** follows `012` (which removed TopSim as a usable measure)
**Question:** what does the emergent-communication literature actually establish, once TopSim is off
the table?

---

## 1. Before searching  ← written first (S2)

**Expected to find — including what I expect to hurt:**

- **Zero-shot cross-play probably fails.** My prior is that independently trained agents do *not*
  understand each other without coordination. Zero-shot coordination is a known hard problem in
  multi-agent RL — Other-Play and the Hanabi line exist precisely because self-play agents fail with
  unseen partners. **I expect this to come back negative.**
- **Compositionality likely does not emerge by default.** I expect findings that it requires specific
  pressures — transmission bottlenecks, population turnover, ease-of-teaching — rather than arising
  automatically.
- **Iterated learning is real and well-established** (Kirby). Compositionality emerges from a
  transmission bottleneck. That is a *mechanism*, and mechanisms are more informative than metrics.

**The distinction I am committing to in advance, so it cannot be deployed post-hoc:**

Universality predicts **shared invariants under coarse-graining**, not identical protocols. Water and
uniaxial magnets share critical exponents; you cannot pour a magnet. **So class membership does not
entail mutual intelligibility**, and cross-play is a *one-sided* test:

- Cross-play **succeeds** → strong support, more than universality strictly requires.
- Cross-play **fails** → **uninformative for the universality claim**, but genuinely bad for stronger
  readings of UL as a shared code.

**I am writing this down before searching precisely because it is the kind of reasoning that becomes
an unfalsifiable escape hatch when produced afterward.** If cross-play fails and I then invoke this
distinction, the record should show it was committed to in advance — and if I find myself reaching
for any *other* rescue not written here, that is rationalization and should be called as such.

**Would change the plan if:**
- Compositionality is shown to require pressures with no analogue outside a training loop → the
  cross-substrate claim weakens sharply.
- Coarse-grained descriptors are shown *not* to cluster into few classes → the universality claim
  fails its own central prediction.

**Objects mathematical?** No — this roster is empirical throughout, so **R2a governs**: any null must
have its operationalization named before it is treated as decisive, and I must be willing to raise
that objection regardless of which way the result cuts.

---

**Status:** closed

## 2. Searches run

| Query | Direction | Result |
|---|---|---|
| Zero-shot cross-play, independently trained agents | **adversarial (R1)** | Symbolic protocols **fail** — as predicted. But iconic/spatial ones **succeed** |
| Compositionality by default vs. under pressure | **adversarial (R1)** | Requires two pressures — as predicted |

**Counter-evidence: easy to find.** (S7)

## 3. Findings

### 3.1 Zero-shot cross-play fails for symbolic protocols — prediction confirmed

> *"Symbolic channels admit **near-permutation invariance over tokens**, meaning many equally valid
> equilibria can emerge that are **mutually unintelligible across independently trained
> populations**."*
>
> *"Existing symbolic approaches often devolve into **idioglossia** — co-adapted protocols that are
> unintelligible to outsiders."* Off-the-shelf MARL produces *"arbitrary conventions which are
> effective within the training population but lead to miscoordination with unfamiliar partners."*

**Predicted, and it lands where predicted.** Per §1 this is *uninformative for the universality
claim* — class membership does not entail mutual intelligibility — but it is **genuinely bad for any
reading of UL as a shared code that independent systems would simply arrive at.**

The mechanism is worth keeping: **token permutation invariance.** Arbitrary symbols have no
constraint fixing which token means what, so every permutation is an equally good equilibrium. That
is a precise statement of *why* arbitrary symbol systems cannot converge.

### 3.2 The result that cuts the other way — and it is UWS's exact design

> *"Population scaling improves cross-group intelligibility by **anchoring communication on intrinsic
> visual features**, naturally leading to generalization across strangers."*
>
> *"**Sketches communicate iconically through visual resemblance and spatial structure, thereby
> anchoring meaning in perceptual reality, enabling independently trained agents sampled from
> disjoint populations to achieve high mutual intelligibility.**"*

**Iconic, spatially structured, perceptually anchored notation achieves zero-shot mutual
intelligibility across disjoint populations where symbolic protocols fail.**

That is a description of UWS — iconic mark-features plus a spatial placement grammar — and it
identifies *perceptual anchoring* as the mechanism. **This converges with `UWS-PERCEPTUAL-GROUNDING`
from a completely independent direction:** Changizi's finding that 100+ writing systems share
contour statistics matching natural-scene structure. Two unrelated lines land on perceptual
grounding as what makes a notation work across independently developed systems.

It also **dissolves the token-permutation problem** in §3.1: iconicity is precisely the constraint
that breaks permutation invariance, because the mark is not arbitrary with respect to its referent.

**Symmetry check, and it constrains what I may claim here (T6, S1).** I downgraded TopSim on
multiple independent criticisms one note ago. This is **a single paper, read via summary — evidence
tier S1-5, which by our own rule may not be load-bearing.** It would be exactly the asymmetric
scrutiny F-017 documents to bank this at ARGUED while having demoted a result that cut the other
way. **Registered CONJECTURED. Primary source must be read before it carries weight.** Note also
that the finding is *population scaling **plus** iconicity*, not iconicity alone.

### 3.3 Compositionality is not automatic — it requires two pressures

> Kirby et al. (2015): compositionality requires **both compression pressure** (from the learning
> bottleneck) **and communication pressure** (from the need to be expressive); *"compression alone
> produces degenerate results."*
>
> *"Non-compositional languages will not persist over time when the bottleneck on cultural
> transmission is tight."*

**Predicted.** Compositionality does not arise by default.

**What this does to the claim, stated carefully.** It kills any naive "compositionality is inevitable"
reading. But naming the pressures is not a defeat — **a universality class is defined precisely by
which operators are relevant**, and this identifies two: compression and expressivity. Any system
under both should land in the same class regardless of substrate. Neither pressure is human-specific;
any finite learner transmitting through a bottleneck faces them, which is the Christiansen & Chater
extension given a concrete mechanism.

**Flagging this as post-hoc.** I predicted the finding but did not pre-commit to this reading, so it
is a candidate interpretation rather than a result. What makes it more than a rescue is that it is
**testable**: if compositionality appears in *any* system under compression + expressivity pressure
across substrates, that is the universality claim with a mechanism attached. If it appears only
under substrate-specific conditions, the reading fails.

Two further leads, unchecked: *"compositional structure can emerge **without generational
transmission**"* (Cognition), and compositionality emerging in **deep linear networks** (PNAS) —
the latter a substrate-independence datapoint that owes nothing to agents playing games.

## 4. Negatives recorded

**SYMBOLIC-PROTOCOLS-FAIL-CROSSPLAY**

- **Scope (R2):** applies to **arbitrary symbolic** channels, where token permutation invariance
  leaves many equally valid equilibria. Does **not** apply to iconic or perceptually anchored
  channels — §3.2 is the direct counter-case. Do not cite as "emergent protocols never converge."
- **Formalization (R3):** what fails is *arbitrary symbol assignment*, not emergent communication.
- **Revival condition (R4):** already partly met — perceptual anchoring plus population scaling
  produces cross-population intelligibility.
- **Steelman (R6):** symbolic protocols would converge if something fixed which token means what.
  Nothing in an arbitrary channel does. Iconicity is exactly that constraint.
- **Refutation tier (R5):** ARGUED — consistent across the zero-shot coordination literature.

## 5. What changed

- `claims.yaml`: `SYMBOLIC-PROTOCOLS-FAIL-CROSSPLAY`, `ICONIC-GROUNDING-ENABLES-CROSSPLAY`
  (CONJECTURED, single source), `COMPOSITIONALITY-REQUIRES-PRESSURE`.
- **Next, and it is now high value:** read *Drawing with Strangers* (arXiv 2606.10582) primarily,
  at tier S1-2. If it holds, it is the strongest empirical result this project has — and it
  vindicates the notation's design on grounds independent of any claim about necessity.
