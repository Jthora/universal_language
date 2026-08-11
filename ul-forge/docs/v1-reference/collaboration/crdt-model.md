# CRDT Model for Real-Time Collaboration

> How multiple users edit the same glyph simultaneously.

---

## Overview

Real-time collaborative editing of UL glyphs uses a CRDT (Conflict-free Replicated Data Type) to synchronize GIR state across clients. Each client maintains a local replica; edits converge automatically without a central authority.

---

## Why CRDTs?

| Approach | Latency | Complexity | Offline support |
|----------|---------|-----------|----------------|
| Lock-based | High (round-trip) | Low | No |
| OT (Operational Transform) | Medium | High (transform matrix) | Partial |
| **CRDT** | **Low (local-first)** | **Medium** | **Yes** |

CRDTs allow each client to apply edits locally and synchronize asynchronously. Convergence is guaranteed by the mathematical properties of the data type.

---

## CRDT Design for GIR

The GIR is a graph with a flat node pool and an edge list. We model this as:

### Node Pool: LWW-Map (Last-Writer-Wins Map)

Each node is an entry in a map keyed by node ID. Concurrent writes to the same node are resolved by timestamp (last writer wins).

```
NodePool = LWW-Map<NodeId, Node>
```

### Edge List: Add-Remove Set (OR-Set)

Edges are stored in an Observed-Remove Set. Any client can add or remove edges. Concurrent add+remove of the same edge resolves to "add wins" (the edge remains).

```
EdgeList = OR-Set<Edge>
```

### Metadata: LWW-Register

Top-level GIR metadata (version, root info) is a single Last-Writer-Wins register.

---

## Library Choice: Yjs

| Library | Language | Maturity | WASM support |
|---------|----------|----------|-------------|
| **Yjs** | **JavaScript** | **Production** | **N/A (native JS)** |
| Automerge | Rust + JS | Production | Yes (WASM) |
| Diamond Types | Rust | Experimental | Yes |

**v1 recommendation: Yjs** — the web editor is the primary collaborative surface, and Yjs has the most mature editor integrations (Monaco, CodeMirror, ProseMirror). The GIR state is stored in a Yjs `Y.Map` (nodes) and `Y.Array` (edges).

If server-side Rust processing of CRDT state is needed later, Automerge (Rust-native) can replace or complement Yjs.

---

## Architecture

```
Client A                      Server                     Client B
   │                            │                           │
   ├── edit node n1 ──────────→ │ ←── add node n6 ─────────┤
   │                            │                           │
   │   Yjs sync protocol       │   Yjs sync protocol      │
   │                            │                           │
   │←── merged state ───────── │ ──── merged state ────────→│
   │                            │                           │
```

The server is a **relay**, not an authority. It forwards Yjs update messages between clients and persists the document state. Any client can compute the merged state independently.

### Room Management

Each GIR document has a **room** identified by document ID. Clients join the room via WebSocket:

```
ws://server/collab/{document_id}
```

Messages:
- `sync_step_1` / `sync_step_2` — Yjs synchronization protocol
- `update` — incremental Yjs update (binary-encoded)
- `awareness` — cursor positions, user names (ephemeral)

---

## GIR ↔ Yjs Mapping

```typescript
// Yjs document structure
const ydoc = new Y.Doc();
const yNodes = ydoc.getMap('nodes');     // Y.Map<NodeId, Node>
const yEdges = ydoc.getArray('edges');   // Y.Array<Edge>
const yMeta = ydoc.getMap('meta');       // Y.Map (version, root)

// Apply a local edit
yNodes.set('n7', { id: 'n7', type: 'point', sort: 'e' });

// Observe remote edits
yNodes.observe(event => {
  // Reconstruct GIR from Yjs state
  const gir = yDocToGir(ydoc);
  // Validate and re-render
  const result = validate(gir);
  if (result.valid) render(gir);
});
```

---

## Conflict Scenarios

| Scenario | CRDT resolution | UL-specific handling |
|----------|----------------|---------------------|
| Two users add different nodes | Both nodes appear (no conflict) | Valid — new nodes just appear |
| Two users move same node | Last writer wins (LWW) | May need user notification |
| User A deletes node, User B edits it | Delete wins in LWW-Map | Node disappears; B's edits lost |
| Two users add conflicting edges | Both edges appear (OR-Set) | Validator catches sort violations |

After CRDT merge, the validator runs. If the merged GIR has sort violations or structural issues, the web editor highlights them for collaborative resolution.

---

## Offline Support

Because CRDTs are local-first:
1. User edits while offline — changes stored locally in IndexedDB
2. User reconnects — Yjs syncs accumulated changes
3. Convergence happens automatically

No special offline mode is needed. The same code path handles online and offline editing.

---

## Scope in v1

Real-time collaboration is a Phase 4 feature. The CRDT infrastructure is designed now but implemented after the core pipeline and web editor are stable. The key Phase 4 deliverables are:

1. Yjs integration in the web editor GIR store
2. WebSocket relay server (simple Node.js or Rust service)
3. Room management (create, join, leave)
4. Awareness protocol (cursor sharing)
5. Persistence (server-side Yjs document storage)
