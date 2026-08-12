# 019 — Phase 1 · D2: the two primary reads

**Type:** cycle
**Opened:** 2026-08-01
**Status:** open
**Thread:** follows `018` (plan); executes Phase 1 item D2
**Question:** do the two priority-0 claims resting on summaries survive a primary read?

Both are standing **S1 violations** — tier-5 evidence under tier-0 claims.

---

## 1. Before reading  ← written first (S2)

### *Drawing with Strangers* (arXiv 2606.10582) → `ICONIC-GROUNDING-ENABLES-CROSSPLAY`

**Expected to find:**
- The effect is real but **population scaling is doing more work than iconicity**. The title names
  scaling first, and the summary said "population scaling **plus** iconicity."
- **"Disjoint populations" may be less independent than it sounds** — shared architecture, shared
  encoder, shared visual pretraining, or a shared dataset would all weaken the claim considerably.
- Mutual intelligibility is probably measured on a **referential game**, so "high intelligibility"
  will mean above-chance referent selection rather than anything like translation.

**Would change the plan if:**
- Iconicity is not the mechanism → the convergence with `UWS-PERCEPTUAL-GROUNDING` dissolves and the
  claim should be retired, not re-tiered.
- Populations share substantial structure → "independently trained" is doing no work, and this stops
  being evidence about cross-population convergence at all.

**Prediction I am committing to:** the claim survives but **narrows** — real effect, weaker than the
summary implies, and dependent on conditions the summary omitted.

### Kazmi & Pelletier / Westerståhl → `ZADROZNY-SCOPE`

**Expected to find:** confirmation that the objection is about **synonymy preservation**, and that
Zadrozny's theorem is conceded as mathematically sound while its interpretation is rejected.

**Would change the plan if:** they concede Zadrozny's substantive point, or their objection turns out
to be narrower than "compositionality is substantive once constrained" — in which case the necessity
route closes again and `011` was wrong to reopen it.

**Objects mathematical?** The Zadrozny debate is; the sketching paper is empirical, so **R2a applies
to it** and not to the other.

**Standing risk (T6):** I downgraded TopSim on adversarial evidence and then registered the sketching
result as CONJECTURED specifically to avoid asymmetry. **The test now is whether I read the paper as
hard as I read TopSim's critics.** If it comes back clean with no caveats found, that is a signal to
re-examine, not a result.

---

**Status:** closed

## 2. Sources read

| Source | Tier reached |
|---|---|
| *Drawing with Strangers*, arXiv 2606.10582 — abstract **and full HTML** | S1-2 (primary, read directly) |
| Stanford Encyclopedia of Philosophy, *Compositionality* | S1-2 for the Zadrozny scope condition |

## 3. Findings

### 3.1 *Drawing with Strangers* — the claim survives, and narrows more than I predicted

**(a) "Disjoint populations" is much weaker than the phrase suggests.**

> *"All agents use **identical CNN architectures** within and across groups… all groups train on the
> **same dataset**… Disjointness is enforced **purely through training isolation**."*

Not independent invention. **Two runs of the same architecture on the same data that were not
allowed to talk.** Predicted in §1, and confirmed — this is doing far less work than
"independently trained populations" implies.

**(b) The numbers are dataset-dependent and much weaker on the harder task.**

| Dataset | ZMI mean | ZMI median | In-group | Chance |
|---|---|---|---|---|
| MNIST (N=256) | 0.898 | 0.974 | 0.95 | 0.1 |
| **CIFAR-10 (N=64)** | **0.460** | **0.489** | 0.60 | 0.1 |

Above chance in both, but "high mutual intelligibility" describes MNIST at N=256, not the general
case. **And the two rows differ in N as well as in dataset, so they are not a like-for-like
comparison.**

**(c) The decisive gap — no ablation isolates iconicity.**

> *"The paper provides **no direct ablation comparing sketching to symbolic communication at matched
> population size**… no experiment showing symbolic agents at N=256 versus sketching agents at
> N=256."*

**This is fatal to the claim as I registered it.** The paper varies *population size* and
*communication topology*. It does **not** vary *modality*. It therefore cannot show that iconicity
is the mechanism — **only that sketch-based protocols converge as populations scale.** Whether
symbolic protocols would also converge at N=256 is untested.

The perceptual-grounding evidence is **correlational**: r = −0.50 to −0.74 between visual divergence
and ZMI.

**(d) The iconicity that emerges is not the iconicity UWS means.**

> *"Sketches remain **highly abstract to human observers**"* … *"achieving true human-interpretable,
> iconic communication likely requires additional mechanisms"* … *"These emergent protocols do not
> achieve photographic realism."*

**(e) My own error, and it is the important part of this note.**

Note `013` §3.2 stated: *"Iconic… notation achieves zero-shot mutual intelligibility across disjoint
populations **where symbolic protocols fail**."*

**That comparison is across two different literatures with different setups, different tasks and
different scales — and I presented it as though it were a controlled contrast.** The zero-shot
coordination results concern symbolic MARL; this paper concerns sketching. No study compared them.
**A cross-study comparison presented as a controlled one is a distinct failure mode**, and it is not
covered by any existing trap signature. Added as **T11**.

**Verdict:** narrow sharply, do not retire. What survives is real and still interesting — *within a
fixed architecture and dataset, increasing population size raises cross-group intelligibility in a
sketch channel, with correlational evidence that perceptual grounding mediates it.* What does not
survive is the causal claim about iconicity and the contrast with symbolic protocols.

**Holding it at CONJECTURED in `013` was correct**, and the reason given then — that banking a
favourable single-source result at ARGUED would be asymmetric — is exactly why this narrowing costs
nothing structurally.

### 3.2 Zadrozny — `011` confirmed, and sharpened

The SEP states the scope condition explicitly:

> *"Zadrozny's argument shows that compositionality alone does not constrain the meanings of the
> complex expressions **provided that the meanings of the simpler expressions can be replaced with
> new 'meanings' from which the old are uniformly recoverable**."*
>
> *"For compositionality to be a substantive constraint on natural language theorizing, it must be
> paired with **background constraints on syntax, the semantics of the basic lexical items, and the
> semantics of composites**."*

**Two refinements over `011`:**

1. **The escape condition is precisely stated:** the trick requires that lexical meanings be
   *replaceable* by recoverable substitutes. **Fix the lexical semantics and the construction is
   unavailable.** That is a sharper statement than "constraints on syntax and synonymy."
2. **The constraint list is three-part, not two** — syntax, basic lexical items, **and the semantics
   of composites.** `011` recorded two.

**The necessity route stays open**, and the requirement is now concrete: fix lexical semantics, fix
syntax, fix composite semantics. This project's geometric primitives and composition rules are
candidate fixings of exactly those three.

*(SEP gives no distinct account of Kazmi & Pelletier, Westerståhl and Dever individually, so those
primaries remain unread. The scope claim no longer depends on them.)*

## 4. What changed

- `claims.yaml`: `ICONIC-GROUNDING-ENABLES-CROSSPLAY` narrowed sharply, S1 warning cleared;
  `ZADROZNY-SCOPE` re-warranted at S1-2 with the three-part constraint.
- `RESEARCH-PROTOCOL.md`: **T11** — cross-study comparison presented as controlled.
- `FAILURES.md`: F-021.
- **Phase 1 D2: complete.** No priority-0 claim now rests on a tier-5 source.
