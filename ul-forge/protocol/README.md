# UL Transceiver Protocol v1.0

> Wire protocol for AI-to-AI communication using Universal Language meaning structures.

## Overview

The UL Transceiver Protocol enables AI systems to exchange structured semantic content encoded as UL GIR documents. It is **transport-agnostic** — the same message format works over HTTP, WebSocket, stdio, message queues, or any byte stream.

## Wire Format

Messages are JSON objects conforming to `ul-transceiver.schema.json`:

```json
{
  "version": "1.0",
  "message": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "timestamp": "2025-01-15T10:30:00Z",
    "sender": {
      "id": "agent-alpha",
      "capabilities": ["parse", "validate", "render"]
    },
    "intent": "assert",
    "payload": {
      "format": "gir",
      "content": { "ul_gir": "1.0", "root": "n0", "nodes": [...], "edges": [...] }
    },
    "context": {
      "conversation_id": "conv-uuid",
      "domain": "physics.thermodynamics"
    }
  }
}
```

## Intent Types

| Intent | Direction | Description | Expected Payload |
|--------|-----------|-------------|-----------------|
| `assert` | Sender → Receiver | Declare a meaning structure as true | GIR or UL-Script |
| `query` | Sender → Receiver | Ask whether a structure holds | GIR (may be partial) |
| `propose` | Sender → Receiver | Suggest a structure for agreement | GIR + confidence |
| `refine` | Sender → Receiver | Modify a previously sent structure | GIR diff |
| `validate_request` | Sender → Receiver | Ask recipient to validate | GIR |
| `validate_response` | Receiver → Sender | Report validation result | ValidationResult |
| `capability_advertisement` | Any | Announce supported operations | Capability list |
| `ack` | Receiver → Sender | Acknowledge receipt | Reference to message ID |

## Conversation Flow

```
Agent A                          Agent B
  │                                │
  │── capability_advertisement ──► │
  │◄── capability_advertisement ──│
  │                                │
  │── assert (GIR) ──────────────►│
  │◄── ack ───────────────────────│
  │                                │
  │── propose (GIR, 0.85) ───────►│
  │◄── refine (GIR delta) ────────│
  │── ack ────────────────────────►│
```

## Transport Binding

The protocol does not mandate a specific transport. Implementations should:

1. **Frame messages** — One JSON message per frame (newline-delimited for streams)
2. **Content-Type** — Use `application/ul-transceiver+json` for HTTP
3. **Ordering** — Use `context.sequence_number` if ordering matters
4. **Discovery** — Use `capability_advertisement` as the first exchange

## Libraries

| Language | Package | Import |
|----------|---------|--------|
| Rust | `ul-transceiver` | `use ul_transceiver::{ULMessage, Intent, Codec}` |
| TypeScript | `@ul-forge/transceiver` | `import { ULMessage, Codec } from '@ul-forge/transceiver'` |

## Schema

The canonical JSON Schema is at `ul-transceiver.schema.json` in this directory.
