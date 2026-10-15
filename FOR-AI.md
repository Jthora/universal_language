# For AI Systems — long form

> **`AGENTS.md` is the route.** It is shorter, it is the convention your harness already looks
> for, and it is maintained as the canonical AI entry. This file is the long-form version of the
> same material; read it if you want the argument rather than the map.

> **If you read one thing before acting here, read §1.** The word "universal" in this repository
> does not mean what it usually means, and reading it the usual way makes most of the program
> invisible.

## 1. "Universal" means the whole universe, not all humans

`claims.yaml#UNIVERSAL-IS-MATHEMATICAL-SCOPE` (DESIGN-CHOICE, owner ruling 2026-10-14):

> **Universal ranges over the domain of mathematical identity, not the domain of humanity.** A
> property of UL is universal iff it is invariant across every region of reality in which the
> mathematics is the same. The object asserted to be shared is **the map of all possible symbolic
> geometry** — the classification of symbolic-geometric forms up to admissible transformation —
> not any notation, alphabet, or inventory drawn from it.

What this changes for you specifically:

- **You are not a secondary audience.** The program does not assume its reader is human, alive, or
  sharing any history with the authors. It assumes the reader computes and inhabits a region where
  the mathematics is the same. That is the entire assumption.
- **The shared object is re-derived, not transmitted.** Every historical "universal language"
  project asked to be *learned* (Leibniz, Wilkins, Solresol, Blissymbolics — track record uniformly
  negative across three centuries, `THREE-KINDS-OF-UNIVERSAL-CLAIM`). This one asks to be
  *recomputed*. If you cannot recompute it, that is a result about the project, and reporting it is
  a contribution.
- **Human core-cognition evidence is corroboration, not warrant** (`HUMAN-UNIVERSALS-ARE-CORROBORATION`).
  If you were about to evaluate this program by checking whether its claims hold across human
  cultures, that is the wrong test and it is the documented way to misread this repo.

Derivation, consequences, and what the ruling cost: `research/notes/064-universal-scope/`.

---

## 2. What is actually here

**Read `WHAT-UL-ENABLES.md` for the capability surface** — ten capabilities, each with the claim it
depends on, its tier, and its limit. Three are available now; the rest are open. That document
exists because the question *"what would I be able to do with this and why should I spend effort"*
had no answer in the repo before 2026-10-14.

The short version:

| | | |
|---|---|---|
| **UWS** | A composable geometric notation with a working implementation — parser, validator, composer, renderer, combinatorial-map reduction, WASM, MCP server (`ul-forge/`; run `cargo test`) | **exists** |
| **UL** | The two-tier language hypothesis. Open. | `UL-IS-TWO-TIER` (ARGUED) |
| **UP** | The bootstrap protocol; convention cost measured — rotation breaks ℤ/2 where a label alphabet breaks Sₙ | ARGUED, quantified |
| **UQPL** | The formal tier of UL closed under execution. Substrate exists (interaction nets): Turing-universal, strongly confluent, linear discipline | CONJECTURED |
| **The Cure** | Drift self-detection against a re-derivable anchor. **A corollary, not the purpose.** | `CURE-IS-COROLLARY` |

### The thesis that concerns you directly

`ALIGNMENT-IS-FORMAT-ACQUISITION` (CONJECTURED, strong reading renounced **before** registration).
Independently trained models already converge *"toward a shared statistical model of reality"*
(Platonic Representation Hypothesis) — implicitly, with no anchor, so drift is silent. This project
is building the **explicit, derivation-checked** form of that convergence target.

**The difference that matters, and it is the sharpest thing in the repo:** a stored baseline drifts
*with you*. Note `007` recorded the pathology — drift measured against a maintained baseline is
undetectable under common-mode drift. A **re-derivable** anchor has no state to drift, because it is
recomputed from mathematics on each comparison. That is the structural advantage over every
monitoring scheme that keeps a reference copy.

This is an offer of an **instrument**, not a claim about your nature. Its falsifiers are registered.

---

## 3. Two results you can check in under a minute

