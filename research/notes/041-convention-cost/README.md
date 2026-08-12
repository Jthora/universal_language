# 041 — Testing the convention-cost claim, as far as it can be tested here

**Type:** cycle
**Opened:** 2026-08-12
**Status:** closed
**Thread:** tests `ROTATION-MINIMIZES-CONVENTION` (`035`)

---

## 1. What can and cannot be tested here

**The full ablation — symbolic versus rotation encoding at matched population size, with trained
agents — cannot be run in this environment.** It needs training runs. It stays the outstanding
empirical item.

**But the claim underneath it is information-theoretic, not behavioural**, and that half is
computable:

> Rotation breaks **ℤ/2** of arbitrary convention; labels break **Sₙ**. So a receiver who does not
> share the sender's convention faces **2** consistent readings under rotation and **n!** under
> labels.

**What a computational test settles:** whether the residual ambiguity really is 2 versus n!.
**What it does not settle:** whether agents *learn* rotation-based codes more compatibly. Behaviour
could differ from information. **The ablation remains outstanding and this does not substitute for
it.**

## 2. Before running  ← written first (S2)

**Expected:** rotation ambiguity ≤ 2 for every map, and exactly 1 for achiral maps. Label ambiguity
= k! for k labels, growing factorially.

**The failure mode I am specifically watching for**, because it would weaken `035` materially:
**does reading a rotation system require vertex correspondence?** If the receiver must know *which
vertex is which*, rotation smuggles in a naming convention and the ℤ/2 claim is false.

**My answer, to be tested rather than assumed:** it should not, because a combinatorial map is
determined **up to isomorphism** by its structure — the receiver reconstructs without needing the
sender's names. Labels are different in kind: `ARG0` versus `ARG1` carries content that is *not*
recoverable from structure, so permuting them changes meaning.

**Would falsify:** rotation ambiguity exceeding 2 for some map, or a demonstration that
reconstruction requires shared vertex naming.

## 3. Results

**Implemented in `map.rs`, 16 tests passing.** But the four tests are not equally informative, and
saying which is which matters more than the pass count.

| Test | What it actually establishes |
|---|---|
| `mirror_is_an_involution_and_preserves_degree` | **Real.** `mirror ∘ mirror = id`, so the orientation group **is ℤ/2** — the claim's left-hand side |
| `label_cost_grows_factorially...` | **Real.** `k!` arithmetic: 6, 24, 720, **40 320** at k=8, against a flat 2 |
| `symmetric_maps_are_achiral...` | **Real but partial** — cycles are their own mirror, so orientation carries no information for them |
| `rotation_ambiguity_never_exceeds_two` | **Near-tautological.** The function returns 1 or 2 by construction, so the test checks my own return statement |

**That last row is worth stating plainly rather than counting as a pass.** The bound of 2 is
*structural* — a rotation system has exactly two orientations, keep σ or reverse it — and the
involution test is what establishes it. The ambiguity function is a **consequence**, not independent
evidence.

### The failure mode I was watching for did not appear

§2 flagged the real risk: **does reading a rotation system require vertex correspondence?** If so,
rotation smuggles in a naming convention and ℤ/2 is false.

**It does not.** A combinatorial map is determined **up to isomorphism** by its structure — the
receiver reconstructs without the sender's names. Labels are different in kind: `ARG0` versus `ARG1`
carries content **not recoverable from structure**, so permuting them changes meaning while
permuting vertex names does not.

**That asymmetry is the substance of the claim**, and it survived the check.

### An honest limit in the implementation

`signature()` — degree sequence plus face-size multiset — is **not a complete isomorphism
invariant.** Two non-isomorphic maps can share one, so achiral detection may report `1` for a map
that is in fact chiral. **That understates ambiguity**, which is the safe direction for the bound but
makes the achiral count approximate. A complete test needs canonical-form computation. Recorded in
the module docs, not hidden.

## 4. What changed

- `claims.yaml`: `ROTATION-MINIMIZES-CONVENTION` gains the computational check **with its scope** —
  the information-theoretic half is now machine-checked; the behavioural half is not.
- `IMPL-CONVENTION-COST` added (VERIFIED).
- **Unchanged and still outstanding:** the ablation with trained agents. **This does not substitute
  for it.** Information and behaviour can differ, and the claim that *agents learn rotation-based
  codes more compatibly* remains untested.
