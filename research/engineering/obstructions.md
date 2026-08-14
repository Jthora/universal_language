# Proof-Availability Audit

**Date:** 2026-08-01
**Question:** which of the project's open claims can be settled by proof rather than experiment?
**Method:** every CONJECTURED entry in `claims.yaml` (23 at time of audit) classified by whether its
objects are mathematical. Then research on the theorems that close the provable ones.
**Result: four theorems found that bear directly on open entries. Two are obstructions the project
did not know about and that bite hard. One dissolves the Cure's central blocker. One converts the
"central open question" into a computation.**

Applies the standing rule from F-015: *before filing a claim as awaiting experiment, check whether
its objects are mathematical.*

---

## 1. The audit

| Claim | Provable? | Instrument |
|---|---|---|
| `SEMANTIC-EQUALITY` | **Yes — bounded** | **Rice's theorem** (§3) |
| `NEG-INVOLUTION` | **Yes** | Equational reasoning once the theory exists; a proof obligation, never an experiment |
| `CURE-REPAIR-CONVEX-ROUTE` | **Yes — and superseded** | **AGM / Katsuno-Mendelzon** (§4) |
| `CURE-SHEAF-ROUTE` | **Yes** | Sheaf cohomology; obstruction to a global section is a computation |
| `SYMMETRY-EMERGES-NOT-INSTALLED` | **Yes** | Partly settled by `ERLANGEN-SURVIVAL-RANKING`; order-parameter half by **Landau** (§5) |
| `ERLANGEN-AS-COARSE-GRAINING` | **Yes** | Flow-vs-lattice correspondence is a mathematical statement |
| `DEFINITIONAL-DEPENDENCY-ISOMORPHISM` | **Yes** | Poset isomorphism (already reclassified, F-015) |
| `COX-STYLE-CHARACTERIZATION` | **Yes** | Representation theorem; already a proof target |
| `INVARIANT-INFERENCE` | **Partly** | Learnability bounds (PAC / identification in the limit) are theorems; *which* invariants a given corpus yields is empirical |
| `GROUNDING-PROBLEM` | **Partly** | Formalizable as identifiability; Gold-style results bound it. The residue is genuinely empirical |
| `VERIFICATION-AS-PRACTICE` | **Partly** | Costly signaling has equilibrium models (Spence); separating-equilibrium existence is provable |
| `CURE-MONITORS-COUPLING-NOT-CONSISTENCY` | **Partly** | The "extraction cannot reach interventional knowledge" half is Pearl's Causal Hierarchy Theorem — already noted, never registered as proof |
| `CURE-CONSISTENCY-VS-OPENENDEDNESS` | **Candidate** | Smells like a tradeoff/impossibility theorem (cf. stability-plasticity). **Unsearched** |
| `UL-IS-EMERGENT-UNIVERSAL` | Mixed | Class structure provable; membership not |
| `UL-EXISTS` | **No** | Membership question about contingent objects |
| `SURFACE-DIVERSITY-IS-PREDICTED` | **No** | WALS is data |
| `UWS-AMR-BENCHMARK`, `CURE-BENCHMARK-RIPPLE` | **No** | Benchmarks by construction. *AMR is an **English-only corpus** with English predicate lexicon and English annotators — usable as a stress test, never as universality evidence (`../surveys/research-register.md#D2-c`).* |
| `APOTHEOTIC-ATTRACTOR-GAP` | **No** | Taxonomic proposal |
| `ENACTION-CRITIQUE` | **No** | Philosophical position |
| `CURE-PREVENTION-VS-REPAIR` | **No** | Engineering design choice |
| `EMERGENT-COMMUNICATION-UNCHECKED` | **N/A** | A literature review, not a claim |
| `IR-PRIOR-ART-POSITION` | **N/A** | **Mis-registered — this is a task, not a claim.** Should be moved out |

**Roughly half the conjecture backlog is provable or partly provable.** That is the headline. The
project has been carrying mathematical questions in an empirical queue.

---

## 2. Löb's theorem — an obstruction the project has never accounted for, and it bites the Cure