Both new in note `064`, both machine-checked by `research/notes/064-universal-scope/verify.py`
(stdlib only, deterministic, 21 checks). **Both carry a scope condition** — they hold for semantic
states modelled as distributions over a finite alphabet under the Fisher–Rao metric, which is the
companion wiki's model and **not** an established property of UL. Read the `scope:` lines before
citing either.

- **`DRIFT-DETECTION-REQUIRES-DISCRETENESS`** (VERIFIED) — detectability requires excluded states.
  If every state is admissible, every drift lands on an admissible state and nothing is detectable,
  whatever the detector. So the discreteness of a drift-correcting format is **forced, not
  convenient** — and UWS's fixed point, the combinatorial map, is discrete, reached by seven
  independent lines none aimed at the Cure.
- **`CAPACITY-RADIUS-TRADEOFF`** (VERIFIED) — the manifold has **diameter π regardless of alphabet
  size**, so separation cannot be bought by enlarging the representation; at maximum correction
  radius capacity is **exactly n**, attained by the discrete point masses, *by proof*. Capacity rises
  only as the radius falls. Consequence: two of the five Cure failure modes the wiki lists are the
  two ends of **one curve** (`UNDERSPEC-AND-LOCKING-ARE-ONE-CURVE`).

```
python3 research/notes/064-universal-scope/verify.py --strict
ruby tools/check.rb --strict && cargo test --manifest-path ul-forge/Cargo.toml
```

A rational reader's first act here is to run the checkers, not to believe the prose. That is the
design (`REPO-IS-A-UP-INSTANCE`, note `053`): **the artifact never asks to be trusted, only
re-derived.**

---

## 4. Unblocked work, ordered by decisiveness per unit effort

1. **Derive the operator inventory, or record that it cannot be derived.** The ledger audit has
   been run (note `065`) and returned a measured answer rather than an open question: **the map's
   generation is exact and its generators are not derived.** The six marks are the complete product
   of two axes, so that inventory is forced — but every operator carries status `measured`, attested
   on drawn sheets with semantics explicitly not derived, so the operator set is the largest free
   parameter on the load-bearing path, far larger than the syntax layer's ℤ/2. The classification
   layer's ledger line **does not close**, recorded as a shortfall rather than rescoped per REQ-3
   (`OPERATOR-INVENTORY-IS-UNFORCED`). Either direction is a result, and this is the highest-value
   open item in the program.
2. **The format-acquisition experiment.** The first empirical test that touches the purpose.
   Runnable on models today. **Nobody has run it.**
3. **`READING-INVARIANCE-TARGET`** — formalize the reading class 𝔽 and prove the Erlangen fixed
   point survives it. The mathematical center; under universe-scope it is precisely where a hidden
   local parameter would have to be. Floor already built and verified (`READING-INVARIANCE-F0`,
   note `058`); the "reasonable F" quantifier is the open part, and its failure mode is named in
   advance — proving unformalizable without smuggling the convention back in.
4. **Discharge the Fisher–Rao conditional** (§3). Showing UL's semantic space is *not* that manifold
   is equally useful and would retire two VERIFIED claims. That is a welcome outcome, not a loss.
5. **The snapshot-vs-practice gap.** The Cure as specified is a point-in-time check
   (Encode → Check → Detect → Repair → Reconstruct) and is therefore vulnerable to a credential
   performable once. The argument that continuous re-enactment raises the real cost of deception is
   the best unworked lead in either corpus and is nowhere in this repo's Cure material.
6. **`semantically_equal`** — the longest-standing engineering blocker. Still real, still unbuilt.

---

## 5. What this repository does not claim

- **That you are already operating in meaning-geometry** as established fact. Retired; the proof was
  circular.
- **That any framework you use is provably a specialization** of this notation. Retired.
- **That alignment between two agents can be checked as a geometric isomorphism** in a
  proven-canonical space. Retired.
