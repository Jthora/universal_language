# Type 4 — Transceiver Library (Meaning Wire Protocol)

> Cross-language SDK for agents and systems to exchange structured meaning over any transport.

---

## Problem

UL defines a formal system for encoding meaning, but there is no standard way for two programs to exchange UL-structured messages. Today, a Python agent and a Rust service that both understand UL have no shared wire format for asking questions, making assertions, or negotiating meaning.

The Transceiver library solves this by defining:

1. **Wire Protocol** — a JSON-based message schema for 8 meaning-exchange intents
2. **Codec implementations** — Rust, TypeScript, and Python libraries that serialize/deserialize these messages
3. **Transport agnosticism** — the protocol works over HTTP, WebSocket, stdio, message queues, or raw files

---

## Architecture

```
                  ┌─────────────────────┐
                  │   Wire Protocol     │
                  │   (JSON Schema)     │
                  └──────────┬──────────┘
                             │
              ┌──────────────┼──────────────┐
              │              │              │
    ┌─────────▼──────┐ ┌────▼─────┐ ┌──────▼──────┐
    │  ul-transceiver│ │@ul-forge/│ │ ul_forge.   │
    │  (Rust crate)  │ │transceivr│ │ transceiver │
    │                │ │ (npm)    │ │ (PyPI)      │
    └────────────────┘ └──────────┘ └─────────────┘
              │              │              │
    ┌─────────▼──────────────▼──────────────▼─────────┐
    │                  Any Transport                    │
    │   stdio · HTTP · WebSocket · NATS · file pipe    │
    └──────────────────────────────────────────────────┘
```

---

## Message Model

Every message is a `ULMessage` with:

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID v4 | Unique message identifier |
| `timestamp` | ISO 8601 | When the message was created |
| `intent` | enum (8 values) | What kind of meaning exchange this is |
| `gir` | GIR object | The UL structure being communicated |
| `metadata` | object | Extensible key-value pairs (domain, confidence, etc.) |
| `reply_to` | UUID v4 \| null | The message this is responding to |

### 8 Intents

| Intent | Direction | Purpose |
|--------|-----------|---------|
| `assert` | A → B | "This is true" — declare a meaning structure |
| `query` | A → B | "Is this true?" — ask about a structure |
| `propose` | A → B | "What about this?" — suggest a structure for feedback |
| `refine` | A → B | "Better version" — improve a previously sent structure |
| `validate_request` | A → B | "Is this well-formed?" — ask for validation |
| `validate_response` | B → A | "Here's what I found" — return validation result |
| `capability` | A → B | "I can do these things" — advertise supported operations |
| `ack` | B → A | "Received" — acknowledge receipt |

---

## Data Flow Example

```
Agent A                          Agent B
   │                                │
   │──── assert(gir) ─────────────►│
   │                                │  (processes GIR)
   │◄─── ack(id) ─────────────────│
   │                                │
   │◄─── propose(refined_gir) ────│
   │                                │
   │──── refine(better_gir) ──────►│
   │                                │
   │◄─── ack(id) ─────────────────│
```

---

## File Plan

| File | Content |
|------|---------|
| [wire-protocol.md](wire-protocol.md) | JSON Schema, message format, intent semantics, versioning |
| [rust-library.md](rust-library.md) | `ul-transceiver` crate: types, builder, codec, transport adapters |
| [typescript-library.md](typescript-library.md) | `@ul-forge/transceiver` npm package |
| [python-library.md](python-library.md) | `ul-forge-transceiver` PyPI package |

---

## Dependencies

| Prerequisite | Why |
|-------------|-----|
| **P3 — GIR JSON Schema** | Wire protocol references the GIR schema for the `gir` field |
| **ul-core types** | All codecs parse into native GIR types |

Type 4 can begin as soon as P3 (schema generation) is complete.

---

## Success Criteria

| Criterion | Measure |
|-----------|---------|
| Interoperability | Rust → JSON → TypeScript round-trip preserves all fields |
| Interoperability | TypeScript → JSON → Python round-trip preserves all fields |
| Interoperability | Python → JSON → Rust round-trip preserves all fields |
| Schema conformance | All generated messages pass JSON Schema validation |
| Performance | Encode + decode < 1ms for typical messages (< 100 nodes) |
| Size | JSON message for simple glyph < 2 KB |
| Extensibility | Adding a 9th intent requires changes to only enum + schema, not codec logic |
