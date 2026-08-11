# Theorem Prover Interface

> Formal verification of UL glyph properties using SAT/SMT solvers.

---

## Use Case

The validator checks structural and sort constraints. But some properties of UL glyphs require **formal reasoning**:

1. **Is this glyph a valid instance of a Σ_UL operation?** — Verify that a glyph's structure matches one of the 13 operation signatures
2. **Are two glyphs semantically equivalent?** — Prove that two different GIR representations encode the same meaning
3. **Does this composition preserve sort correctness?** — When combining glyphs, prove the result satisfies all sort constraints
4. **Is this template exhaustive?** — Prove that the template library covers all possible Tier-1 glyphs

---

## Approach: SMT Encoding

Encode GIR constraints as SMT-LIB formulas and solve with Z3.

### Sort Constraints as SMT

```smt-lib
; Sorts
(declare-datatypes () ((Sort Entity Relation Modifier Assertion)))

; Node sort assignment
(declare-fun node_sort (Int) Sort)

; Edge constraint: 'connects' requires source and target to be Entity or Relation
(assert (forall ((e Int))
  (=> (is-connects-edge e)
      (and (or (= (node_sort (edge-source e)) Entity)
               (= (node_sort (edge-source e)) Relation))
           (or (= (node_sort (edge-target e)) Entity)
               (= (node_sort (edge-target e)) Relation))))))

; Predicate operation: Entity × Relation × Entity → Assertion
(assert (forall ((s Int) (r Int) (o Int))
  (=> (is-predicate s r o)
      (and (= (node_sort s) Entity)
           (= (node_sort r) Relation)
           (= (node_sort o) Entity)))))
```

### Geometric Realizability as SMT

```smt-lib
; Each node has a position
(declare-fun x (Int) Real)
(declare-fun y (Int) Real)
(declare-fun radius (Int) Real)

; Containment: child is inside parent
(assert (forall ((p Int) (c Int))
  (=> (contains p c)
      (< (distance (x c) (y c) (x p) (y p))
         (radius p)))))

; Non-overlap of siblings
(assert (forall ((a Int) (b Int))
  (=> (and (siblings a b) (not (= a b)))
      (> (distance (x a) (y a) (x b) (y b))
         (+ (radius a) (radius b))))))
```

---

## Verification Tasks

### 1. Operation Signature Verification

Given a GIR subgraph, prove it matches one of the 13 Σ_UL operation signatures.

```
Input: GIR subgraph
Output: "This is a predicate(e, r, e) → a" or "No matching operation"
```

### 2. Equivalence Checking

Given two GIR documents, prove they are isomorphic (same graph structure up to node relabeling).

```
Input: GIR_A, GIR_B
Output: "Isomorphic" (with mapping) or "Not isomorphic"
```

This is the graph isomorphism problem — NP for general graphs but tractable for the small graphs UL produces (typically < 50 nodes). Use canonical labeling (nauty/bliss algorithms).

### 3. Composability Proof

Given two glyphs intended to be composed, prove the composition is sort-correct.

```
Input: GIR_A, GIR_B, composition_edge
Output: "Valid composition" or "Sort violation at edge E"
```

---

## Implementation

### Python (for Jupyter/research)

```python
from z3 import *

def verify_predicate(subject_sort, relation_sort, object_sort):
    s, r, o = Consts('s r o', SortEnum('Sort', ['Entity', 'Relation', 'Modifier', 'Assertion']))
    solver = Solver()
    solver.add(s == subject_sort, r == relation_sort, o == object_sort)
    solver.add(s == Entity, r == Relation, o == Entity)
    return solver.check() == sat
```

### Rust (for core library, optional)

Use the `z3` Rust bindings or a lighter-weight SAT solver for real-time constraint checking.

---

## Scope in v1

Theorem proving is Phase 4 and the lowest-priority AI interface. The validator (Phases 1) handles constraint checking procedurally. The theorem prover adds:
- Formal proofs (not just pass/fail)
- Equivalence checking (validator cannot do this)
- Exhaustiveness proofs for the template library

The SMT encoding is straightforward once the sort system is finalized. Z3 is the recommended solver.
