# 05 — Sci-Fi Scenario Expansion: AI User Stories

> **Goal:** Every "sci-fi" scenario must be a *real workflow* that exercises multiple operations, crosses SVG boundaries, and produces structurally verifiable GIR — not a round-trip of a string that happens to have a dramatic comment. These are user stories from the future, tested in the present.

---

## Design Principle

Each scenario below follows a strict template:
1. **User story** — A one-paragraph description of what the AI agent is trying to do
2. **UL encoding** — The specific UL structures involved (not just one expression, but a sequence of operations)
3. **Structural assertions** — What properties the GIR must have (not just "it parsed")
4. **Pipeline test** — At least one SVG serialization boundary must be crossed
5. **Verification criterion** — What would prove this scenario "works" for a real agent

The previous tests said "consciousness modeling" and tested `○_x{○{●_x}}`. That proves the parser handles nested variable slots. It doesn't prove UL can model consciousness. The tests below actually *build* complex structures that would be required for each use case.

---

## Scenario A: Federated Knowledge Synthesis

### User Story
Three specialist AI agents — a physics agent, a biology agent, and a philosophy agent — each contribute domain-specific knowledge graphs. A coordinator agent must *merge* these into a unified ontology, detecting structural isomorphisms across domains (e.g., "conservation law in physics ≅ homeostasis in biology ≅ persistence in philosophy").

### Test Sequence

**Step 1: Each agent encodes its domain concept**
```
Physics:    ○_phys{[]{○{●} ↔ ○{●}}}           # conservation: necessarily, two concepts in symmetric exchange
Biology:    ○_bio{○{● | ● | ●} → ○{● | ● | ●}} # homeostasis: many-part system maps to stable many-part system
Philosophy: ○_phil{[]{●_x → ●_x}}                # persistence: necessarily, entity self-relates across time
```

**Step 2: Each serializes to SVG, sends to coordinator**
- Parse each → render SVG → extract GIR
- Assert: each GIR has correct structural properties (necessity modal, enclosure, appropriate operators)

**Step 3: Coordinator detects structural similarity**
- For each pair: `intersect` the two GIR structures (using `&` — intersection operator)
- Assert: intersection is non-trivial (shared structural pattern exists)

**Step 4: Coordinator builds the unified ontology**
- `conjoin` all three domain assertions
- Wrap in `declare{}` (force: this is an ontological commitment)
- Assert: final GIR contains all three substructures
- Assert: final GIR has force = Declare

**Step 5: Serialize unified ontology, send back to all agents**
- Render final → SVG → extract → deparse
- Assert: each agent's original domain concept is recoverable from the conjunction

### Structural Assertions
- `Contains` edges form correct three-branch tree under conjunction
- Modal contexts are preserved per-substructure
- Variable bindings don't leak between domains (phys, bio, phil scopes are independent)
- Force annotation on outer frame is `Declare`

**Test count:** ~8 tests (1 per step + composite assertions)

---

## Scenario B: Autonomous Scientific Hypothesis Generation

### User Story
An AI research agent generates a novel hypothesis by composing known patterns. It starts with an observed phenomenon (data pattern), abstracts it into a structural form, composes it with a known theoretical template, and produces a falsifiable prediction.

### Test Sequence

**Step 1: Encode observed pattern**
```
observation = predicate(○{● | ● | ●}, →, ○{●})
# "A collection of entities produces a single entity" (emergence pattern)
```

**Step 2: Abstract the pattern into a modifier**
```
pattern = abstract(embed(observation))
# Nominalize the observation, then abstract it into a reusable pattern
```
- Assert: result has sort = Modifier

**Step 3: Apply pattern to a new domain**
```
new_domain_entity = parse("○{~ → ~ → ~}")  # "a chain of processes"
hypothesis = modify_entity(pattern, new_domain_entity)
```
- Assert: result has sort = Entity
- Assert: result contains both the abstracted pattern structure and the new domain structure

**Step 4: Generate falsifiable prediction**
```
prediction = predicate(hypothesis, parse("→"), parse("○{●}"))
# "The patterned process-chain produces a single emergent entity"
```
- Assert: result has sort = Assertion

**Step 5: Wrap in evidential hedge and serialize**
```
hedged_prediction = modify_assertion(parse("~?"), prediction)
# "Maybe this is true" (epistemic humility)
```
- Render to SVG → extract → verify structure survives
- Assert: assertion modifier = Hedged
- Assert: full predication structure intact after extraction

### Structural Assertions
- Each intermediate step has the correct output sort
- The `embed → abstract` chain produces a valid modifier from an assertion
- The modifier, when applied, creates a `ModifiedBy` edge in the GIR
- The final structure contains the observation pattern as a recognizable subgraph

**Test count:** ~10 tests

---

## Scenario C: Multi-Agent Formal Debate