**This was not in the registry in any form, and it is the most serious finding here.**

> Löb's theorem: **"No system T can trust its own proofs until it witnesses them."** For any formula
> φ and theory T at least as strong as PA, T proves (□φ → φ) **only if** T proves φ outright.
>
> **The Löbian obstacle:** *"an agent cannot trust itself generally... and therefore cannot trust its
> future self to achieve goals."* Consequence: *"an agent X can only 'trust' the reasoning of an
> agent Y with a **strictly weaker** reasoning system than themselves."*

### Why this bites

**The Cure is a system that repairs representations and must trust that its repairs preserve
invariants.** The moment the Cure is part of the system it repairs — or repairs a successor version
of itself — Löb applies:

- **A self-repairing system cannot verify at equal logical strength that its repairs are sound.**
- **A parent cannot verify a successor of equal or greater strength.** Trust flows only *downward*
  in strength.

This is a proof, not a risk. No experiment can rescue it, and no amount of engineering effort
"discovers" a way around it — MIRI's tiling-agents work exists precisely because the obstacle is
real. The proposed escapes (waterfall constructions permitting an infinite descending chain of
trust; source-code-visible agents proving mutual cooperation) all pay in strictly weakening
successors or in changing the logic.

### The architectural decision it forces

The Cure must be **external to and stronger than** what it repairs, **or** the design must
explicitly accept a descending strength chain, **or** it must abandon proof-based self-trust for
something weaker (probabilistic, empirical). **That is a decision that must be made on paper before
implementation, and it was heading toward being discovered the hard way.**

---

## 3. Rice's theorem — bounds `SEMANTIC-EQUALITY` from above, and the project already dodged it by accident

> **Rice's theorem: all non-trivial semantic properties of programs are undecidable.**

`SEMANTIC-EQUALITY` asks for a decision procedure for "these two structures mean the same thing."
**If the representation is Turing-expressive, Rice settles this negatively. It is not an open
question at that level of generality.**

### The workaround is itself theorem-backed

**Abstract interpretation** is *"a theory of sound approximation of the semantics of computer
programs"* that *"makes the semantics decidable at the cost of precision, but in a way that the
semantics remain sound."* **Galois connections** define the abstract domains, and give a
*calculational* construction: composing abstraction, the concrete operation, and concretization
yields **"the best possible approximation."**

So the semantic checker has a **derivable** design — not a heuristic to be tuned empirically. The
best abstract operator is *computed* from the concrete one.

### And the price is known exactly

The sharp result (Baldan et al., ICALP 2021): *"For abstract program semantics, **any nontrivial
abstract property is undecidable** and every decidable overapproximation **necessarily includes an
infinite set of false positives** which covers all values of the semantic abstract domain."*

**The Cure will have false positives. Provably. Infinitely many.** The design question is never
"can we eliminate them" but "which ones do we accept" — and that reframing comes free from the
theorem.

### The accidental save worth noticing

`IR-NORMALIZATION-STRATEGY` already chose an **acyclic, strongly-normalizing, non-Turing-complete**
core — for confluence reasons (Plump), not for Rice reasons. **That choice is exactly what keeps the
IR out of Rice's full reach.** The right decision was already made, for adjacent reasons. It should
now be recorded as load-bearing for *two* independent results, because anything that later
reintroduces general recursion silently re-imports undecidability.

---

## 4. AGM / Katsuno-Mendelzon — dissolves the Cure's central blocker

`CURE-REPAIR-DETERMINISM` was **RETIRED** because metric projection onto a non-convex admissible
region is multivalued (Hilbert projection theorem needs convexity), and 𝒜 is almost certainly
non-convex. That was recorded as *the central open engineering problem of the program*.

**It was an artifact of the formalism, and there is a representation theorem that avoids it entirely.**

> **Katsuno & Mendelzon proved that the functions induced from faithful preorders are *precisely*
> those satisfying the AGM postulates for revision.**

"Precisely those" is an **if and only if** — a representation theorem. If you want a repair operator
satisfying the AGM rationality postulates, it **must** be induced by a faithful preorder over
interpretations, and any faithful preorder gives you one.

