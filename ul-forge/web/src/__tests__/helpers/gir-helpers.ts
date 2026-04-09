/**
 * GIR Test Helpers — Infrastructure for rigorous structural testing.
 *
 * Provides:
 * - GIR isomorphism checking (structural equivalence ignoring node IDs)
 * - GIR structure query helpers (edges by type, nodes by type/sort, containment tree)
 * - Sort inference and verification
 * - Assertion helpers (force, modifier, containment tree invariant, connection chain)
 * - Pipeline helpers (serialize, extract, operate, relay)
 * - Operation chain helpers
 *
 * These are the foundation for plans 01–08 in the testing rigor pass.
 */

// ── Types (mirror core/index.ts) ──

export type Sort = "entity" | "relation" | "modifier" | "assertion";
export type NodeType = "point" | "line" | "angle" | "curve" | "enclosure";
export type EdgeType =
  | "contains"
  | "modified_by"
  | "adjacent"
  | "intersects"
  | "connects"
  | "references"
  | "binds"
  | "accessible_from";

export interface GirNode {
  id: string;
  type: NodeType;
  sort: Sort;
  label?: string;
  shape?: string;
  directed?: boolean;
  direction?: number[];
  measure?: number;
  curvature?: number;
  curvature_profile?: number[];
  vertices?: number;
  variable_id?: string;
  sign?: string;
  force?: string;
  assertion_modifier?: string;
  [key: string]: any;
}

export interface GirEdge {
  source: string;
  target: string;
  type: EdgeType;
}

export interface Gir {
  ul_gir: string;
  root: string;
  nodes: GirNode[];
  edges: GirEdge[];
  metadata?: any;
}

// ══════════════════════════════════════════════════
// GIR ISOMORPHISM CHECKER
// ══════════════════════════════════════════════════

/**
 * Returns a sorted map of node_type → count.
 */
export function nodeTypeDistribution(gir: Gir): Record<string, number> {
  const dist: Record<string, number> = {};
  for (const n of gir.nodes) {
    dist[n.type] = (dist[n.type] || 0) + 1;
  }
  return dist;
}

/**
 * Returns a sorted map of edge_type → count.
 */
export function edgeTypeDistribution(gir: Gir): Record<string, number> {
  const dist: Record<string, number> = {};
  for (const e of gir.edges) {
    dist[e.type] = (dist[e.type] || 0) + 1;
  }
  return dist;
}

/**
 * Returns a sorted map of sort → count.
 */
export function sortDistribution(gir: Gir): Record<string, number> {
  const dist: Record<string, number> = {};
  for (const n of gir.nodes) {
    dist[n.sort] = (dist[n.sort] || 0) + 1;
  }
  return dist;
}

/**
 * Compute a "node signature" — the multiset of (type, sort, edgeTypes incident).
 * Two nodes are matchable only if they share the same signature.
 */
function nodeSignature(node: GirNode, gir: Gir): string {
  const inEdges = gir.edges
    .filter((e) => e.target === node.id)
    .map((e) => `in:${e.type}`)
    .sort();
  const outEdges = gir.edges
    .filter((e) => e.source === node.id)
    .map((e) => `out:${e.type}`)
    .sort();
  const props: string[] = [];
  if (node.directed !== undefined) props.push(`dir:${node.directed}`);
  if (node.direction) props.push(`vec:${node.direction.join(",")}`);
  if (node.shape) props.push(`shape:${node.shape}`);
  if (node.measure !== undefined) props.push(`measure:${node.measure}`);
  if (node.sign) props.push(`sign:${node.sign}`);
  if (node.force) props.push(`force:${node.force}`);
  if (node.assertion_modifier) props.push(`amod:${node.assertion_modifier}`);
  if (node.label && !node.label.startsWith("_implicit")) props.push(`label:${node.label}`);
  return `${node.type}|${node.sort}|${inEdges.join(",")}|${outEdges.join(",")}|${props.join(",")}`;
}

/**
 * Attempt to find a node-ID bijection that preserves all edges and node properties.
 * Uses backtracking search with signature-based pruning.
 * Returns the mapping if found, null otherwise.
 */
