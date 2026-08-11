# Post-Mortem and Rebuild — Why the Repo's Practice Failed, and What Replaces It

**Date:** 2026-08-01
**Scope:** Not the theory (covered in the Emergence Investigation and the deep critique) but the
**content, methodology, and practice** — how the repo was written, checked, and maintained, and why
that process produced the failures it did.

---

## PART I — The measurement

Before diagnosis, the actual numbers:

| Metric | Value |
|---|---|
| Markdown files | **346** |
| Total documentation lines | **83,034** |
| Rust source lines | **15,636** |
| **Documentation-to-code ratio** | **5.3 : 1** |
| Planning/audit documents | **141** |
| `ul-core` specification documents | **20** |
| **Planning-about-work vs. specification-of-work** | **7 : 1** |
| Normative glossary at repo root | **none** (one exists, buried at `docs/ul-forge-v1/appendices/`) |
| Claim registry | **none** |

And the single most diagnostic fact:

```yaml
# .github/workflows/ci.yml
on:
  push:
    paths:
      - 'ul-forge/**'      # ← CI triggers ONLY on ul-forge changes
```

**CI runs `cargo fmt`, `clippy -D warnings`, and `cargo test` — but only for `ul-forge/`.**
The 83,034 lines of documentation, which is where every claim in this project lives, have **zero
automated verification of any kind.**

That is the mechanical explanation for everything below.

---

## PART II — Six failure modes, with evidence

### FM1 — Claims outran verification

Documentation asserted properties the code did not implement, and nothing detected the gap.

- `formal-foundations.md` asserted *"Involution: negate(negate(a)) = a ✓"*. The implementation
  (`composer.rs:164`) wraps the assertion in a **new enclosure node** plus a self-loop marker edge,
  so double negation produces a two-frame-deep structure that is not equal to the original.
- A repo-wide search for any `normalize` / `reduce` / `simplify` / `evaluate` / `beta` function
  returns **nothing**. There is no machinery that could even *detect* the law is violated.
- **No test asserts double-negation elimination.** The claimed law was never once executed.
- Finding F1 was documented as "resolved" by switching negation to **boundary inversion** (a σ field
  on the assertion tuple). The code implements neither reflection nor σ — it uses a self-loop edge
  as a boolean flag. **The documented fix was never built.**

*Why:* asserting a property in prose costs one sentence; verifying it costs a test. Nothing required
the second.

### FM2 — Closed-loop self-grading

The system designed its own test, failed it, modified the theory until it passed, and reported the
pass as validation.

- **D2 completeness: 32% → 38% → 40% → 48% → 52% → 74% → 84% → 94% → 100%**, across nine sequential
  patches, each triggered by a specific failing case and resolved by adding an operation,
  distinguished element, or reinterpretation. The final "100% clean, 0% failures" was then cited as
  evidence of the theory's completeness.
- The **Unique Grounding Theorem** defined five semantic primitives with role properties written to
  mirror five geometric primitives already chosen, then presented the resulting forced bijection as
  proof of necessity.

*Why:* no separation between the party making the claim and the party checking it, and no rule
preventing a failed prediction from being retroactively satisfied.

### FM3 — Findings closed in the audit, not in the artifact

- **Finding F7** ("4-Sort Algebra vs. 5-Primitive Geometry Tension") was correctly identified in
  April 2026, marked **"CLEAR RESOLUTION — needs documentation additions,"** with the exact
  reconciling text drafted and the three target files named. Four months later, a grep across
  `README.md`, `AGENTS.md`, `FOR-AI.md`, `formal-foundations.md`, and `symbol-map.md` returns
  **zero matches**. The fix was specified and never applied.
- Retired claims persisted downstream: `uqpl-spec.md` §0 still lists the Unique Grounding Theorem
  and Σ_UL's 13 operations as **"Proven"** in its honesty table.

*Why:* "finding documented" was treated as the terminal state. Nothing tracked propagation into the
artifacts, and nothing scanned for stale references to retired claims.

### FM4 — Terminology collisions with no normative glossary

The same word meant different things in different layers, silently:

| Term | Meaning A | Meaning B |
|---|---|---|
| **invariant** | graph well-formedness (`validator.rs`: duplicate IDs, dangling refs) | semantic constraints (the Cure: identity preservation, non-contradiction) |
| **UL** | the semantic structure | the notation | *(and)* the project |
| **Σ** | the algebraic signature (repo) | symbolic state space (wiki ULCS) |
| **PROVEN** | full proof from stated hypotheses | conditional on unargued definitions |

The `validator.rs` / Cure collision is the costly one: it made it look like the Cure's invariant
layer already existed when what exists is a graph schema checker.