### User Story
Two AI agents engage in a structured debate. Agent A asserts a thesis. Agent B queries it, then proposes a counterfactual challenge. Agent A responds with a strengthened claim (necessity). A judge agent evaluates by checking structural coherence.

### Test Sequence

| Turn | Agent | Action | UL Operation Chain |
|---|---|---|---|
| 1 | A | Assert thesis | `assert{○{● → ●} → ○{●}}` → SVG |
| 2 | B | Extract, query it | Extract A's GIR → embed → build `query{embed(A) → ○_challenge}` → SVG |
| 3 | A | Extract challenge, build counterfactual defense | Extract B's GIR → build `assert{[]->{embed(B_challenge)}{negate(embed(B_challenge))}}` → SVG |
| 4 | B | Extract defense, propose alternative | Extract A's defense → build `disjoin(A_defense, query{<>{alternative}})` → SVG |
| 5 | Judge | Extract all, evaluate | Collect all 4 SVGs → extract all → verify structural coherence |

### Structural Assertions per Turn
- **Turn 1:** Force = Assert, predication structure correct
- **Turn 2:** Force = Query, embedded previous assertion as entity, variable slot for challenge
- **Turn 3:** Counterfactual structure has two branches (antecedent, consequent), inner negation has flipped sign
- **Turn 4:** Disjunction has two branches, one with Query force
- **Turn 5:** All 4 structures extractable and composable without error

**Test count:** ~12 tests

---

## Scenario D: Real-Time Swarm Coordination Protocol

### User Story
A swarm of 100 micro-agents must coordinate a distributed task. Each agent has a capability (encoded as a UL concept), a current state (encoded as an assertion), and receives directives from a coordinator. The protocol must handle: broadcast messages, capability matching, task assignment, and completion reporting.

### Test Sequence

**Step 1: Capability advertisement (10 agents)**
```
agent_1_capability = declare{○_cap1{● → ● → ●}}  # "I can do sequential processing"
agent_2_capability = declare{○_cap2{● | ● | ●}}    # "I can do parallel storage"
...
agent_10_capability = declare{○_cap10{~ → ●}}       # "I can transform process to entity"
```
- All 10 → SVG → extract → verify each has force = Declare, unique structure

**Step 2: Coordinator broadcasts task decomposition**
```
task = direct{○{subtask_1 | subtask_2 | ... | subtask_10}}
```
- Where each subtask_i is `○_ti{structural_requirement}`
- Serialize → broadcast → each agent extracts

**Step 3: Capability-task matching**
- For each agent: `intersect(capability_gir, task_requirement_gir)`
- Non-trivial intersection → match → agent accepts
- Assert: at least some intersections are non-trivial

**Step 4: Task acceptance**
```
agent_3_acceptance = commit{embed(subtask_3) → ●_agent3}
```
- Serialize → send to coordinator → extract → verify force = Commit, binding correct

**Step 5: Completion reporting**
```
agent_3_report = assert{subtask_3 → ○{result}}
```
- Coordinator collects all reports → `conjoin` all → wrap in `declare{...}`
- Final SVG contains the complete task result

### Structural Assertions
- 10 distinct capability structures survive independent SVG cycles
- Task decomposition preserves 10 sub-branches under top-level adjacency
- Capability-task intersection produces meaningful structural overlap
- Variable bindings in acceptance messages correctly cross-reference task subtrees
- Final conjunction contains all sub-results as distinct subgraphs

**Test count:** ~20 tests (scaled down from 100 to 10 agents)

---

## Scenario E: Self-Modifying Ontology (Recursive Self-Improvement)

### User Story
An AI agent can represent its own knowledge structure in UL, analyze it for gaps, and produce *new UL expressions* that extend its own ontology. This is UL's deepest promise: a language that can describe its own structure and generate extensions.

### Test Sequence

**Step 1: Encode current knowledge**
```
knowledge_base = ○_kb{
  ○{● → ●} |      # fact 1: some causal relation
  ○{● ↔ ●} |      # fact 2: some symmetric relation
  ○{○{●} → ●}     # fact 3: concept relates to entity
}
```

**Step 2: Encode a meta-observation about the knowledge base**
```
meta_observation = assert{
  embed(knowledge_base) → ○_gap{○}  # "my knowledge base relates to an empty gap"
}
```
- Assert: the embedded knowledge base appears as an entity node
- Assert: the gap `○_gap` is a variable slot (unknown to be filled)

**Step 3: Generate a hypothesis to fill the gap**
```
gap_filler = ○_gap{● ← ● ← ●}  # "the gap is: a reverse causal chain"
```

**Step 4: Bind the hypothesis to the gap**
```
extended_kb = bind(○_gap, conjoin(embed(knowledge_base), embed(predicate(gap_filler, →, knowledge_base))))
```
- Assert: result is an assertion with the gap variable bound
- Assert: the knowledge base now contains the original 3 facts + the new hypothesis