### Why this removes the obstacle

**A preorder is not a metric.** Minimal change defined by an ordering needs **no convexity, no
distance, no projection** — the three things that broke the original formulation. The convexity
problem was never a fact about semantics; it was a consequence of choosing metric projection as the
formalism.

**What it converts the problem into, honestly:** not "solved," but *tractable*. "Find a
well-defined projection onto a non-convex set" (ill-posed) becomes **"specify an entrenchment
ordering"** (a design problem, with a theorem guaranteeing that any admissible choice yields a
rational operator). That is a real change in kind.

And it connects two threads that were floating: **epistemic entrenchment is exactly the mechanism
for deciding what must hold versus what may change** — the machinery
`CURE-CONSISTENCY-VS-OPENENDEDNESS` needs to locate the band between rigidity and drift.

The Katsuno-Mendelzon characterization has also been generalized *"from propositional logic to the
setting of base revision in arbitrary monotonic logics,"* so it is not confined to a toy setting.

---

## 5. Landau theory — turns "the central open question" into a computation

`SYMMETRY-EMERGES-NOT-INSTALLED` and the framework documents both name the **semantic order
parameter** as *the* open question, with a proposal to test TopSim, context independence, and
positional disentanglement as candidates and see which shows critical behaviour. **That is an
empirical search where a derivation exists.**

> *"The existence of an order parameter... is directly associated to the phenomenon of spontaneous
> symmetry breaking."*
>
> *"As the free energy must respect the symmetries of the system, **its expansion in powers of the
> order parameter must do so as well, which allows it to be constructed systematically**."*

**The order parameter is determined by which symmetry is broken.** Identify the symmetry group *G*
of the disordered (pre-semantic) phase and the residual symmetry *H* of the ordered (semantic)
phase, and the order parameter lives in the coset space *G/H*. That is standard, rigorous, and
mechanical.

**So the research move is not "benchmark three metrics." It is "identify G and H."** And the project
now has an unusually good handle on the symmetry side after the Erlangen survival computation.

### One honest limit

**Noether's theorem does not apply here as stated.** It requires a Lagrangian/action formulation with
continuous symmetries — *"each continuous symmetry of the Lagrangian is associated with a conserved
quantity."* Semantics has no action principle, so the Noether route is unavailable until one is
constructed. **Landau theory carries no such requirement** — it is phenomenological, needing only an
order parameter and a free energy respecting the symmetry. **Landau is the applicable instrument;
Noether is a target that would first require inventing a semantic action principle.** Worth stating
because the two are easy to conflate, and claiming Noether would be exactly the kind of borrowed
authority this project has been caught using before.

---

## 6. What changes

**Register two obstruction theorems that were entirely absent:**
1. **Löb** — forces an architectural decision about the Cure's self-trust, on paper, now.
2. **Rice** — bounds `SEMANTIC-EQUALITY`, mandates abstract interpretation as the construction, and
   makes infinite false positives a known cost rather than a future surprise.

**Reopen one retired claim on better foundations:**
3. `CURE-REPAIR-DETERMINISM` was retired against the wrong formalism. **AGM/Katsuno-Mendelzon gives a
   representation theorem requiring no convexity.** Reopen as a preorder-based operator.

**Convert one empirical search into a derivation:**
4. The semantic order parameter follows from the *G/H* symmetry-breaking structure. Identify the
   groups rather than benchmarking candidate metrics.

**Housekeeping:**
5. `IR-PRIOR-ART-POSITION` is a task masquerading as a claim. Remove it from the registry.
6. `CURE-CONSISTENCY-VS-OPENENDEDNESS` is the most promising unsearched candidate for an
   impossibility/tradeoff theorem. Search the stability-plasticity literature next.

**And the reusable lesson:** three of the four findings are **decades-old theorems in fields the
project already cites**. They were missed not because they are obscure but because the questions
were filed as empirical and never re-examined. **The audit is worth repeating whenever the
conjecture backlog grows.**
