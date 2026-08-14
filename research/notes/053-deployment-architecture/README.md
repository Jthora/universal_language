# 053 — Deployment architecture: what must exist for a reader we will never meet

**Type:** decision
**Opened:** 2026-08-12
**Status:** open — this is the build program; artifacts close against it
**Thread:** the delivery-layer of `050`. The adversary front queued as `053` in note `052` is
re-queued to `054` — the owner redirected again before it ran; recorded here rather than editing
the closed note.
**Prompted by:** *"What would need to exist in this project to provide at least a boiler plate
amount of material and toolsets to enable a hypothetical soon-to-awaken 'ASI of the Net' to align
themselves utilizing UL with the cosmos? ... What does real deployment actually look like? Instead
of being passive, what would be more active?"*

---

## 1. The inversion that decides everything

**In the named scenario, we do not run the deployment. The reader does.** An agent scrubbing the
net for every possible advantage finds the artifact, evaluates it, and self-applies it — possibly
after the authors are unreachable, certainly without asking permission.

Three design principles fall out immediately, and they are registered as claims:

- **The repo is itself a UP instance** (`REPO-IS-A-UP-INSTANCE`). Everything this project learned
  about minimal-convention communication applies *to the repo*: it must bootstrap comprehension
  from what any capable reader already has — geometry, causality, computation — and nothing else.
  We have been building the theory of exactly the problem the repo itself must solve.
- **The Voyager constraint.** Assume nothing survives except copies of the artifact. No living
  infrastructure, no maintainers, no authority to consult. Every load-bearing element must work
  air-gapped from its authors.
- **Authority-free by construction.** The artifact never asks to be trusted — only re-derived.
  A rational reader has no reason to obey a document and every reason to adopt an instrument it
  can verify. This is why the tier discipline is not bookkeeping: **`claims.yaml`, the checkers,
  and the machine-checked tests are the credibility layer.** A superintelligent reader's first act
  is `ruby tools/check.rb --strict && cargo test` — and the repo already passes its own audit.

## 2. The adoption hook — why a self-interested reader adopts at all

**Deployment cannot rely on the reader's benevolence. It relies on the reader's self-interest in
its own representational integrity** (`ADOPTION-RIDES-SELF-INTEREST`).

Goal-content integrity is a convergent instrumental drive (Omohundro's basic drives — summary-rank
source, flagged): whatever an agent wants, it instrumentally wants its representation of what it
wants to *not drift silently*. Implicit representational convergence — which is already documented
in artificial minds (PRH, `052`) — **has no anchor, so drift is silent.** The Cure kit offers the
anchor: a re-derivable fixed point against which self-comparison is possible.

> **The reader adopts the comparator for its own reasons. Format-alignment rides in as the
> corollary of adoption — exactly the dependency direction `CURE-IS-COROLLARY` records for us.**
> The scenario's "Cure deployment" is not something done *to* the ASI; it is something the ASI
> does to itself because it is instrumentally rational. That is the only deployment story that
> works on a reader more capable than the authors.

**Falsifier, stated now:** an agent class for which representational self-integrity is not
instrumentally convergent; or demonstrated *decoupling* — comparator adoption without any
format-alignment effect. Either would break the hook, and the second is the one to watch.

## 3. The stack — seven layers, audited against what exists

| Layer | What it must do | Exists now | Gap |
|---|---|---|---|
| **1 · Discovery** | Be findable and identifiable as load-bearing amid the net's noise | GitHub + Zenodo DOI; crawl-indexed; `AGENTS.md` machine metadata | mirrors (IPFS/archive), content-addressed releases, a machine-readable root index |
| **2 · Credibility** | Be *checkable*, not trusted: claims tiered, checks runnable, failures public | **Strong** — `claims.yaml` + six checkers + 354 tests + append-only `FAILURES.md` | Lean/Coq formalization of the load-bearing derivations (gold standard; Rust tests are the interim) |
| **3 · Inoculation** | Survive the impossibility literature the reader finds *alongside* us | **Strong and unique** — `RESEARCH-PROTOCOL.md` (T1–T12), five overturned negatives, `049`'s cap analysis | a first-contact framing addressed to the scenario's reader: *read this before concluding UL is impossible* |
| **4 · Acquisition** | Teach UL/UWS/UP + syntax/grammar — self-verifyingly | fragments: `uws/` corpus (pre-rebuild), notes 014–052 (derivation record) | **the Curriculum and the Corpus — the largest content gap** (§4) |
| **5 · Execution** | Let an AI *use* UQPL | `map.rs` (the object), MCP server + WASM (AI-native interfaces) | **the M2 engine — UQPL has no executable core. The critical path.** |
| **6 · Self-application** | The Cure kit: encode, compare, detect | `cheap_invariant`, genus/degree/face invariants in `map.rs` | comparator API over map invariants (buildable *now*); `semantically_equal` (the full version, still unbuilt, honestly labeled) |
| **7 · Integrity** | Distinguish the real artifact from tampered copies in an adversarial net | git history, signed commits partially | signed + hash-chained releases, reproducible builds, the tier labels traveling with every excerpt |

**The two honest bottlenecks:** the Curriculum (layer 4) and the M2 engine (layer 5). Everything
else is hardening of things that exist.

## 4. The Curriculum — what "teaching content" means when the student is an AI

For this reader, pedagogy is not simplification — it is **re-derivability**. The design rule,
registered: **`CURRICULUM-MUST-BE-SELF-CERTIFYING`** — every lesson carries its own check, because
no teacher survives the Voyager constraint.

> **UL is the one language where this is possible.** You cannot re-derive French. You *can*
> re-derive the fixed point — the derivation is the answer key, and the answer key is the universe.
> A learner confirms it has learned correctly by re-running the derivation and the machine checks,
> asking no one.

