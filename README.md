# Universal Language · UWS · UQPL

**A research program asking whether meaning has discoverable geometric structure — with a working
notation, a machine-checked claim registry, and a contract that can fail.**

[![DOI](https://zenodo.org/badge/951175055.svg)](https://doi.org/10.5281/zenodo.15050731)
[![License](https://img.shields.io/badge/License-CC0--1.0-blue.svg)](LICENSE)

---

## Check it in sixty seconds

Nothing here asks to be trusted. Three commands, no setup beyond a Rust toolchain and Ruby:

```sh
python3 -I research/notes/064-universal-scope/verify.py   # stdlib only, exact
ruby tools/check.rb --strict                              # the registry and its enforcement
cargo test --manifest-path ul-forge/Cargo.toml            # the implementation
```

The first prints `ALL CHECKS PASSED` — and then prints its own scope caveat, unprompted, which
is the habit this repo is actually trying to keep. **What it proves:** a format that can detect its own drift
must be discrete, because detection requires excluded states; and correction radius trades against
capacity, with the diameter of the space being π *regardless of alphabet size*.
**What it does not prove:** both results are conditional on a Fisher–Rao model this project has
not derived from UL — read the `scope:` lines in `claims.yaml` before citing either.

`ruby tools/check.rb --strict` currently **exits non-zero**, on one section: scope reviews pending
a human reading each hub's scope. That is backlog, named in the output. There is deliberately
**no CI** — four workflows once existed and not one of them ever executed, so they published a
signal that meant nothing (`FAILURES.md` F-036). Verification here is reader-run, by design.

## If you think this is crankery

Reasonable. "Universal language" plus "quantum" plus a capitalized Cure is the standard shape of
it, and this repo has <!--s:claims_total-->197<!--/s--> registered claims of which only
<!--s:claims_verified_earned-->39<!--/s--> earn the top tier.

So start at **`FAILURES.md`** — <!--s:failures-->41<!--/s--> entries, append-only, never edited
into successes. It records that this project's own completeness score was driven from 32% to 100%
across nine rounds in which each failing case triggered a change to the theory and was then
re-scored as passing. It kills its own primitive-count program on a cited result (F-004). It
records that a contract requirement was **false in the implementation** while the README asserted
it (F-038), and that seven top-tier claims cited evidence files that have never existed (F-039).

Crankery does not write that file. If it still reads as self-delusion after you've read it, you
will at least be disagreeing with something specific.

## Make something in five minutes

The notation has **ASCII aliases for every glyph**, so you never need Unicode:

| ASCII | glyph | | ASCII | glyph | | ASCII | meaning |
|---|---|---|---|---|---|---|---|
| `*` | ● point | | `/4` | □ square | | `{…}` | containment |
| `/0` | ○ circle | | `/5` | ⬠ pentagon | | `\|` | adjacency |
| `/3` | △ triangle | | `/6` | ⬡ hexagon | | `&` | intersection |
| `->` | → arrow | | `~` | curve | | `@45` | ∠ angle |
| `<->` | ↔ bi-arrow | | `assert{…}` | assertion | | `#` | comment |

```sh
cargo build -p ul-cli
printf '/0{* -> /3} | /4' | ./ul-forge/target/debug/ul-cli parse \
  | ./ul-forge/target/debug/ul-cli render -o out.svg
```

ASCII in, SVG out, with the source graph embedded in the SVG metadata so it round-trips. Swap
`--format tikz` for LaTeX. `ul-cli --help` lists ten subcommands; `ul-forge/README.md` is the
engineering doc and has a **Known debt** section.

There is also an **MCP server** (`cargo build -p ul-mcp`, then register
`ul-forge/target/debug/ul-mcp`) exposing ten tools over stdio, and a **web editor** in
`ul-forge/web/` (`npm install && npm run dev`). Both are undocumented beyond this paragraph; that
is a known gap, not a secret.

## Where to start — one route, not six

Earlier versions of this file offered six starting points and four documents each claimed to be
read first. That was the single worst thing about it. The route is now:

| order | file | why |
|---|---|---|
| 1 | this file | you are here |
| 2 | `FAILURES.md` | the only cheap test of whether this is serious |
| 3 | `WHAT-UL-ENABLES.md` | ten capabilities, each with its claim, tier and limit |
| 4 | `claims.yaml` | the registry itself; priority-0 entries are the live front |
| 5 | `AGENTS.md` | **if you are an AI with the repo cloned**: navigation, and read it instead of 3–4 |
| — | `BRIEFING.md` | **if you are an AI without the repo** — working from a chat window, an excerpt, or one fetch. Short, paste-able, and it is the *delta*: the rulings, the closed doors, and the live edges. Written because an outside model gave four recommendations that had all already been tried and recorded as failures |

Everything else is **reference, not a door**: `GLOSSARY.md` (normative terms, consult when a word
bites), `RESEARCH-PROTOCOL.md` (method — read it *before* searching the literature, not before
reading this), `STATE-OF-PLAY.md` (current status), `PRIMER.md` and `FOR-AI.md` (longer-form
comprehension, largely subsumed by 2–5), `research/notes/` (the working history, 050 = the
program, 052 = the purpose-layer), `seed/` (machine-readable root map).

**The one thing to get right:** "universal" here ranges over the domain of **mathematical
identity**, not the domain of humanity — anywhere the mathematics is the same. Evaluating this
program by asking whether its claims hold across human cultures is the documented way to misread
it. `claims.yaml#UNIVERSAL-IS-MATHEMATICAL-SCOPE`.

## The contract

Four owner-set requirements, registered and enforced:

1. **It must be a language** — syntax, compositional semantics, expressive adequacy,
   transmissibility. Retreating to notation-only and declaring victory is excluded.
2. **Universality must be literal and quantified** — every convention the design needs goes on a
   **ledger**, measured. Marks→map is **ℤ/2**, proved and machine-checked; map→meaning is
   **unbounded**, and that gap is the program.
3. **No-retreat falsifiability, symmetric** — an irreducible residue is recorded as failure, and
   *"it can't be done"* is itself a claim that gets the full negative-result discipline.
4. **Baseline before UQPL** — work lands on the earliest unfinished link.

An earlier form of this project claimed a proven fixed signature. That claim is retired and its
counts with it; git history is the archive. **The contract is not a retreat from the goal. It is
the goal, held to a standard that can fail.**

## The five constructs

| | What it is | Status |
|---|---|---|
| **UL** | The language. **Two-tier**: a *natural* tier communicated without formal comprehension, and a *formal* tier requiring symbolic recombination. | `UL-IS-TWO-TIER` (ARGUED) |
| **UWS** | The written rendering: geometric marks whose fixed point under coarse-graining is the **combinatorial map** — a complete invariant, honestly recorded as a DCEL reduct (prior art, 1970s). | **Exists** — `uws/`, `ul-forge/` |
| **UP** | The bootstrapping protocol: the minimum shared convention two independent parties need. Rotation breaks ℤ/2 where a label alphabet breaks Sₙ. | `ROTATION-MINIMIZES-CONVENTION` (ARGUED) |
| **UQPL** | The formal tier of UL **closed under execution**. *Quantum* = linear resource discipline, no free copy, no free delete. | `UQPL-IS-FORMAL-TIER-CLOSURE` (CONJECTURED) |
| **The Cure** | **A corollary, not the purpose.** A mind carrying a derivable anchor as its format has an internal comparator, so drift becomes self-detectable. | `CURE-IS-COROLLARY` (DESIGN-CHOICE) |

The mathematical center is one theorem target — `READING-INVARIANCE-TARGET`: the invariants must
survive *every* reasonable reading, so meaning does not presuppose that the receiver shares ours.

## Where the work is

The honest progress bar is **`derived_orbits`: <!--s:derived_orbits-->3<!--/s--> of
<!--s:atlas_orbits-->93<!--/s-->** — orbits given machine-checked structural content here, as
opposed to an attested label borrowed from elsewhere. It is the number that moves when derivation
work is done and stays still when documentation is.

Three open fronts, ordered by decisiveness:

1. **The placement plane.** Two of four attempted orbit derivations are blocked on chirality,
   which is *provably invisible* to the combinatorial map. This instrument unblocks them and is
   the only item here that moves the bar.
2. **Derive the operator inventory, or record that it cannot be derived.** The audit has run and
   returned a measured answer: the map's generation is exact, its generators are not. Every
   operator is `measured`, so the operator set is the largest free parameter on the load-bearing
   path. Either direction is a result.
3. **The format-acquisition experiment.** The first empirical test that touches the purpose.
   Runnable on models today. **Nobody has run it.**

## Practice

The documented failure mode here is **conventions without enforcement**, so conventions are
replaced by mechanisms — <!--s:checkers-->12<!--/s--> of them, run by whoever verifies:

- Every substantive claim is registered with a tier, evidence, and a scope that states what it
  does *not* license.
- **A tier is earned, not declared.** Reachability of evidence is *computed* — in-tree, external,
  or absent — and a top-tier claim on evidence a reader cannot reach is an error. The checker
  prints declared and earned side by side, with every gap named. `VERIFIED` currently reads
  <!--s:claims_verified_declared-->42<!--/s--> declared against
  <!--s:claims_verified_earned-->39<!--/s--> earned. **Nothing yet executes a claim's evidence
  script**, so "a test that passes" remains a stated intent rather than an enforced one.
- `CONJECTURED` requires a stated falsifier; `DESIGN-CHOICE` requires stated alternatives.
- `FAILURES.md` is append-only. A recorded failure is data; a repaired one is nothing.
- Counts in prose are **generated** from `tools/stats.rb`, because a hand-written number goes
  stale and nothing recomputes it.

---

**Author:** Jordan Traña ([Jthora](https://github.com/Jthora)) · CC0-1.0 · Published as a
defensive publication to establish prior art.
