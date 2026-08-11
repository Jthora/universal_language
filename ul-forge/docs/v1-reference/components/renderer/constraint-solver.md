# Constraint Solver Specification

> The system that computes geometrically valid positions for composed glyphs that don't match templates.

---

## Overview

When a glyph is too novel for template matching, the constraint solver computes node positions by satisfying geometric constraints derived from the GIR's edge types and node properties.

**Input:** GIR graph + bounding boxes (from top-down allocation)  
**Output:** (x, y, scale, rotation) for each node

---

## Constraint Types

### Hard constraints (must be satisfied)

| Constraint | Source | Formula |
|-----------|--------|---------|
| **Containment** | `contains` edge | `child.bbox ⊂ parent.bbox` |
| **Angle measure** | `angle` node | `angle(line_a, line_b) = node.measure ± ε` |
| **Direction** | `line` node | `atan2(end.y - start.y, end.x - start.x) = direction_angle ± ε` |
| **Non-overlap** | Absence of `intersects` edge | `bbox(a) ∩ bbox(b) = ∅` for non-intersecting siblings |
| **Overlap** | `intersects` edge | `area(bbox(a) ∩ bbox(b)) > 0` for intersecting marks |

### Soft constraints (optimized, not required)

| Constraint | Priority | Formula |
|-----------|----------|---------|
| **Centering** | High | Children centered in parent's bbox |
| **Equal spacing** | Medium | Siblings equally spaced within parent |
| **Aspect ratio** | Medium | Enclosures maintain natural shape (circles are round, not elliptical) |
| **Compactness** | Low | Minimize total bounding box area |
| **Balance** | Low | Visual center of mass near geometric center |

---

## Solver Architecture

```
Input: nodes[], edges[], bounding_boxes[]

Step 1: Generate constraint set
  for each edge:
    hard_constraints.add(constraint_from_edge(edge))
  for each node:
    hard_constraints.add(constraint_from_node(node))
  soft_constraints = default_aesthetic_constraints()

Step 2: Initial layout (heuristic)
  positions = heuristic_placement(nodes, bounding_boxes)

Step 3: Iterative refinement
  for iteration in 0..MAX_ITERATIONS:
    violation = compute_violation(positions, hard_constraints)
    if violation < EPSILON:
        break
    gradient = compute_gradient(positions, hard_constraints + soft_constraints)
    positions = positions - STEP_SIZE * gradient

Step 4: Verify
  assert all hard constraints satisfied within tolerance
  return positions
```

---

## Heuristic Initial Placement

Good initial placement drastically reduces solver iterations:

```
function heuristic_placement(nodes, bboxes):
    // Place enclosures centered in their bboxes
    for node in nodes where node.type == "enclosure":
        node.pos = node.bbox.center()
        node.radius = min(node.bbox.width, node.bbox.height) * 0.4
    
    // Place points at the center of their parent's interior
    for node in nodes where node.type == "point":
        parent = tree_parent(node)
        node.pos = parent.pos  // center of parent enclosure
    
    // Place lines between their connected endpoints
    for node in nodes where node.type == "line":
        src = connected_source(node)
        dst = connected_target(node)
        node.start = src.pos
        node.end = src.pos + node.direction * distance(src.pos, dst.pos)
    
    // Place angles at the vertex of their parent lines
    for node in nodes where node.type == "angle":
        parent_line = tree_parent(node)
        node.pos = parent_line.start  // vertex position
```

---

## Constraint Formulation

Each constraint is expressed as a differentiable function that returns 0 when satisfied:

```
containment_constraint(child, parent):
    return max(0, distance(child.pos, parent.pos) + child.radius - parent.radius)

angle_constraint(angle_node, line_a, line_b):
    actual = angle_between(line_a.direction, line_b.direction)
    return (actual - angle_node.measure)²

non_overlap_constraint(a, b):
    overlap = bbox_overlap_area(a.bbox, b.bbox)
    return overlap²  // penalty for unexpected overlap

overlap_constraint(a, b):
    overlap = bbox_overlap_area(a.bbox, b.bbox)
    return max(0, MIN_OVERLAP - overlap)²  // penalty for insufficient overlap
```

---

## Solver Choice

For v1, use a simple gradient descent solver. It's adequate because:
- Individual glyphs have few nodes (typically < 20)
- Hard constraints are mostly geometric (differentiable)
- The heuristic placement starts close to the solution

**Future (if needed):**
- **Cassowary:** Linear constraint solver. Fast for bounding box constraints. Used by Apple Auto Layout.
- **SLSQP:** Sequential Least Squares Programming. Handles nonlinear constraints (angles, curvatures).
- **Penrose-style:** Declarative constraint specification with automatic differentiation.

---

## Tolerance

Geometric constraints are satisfied within tolerance:
- **Position:** ε = 0.5 pixels at default scale
- **Angle:** ε = 0.5 degrees
- **Containment:** ε = 1 pixel margin

If hard constraints cannot be satisfied within tolerance (e.g., too many marks in too small a space), the solver reports a `LayoutWarning` with the unsatisfied constraints and proceeds with the best approximate layout.

---

## Performance Budget

| Glyph size | Max solver time | Typical |
|-----------|----------------|---------|
| Simple (1-5 nodes) | 1ms | < 0.1ms (template hit) |
| Medium (5-20 nodes) | 10ms | 2-5ms |
| Complex (20-50 nodes) | 50ms | 10-30ms |
| Very large (50-100 nodes) | 200ms | 50-100ms |

These budgets assume Level 2 constraint solving. Level 1 (templates) is effectively instant.