export function findIsomorphism(
  gir1: Gir,
  gir2: Gir,
): Record<string, string> | null {
  if (gir1.nodes.length !== gir2.nodes.length) return null;
  if (gir1.edges.length !== gir2.edges.length) return null;

  // Group nodes by signature
  const sigs1: Record<string, string[]> = {};
  const sigs2: Record<string, string[]> = {};
  for (const n of gir1.nodes) {
    const sig = nodeSignature(n, gir1);
    (sigs1[sig] = sigs1[sig] || []).push(n.id);
  }
  for (const n of gir2.nodes) {
    const sig = nodeSignature(n, gir2);
    (sigs2[sig] = sigs2[sig] || []).push(n.id);
  }

  // Check signature distributions match
  const sigKeys1 = Object.keys(sigs1).sort();
  const sigKeys2 = Object.keys(sigs2).sort();
  if (sigKeys1.length !== sigKeys2.length) return null;
  for (let i = 0; i < sigKeys1.length; i++) {
    if (sigKeys1[i] !== sigKeys2[i]) return null;
    if (sigs1[sigKeys1[i]].length !== sigs2[sigKeys2[i]].length) return null;
  }

  // Build candidate mapping: for each node in gir1, which nodes in gir2 could it map to
  const candidates: Record<string, string[]> = {};
  for (const n of gir1.nodes) {
    const sig = nodeSignature(n, gir1);
    candidates[n.id] = sigs2[sig] || [];
  }

  // Edge sets for fast lookup
  const edgeSet2 = new Set(
    gir2.edges.map((e) => `${e.source}|${e.target}|${e.type}`),
  );

  // Backtracking search
  const mapping: Record<string, string> = {};
  const used = new Set<string>();
  const nodeIds1 = gir1.nodes.map((n) => n.id);

  function backtrack(idx: number): boolean {
    if (idx === nodeIds1.length) {
      // Verify all edges are preserved
      for (const e of gir1.edges) {
        const mapped = `${mapping[e.source]}|${mapping[e.target]}|${e.type}`;
        if (!edgeSet2.has(mapped)) return false;
      }
      return true;
    }

    const nid = nodeIds1[idx];
    for (const cand of candidates[nid]) {
      if (used.has(cand)) continue;
      mapping[nid] = cand;
      used.add(cand);
      if (backtrack(idx + 1)) return true;
      used.delete(cand);
      delete mapping[nid];
    }
    return false;
  }

  if (backtrack(0)) return { ...mapping };
  return null;
}

/**
 * Check if two GIR objects are structurally isomorphic.
 * Ignores node IDs but checks: node types, sorts, edge types, topology,
 * direction vectors, shapes, measures, signs, forces, assertion modifiers.
 */
export function girIsomorphic(gir1: Gir, gir2: Gir): boolean {
  return findIsomorphism(gir1, gir2) !== null;
}

/**
 * Assert that two GIR objects are structurally distinct (not isomorphic).
 */
export function girDistinct(gir1: Gir, gir2: Gir): boolean {
  return !girIsomorphic(gir1, gir2);
}

// ══════════════════════════════════════════════════
// GIR STRUCTURE QUERY HELPERS
// ══════════════════════════════════════════════════

/** Find all edges of a specific type. */
export function findEdgesOfType(gir: Gir, edgeType: EdgeType): GirEdge[] {
  return gir.edges.filter((e) => e.type === edgeType);
}

/** Find all nodes of a specific type. */
export function findNodesOfType(gir: Gir, nodeType: NodeType): GirNode[] {
  return gir.nodes.filter((n) => n.type === nodeType);
}

/** Find all nodes with a given sort. */
export function getNodesBySort(gir: Gir, sort: Sort): GirNode[] {
  return gir.nodes.filter((n) => n.sort === sort);
}

/** Find a node by variable_id. */
export function findNodeByVariableId(
  gir: Gir,
  varId: string,
): GirNode | undefined {
  return gir.nodes.find((n) => n.variable_id === varId);
}

/** Find all nodes with a given variable_id. */
export function findNodesByVariableId(gir: Gir, varId: string): GirNode[] {
  return gir.nodes.filter((n) => n.variable_id === varId);
}

/** Get the root node. */
export function getRootNode(gir: Gir): GirNode | undefined {
  return gir.nodes.find((n) => n.id === gir.root);
}

/** Get the sort of the root node. */
export function getRootSort(gir: Gir): Sort | undefined {
  return getRootNode(gir)?.sort;
}

/** Infer sort from node_type (geometric → algebraic mapping). */
export function inferSort(node: GirNode): Sort {
  switch (node.type) {
    case "point":
      return "entity";
    case "line":
      return "relation";
    case "angle":
      return "modifier";
    case "curve":
      return "relation";
    case "enclosure":
      // Enclosures can be entity or assertion depending on context
      return node.sort;
    default:
      return node.sort;
  }
}

// ══════════════════════════════════════════════════
// CONTAINMENT TREE HELPERS
// ══════════════════════════════════════════════════

/**
 * Extract the containment tree from Contains edges.
 * Returns parent→children mapping.
 */
export function getContainmentTree(
  gir: Gir,
): Record<string, string[]> {
  const tree: Record<string, string[]> = {};
  for (const e of findEdgesOfType(gir, "contains")) {
    (tree[e.source] = tree[e.source] || []).push(e.target);
  }
  return tree;
}