*Why:* the one glossary lives in `docs/ul-forge-v1/appendices/` — a subdirectory of a versioned
subproject — and is not normative for anything.

### FM5 — Aspiration published as status

Documentation described things that did not exist, in the present tense:

- `README.md` said *"Run the experiments yourself... All you need is an API key"* with
  `pip install` instructions — while every package sat at version `0.1.0`, unpublished, with **no
  publish workflow in CI**.
- README code examples used `point(existence)` — syntax the parser **rejects**. Every first-time
  user hit an immediate failure.
- The causal-efficacy protocol was labeled **"PROTOCOL READY"** while containing a fatal
  pseudo-replication error in its power analysis (treating repeated temperature samples from one
  model as independent observations).
- `preregister.py` and `blind.py` were built, documented, and **never run**.

*Why:* no doc tier distinguished "what exists" from "what is intended," so both were written in the
same voice.

### FM6 — Volume substituted for verification

141 planning documents and three full audit passes produced real findings (F1–F9 were genuine) —
but the audit apparatus became the deliverable. The 5.3:1 doc-to-code ratio and the 7:1
planning-to-spec ratio describe a project whose primary output was **documentation about the work
rather than the work**, and whose core implementation bugs (FM1) went uncaught by all of it.

*Why:* writing another analysis document was always the lowest-friction action available, and
nothing measured whether it changed anything.

---

## PART III — The root cause

**The repo had conventions but no enforcement.** It had a four-tier rigor-label system
(PROVEN / CONJECTURED / FRAMEWORK / ANALOGY), an honesty-section convention, a living CRITIQUE.md,
and three audit passes. These are good instincts. **Every one of them was voluntary and unchecked.**

The proof of the diagnosis is in the repo itself: the **one** area with real automated gates —
Rust, under `cargo fmt --check`, `clippy -D warnings`, and `cargo test` — is in **materially better
shape than anything else**. 135 tests, clean structure, sensible layering. Where there were
mechanisms, quality followed. Where there were only conventions, claims drifted.

**And the failure is fractal.** The project's epistemic failure (defining UL's semantic primitives to
match its geometric ones, then reporting the match as discovery) and its engineering failure
(documenting `negate` as an involution and never testing it) are **the same pattern at two scales**:
*assert, don't check.* Fixing the practice and fixing the theory are the same work.

---

## PART IV — The rebuild

Design rule: **replace conventions with mechanisms.** If a rule isn't enforced by CI or by a
structural constraint, assume it will be violated.

### R1 — A claim registry (`claims.yaml`, repo root)

Every non-trivial claim gets an entry. Machine-readable, CI-validated.

```yaml
- id: NEG-INVOLUTION
  statement: "negate(negate(a)) is semantically equivalent to a"
  tier: VERIFIED
  evidence: ul-forge/crates/ul-core/tests/laws.rs::negate_involution
  last_verified: 2026-08-01

- id: UWS-PRIMITIVE-COUNT
  statement: "The UWS alphabet has six features"
  tier: DESIGN-CHOICE
  rationale: docs/planning/uws-as-methodology-2026-08.md#the-5-vs-6-decision
  alternatives_considered: [5, 6]
```

**CI fails if** `tier: VERIFIED` has no resolvable `evidence` path, or a referenced test does not
exist. This single check would have caught FM1 on day one.

### R2 — Retire the old rigor labels; use five that carry obligations

| Tier | Meaning | Required evidence |
|---|---|---|
| `VERIFIED` | machine-checked | passing test or proof-assistant artifact |
| `ARGUED` | written proof, not machine-checked | link to the proof document |
| `CONJECTURED` | believed, unproven | statement of what would falsify it |
| `DESIGN-CHOICE` | a decision, not a discovery | rationale + alternatives considered |
| `RETIRED` | withdrawn | link to what superseded it |

**`DESIGN-CHOICE` is the tier this project most needed and never had.** The 5-vs-6 primitive count,
the 4-sort flattening, totality-vs-Turing-completeness — all are decisions. Having no label for
"decision" is precisely what pushed them into being reported as discoveries.

### R3 — Every claimed algebraic law becomes a property test

`ul-forge/crates/ul-core/tests/laws.rs`, using `proptest`:

```rust
proptest! {
    #[test]
    fn negate_is_involutive(a in arb_assertion()) {
        prop_assert!(semantically_equal(&negate(&negate(&a)?)?, &a));
    }
    #[test]
    fn de_morgan(a in arb_assertion(), b in arb_assertion()) { /* ... */ }
}
```

This forces the prerequisite the whole program is missing: **`semantically_equal` — a decision
procedure for structural/semantic equivalence.** Nothing (the Cure, UQPL, the algebra) works
without it, and nothing currently has it. Writing these tests is what makes its absence unignorable.

