# Type 4 — Wire Protocol Specification

> JSON-based message schema for exchanging UL-structured meaning between systems.

---

## Protocol Version

```
UL-Wire/0.1
```

The version string appears in the `protocol` field of every message. Breaking changes increment the minor version until 1.0, after which semver applies.

---

## Message Envelope

Every message is a JSON object with this top-level structure:

```json
{
  "protocol": "UL-Wire/0.1",
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2025-01-15T10:30:00Z",
  "intent": "assert",
  "reply_to": null,
  "payload": { ... },
  "metadata": { ... }
}
```

### Required Fields

| Field | Type | Constraints |
|-------|------|-------------|
| `protocol` | string | Must match `UL-Wire/X.Y` pattern |
| `id` | string | UUID v4 format |
| `timestamp` | string | ISO 8601 with timezone |
| `intent` | string | One of 8 defined intents |

### Optional Fields

| Field | Type | Default |
|-------|------|---------|
| `reply_to` | string \| null | `null` |
| `payload` | object | `{}` |
| `metadata` | object | `{}` |

---

## Intent Definitions

### `assert`

Declare a meaning structure as true / established.

**Payload:**
```json
{
  "gir": { /* GIR object per SCHEMA.md */ }
}
```

**Use cases:**
- Agent tells another agent "here is a fact expressed in UL"
- System publishes a new UL structure to subscribers
- Game engine declares the current cosmic rule state

---

### `query`

Ask whether a structure holds, or request structures matching a pattern.

**Payload:**
```json
{
  "gir": { /* GIR pattern — may contain wildcards */ },
  "match_mode": "exact" | "structural" | "semantic"
}
```

| match_mode | Behavior |
|------------|----------|
| `exact` | GIR must match node-for-node |
| `structural` | Same topology, labels may differ |
| `semantic` | Same meaning, structure may differ |

**Expected response:** `assert` (if match found) or `ack` with `status: "no_match"`.

---

### `propose`

Suggest a structure for consideration without asserting it as true.

**Payload:**
```json
{
  "gir": { /* proposed GIR */ },
  "confidence": 0.85
}
```

The `confidence` field (0.0–1.0) expresses how confident the sender is in the proposal. This is advisory — the receiver decides what to do.

**Expected response:** `ack`, `refine`, or `assert` (if receiver accepts as-is).

---

### `refine`

Improve a previously sent structure. Must reference the original message via `reply_to`.

**Payload:**
```json
{
  "gir": { /* improved GIR */ },
  "changes": [
    { "type": "added_node", "node_id": "n5" },
    { "type": "removed_edge", "edge_id": "e2" },
    { "type": "modified_node", "node_id": "n1", "field": "label", "old": "flow", "new": "directed_flow" }
  ]
}
```

The `changes` array is optional but recommended — it helps the receiver understand what changed without full diff computation.

---

### `validate_request`

Ask for validation of a GIR structure.

**Payload:**
```json
{
  "gir": { /* GIR to validate */ },
  "checks": ["schema", "sorts", "invariants", "geometry"]
}
```

The `checks` array specifies which validation layers to run. Omit to run all.

**Expected response:** `validate_response`.

---

### `validate_response`

Return validation results. Must have `reply_to` set to the `validate_request` message ID.

**Payload:**
```json
{
  "valid": true,
  "errors": [],
  "warnings": ["Node n3 has no outgoing edges"],
  "checks_run": ["schema", "sorts", "invariants"]
}
```

---

### `capability`

Advertise what operations this system supports.

**Payload:**
```json
{
  "capabilities": [
    {
      "name": "parse",
      "description": "Parse UL-Script text into GIR",
      "input": "text/ul-script",
      "output": "application/gir+json"
    },
    {
      "name": "render",
      "description": "Render GIR to SVG",
      "input": "application/gir+json",
      "output": "image/svg+xml",
      "options": { "formats": ["svg", "tikz"] }
    },
    {
      "name": "validate",
      "description": "Validate GIR structure",
      "input": "application/gir+json",
      "output": "application/validation-result+json"
    }
  ],
  "protocol_versions": ["UL-Wire/0.1"],
  "encoding": ["json", "msgpack"]
}
```

