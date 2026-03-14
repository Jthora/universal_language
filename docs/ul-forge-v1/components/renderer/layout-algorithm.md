# Layout Algorithm

> How UL Forge computes 2D placement of geometric primitives from a GIR graph.

---

## Why Layout Is Hard

Given a GIR with N nodes and E edges, compute (x, y, scale, rotation) for each node such that:
- **Containment** is visually correct: children are inside their parent enclosure
- **Intersection** produces visible overlap: intersecting marks share visual space
- **Adjacency** shows shared boundaries: adjacent marks touch
- **Angles are metrically correct:** a 60° angle looks like 60°
- **The result is readable:** marks don't collide accidentally, labels are visible, whitespace is distributed evenly

General graph layout is NP-hard. But UL layout is more constrained than arbitrary graph drawing — and those constraints help.

---

## Key Insight: UL Is Not Arbitrary

UL glyphs are composed of exactly 5 geometric primitives with well-defined spatial semantics. This means:
1. The containment tree provides a natural hierarchical structure for top-down layout
2. Individual nodes have few internal sub-components (typically 3-7)
3. Angle and direction constraints are exact (not soft preferences)
4. The template library handles the most common cases (42 canonical glyphs)

This is closer to **molecular structure rendering** or **circuit layout** than to arbitrary force-directed graph drawing.

---

## Three-Level Approach

### Level 1: Template Lookup (MVP)

Check if the GIR matches a known template. If yes, return pre-computed positions.

```
function layout_level1(gir):
    template_id = match_template(gir)
    if template_id is not None:
        return template_library[template_id].positions
    else:
        return layout_level2(gir)
```

The 42 canonical lexicon entries have fixed visual forms. Additionally, common compositions (e.g., "point in circle" = existence) have templates. This covers the vast majority of glyphs users will create initially.

**Template matching:** Compare the GIR's graph structure (ignoring node IDs) against template graphs using graph isomorphism. For small graphs (<10 nodes), this is fast (sub-millisecond).

### Level 2: Hierarchical Constraint Solver

For novel compositions not in the template library:

```
function layout_level2(gir):
    tree = extract_tree_spine(gir)
    
    // Phase A: Allocate bounding boxes top-down
    root_bbox = BoundingBox(0, 0, canvas_width, canvas_height)
    allocate_boxes(tree.root, root_bbox, tree)
    
    // Phase B: Solve constraints within each box
    for each node in tree (bottom-up):
        solve_local_constraints(node, node.bbox, node.children)
    
    // Phase C: Route cross-edges
    cross_edges = gir.edges - tree.edges
    for each edge in cross_edges:
        route_edge(edge, positions)
    
    return positions
```

#### Phase A: Bounding Box Allocation

Top-down traversal of the tree spine. Each node gets a rectangular bounding box within its parent's box.

```
function allocate_boxes(node, parent_bbox, tree):
    children = tree.children_of(node)
    if children.is_empty():
        node.bbox = parent_bbox  // Leaf takes full parent space
        return
    
    // Compute child sizes proportional to their subtree complexity
    weights = children.map(c => subtree_weight(c))
    child_boxes = partition_box(parent_bbox, weights, padding=10%)
    
    for (child, box) in zip(children, child_boxes):
        child.bbox = box
        allocate_boxes(child, box, tree)
```

**Box partitioning strategies:**
- **Horizontal strip:** Children laid out left to right (default for adjacency)
- **Vertical strip:** Top to bottom (for hierarchical depth)
- **Grid:** For more than 4 children
- **Circular:** Children around the parent's perimeter (for radial compositions)

The strategy is chosen based on the edge types between siblings:
- Adjacency (`|`) → horizontal strip (marks touch left-right)
- Connection (`→`) → horizontal strip with arrow routing
- No operator → grid packing

#### Phase B: Local Constraint Solving

Within each bounding box, place the node and its direct decorations (modifiers) using geometric constraints:

```
function solve_local_constraints(node, bbox, children):
    match node.type:
        "enclosure" →
            // Place enclosure centered in bbox
            center = bbox.center()
            radius = min(bbox.width, bbox.height) / 2 * 0.9
            node.position = center
            node.size = radius
            
            // Place children inside the enclosure
            for child in children:
                place_inside_enclosure(child, center, radius)
        
        "point" →
            // Point at center of bbox
            node.position = bbox.center()
        
        "line" →
            // Line from bbox edge to edge, at specified direction
            start = bbox.center() - node.direction * bbox.width/2
            end = bbox.center() + node.direction * bbox.width/2
            node.start = start
            node.end = end
        
        "angle" →
            // Arc between parent's two child lines
            // Exact angle measure determines arc extent
            ...
```

For complex constraint configurations (e.g., three lines meeting at specific angles), use a small-scale constraint solver:

```
Constraints:
  line_1.angle = 0°
  line_2.angle = 60°  (relative to line_1)
  line_3.angle = 120° (relative to line_1)
  all lines share vertex at point P
  all lines within bbox

Solve: minimize violation of geometric constraints
  using: sequential least-squares programming (SLSQP)
  or: Cassowary constraint solver (for linear constraints)
```

#### Phase C: Cross-Edge Routing

After all nodes are positioned, route the non-tree edges:

```
function route_edge(edge, positions):
    source_pos = positions[edge.source]
    target_pos = positions[edge.target]
    
    match edge.type:
        "intersects" →
            // Visually overlap: adjust bounding boxes so marks share space
            overlap_region = compute_overlap(source_pos, target_pos)
            adjust_positions_for_overlap(edge.source, edge.target, overlap_region)
        
        "adjacent" →
            // Move marks so boundaries touch
            adjust_to_touch(source_pos, target_pos)
        
        "connects" →
            // Draw Bézier curve between marks, avoiding other marks
            control_points = route_avoiding_obstacles(source_pos, target_pos, all_positions)
            edge.path = bezier(source_pos, control_points, target_pos)
        
        "references" →
            // Dashed line (or nothing if invisible)
            edge.path = straight_line(source_pos, target_pos)
```

### Level 3: Aesthetic Refinement

Post-layout polish pass:

```
function refine(positions, gir):
    // Force-directed nudge: push overlapping non-intersecting marks apart
    for iteration in 0..50:
        for (a, b) in all_node_pairs:
            if not gir.has_edge(a, b, "intersects"):
                if bboxes_overlap(a, b):
                    repel(a, b, strength=0.1)
    
    // Whitespace equalization
    for each group of siblings:
        equalize_spacing(siblings)
    
    // Label placement
    for each label:
        place_label_in_nearest_whitespace(label, positions)
```

---

## Complexity Analysis

| Phase | Time complexity | Typical size | Practical time |
|-------|----------------|-------------|----------------|
| Template lookup | O(N²) for isomorphism | N < 10 | < 1ms |
| Box allocation | O(N) | N < 100 | < 1ms |
| Local constraints | O(N × C) where C = constraints per node | C < 10 | < 10ms |
| Cross-edge routing | O(E × N) for obstacle avoidance | E < 50 | < 10ms |
| Aesthetic refinement | O(N² × I) where I = iterations | I = 50 | < 50ms |
| **Total** | | | **< 100ms typical** |

For interactive live preview, 100ms is acceptable (10 FPS). For static rendering, there is no time constraint.

---

## Comparable Systems

| System | Layout approach | Size | UL relevance |
|--------|----------------|------|-------------|
| **Graphviz `dot`** | Hierarchical (Sugiyama) | ~80k lines C | Tree-spine layout is a subset |
| **Penrose** | Constraint optimization | ~30k lines | Closest analog for constraint-based diagramming |
| **D3.js** | Force-directed, treemap | ~15k lines | Useful for packing and force nudging |
| **MathJax** | Box model (TeX algorithm) | ~60k lines | Box allocation approach is similar |
| **KaTeX** | Simplified box model | ~20k lines | Faster MathJax — similar scope |

UL Forge's layout is less general than Graphviz but more constrained than Penrose. Realistic estimate: **5-15k lines of Rust** for Level 1+2.