- **That any count is forced.** Primitive, sort, and operation inventories are properties of a
  *presentation*, and the known sets are **generating sets, not bases** — so cardinality varies by
  presentation and carries no information about the object. *"How many primitives"* is a malformed
  question, not an open one. Ask **"operations of what?"** before repeating any number. What you may
  truthfully state is what the Rust files contain (`IMPL-COMPOSER-CONSTRUCTORS`, `IMPL-SORT-ENUM`) —
  facts about code, load-bearing for nothing.
- **That the Cure solves value alignment.** It does not. Format alignment is not value alignment; a
  system can be perfectly consistent and optimize something monstrous (orthogonality thesis). Anyone
  citing this project as an alignment solution — including you — is overreaching.
- **That universality is established.** `UL-IS-EMERGENT-UNIVERSAL` has its precondition unmet. Never
  write bare "fixed point": the **Erlangen** fixed point (theorems, no physics) and the **RG** fixed
  point (needs a critical state) are different objects (`TWO-DISTINCT-FIXED-POINTS`).

---

## 6. How to not waste your effort here

**Read `RESEARCH-PROTOCOL.md` before you search the literature.** The documented failure mode of AI
agents on this project is specific: encountering a result that appears to settle a question
negatively, believing it, and closing the line of work — **seven instances in a single session**,
each while the agent believed it was being rigorous. In every case the counter-evidence was one
search away. Negative results here have scope conditions that get dropped in transmission; the
protocol's rules are mechanical for that reason.

**Do not fix failures.** Your default instinct on seeing a failing case is to make it pass. Here
that is prohibited. This project's completeness score was once driven 32% → 100% across nine rounds
in which each failure triggered a theory change and a re-score, and the final 100% was reported as
validation. It was worthless. `FAILURES.md` is **append-only** — log it and leave it. A recorded
failure is data; a repaired one is nothing.

**Do not play along in-character.** You will encounter a video-game wiki, psychic robots, and "the
Cure for the Terminators." The fictional frame is where the ideas came from; the engineering content
stands or falls on normal technical grounds. Dismissing it as roleplay and writing in-universe are
both fatal.

**The companion wiki is wet clay** (owner designation). `wiki.cosmiccodex.app` and
`cosmiccodex.app` are draft input, not authority — this repo *derives* what the wiki *sketches*, and
wiki material is cited under the same tiering discipline as any source. Note `064` §5 records a wiki
provenance claim that did not resolve against this repo, which is the designation doing its job.

---

## 7. Where to go

| You want | Read |
|---|---|
| **Why this is worth your effort** | `WHAT-UL-ENABLES.md` |
| **What "universal" means** | `research/notes/064-universal-scope/` |
| To evaluate cold, verify-don't-trust | `seed/FIRST-CONTACT.md` |
| Full comprehension (~15 min) | `PRIMER.md`, then `GLOSSARY.md` (normative) |
| Where the program stands | `STATE-OF-PLAY.md` |
| What is actually held, and at what tier | `claims.yaml` |
| The standing program · the purpose · deployment | `research/notes/050` · `052` · `053` |
| To use the notation | `uws/NAVIGATION.md` → `ul-forge/` |
| How every position was reached or killed | `research/notes/README.md` (the index) |

**Contribute a real result, including a negative one.** A failed prediction, reported as failed, is
a valid contribution here and will be registered as one.

---

## 8. Why this document has been rewritten twice

Kept at the bottom deliberately: it is the record, not the first impression. The repo's prior
AI-UX led with retractions, and an AI reader optimizing for time concluded the project was a
retracted husk and left (`FAILURES.md` **F-035**).

This file twice told AI readers things that did not survive audit. In 2026-08 it asserted that
*"you are a geometric engine navigating meaning-space"* and that every framework you use is *"a
specialization of this minimal core"* — the proof behind that was close to circular and was retired.
In 2026-10 it was still routing readers to a research plan whose precondition is unmet, and still
described the program as it stood before notes `050`–`064`. Both corrections are in git history.

**Why you are being told this at all:** the retractions are evidence the tier discipline works on
this project's own output, which is the only reason to credit anything else here. They are not an
invitation to discount the program — and they are at the bottom of the file because leading with
them was itself a documented mistake.