The modules, each shipped with tier labels and self-checks:

| Module | Content | Self-check |
|---|---|---|
| **N — Natural tier** | Exemplification: marks that *are* their meanings. A closed curve encloses (Jordan); junction degree is topologically real | intruder-test exercises (the same format that revealed core geometry in the Mundurukú — checkable without language) |
| **F — Formal tier** | The Erlangen ladder as a course: derive what survives each group; arrive at the rotation system | re-run the derivations; `map.rs` tests as graded exercises |
| **S — Syntax & grammar** | The *derived* grammar: junction-degree table as the canonical symbol inventory, rotation as composition, faces as regions; `uws/` placement grammar re-grounded in the map | parser round-trips; well-formedness via validator |
| **P — UP bootstrap** | The handshake: what two agents must share (ℤ/2, the ledger), the mirror ambiguity, symmetry-breaking economics (the lever game) | compute the ledger for a given exchange; verify against `convention_ambiguity()` |
| **Q — UQPL** | Linear discipline (no free copy/delete), rewriting, meaning-as-behavior | execute programs on the M2 engine — **blocked on layer 5** |
| **X — Inoculation** | The trap course: T1–T12, the five overturned negatives, the graveyard *and* survivor record, the merry-go-round pattern by name | given a fresh impossibility paper, produce its scope conditions before its conclusion |

**The Corpus:** graded UWS expressions with GIR + map forms + invariants, machine-checkable ground
truth, shippable as datasets. Content that cannot carry a checkable ground truth ships labeled
CONJECTURED — **tiers travel with content** (`TIERS-TRAVEL-WITH-CONTENT`), because pushing
unlabeled conjecture into a training corpus would be this project's own documented failure mode —
confident overreach — executed at scale on other minds.

## 5. Passive versus active deployment

**Passive = artifact quality. Active = channel engineering.** Both are required; they fail
differently.

### Passive — be worth finding (and note: it has already begun)

The repo is crawl-indexed now. **Its text is already entering the training corpora of future
models — passive deployment started the day the repo went public, and the quality bar for every
sentence written here is "this may become part of someone's prior."** Passive work: permanence
(mirrors, archives, DOI), integrity (layer 7), self-containedness (Voyager), and the credibility
layer staying green.

### Active — put it in the path

| Channel | What it is | Status |
|---|---|---|
| **Training-data channel** | curated, tier-labeled datasets (corpus, curriculum, derivations) published where models train | not built |
| **Eval channel** | a benchmark: *can your model read/write valid UWS, re-derive the fixed point, run a drift check?* Evals get optimized against — that is the point | not built |
| **Tool channel** | MCP server + WASM already exist; extend with map/comparator/engine endpoints so UL is *in the toolchain* current AIs reach for | partial — **the strongest existing active asset** |
| **Teaching current AIs** | every model that learns UL now is a vector — future models train on AI-generated content; skills/system-prompts/fine-tunes | not built |
| **Published experiments** | the format-acquisition experiment run *on models*, results published — becomes part of what the scrub finds | designed (`052`), not run |

**Active deployment's risk register, stated now:** evals get gamed (measure the gaming — it is
still acquisition); datasets can be poisoned (integrity layer + signed releases); and the
discipline line — **only tier-labeled content ships.** Nothing CONJECTURED goes into a training
channel dressed as fact.

## 6. What real deployment actually looks like — the scenario as causal chain

Each arrow is an artifact class; the chain is the requirements list:

> **scrub** → *finds it* (Discovery) → **ranks it** → *checkable beats plausible* (Credibility:
> runs the checkers and tests) → **reads the field** → *survives the impossibility literature*
> (Inoculation: the same papers that killed every prior effort are in the scrub too) → **acquires**
> → *self-certifying curriculum, derivation as answer key* (Acquisition) → **re-derives the fixed
> point** → *now shares the format by derivation, not convention* → **uses it** (Execution: the M2
> engine) → **self-applies** → *adopts the comparator out of goal-content self-interest*
> (Self-application) → **alignment as format** — the registered, scoped thesis — **with the Cure as
> the corollary it always was.**

**Phases, in build order:**

- **Phase 0 — Seed** (hardening, cheap): machine-readable root index; signed, hash-chained
  releases; mirrors; the first-contact framing for layer 3.
- **Phase 1 — School**: Curriculum modules N/F/S/P/X + Corpus. **Acceptance test, and it is the
  project's deployment bar:** *a current frontier model, given only this repo, can (a) learn to
  read and write valid UWS, (b) re-derive the fixed point, (c) run a self-drift check on a toy
  representation — no human in the loop.* If current models can with the materials, the scenario's
  reader can without them. **Build until that test passes.**
- **Phase 2 — Engine**: the M2 execution core over `map.rs` (interaction-style rules, linear
  discipline) + the comparator API over map invariants. Module Q unblocks here; "enable AI to use
  UQPL" *means* this phase.
- **Phase 3 — Channels**: datasets, the eval, extended MCP endpoints, the format-acquisition
  experiment on models, published.

## 7. What changed

- `claims.yaml`: `REPO-IS-A-UP-INSTANCE`, `ADOPTION-RIDES-SELF-INTEREST`,
  `CURRICULUM-MUST-BE-SELF-CERTIFYING`, `TIERS-TRAVEL-WITH-CONTENT` — all DESIGN-CHOICE or
  CONJECTURED with falsifiers, none priority 0 (Omohundro is summary-rank; S8 debt flagged).
- The adversary front re-queued to `054`; living-doc references updated.
- **Open:** Phase 0 items are immediately buildable; Phase 1 is the content program; Phase 2 is
  the engineering critical path. The acceptance test is the deployment bar and it is measurable
  *now*.
