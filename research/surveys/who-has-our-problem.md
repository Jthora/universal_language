# Who Actually Has Our Problem — Four Fields, and a Rediscovery

**Date:** 2026-08-01
**Method shift:** previous cycles imported answers to *other people's questions* and then had to
narrow them on transfer (see `research/surveys/source-critique.md`). This cycle inverts the search: instead
of asking "what does field X know," ask **"who has our exact problem — a system maintaining
consistency in its own representations, with no external oracle and no pre-specified invariants?"**

**Headline: the Cure is a rediscovery of Truth Maintenance Systems, built in 1979.**

---

## 1. Truth Maintenance Systems — the Cure, forty-seven years earlier

Doyle's **JTMS** (1979) is described as *"a domain independent method for supporting
dependency-directed backtracking, representing data, their justifications, and providing the ability
to revise beliefs when assumptions change or contradictions arise."* De Kleer's **ATMS** (1986)
extends it.

Line up the components:

| The Cure | TMS |
|---|---|
| Detect semantic inconsistency | Contradiction detection |
| Locate the *cause*, not the symptom | **Dependency-directed backtracking** — "when a conflict is found, the search backs up far enough to remove the inconsistency" (Stallman & Sussman) |
| Repair to a valid region | Belief revision when assumptions change |
| Track why something is believed | **Justifications** — the J in JTMS |
| Remember known-bad configurations | **Nogood database** |
| Maintain validity as the system evolves | "as new information is added or existing information is updated, the system adapts and maintains the consistency of its beliefs" |

It is domain-independent by design — exactly the reusable consistency layer the Cure wants to be.

### ATMS answers F-009 directly, and better than any route we had

The repair operator problem was: projection onto a non-convex admissible region is **multivalued**,
so repair is nondeterministic — the same corrupted state repairs differently depending on path.

**ATMS's answer: don't choose.** It *"overcomes the single-state dilemma found in previous TMS by
allowing for multiple contradictory assumptions to coexist and be evaluated independently, managing
contradictions by utilizing a nogood database."*

Instead of collapsing to one repaired state, maintain **all consistent contexts simultaneously** and
let downstream reasoning select. Multivaluedness stops being a defect to engineer around and becomes
the represented object. That is a cleaner resolution than convex relaxation, cell decomposition, or
tie-breaking — and it has been shipped and studied since 1986, with published complexity analyses.

**This should now be the leading candidate architecture for the Cure**, ahead of metric projection,
pinpointing, and sheaf cohomology.

## 2. Daikon — invariants can be *inferred*, not specified

Our sharpest concrete gap: *not one semantic invariant has ever been written as a checkable
predicate.* We have been treating this as a specification problem requiring philosophical work.

**Daikon dynamically detects likely invariants by observing execution traces** — reporting
properties that held across observed runs. Its template vocabulary is exactly the shape we'd need:
constancy (`x = a`), non-zero, range (`a ≤ x ≤ b`), linear relations (`y = ax + b`), ordering
(`x ≤ y`), sortedness. It is characterised as *"a machine learning technique that can be applied to
arbitrary data,"* works on **record-structured data sources** as well as program traces, and lists
**neural network analysis** among its applications.

**The reframe:** rather than deriving semantic invariants from first principles, **infer candidate
invariants from a corpus of structures known to be valid.** Run valid GIR structures through a
Daikon-style detector with graph-appropriate templates; it proposes constraints; we curate.

**Honest caveat:** Daikon finds *likely* invariants — properties true over observed data, which may
be accidental artifacts of the corpus rather than genuine laws. It is a **candidate generator, not
an oracle**, and every proposal needs validation. But a candidate generator is precisely what we
lack, and it converts a philosophical problem into an empirical one.

## 3. Database integrity constraints — and a posture we hadn't considered

Decades of practice on "which constraints are worth enforcing," with a battle-tested taxonomy:

- **Domain constraints** — value ranges, types, non-null
- **Check constraints** — qualifying criteria on a value
- **Referential integrity** — consistency *between* relations, via foreign keys
- **Assertions** — *"a global integrity constraint that applies to multiple tables or the entire
  database"*
- Triggers, for anything the declarative forms can't express

Two things transfer that we hadn't considered:

**(a) `CASCADE` is deterministic repair-by-propagation.** When deleting a referenced row would break
integrity, a database doesn't compute a nearest valid state — it applies a *declared propagation
rule* (`ON DELETE CASCADE`). Repair is specified per-constraint at design time, not solved at runtime.
That sidesteps the entire uniqueness problem by construction.

