# 04 — Agent Workflow Pipeline Tests

> **Goal:** Test the *complete agent-to-agent pipeline* end-to-end. Not round-trip of a string. Not one operation. The full real workflow: Agent A builds meaning via operations → serializes to SVG → transmits → Agent B extracts GIR → manipulates → serializes → sends back → Agent A verifies nothing was lost.

---

## Problem Statement

No existing test simulates what agents will actually do. The AGENTS.md describes UL as "trade currency for machine intelligence" — but no test demonstrates a trade. Every test either:
- Round-trips a string (proves parser/deparser, not agent workflow)
- Calls one operation (proves the function works in isolation)

The gap: **no test chains operations across the SVG serialization boundary**. If Agent A builds `predicate(●, →, ●)`, renders to SVG, and Agent B extracts the GIR and applies `negate()` — does the negation work on the *extracted* GIR? Does the result re-serialize correctly? This is untested.

---

## Test Categories

### 4.1 — Operation-Across-SVG-Boundary

Agent A produces a GIR via operation. Serializes to SVG. Agent B extracts and applies another operation.

| Step | Agent A Action | SVG Boundary | Agent B Action | Verification |
|---|---|---|---|---|
| 1 | `predicate(●, →, ●)` | → SVG → extract | `negate()` | Result is negated assertion |
| 2 | `predicate(●, →, ●)` | → SVG → extract | `embed()` | Result is entity (nominalized) |
| 3 | `modify_entity(∠45, ●)` | → SVG → extract | Use in `predicate()` | Valid assertion with modified entity |
| 4 | `compose(→, →)` | → SVG → extract | `invert()` | Reversed composed relation |
| 5 | `conjoin(a₁, a₂)` | → SVG → extract | `negate()` | De Morgan: should ≅ `disjoin(¬a₁, ¬a₂)` |
| 6 | `quantify(∠45, ●)` | → SVG → extract | `bind(○_x, ...)` | Bound quantified assertion |
| 7 | `abstract(●)` | → SVG → extract | `modify_relation(result, →)` | Modified relation |
| 8 | Full chain: A builds, B extends, A extends again | 2 SVG boundaries | C reads final | 3-agent relay |

**Test count:** ~25 cross-boundary tests

### 4.2 — Multi-Agent Publish-Subscribe Simulation

Agent A publishes an SVG. Agents B, C, D each independently extract and operate on it. All results must be consistent.

| Test | Setup | Assertion |
|---|---|---|
| Parallel extraction | A publishes `○{● → ●}` as SVG. B, C, D each extract. | All three get identical GIR |
| Divergent operations | B applies `negate`, C applies `embed`, D applies `modify_assertion` | Each produces valid but different GIR |
| Convergent merge | B and C each build assertions, D `conjoin`s them | Conjunction of separately-built structures is valid |
| Conflict detection | B negates, C doesn't. Compare. | Structures are provably distinct (not isomorphic) |

**Test count:** ~15 publish-subscribe tests

### 4.3 — Incremental Build Across Serialization Boundaries

An agent builds a complex structure step by step, serializing to SVG after each step (simulating persistent storage or network transmission).

| Step | Action | SVG Round-Trip | Running Structure |
|---|---|---|---|
| 1 | Parse `●` | Serialize → extract | Entity |
| 2 | `predicate(result, →, ●)` | Serialize → extract | Assertion |
| 3 | `embed(result)` | Serialize → extract | Entity (nominalized assertion) |
| 4 | `predicate(result, →, ○{●})` | Serialize → extract | Assertion with embedded nominalization |
| 5 | `negate(result)` | Serialize → extract | Negated complex assertion |
| 6 | `modify_assertion(∠45, result)` | Serialize → extract | Modified negated assertion |
| 7 | Compare final GIR built incrementally vs built in one shot | — | Must be structurally isomorphic |

**Variants:**
- 5-step incremental build
- 10-step incremental build  
- Build same structure incrementally vs all-at-once, verify isomorphism
- Interruption test: build 3 steps, discard, rebuild from step-2 SVG

**Test count:** ~20 incremental tests

### 4.4 — Message Protocol Simulation

Two agents have a "conversation" where each message is a force-annotated UL expression serialized as SVG.

| Turn | Agent | Action | Expression |
|---|---|---|---|
| 1 | A | Asserts a fact | `assert{● → ●}` → SVG |
| 2 | B | Extracts, queries about it | Extract A's GIR, build `query{embed(A_gir) → ○_x}` → SVG |
| 3 | A | Extracts query, responds with commitment | Extract B's GIR, build `commit{● → ●_x}` → SVG |
| 4 | B | Verifies commitment matches query | Extract, check structural compatibility |
| 5 | A | Declares resolution | `declare{conjoin(original_fact, commitment)}` → SVG |

**Verification at each step:**
- SVG extraction preserves all structural properties
- Force annotations survive serialization
- Variable bindings cross-reference correctly after extraction
- Final structure contains all intermediate structures as subgraphs

**Test count:** ~15 protocol tests

### 4.5 — Hostile Agent Simulation (Robustness)

What happens when an agent sends malformed or adversarial SVGs?

| Test | Hostile Action | Expected Behavior |
|---|---|---|
| Truncated SVG | Remove CDATA section | Extraction fails cleanly (no crash) |
| Corrupted GIR JSON | Modify JSON in CDATA | Parse error on extraction |
| Valid SVG, wrong GIR | Swap GIR for a different expression | Extraction succeeds but structure doesn't match visual |
| Extra nodes injected | Add spurious nodes to GIR JSON | Operations still work on corrupted graph? |
| Missing edges | Remove edges from GIR JSON | Deparse produces degraded but parseable output? |
| Sort field tampered | Change entity sort to "assertion" | Operations should detect sort mismatch |
| Extremely large GIR | 10,000 nodes in CDATA | Extraction doesn't hang or OOM |

**Test count:** ~15 robustness tests

---

## Estimated Total: ~90 agent pipeline tests

## Implementation Approach

Core helper: `agentPipeline` — simulates the full agent workflow:

```typescript
function agentSerialize(gir: any): string {
  const svg = wasm.renderSvg(JSON.stringify(gir), 256, 256);
  return svg;
}

function agentExtract(svg: string): any {
  const m = svg.match(/<!\[CDATA\[([\s\S]*?)\]\]>/);
  if (!m) throw new Error("No GIR in SVG");
  return JSON.parse(m[1]);
}

function agentOperate(gir: any, op: string, ...extraGirs: any[]): any {
  const args = [JSON.stringify(gir), ...extraGirs.map(g => JSON.stringify(g))];
  return JSON.parse(wasm.applyOperation(op, JSON.stringify(args)));
}

// Full pipeline: build → serialize → extract → operate → serialize → extract → verify
function fullRelay(buildOps: Step[], verifyFn: (gir: any) => void) { ... }
```

Key design principle: **Every test crosses at least one SVG serialization boundary.** If a test doesn't convert to SVG and back, it belongs in 01 or 02, not here.
