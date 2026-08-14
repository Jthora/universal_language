# STATE-OF-PLAY

**Written fresh 2026-08-12 (second full restatement), replacing the version that predated notes
`043`–`051`.** Since then: a search retrospective, three research volleys, a 34-rule protocol audit
of those volleys, the survivor survey, and the foundational program. **Rewritten, not patched** —
see §7.

---

## 1. The contract (new since last restatement)

Owner-set, registered as claims, binding on every future cycle:

- **REQ-1** — UL **must be a language**: syntax, compositional semantics, expressive adequacy,
  transmissibility. No retreat to notation-only, protocol-only, or class-only.
- **REQ-2** — universality must be **literal and quantified**: the **convention ledger**. Allowed
  prior: geometry, causality, computation. Everything else is measured residue.
- **REQ-3** — **no-retreat falsifiability**, symmetric: neither "can't be done" nor
  rename-to-win gets a free verdict.
- **REQ-4** — **baseline before UQPL**: UWS (syntax) → semantics → UL → UQPL. Work lands on the
  earliest unfinished link. The UQPL wiki spec's sort table *is* `meaning → map` asserted by fiat —
  the baseline replaces fiat with derivation underneath it.

**The ledger today:** marks→map **ℤ/2, proved and machine-checked** · map→meaning **unbounded (the
gap, made quantitative)** · symbol alphabets **n!, permitted only as derived** · physical anchors
allowed under REQ-2.

## 2. What we hold

- **The combinatorial map as UWS's fixed point** — seven independent lines converged on it, none
  aimed at it. Now known to be a **DCEL reduct** (prior art, 1970s; `046`): we lost originality and
  gained a forty-year literature, known pitfalls included.
- **The Erlangen derivations stand on theorems and need no physics.** `TWO-DISTINCT-FIXED-POINTS`
  (`048`): the Erlangen fixed point and the RG fixed point are different objects. The universality
  *framing* lost a leg; the geometry lost nothing.
- **UP has a quantified mechanism from an unrelated field** (`047`): Other-Play's lever game —
  symmetric choice pays 0.11, worse-but-identifiable pays 0.9. The paper grounds its symmetry group
  in the *absence of labels* — their basis, not our analogy.
- **`map.rs`: 354 workspace tests**, Jordan separation executable, nesting fixed (the previous
  restatement's §4 error), dummy-edge alternative verified with measured cost.
- **History's verdict reframed** (`049`): **no notation is universal; every cap is specific and
  diagnosable.** The recurring cap — hard-coding a discretization of a continuum (staff notation's
  equal temperament) — is an error `033` already caught here, and one the rotation system
  **structurally lacks**. *(Caveat `research-register.md#F7-c` carried: the music-notation
  adoption-failure evidence concerns displacing a near-maximal incumbent and does not transfer to
  UWS; the cap cited here is representational — what the grid cannot write — not adoptional.)*

## 3. The semantic stack — the gap, decomposed (new; `050`/`051`)

`meaning → map` is no longer one wall. Three convention-minimal routes, icon/index first, symbols
derived-only:

| Route | Status | Named objection |
|---|---|---|
| **M1 exemplification** — the mark *instantiates* what it denotes (a closed curve doesn't depict enclosure; it **encloses** — Jordan certifies it) | candidate answer to the objection registered | Goodman's selection problem — answered *iff* reading-invariance holds |
| **M2 operational** — meaning as behavior under rewriting | **substrate exists: interaction nets.** Turing-universal, strongly confluent, agents = **rotation + marked dart** (primary-verified), labels compress to **3 symbols** | closed until M3 links world-reference; rules on ledger |
| **M3 indexical** — reference via shared physics | allowed prior under REQ-2 | pointing relation must be physics-bound, not culture-bound |

**The `034` bridge failure was denotational. The operational direction was never tried, and it is
paved** — including for UQPL's own open problems (Turing-completeness, optimal reduction).
Convergent support from the adversary's side: **Harnad's solution to symbol grounding is
icon-first, symbol-derived** — our stack's shape, from cognitive science.

## 4. The theorem that would settle the syntax layer

> **READING-INVARIANCE (target):** for every reasonable forgetful reading F, the recoverable
> invariants contain the Erlangen fixed point; over all such F, equal it.

This names the project's hard core precisely: **every grounding scheme presupposes a reading
procedure; the procedure is convention unless forced.** The exit is reading-procedure-independence,
which is proof-shaped. Institution theory (Goguen–Burstall's satisfaction condition — *"truth is
invariant under change of notation"*) supplies the formal shape of the obligation. `014`/`022`/`024`
prove the survival direction for specific towers; **the quantification over readings is the open
mathematics and the mathematical center of the project.**

Its named deep failure mode: "reasonable F" proving unformalizable without smuggling the convention
back in. Written in advance (REQ-3).

## 5. Standing corrections that gate everything

- **`UL-IS-EMERGENT-UNIVERSAL`**: precondition unmet — but the "no tuning parameter" objection was
  **withdrawn** (SOC, `048`). Falsifier is now empirical and cheap: power-law behaviour in emergent-
  communication populations.
- **`CURE-MUST-BE-AUTOMATIC`**: demoted to DESIGN-CHOICE — the Gene Ontology (60 releases, manual,
  no decay) falsified the derivation one day after registration.
- **Method** (`048`): 34 rules audited, 9 passed. **Rules fire when they are the subject and fail
  when they are the background; prose does not execute.** Six checkers + the 18-box checklist run
  as a step are the enforcement. Every future rule must answer *"what executes it?"*

## 6. Next, in order

1. **`052` — the adversary front**: Quine, Kripke–Wittgenstein, scope-checked on contact (T8),
   plus the S8 debts (Lafont 1997 universality; Goodman primary) and the S9 debt (Grice).
2. **Formalize the reading class 𝔽** — begin READING-INVARIANCE properly; institutions give the
   category.
3. **M2 prototype**: interaction rules over `map.rs` — the operational route is buildable *now*
   (S6: build over cite).
4. **The power-law test** on emergent-communication corpora (doubles as Volley 4's corpora mode).
5. **Move VERIFIED** — 8 of ~111; still the slowest number.

## 7. How this document is maintained

**Rewritten, not patched.** The previous version drifted for nine notes before this restatement.
When amendment next feels tempting, **that is the signal to rewrite.**