**Step 5: Verify the extension is self-consistent**
- Serialize extended_kb → SVG → extract → verify all original structures present
- Apply `negate` to the extension → verify it produces a distinct structure (extension is non-trivial)

### Structural Assertions
- `embed(knowledge_base)` produces an entity containing all original subgraphs
- Variable binding of `○_gap` creates `Binds` edge in GIR
- Extended knowledge base has strictly more nodes/edges than original
- Original subgraphs are recoverable (no information loss during extension)

**Test count:** ~12 tests

---

## Scenario F: Cross-Species Communication Bridge

### User Story  
An interstellar probe AI encounters a non-human intelligence that communicates in a completely different modality (not language, not visual — pure relational structure). The only common ground is *structural meaning* — the geometric primitives. The probe must:
1. Receive a structure (as raw GIR from the alien system)
2. Verify it's well-formed UL
3. Identify the meaning via structural analysis
4. Construct a response using only UL operations
5. Transmit the response as SVG (geometric visual = universal rendering)

### Test Sequence

**Step 1: Receive alien structure (raw GIR)**
Construct a GIR *programmatically* (not by parsing UL-Script) to simulate receiving structure from a non-UL source:
```typescript
const alienGir = {
  ul_gir: "1.0",
  root: "n1",
  nodes: [
    { id: "n1", type: "enclosure", shape: "circle" },
    { id: "n2", type: "point" },
    { id: "n3", type: "line", directed: true, direction: [1, 0] },
    { id: "n4", type: "point" },
    { id: "n5", type: "enclosure", shape: "circle" },
    { id: "n6", type: "point" },
  ],
  edges: [
    { source: "n1", target: "n2", type: "contains" },
    { source: "n1", target: "n3", type: "contains" },
    { source: "n1", target: "n4", type: "contains" },
    { source: "n2", target: "n3", type: "connects" },
    { source: "n3", target: "n4", type: "connects" },
    { source: "n1", target: "n5", type: "adjacent" },
    { source: "n5", target: "n6", type: "contains" },
  ]
};
```
This encodes: `○{● → ●} | ○{●}` — "a concept with a causal relation, adjacent to another concept with an entity."

**Step 2: Deparse the alien GIR**
```typescript
const ulScript = wasm.deparseGir(JSON.stringify(alienGir));
```
- Assert: produces valid UL-Script
- Assert: re-parsing produces structurally equivalent GIR

**Step 3: Apply structural analysis**
- Count nodes by type → `{point: 3, line: 1, enclosure: 2}`
- Count edges by type → `{contains: 4, connects: 2, adjacent: 1}`
- Identify: this is a predication inside a concept, adjacent to a bare concept

**Step 4: Construct response**
```
response = assert{alienGir → ○{alienGir}}
```
"I assert that your message maps to a concept containing your message" (acknowledgment + embedding)
- Build programmatically: `predicate(embed(alienGir_as_assertion), →, embed(wrap_in_concept(alienGir)))`

**Step 5: Serialize and transmit**
- Render response → SVG → the SVG *is the universal visual rendering*
- Verify SVG contains all geometric primitives corresponding to the algebraic structure
- Verify GIR is extractable from SVG
- Verify extracted GIR deparses to valid UL-Script

### Structural Assertions
- Programmatically-constructed GIR round-trips through deparser
- Alien GIR can be used as input to Σ_UL operations without parsing
- Response GIR contains alien GIR as a subgraph
- SVG rendering produces visual elements for every node in the GIR

**Test count:** ~15 tests

---

## Scenario G: Collective Intelligence Emergence

### User Story
N agents each hold partial knowledge. No single agent has the full picture. By exchanging UL structures and applying compositional operations, the *collective* can derive conclusions that no individual could. This is the "more than the sum of parts" test.

### Test Sequence

**Step 1: Distribute partial knowledge (5 agents)**
```
Agent 1 knows: assert{● → ○_a}          # "entity causes concept A"
Agent 2 knows: assert{○_a → ○_b}         # "concept A causes concept B"
Agent 3 knows: assert{○_b → ○_c}         # "concept B causes concept C"
Agent 4 knows: assert{[]{○_c → ●}}       # "necessarily, concept C causes entity"
Agent 5 knows: query{● → ●}              # "does entity cause entity?" (the question)
```

**Step 2: Chain of deduction via pairwise exchange**
- Agent 1 sends to Agent 2: SVG of assertion 1
- Agent 2 extracts, composes with own knowledge: `conjoin(assertion_1, assertion_2)` → new knowledge
- Agent 2 sends to Agent 3: SVG of conjunction
- Continue chain...