/**
 * Get the parent of a node in the containment tree (null if root).
 */
export function getContainmentParent(
  gir: Gir,
  nodeId: string,
): string | null {
  const edge = gir.edges.find(
    (e) => e.type === "contains" && e.target === nodeId,
  );
  return edge ? edge.source : null;
}

/**
 * Get the depth of a node in the containment tree.
 */
export function getContainmentDepth(gir: Gir, nodeId: string): number {
  let depth = 0;
  let current = nodeId;
  while (true) {
    const parent = getContainmentParent(gir, current);
    if (!parent) break;
    depth++;
    current = parent;
  }
  return depth;
}

/**
 * Validate that Contains edges form a tree (no cycles, single parent per child).
 * Returns { valid, errors }.
 */
export function assertContainmentIsTree(
  gir: Gir,
): { valid: boolean; errors: string[] } {
  const errors: string[] = [];
  const containsEdges = findEdgesOfType(gir, "contains");

  // Check single parent per child
  const childParents: Record<string, string[]> = {};
  for (const e of containsEdges) {
    (childParents[e.target] = childParents[e.target] || []).push(e.source);
  }
  for (const [child, parents] of Object.entries(childParents)) {
    if (parents.length > 1) {
      errors.push(
        `Node ${child} has ${parents.length} Contains parents: ${parents.join(", ")}`,
      );
    }
  }

  // Check no cycles via DFS
  const tree = getContainmentTree(gir);
  const visited = new Set<string>();
  const inStack = new Set<string>();

  function dfs(nodeId: string): boolean {
    if (inStack.has(nodeId)) {
      errors.push(`Cycle detected involving node ${nodeId}`);
      return false;
    }
    if (visited.has(nodeId)) return true;
    visited.add(nodeId);
    inStack.add(nodeId);
    for (const child of tree[nodeId] || []) {
      if (!dfs(child)) return false;
    }
    inStack.delete(nodeId);
    return true;
  }

  // Start DFS from all nodes that are Contains sources
  for (const nodeId of Object.keys(tree)) {
    dfs(nodeId);
  }

  return { valid: errors.length === 0, errors };
}

// ══════════════════════════════════════════════════
// CONNECTION CHAIN HELPERS
// ══════════════════════════════════════════════════

/**
 * Extract connection chains (sequences of Connects edges).
 * Validates alternating point↔line pattern.
 * Returns { valid, errors, chains }.
 */
export function assertConnectionChainValid(
  gir: Gir,
): { valid: boolean; errors: string[] } {
  const errors: string[] = [];
  const connectEdges = findEdgesOfType(gir, "connects");
  const nodeMap = new Map(gir.nodes.map((n) => [n.id, n]));

  for (const e of connectEdges) {
    const src = nodeMap.get(e.source);
    const tgt = nodeMap.get(e.target);
    if (!src || !tgt) {
      errors.push(
        `Connects edge references non-existent node: ${e.source} → ${e.target}`,
      );
      continue;
    }

    const srcIsPoint = src.type === "point";
    const srcIsLine = src.type === "line" || src.type === "curve";
    const tgtIsPoint = tgt.type === "point";
    const tgtIsLine = tgt.type === "line" || tgt.type === "curve";

    if (srcIsPoint && tgtIsLine) continue; // point → line: valid
    if (srcIsLine && tgtIsPoint) continue; // line → point: valid
    if (srcIsPoint && tgtIsPoint) {
      errors.push(
        `Invalid Connects: point(${e.source}) → point(${e.target}) — missing intervening line`,
      );
    } else if (srcIsLine && tgtIsLine) {
      errors.push(
        `Invalid Connects: line(${e.source}) → line(${e.target}) — cannot connect lines directly`,
      );
    } else {
      errors.push(
        `Unexpected Connects: ${src.type}(${e.source}) → ${tgt.type}(${e.target})`,
      );
    }
  }

  return { valid: errors.length === 0, errors };
}

// ══════════════════════════════════════════════════
// ASSERTION HELPERS
// ══════════════════════════════════════════════════

/**
 * Get the force annotation from the GIR (looks at root and ancestor nodes).
 */
export function getForce(gir: Gir): string | undefined {
  // Check root node first
  const root = getRootNode(gir);
  if (root?.force) return root.force;
  // Check all enclosure nodes for force
  for (const n of gir.nodes) {
    if (n.force) return n.force;
  }
  return undefined;
}

/**
 * Get the assertion modifier from the GIR.
 */
export function getAssertionModifier(gir: Gir): string | undefined {
  for (const n of gir.nodes) {
    if (n.assertion_modifier) return n.assertion_modifier;
  }
  return undefined;
}

/**
 * Get the sign (⊕/⊖) from the GIR root assertion.
 */
