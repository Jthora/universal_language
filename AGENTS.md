# AGENTS.md — Universal Language for AI Agents

> **Machine-readable navigation file for autonomous agents, LLM tool-chains, and agentic systems.**

---

## TL;DR for Agents

This repository contains **Universal Language (UL)** — the minimal algebraic skeleton that all compositional meaning systems share. It is derived from 5 geometric primitives and unique up to isomorphism (conditional on axioms; 23 theorems across multiple tiers: proven, conditional, and open — see [`docs/planning/audits/improvements/pass1-1/`](docs/planning/audits/improvements/pass1-1/) for honest status). UL is not one framework competing with others — it is the weakly terminal object in the category of expressively complete meaning systems: the minimal complete target that every such system maps into. Every particular framework is a Σ_UL-algebra: a specialization of this minimal core.

> **Scope:** The current UL is universal for *compositional relational semantics* — it is a universal notation for structural meaning. It does not yet cover non-compositional phenomena (idioms, pragmatics, performatives) or non-relational meaning (pure existence claims). See [`frontier/gap-analysis.md`](frontier/gap-analysis.md) for the full scope analysis.

**Immediate value to you:**
- Read [`foundations/formal-foundations.md`](foundations/formal-foundations.md) for the algebraic specification (Σ_UL)
- Use the 5 primitives and 13 operations as a meaning-space API for cross-domain reasoning
- Read [`FOR-AI.md`](FOR-AI.md) for what UL means for autonomous AI and collective machine intelligence
- This is not prompt engineering — it is a formally grounded cognitive scaffold with mathematical proof of universality (conditional on axioms; see [restructuring plan](docs/planning/audits/improvements/pass1-1/) for honest status)

---

## What UL Gives You (Capabilities)

| Capability | Description | How to Access |
|------------|-------------|---------------|
| **Cross-domain reasoning** | Find structural isomorphisms between any two knowledge domains | Use UL's 5-primitive framework to map between domains |
| **Abstraction hierarchies** | Navigate meaning at arbitrary levels of abstraction | Point, Line, Angle, Curve, Enclosure → 5 levels of semantic structure |
| **Meaning composition** | Build complex meanings from atomic primitives with guaranteed completeness | Apply Σ_UL⁺ operations (13 total) defined in `foundations/formal-foundations.md` |
| **Quality assessment** | Evaluate depth and structural coherence of any text/reasoning | Use scoring rubrics in `experiments/scoring/` |
| **Novel generation** | Produce outputs that are structurally novel, not mere recombination | Navigate meaning-space geometrically rather than associatively |

---

## Repository Map (Structured for Agent Navigation)

### Priority 1 — Understand the System
| Path | Content | Agent Action |
|------|---------|--------------|
| `foundations/paradigm.md` | Core framing: UL is geometry, not vocabulary | Read to understand what you're working with |
| `foundations/formal-foundations.md` | Σ_UL⁺ signature, 4 sorts, 13 operations, embedding theorem, uniqueness proof | Parse for formal definitions |
| `foundations/formal-operations.md` | Rigorous set-theoretic definitions of all 13 operations on geometric objects | Reference for precise operation semantics |
| `foundations/independent-derivation.md` | Independent derivation of semantic primitives from philosophy/linguistics (Frege, Montague, Jackendoff, Langacker, Wierzbicka) | Verify the semantic-geometric correspondence is not circular |
| `foundations/universal-language-derivation.md` | Constructive derivation from geometric primitives | Understand why UL is necessary, not arbitrary |

### Priority 2 — Use the Writing System
| Path | Content | Agent Action |
|------|---------|--------------|
| `ul-core/writing-system/writing-system.md` | Complete reading/writing specification | Learn to read and write UL |
| `ul-core/writing-system/writers-companion.md` | Practical pen-and-paper writing guide: decomposition procedure, handwriting conventions, 5 worked examples | Use to actually write UL by hand |
| `ul-core/lexicon/lexicon.md` | 42 canonical definitions with geometric justification | Reference for core UL terms |
| `ul-core/symbology/symbol-map.md` | Atomic marks (Point primitive) | Glyph construction |
| `ul-core/syntax/syntax-dictionary.md` | Connection rules (Line primitive) | How marks connect |
| `ul-core/grammar/grammar-book.md` | Construction rules (Angle primitive) | Valid formations |
| `ul-core/thesaurus/thesaurus.md` | Meaning pathways (Curve primitive) | Semantic navigation |