**Step 3: Final agent has the complete chain**
- Agent 5 receives the conjunction of all 4 assertions
- The chain `● → ○_a → ○_b → ○_c → ●` entails `● → ●`
- Agent 5 can verify: the query `● → ●` is structurally implied by the conjunction

### Structural Assertions
- Each pairwise conjunction preserves both substructures
- Variable names (`_a`, `_b`, `_c`) create correct binding chains
- The final conjunction has 4 subgraphs, each identifiable
- The transitive chain is detectable in the GIR edge structure
- No information is lost across the 4 SVG serialization boundaries

**Test count:** ~15 tests

---

## Scenario H: Temporal Reasoning and Prediction

### User Story
An AI agent maintains a temporal model: past states (asserted), present state (asserted with emphasis), and future predictions (hedged or modal). It must update this model as new information arrives, and the temporal structure must be preserved through serialization.

### Test Sequence

**Step 1: Encode temporal model**
```
past    = assert{○_t1{● → ●}}           # "at t1, A caused B" (certain)
present = !{○_t2{● → ● → ●}}            # "at t2, A causes B causes C" (emphatic)
future  = ~?{<>{○_t3{● → ○{●}}}}         # "at t3, maybe possibly A causes a concept" (hedged possibility)
```

**Step 2: Compose temporal model**
```
temporal_model = conjoin(past, conjoin(present, future))
```
- Serialize to SVG → extract

**Step 3: New information arrives — update the model**
```
update = assert{○_t2{● → ● → ● → ●}}  # revised present: chain is longer
```
- Extract existing model from SVG
- Replace present assertion with new one (disjoin old, conjoin new)
- Re-serialize → verify temporal structure preserved

**Step 4: Generate prediction update**
- Based on trend (chain grows: 2→3→4), predict future chain length
- Build: `~?{<>{○_t3{● → ● → ● → ● → ●}}}` (5-chain prediction)
- Verify: hedged possibility, contains 5-chain, correct temporal label

### Structural Assertions
- Assertion modifiers (emphatic, hedged) preserved through conjunction
- Modal operators (possibility) compose correctly with assertion modifiers
- Variable labels (t1, t2, t3) maintain distinct scopes in conjunction
- Temporal update preserves past unchanged while modifying present/future
- Each temporal slice has correct force/modifier combination

**Test count:** ~12 tests

---

## Scenario I: Adversarial Epistemology (AI Lie Detection)

### User Story
Agent A claims to know something. Agent B suspects A is lying. B must use UL's structural analysis to detect inconsistency — if A's claimed knowledge contains a logical contradiction (conjoin of X and negate(X)), B can prove the inconsistency structurally.

### Test Sequence

**Step 1: Agent A makes a claim**
```
claim = assert{conjoin(predicate(●, →, ●), negate(predicate(●, →, ●)))}
```
This is `assert{(A→B) ∧ ¬(A→B)}` — a conjunction containing a statement and its negation.

**Step 2: Agent B extracts and analyzes**
- Extract GIR from A's SVG
- Detect: the conjunction has two branches that are negation-related
- Structural test: find pairs where `negate(branch_i) ≅ branch_j`

**Step 3: Agent B builds proof of inconsistency**
```
proof = assert{embed(claim) → negate(embed(claim))}
# "A's claim implies its own negation"
```
- Serialize proof → SVG → both agents can verify

### Structural Assertions
- The contradictory claim is technically parseable and serializable (UL doesn't prevent contradictions — it represents them)
- The negate-detection algorithm correctly identifies structural negation pairs
- The proof structure embeds the original claim as a nominalized entity
- The self-referential nature of the proof (claim implies its own negation) is representable

**Test count:** ~8 tests

---

## Estimated Total: ~112 sci-fi scenario tests

## Implementation Notes

### What makes these different from the current "sci-fi" tests

| Current Test | Problem | New Test Approach |
|---|---|---|
| `expectRoundTrip("○_x{○{●_x}}")` | Proves parser handles the syntax | Multi-step operation chain with GIR structural verification |
| `expectRoundTrip("assert{● → ●} \| query{● → ●}")` | Proves two force-annotated expressions adjacent | Multi-agent exchange where forces are manipulated across SVG boundaries |
| `expectRoundTrip("!{○_x{●_x → ●}}")` | Proves string survives | Agent builds structure via `modify_assertion` on constructed predication, then crosses SVG boundary |

### Reusable infrastructure needed
- `buildKnowledgeBase(facts: string[])`: Constructs conjunction of parsed expressions
- `agentExchange(sender_gir, receiver_op)`: Serialize→extract→operate pipeline
- `detectSubgraph(gir, subgraph)`: Check if one GIR contains another as sub-structure
- `detectContradiction(gir)`: Find pairs of branches where one is the negation of another
- `girIsomorphic(gir1, gir2)`: Structural equivalence (not string equality)
- `girStrictlyLarger(gir_big, gir_small)`: Verify superstructure containment