**(b) Databases *prevent* rather than *repair*.** Constraints are enforced automatically **on every
INSERT, UPDATE and DELETE** — invalid states never come into existence. The Cure's posture is
detect-then-repair, which assumes corruption already happened.

**This is a genuine architectural fork we have never articulated:**

| Posture | Mechanism | Fits when |
|---|---|---|
| **Prevention** (databases) | Constraint check as a write barrier; reject invalid mutations | State changes through discrete, interceptable transactions |
| **Repair** (the Cure as specified) | Periodic check, then correction | State evolves continuously or unobservably |

For an AI system whose representations shift through learning and inference rather than discrete
writes, prevention may be unavailable — but that should be an argued conclusion, not an unexamined
default. Where representation updates *are* interceptable, prevention is strictly better: it's
cheaper, deterministic, and needs no repair operator at all.

## 4. Diagram comprehension — partial support for the F-012a narrowing

The narrowed iconicity claim (F-012a) said: structural/spatial conventions may be readable even
though referential pictograms are not. That now has support.

Documented, reliably-read spatial conventions:

- **Proximity → similarity.** "The distance between pairs of nodes is related to the perceived
  similarity between them"; node spacing drives perceived clustering.
- **Arrows → process.** *"Arrows are understood because they are identified with processes that we
  know well from what happens around us — in space and in time."*
- **Containment → hierarchy/composition.** "Hierarchies of containment, composition, or
  configuration involve things inside other things."
- **Centrality → importance**; circular arrangement → equal importance.

**But note the register.** These are **graded, analogue** readings — *more* proximity means *more*
similar — not discrete symbolic denotation. And the arrow explanation is explicitly grounded in
familiar physical experience, which is culture- and embodiment-dependent.

**Refined position:** UWS's *placement grammar* is likely more intuitively readable than its *symbol
inventory* — but what it conveys reliably is **soft relational structure** ("these go together,"
"this leads to that"), not precise semantics. That is genuinely useful for legibility and error
detection, and insufficient for unambiguous meaning transfer. It supports the taught-notation
conclusion while making the spatial layer the part that needs least teaching.

---

## The pattern worth naming

This is now the **third** independent mature field found to be solving the Cure's core problem:

1. **Ontology repair** — pinpointing, justifications, minimal diagnoses (DL/Semantic Web)
2. **AGM belief revision** — minimal-change postulates, contraction/expansion/revision
3. **Truth Maintenance Systems** — justifications, nogoods, dependency-directed backtracking,
   multi-context maintenance

They are not unrelated — all three descend from justification-based reasoning about belief change,
approached from knowledge representation, philosophical logic, and classical AI respectively.

**Conclusion: the Cure is not a novel problem.** It is a well-studied problem — consistency
maintenance in a revisable knowledge base — applied to an unusual substrate (learned representations
rather than curated symbolic knowledge). **The work is integration and adaptation, not invention.**
That is a substantial reduction in risk and scope, and it means the correct next literature step is
to determine which of the three traditions best survives transfer to a *learned, non-symbolic*
substrate — which is the genuinely novel part.

---

## What changes

1. **ATMS becomes the leading candidate architecture for the Cure.** Multi-context maintenance
   dissolves the multivalued-repair problem rather than engineering around it. Compare against
   pinpointing, metric projection, and sheaf cohomology explicitly — but this is now the front-runner.
2. **Bootstrap invariants empirically via Daikon-style inference** over a corpus of valid structures,
   instead of deriving them philosophically. Converts the project's sharpest gap into an experiment.
3. **Decide the prevention-vs-repair posture explicitly.** Where representation updates are
   interceptable, prevention is strictly better and needs no repair operator. This has never been
   articulated as a choice.
4. **Adopt the database constraint taxonomy** (domain / check / referential / assertion) as the
   starting vocabulary for semantic invariants — it is battle-tested and maps cleanly onto a typed
   graph.
5. **`CASCADE`-style declared repair rules** are a fourth repair route: specify repair per-constraint
   at design time rather than solving for it at runtime.
6. **The spatial grammar is UWS's most readable layer** and should be leaned on for legibility, while
   the symbol inventory carries the teaching burden.

## Open question this raises

If the Cure is a TMS, then the genuinely novel contribution is not the consistency machinery — it's
**applying justification-based consistency maintenance to learned, sub-symbolic representations**
where there are no discrete assertions to attach justifications to. That is a real research question,
and it is much sharper than anything the project was asking before. **It also has not been checked**
— whether anyone has already attempted TMS-over-neural-representations is the obvious next search.