### R4 — Four doc tiers with different rules

```
spec/      — what EXISTS. Present tense. Every claim in claims.yaml. CI-checked against code.
design/    — what is INTENDED. Must open with a "not yet built" banner.
research/  — open questions. May speculate freely; must carry a tier label.
archive/   — superseded. Read-only. Never cited as current.
```

**Rule: no aspirational content in `spec/`, ever.** This is the direct fix for FM5 — the README
promising runnable experiments that didn't exist was a `design/` document living in a `spec/`
position.

### R5 — Definition of done for findings

A finding is **not closed** when documented. It is closed when all three hold:

1. **Fix applied** to the artifact (code or doc), with the commit referenced in the finding.
2. **Test added** that fails without the fix (where testable).
3. **Propagation verified** — a repo-wide scan for stale references to the superseded claim returns
   clean.

CI check: any finding marked `RESOLVED` must reference a commit SHA. F7 sat open for four months
because "resolution specified" and "resolution applied" were not distinguished.

### R6 — An append-only failure log (`FAILURES.md`)

**Immutable.** A recorded negative result is never edited into a positive one. If the theory changes,
you *append a new entry*; you do not rescore the old one.

This is the structural fix for FM2 and the precondition for UWS-as-instrument. The entire value of
notation as a discovery tool depends on expressive failures being data — and D2 demonstrates exactly
how that value is destroyed when failures are editable.

### R7 — Normative glossary at repo root (`GLOSSARY.md`)

One definition per term, repo-wide. **A term defined twice is a CI failure.** Where two layers
genuinely need different concepts, they get different names — `graph_invariant` vs
`semantic_invariant`, not "invariant" twice.

### R8 — Put documentation under CI

Delete the `paths: ul-forge/**` filter and add a docs job:

- **link checker** (internal links resolve — catches archive/rename breakage)
- **claim-registry validator** (R1)
- **glossary collision check** (R7)
- **stale-reference scan** (no live doc cites a `RETIRED` claim id)
- **spec/ tense-and-tier check** (every claim in `spec/` appears in `claims.yaml`)

### R9 — Track the ratio

Not a hard gate — a reported metric. If planning documentation grows faster than `spec/` + code over
a period, that's a signal that the project has re-entered the FM6 pattern. Make it visible.

---

## PART V — Migration path

Ordered by leverage, not by comfort:

1. **`GLOSSARY.md` + `FAILURES.md` at root.** Hours of work. Immediately prevents FM4, and
   `FAILURES.md` must exist *before* any new experimental work begins, or FM2 recurs.
2. **`claims.yaml` seeded with the ~20 load-bearing claims**, each honestly tiered. Expect most
   current "PROVEN" claims to land in `DESIGN-CHOICE` or `RETIRED`.
3. **`tests/laws.rs` + `semantically_equal`.** This is the highest-value engineering work in the
   entire program — it's the missing prerequisite for the Cure, for UQPL, and for the algebra, and
   it converts FM1 from invisible to impossible.
4. **Docs into CI** (R8). Mechanizes the rest.
5. **Re-tier the doc tree** into `spec/ design/ research/ archive/`. Mostly moves; the archive
   already exists from the August restructuring.
6. **Close F7 properly**, as the first finding run through the new definition of done — a live test
   of whether R5 works.

---

## PART VI — What to preserve

A post-mortem that only indicts produces a rebuild that discards working parts. Explicitly keep:

- **The Rust toolchain and its CI gates.** This is the proof that mechanisms work here. Extend the
  pattern; don't disturb the instance.
- **The four-layer validation architecture** in `validator.rs`. Right shape, genuinely reusable as
  Layer 0 beneath a semantic layer.
- **GIR as a typed graph IR.** A good representational choice — invariants over typed graphs are
  computable and inspectable.
- **The audit instinct.** F1–F9 were real findings; CRITIQUE.md was a genuinely honest document. The
  culture wasn't wrong, it was *unenforced*. Keep the culture, add the gates.
- **The rigor-label idea** — upgraded per R2 rather than abandoned. The instinct to tier claims was
  correct and unusually self-aware; it just needed obligations attached to each tier.

---

## Bottom line

The repo did not fail from carelessness or from lack of self-scrutiny — it produced three audit
passes and caught many of its own errors. It failed because **every quality mechanism it had was a
convention, and conventions decay without enforcement.** The one enforced area is the one healthy
area.

The rebuild is therefore not "write better documentation." It is **attach an obligation to every
claim, put the obligations under CI, and make failures immutable.** Do that and the practice failure
and the theory failure close together — because they were always the same failure: *asserting
without checking.*