**Usage:** Sent at connection establishment or on request. Receivers cache capabilities to know what they can ask for.

---

### `ack`

Acknowledge receipt of a message. Must have `reply_to` set.

**Payload:**
```json
{
  "status": "received" | "accepted" | "rejected" | "no_match" | "error",
  "message": "Optional human-readable explanation"
}
```

---

## Metadata

The `metadata` object is extensible. Reserved keys:

| Key | Type | Meaning |
|-----|------|---------|
| `domain` | string | Knowledge domain (e.g., "physics", "music") |
| `source` | string | Identifier for the sending system |
| `trace_id` | string | Distributed tracing correlation ID |
| `ttl` | integer | Time-to-live in seconds (for caching) |
| `encoding` | `"json"` \| `"msgpack"` | Wire encoding of this message |

Custom keys should use reverse-domain notation: `"com.example.custom_field"`.

---

## Encoding

### JSON (default)

```
Content-Type: application/ul-wire+json
```

Standard JSON encoding. All implementations MUST support this.

### MessagePack (optional)

```
Content-Type: application/ul-wire+msgpack
```

Binary encoding for performance-sensitive transports. Same schema, binary format. Implementations SHOULD support this.

### Negotiation

On transports that support headers (HTTP, WebSocket), use `Accept` and `Content-Type` headers. On headerless transports (stdio, pipes), include `"encoding": "json"` or `"encoding": "msgpack"` in metadata.

---

## Transport Bindings

The wire protocol is transport-agnostic. Here's how it maps to common transports:

### stdio (MCP-style)

Newline-delimited JSON. One message per line.

```
{"protocol":"UL-Wire/0.1","id":"...","intent":"capability","payload":{...}}\n
{"protocol":"UL-Wire/0.1","id":"...","intent":"assert","payload":{...}}\n
```

### HTTP

```
POST /ul-wire HTTP/1.1
Content-Type: application/ul-wire+json

{ "protocol": "UL-Wire/0.1", ... }
```

Response is also a UL-Wire message (typically `ack`, `validate_response`, or `assert`).

### WebSocket

Each WebSocket text frame contains one JSON-encoded message. Binary frames may contain MessagePack-encoded messages.

### File

One message per file, or newline-delimited JSON in a single file. Useful for batch processing and logging.

---

## Versioning

| Version | Status | Changes |
|---------|--------|---------|
| 0.1 | Draft | Initial 8 intents, JSON + msgpack |

### Compatibility Rules

- Messages with unknown intents MUST be acknowledged with `ack(status: "error")`, not silently dropped
- Unknown metadata keys MUST be preserved on forwarding
- Unknown payload fields MUST be ignored (forward-compatible)
- The `protocol` field MUST be checked; reject messages with unsupported major versions

---

## JSON Schema

> Full JSON Schema will be auto-generated from Rust types via `schemars` (see [schema-generation.md](../shared/schema-generation.md)). Below is the structural outline.

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://ul-forge.dev/schemas/ul-wire/0.1/message.json",
  "type": "object",
  "required": ["protocol", "id", "timestamp", "intent"],
  "properties": {
    "protocol": {
      "type": "string",
      "pattern": "^UL-Wire/\\d+\\.\\d+$"
    },
    "id": {
      "type": "string",
      "format": "uuid"
    },
    "timestamp": {
      "type": "string",
      "format": "date-time"
    },
    "intent": {
      "type": "string",
      "enum": ["assert", "query", "propose", "refine",
               "validate_request", "validate_response",
               "capability", "ack"]
    },
    "reply_to": {
      "type": ["string", "null"],
      "format": "uuid"
    },
    "payload": {
      "type": "object"
    },
    "metadata": {
      "type": "object"
    }
  }
}
```

Payload schemas are intent-specific and referenced via `$ref` in the full schema.