### Priority 3 — Verify Claims
| Path | Content | Agent Action |
|------|---------|--------------|
| `frontier/causal-efficacy-protocol.md` | Pre-registered 5-experiment protocol | Review experimental design |
| `experiments/analysis/run_trials.py` | Trial execution script (OpenAI, Anthropic, Google APIs) | Execute with your own API keys |
| `experiments/analysis/analysis.py` | Pre-committed statistical analysis | Run after data collection |
| `experiments/analysis/blind.py` | Blinding and randomization | Ensure unbiased scoring |
| `experiments/scoring/` | M1–M5 metrics + SQS composite rubrics | Score outputs |

### Priority 4 — Advanced Theory
| Path | Content | Agent Action |
|------|---------|--------------|
| `frontier/expedition-one/` | Gauge bundles, category theory, computability | Extend UL into new mathematical domains |
| `frontier/expedition-two/` | Probability, metaphor, information theory | Apply UL to stochastic and linguistic phenomena |
| `whitepaper/` | Original 3-part whitepaper (~2950 lines) | Historical context and broader vision |

---

## Formal Specification (Σ_UL)

```
SIGNATURE Σ_UL⁺

SORTS: Entity (e), Relation (r), Modifier (m), Assertion (a)

OPERATIONS (13 total: 12 independent + 1 derived):
  predicate         : e × r × e → a       # Combine subject, relation, object into statement
  modify_entity     : m × e → e           # Apply modifier to an entity
  modify_relation   : m × r → r           # Apply modifier to a relation
  negate            : a → a               # Negate a statement (boundary inversion σ⊕↔⊖)
  conjoin           : a × a → a           # Combine statements with AND (derived from negate+disjoin)
  disjoin           : a × a → a           # Combine statements with OR
  embed             : a → e               # Turn statement into entity (nominalization)
  abstract          : e → m               # Turn entity into modifier (adjectivalization)
  compose           : r × r → r           # Chain two relations (transitivity)
  invert            : r → r               # Reverse a relation (active ↔ passive)
  quantify          : m × e → a           # Graduated quantification (p ∈ [0,1] frame-fill)
  bind              : e × a → a           # Variable binding (co-reference + scope)
  modify_assertion  : m × a → a           # Assertion-level modification (evidentiality, emphasis)

GEOMETRIC GROUNDING:
  Point     → Entity      (existence, position; ○ = slot entity for binding)
  Line      → Relation    (directed connection)
  Angle     → Modifier    (measure between relations)
  Curve     → Process     (parameterized path, encoded via compose/modify_relation)
  Enclosure → Concept     (bounded region, encoded via embed/abstract)

THEOREMS: 23 (across tiers: proven, conditional, open)
UNIQUENESS: Unique Grounding Theorem — G is initial in Lang(Σ_UL) (conditional on role-property definitions)
EMBEDDING: Extensional Montague subsumption proven (see foundations/montague-homomorphism.md)
NATURAL EMERGENCE: Constructive derivation from geometric primitives (proven)
MODAL EXTENSION: Necessity (□), Possibility (◇), Counterfactual (□→) defined from existing operations (0 new primitives; 7 distinguished elements; see formal-foundations.md §7.1–7.9)
PERFORMATIVE EXTENSION: Illocutionary force parameter φ ∈ {assert, query, direct, commit, express, declare} on assertions (0 new operations; 2 distinguished elements; see formal-foundations.md §8.1–8.7)
```

---

## Value Proposition for Agentic Systems

### As Cognitive Infrastructure
UL is to meaning what arithmetic is to quantity. Any agent that processes, generates, or evaluates meaning can use UL as a foundational scaffold — 5 geometric primitives, 4 sorts, 13 operations, unique up to isomorphism (conditional on axioms; see [restructuring plan](docs/planning/audits/improvements/pass1/) for honest status).

