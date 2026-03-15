/**
 * GIR test fixtures — canonical structures for testing.
 *
 * These mirror the UL formal system primitives and operations.
 */

import type { Gir, Node, Edge } from "../../core";

/** A minimal valid GIR with a single point (Entity). */
export const SINGLE_POINT: Gir = {
  ul_gir: "0.2",
  root: "p",
  nodes: [{ id: "p", type: "point", sort: "entity" }],
  edges: [],
};

/** Two points connected by a directed line (predicate pattern). */
export const PREDICATE_GIR: Gir = {
  ul_gir: "0.2",
  root: "l",
  nodes: [
    { id: "p1", type: "point", sort: "entity" },
    { id: "l", type: "line", sort: "relation", directed: true },
    { id: "p2", type: "point", sort: "entity" },
  ],
  edges: [
    { source: "l", target: "p1", type: "connects" },
    { source: "l", target: "p2", type: "connects" },
  ],
};

/** An enclosure containing a point (embed pattern). */
export const ENCLOSURE_GIR: Gir = {
  ul_gir: "0.2",
  root: "enc",
  nodes: [
    { id: "enc", type: "enclosure", sort: "assertion", shape: "circle" },
    { id: "p", type: "point", sort: "entity" },
  ],
  edges: [{ source: "enc", target: "p", type: "contains" }],
};

/** A point modified by an angle (modify_entity pattern). */
export const MODIFIED_ENTITY_GIR: Gir = {
  ul_gir: "0.2",
  root: "e",
  nodes: [
    { id: "e", type: "point", sort: "entity" },
    { id: "m", type: "angle", sort: "modifier", measure: 45 },
  ],
  edges: [{ source: "e", target: "m", type: "modified_by" }],
};

/** All 5 primitives in one structure. */
export const ALL_PRIMITIVES_GIR: Gir = {
  ul_gir: "0.2",
  root: "enc",
  nodes: [
    { id: "enc", type: "enclosure", sort: "assertion", shape: "circle" },
    { id: "p1", type: "point", sort: "entity" },
    { id: "l", type: "line", sort: "relation", directed: true },
    { id: "p2", type: "point", sort: "entity" },
    { id: "a", type: "angle", sort: "modifier", measure: 90 },
    { id: "c", type: "curve", sort: "relation", curvature: 0.5 },
  ],
  edges: [
    { source: "enc", target: "p1", type: "contains" },
    { source: "enc", target: "l", type: "contains" },
    { source: "l", target: "p1", type: "connects" },
    { source: "l", target: "p2", type: "connects" },
    { source: "l", target: "a", type: "modified_by" },
    { source: "enc", target: "c", type: "contains" },
  ],
};

/** An empty GIR (edge case — minimal valid structure). */
export const EMPTY_GIR: Gir = {
  ul_gir: "0.2",
  root: "p",
  nodes: [{ id: "p", type: "point", sort: "entity" }],
  edges: [],
};

/** A GIR with nested enclosures (depth test). */
export const NESTED_ENCLOSURES: Gir = {
  ul_gir: "0.2",
  root: "outer",
  nodes: [
    { id: "outer", type: "enclosure", sort: "assertion", shape: "circle" },
    { id: "inner", type: "enclosure", sort: "assertion", shape: "triangle" },
    { id: "p", type: "point", sort: "entity" },
  ],
  edges: [
    { source: "outer", target: "inner", type: "contains" },
    { source: "inner", target: "p", type: "contains" },
  ],
};

/** Build a custom GIR inline. */
export function buildGir(
  root: string,
  nodes: Node[],
  edges: Edge[] = [],
): Gir {
  return { ul_gir: "0.2", root, nodes, edges };
}