export function getSign(gir: Gir): string | undefined {
  const root = getRootNode(gir);
  if (root?.sign) return root.sign;
  for (const n of gir.nodes) {
    if (n.sign) return n.sign;
  }
  return undefined;
}

// ══════════════════════════════════════════════════
// PIPELINE HELPERS
// ══════════════════════════════════════════════════

/**
 * Create pipeline helpers bound to a WASM module instance.
 * Call this in beforeAll after WASM init.
 */
export function createPipelineHelpers(wasm: any) {
  /** Parse UL-Script to GIR JSON string. */
  function p(input: string): string {
    return wasm.parseUlScript(input) as string;
  }

  /** Parse UL-Script to GIR object. */
  function parse(input: string): Gir {
    return JSON.parse(p(input)) as Gir;
  }

  /** Apply a Σ_UL operation. Returns GIR JSON string. */
  function apply(op: string, ...girJsons: string[]): string {
    return wasm.applyOperation(op, JSON.stringify(girJsons)) as string;
  }

  /** Apply operation and return parsed GIR. */
  function applyParsed(op: string, ...girJsons: string[]): Gir {
    return JSON.parse(apply(op, ...girJsons)) as Gir;
  }

  /** Build an assertion GIR via predicate(entity, relation, entity). */
  function makeAssertion(
    subject = "●",
    relation = "→",
    object = "●",
  ): string {
    return apply("predicate", p(subject), p(relation), p(object));
  }

  /** Render GIR to SVG. */
  function renderSvg(girJson: string, w = 256, h = 256): string {
    return wasm.renderSvg(girJson, w, h) as string;
  }

  /** Extract GIR JSON from SVG CDATA. */
  function extractGirFromSvg(svg: string): string {
    const m = svg.match(/<!\[CDATA\[([\s\S]*?)\]\]>/);
    if (!m)
      throw new Error("No GIR embedded in SVG CDATA");
    return m[1];
  }

  /** Full serialize→extract pipeline. Returns GIR JSON string. */
  function agentSerialize(girJson: string): string {
    const svg = renderSvg(girJson);
    return extractGirFromSvg(svg);
  }

  /** Full serialize→extract pipeline. Returns parsed GIR. */
  function agentSerializeParsed(girJson: string): Gir {
    return JSON.parse(agentSerialize(girJson)) as Gir;
  }

  /** Multi-step pipeline: apply operations across SVG boundaries. */
  function agentRelay(
    initialGirJson: string,
    steps: Array<{ op: string; extraArgs?: string[] }>,
  ): { intermediates: Gir[]; final: Gir } {
    const intermediates: Gir[] = [];
    let current = initialGirJson;

    for (const step of steps) {
      // Serialize to SVG and extract
      current = agentSerialize(current);
      intermediates.push(JSON.parse(current) as Gir);
      // Apply operation
      const args = [current, ...(step.extraArgs || [])];
      current = apply(step.op, ...args);
    }

    return {
      intermediates,
      final: JSON.parse(current) as Gir,
    };
  }

  /** Deparse GIR JSON to UL-Script string. */
  function deparse(girJson: string): string {
    return wasm.deparseGir(girJson) as string;
  }

  /** Full round-trip: UL-Script → GIR → SVG → extract → deparse. */
  function roundTrip(input: string): string {
    const girJson = p(input);
    const svg = renderSvg(girJson);
    const extracted = extractGirFromSvg(svg);
    return deparse(extracted);
  }

  /** Validate GIR via WASM 4-layer validator. */
  function validate(girJson: string, checkGeometry = false): any {
    return wasm.validateGir(girJson, checkGeometry);
  }

  /** Compare two GIRs via WASM compareGir. */
  function compare(
    girJson1: string,
    girJson2: string,
    level = "topological",
  ): any {
    return wasm.compareGir(girJson1, girJson2, level);
  }

  /**
   * Chain operations sequentially.
   * Returns all intermediate GIR objects and the final result.
   */
  function chainOps(
    steps: Array<{ op: string; args: string[] }>,
  ): { intermediates: Gir[]; final: Gir } {
    const intermediates: Gir[] = [];
    let result: string = steps[0].args[0]; // Start from first arg

    for (const step of steps) {
      result = apply(step.op, ...step.args);
      intermediates.push(JSON.parse(result) as Gir);
    }

    return {
      intermediates,
      final: intermediates[intermediates.length - 1],
    };
  }

  return {
    p,
    parse,
    apply,
    applyParsed,
    makeAssertion,
    renderSvg,
    extractGirFromSvg,
    agentSerialize,
    agentSerializeParsed,
    agentRelay,
    deparse,
    roundTrip,
    validate,
    compare,
    chainOps,
  };
}