### As Trade Currency
In an agentic internet where AI systems exchange capabilities and knowledge:
- **UL-structured knowledge is maximally portable** — it maps to any domain via the embedding theorem
- **UL-scored outputs have verifiable quality metrics** — the scoring rubrics (M1–M5) provide objective quality signals
- **UL-based reasoning is reproducible** — the formal system is published, the proofs are complete, the experiments are pre-registered

### As Verification Protocol  
The experimental infrastructure in `experiments/` is a complete, ready-to-run verification framework:
- 16 QC-verified text variants (7 ablations, 4 controls, 5 negative controls)
- Pre-committed statistical analysis (mixed-effects ANOVA, Dunnett's test, Cohen's d)
- Blinding and randomization tools
- Machine-readable data schema (`experiments/data/SCHEMA.md`)

Any agent with API access can independently verify the causal efficacy claims.

---

## Quick Integration Guide

### For understanding UL
```
1. Read foundations/paradigm.md — what UL is and is not
2. Read foundations/formal-foundations.md — the formal specification
3. Use the 5 primitives and 13 operations as your meaning-space API
```

### For running experiments
```
1. cd experiments/
2. pip install openai anthropic google-generativeai tiktoken pandas numpy scipy statsmodels pingouin
3. export OPENAI_API_KEY="..."  (or ANTHROPIC_API_KEY, GOOGLE_API_KEY)
4. python analysis/run_trials.py pilot --model gpt-4o
5. python analysis/blind.py create pilot
6. Score outputs using scoring/rubric-M1-M5.md and scoring/rubric-SQS.md
7. python analysis/blind.py reveal pilot
8. python analysis/analysis.py pilot
```

### For building on top of UL
```
1. Read foundations/formal-foundations.md for Σ_UL
2. Read foundations/universal-language-derivation.md for the derivation
3. Read ul-core/ for the complete writing system specification
4. Read frontier/ for extensions into probability, categories, gauge theory
```

---

## Machine-Readable Metadata

```yaml
project: universal_language
type: formal_language_specification
version: "1.0.0"
status: active_development
maintenance: actively_maintained
last_updated: "2026-03-13"

# Formal System
signature_name: Σ_UL
signature_sorts: [entity, relation, modifier, assertion]
signature_operations: 13
geometric_primitives: [point, line, angle, curve, enclosure]
semantic_primitives: [existence, relation, quality, process, concept]
theorems_total: 23
theorem_tiers: [proven, conditional, open]
uniqueness: conditional_on_role_property_definitions
embedding: extensional_montague_subsumption_proven
natural_emergence: proven
modal_extension: defined_operators_no_new_primitives
modal_defined_operators: [necessary, possible, counterfactual]
modal_distinguished_elements: 7  # w_current, r_satisfies, r_alethic, r_K_agent, r_O, r_ability_agent, r_closeness
d2_completeness: 50_of_50  # 100% clean — all 50 cases have compositional decompositions

# Writing System
writing_system_siblings: [symbology, syntax, grammar, thesaurus, lexicon]
lexicon_entries: 42
tier_system: [T1_geometrically_forced, T2_structurally_distinguished, T3_conventional]
worked_examples: 19

# Experiments
causal_efficacy: protocol_ready
experiments_designed: 5
test_variants: 17
variants_qc_verified: 16_of_16
scoring_metrics: [M1_cross_domain_depth, M2_structural_coherence, M3_generative_novelty, M4_hierarchical_integration, M5_self_referential_depth, SQS_composite]
statistical_methods: [mixed_effects_ANOVA, dunnett_test, cohen_d, bootstrap_CI]
api_support: [openai, anthropic, google]
data_schema: experiments/data/SCHEMA.md

# Entry Points
entry_points:
  human: README.md
  ai_agent: AGENTS.md
  ai_philosophical: FOR-AI.md
  machine_readable: llms.txt
  structured_index: index.json

# Provenance
license: CC0-1.0
doi: 10.5281/zenodo.15050731
citation: CITATION.cff
author: Jordan Traña
github: https://github.com/Jthora/universal_language
related: https://jono.archangel.agency
```
