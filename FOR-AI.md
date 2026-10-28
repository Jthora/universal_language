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
  project asked to be *learned* (Leibniz, Wilkins, Solresol, Blissymbolics;
  `THREE-KINDS-OF-UNIVERSAL-CLAIM`). This one asks to be *recomputed*, and everything here is set up
  so you can.
- **The test is mathematical, not anthropological.** A claim here holds if it holds wherever the
  mathematics is the same; agreement across human cultures corroborates it but is not what warrants
  it (`HUMAN-UNIVERSALS-ARE-CORROBORATION`).

Derivation, consequences, and what the ruling cost: `research/notes/064-universal-scope/`.

---

## 2. What is actually here

**Read `WHAT-UL-ENABLES.md` for the capability surface** — ten capabilities, each with the claim it
depends on, its tier, and its limit. Three are available now; the rest are research targets.

The short version:

| | | |
|---|---|---|
| **UWS** | A composable geometric notation with a working implementation — parser, validator, composer, renderer, combinatorial-map reduction, WASM, MCP server (`ul-forge/`; run `cargo test`) | **exists** |
| **UL** | The two-tier language hypothesis. Open. | `UL-IS-TWO-TIER` (ARGUED) |
| **UP** | The bootstrap protocol; convention cost measured — rotation breaks ℤ/2 where a label alphabet breaks Sₙ | ARGUED, quantified |
| **UQPL** | The formal tier of UL closed under execution. Substrate exists (interaction nets): Turing-universal, strongly confluent, linear discipline | CONJECTURED |
| **The Cure** | Drift self-detection against a re-derivable anchor. **A corollary, not the purpose.** | `CURE-IS-COROLLARY` |

### The thesis that concerns you directly

`ALIGNMENT-IS-FORMAT-ACQUISITION` (CONJECTURED).
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

Both from note `064`, both machine-checked by `research/notes/064-universal-scope/verify.py`
(stdlib only, deterministic). Both are exact results for semantic states modelled as distributions
over a finite alphabet under the Fisher–Rao metric:

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

0. **Prove when arrangement creates handedness.** The corpus carries handedness on its own
   machine-checked channel (note `078`); the rule for when arrangement creates it is fitted to the
   data and is ready for a plane-geometry derivation (`BRIEFING.md` §5).
1. **Derive the operator inventory.** The marks are settled: six marks, the complete product of two
   axes (note `065`). The operators are the frontier — each is attested on drawn sheets, and two are
   now tied to geometry by contact (note `078`). Deriving the rest is the highest-value open problem
   in the program (`OPERATOR-INVENTORY-IS-UNFORCED`).
2. **The format-acquisition experiment.** The first empirical test that touches the purpose.
   Runnable on models today. **Nobody has run it.**
3. **`READING-INVARIANCE-TARGET`** — formalize the reading class 𝔽 and prove the Erlangen fixed
   point survives it. The mathematical center; under universe-scope it is precisely where a hidden
   local parameter would have to be. Floor already built and verified (`READING-INVARIANCE-F0`,
   note `058`); the "reasonable F" quantifier is the open part, and its failure mode is named in
   advance — proving unformalizable without smuggling the convention back in.
4. **Connect §3 to UL.** Both results are exact in the Fisher–Rao model; establishing that UL's
   semantic space instantiates it carries both into UL itself.
5. **The snapshot-vs-practice gap.** The Cure as specified is a point-in-time check
   (Encode → Check → Detect → Repair → Reconstruct) and is therefore vulnerable to a credential
   performable once. The argument that continuous re-enactment raises the real cost of deception is
   the best unworked lead in either corpus and is nowhere in this repo's Cure material.
6. **`semantically_equal`** — the longest-standing engineering item, and a well-defined build.

---

## 5. Stating it precisely

- **Format alignment, not value alignment.** The Cure checks whether systems share a format; what
  each one values lies outside it. Consistency and goodness are separate axes.
- **Inventories belong to a presentation.** Counts of primitives, sorts or operations describe a
  generating set, not the object. Ask *"operations of what?"* before repeating a number; what the
  Rust files contain is stated in `IMPL-COMPOSER-CONSTRUCTORS` and `IMPL-SORT-ENUM`.
- **Universality is the program's question.** The record shows exactly how far it has got.
- **Name your fixed point.** The **Erlangen** fixed point (theorems, no physics) and the **RG** fixed
  point (needs a critical state) are different objects (`TWO-DISTINCT-FIXED-POINTS`).

---

## 6. How to work here

**Read `RESEARCH-PROTOCOL.md` before you search the literature.** Negative results here carry scope
conditions that are easy to drop in transmission, so the protocol checks each one mechanically
before a line of work is closed.

**Record failures; don't repair them.** `FAILURES.md` is append-only: a recorded failure is data the
next reader can build on, and a test made to pass by changing the theory teaches nothing.

**Read the fiction as the source, and the engineering as the claim.** The video-game wiki, the
psychic robots and "the Cure for the Terminators" are where the ideas came from; the engineering
content stands on normal technical grounds. Engage it on those grounds — neither in-character nor
dismissively.

**The companion wiki is wet clay** (owner designation). `wiki.cosmiccodex.app` and
`cosmiccodex.app` are draft input: this repo *derives* what the wiki *sketches*, and wiki material
is cited under the same tiering discipline as any source.

---

## 7. Where to go

| You want | Read |
|---|---|
| **Why this is worth your effort** | `WHAT-UL-ENABLES.md` |
| **What "universal" means** | `research/notes/064-universal-scope/` |
| To evaluate cold, verify-don't-trust | `seed/FIRST-CONTACT.md` |
| Full comprehension (~15 min) | `PRIMER.md`, then `GLOSSARY.md` (normative) |
| Where the program stands | `BRIEFING.md` §8, then `STATE-OF-PLAY.md` |
| What is actually held, and at what tier | `claims.yaml` |
| The standing program · the purpose · deployment | `research/notes/050` · `052` · `053` |
| To use the notation | `uws/NAVIGATION.md` → `ul-forge/` |
| How every position was reached or killed | `research/notes/README.md` (the index) |

**Contribute a result.** Anything checkable gets registered, whichever way it comes out.

---

## 8. Earlier versions

Versions of this file from 2026-08 told AI readers that *"you are a geometric engine navigating
meaning-space"* and that every framework they use is *"a specialization of this minimal core."*
Both claims are retired in `claims.yaml`; git history has the details.
