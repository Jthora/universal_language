# WebSocket Live Preview Protocol

> Real-time bidirectional protocol for live glyph editing.

---

## Connection

```
ws://localhost:3000/live
```

The WebSocket connection enables two workflows:
1. **Live preview** — client sends UL-Script keystrokes, server sends SVG updates
2. **Collaborative editing** — multiple clients share a GIR document via CRDT operations

---

## Message Format

All messages are JSON:

```json
{
  "type": "message_type",
  "id": "unique_message_id",
  "payload": { ... }
}
```

The `id` field is a client-generated UUID for request-response correlation.

---

## Client → Server Messages

### `edit`

Send a UL-Script edit (incremental or full).

```json
{
  "type": "edit",
  "id": "msg-001",
  "payload": {
    "text": "◯(•) → △(•)",
    "cursor": 12,
    "mode": "full"
  }
}
```

`mode`:
- `"full"` — complete UL-Script text (simple, used initially)
- `"incremental"` — OT delta (future optimization)

### `set_options`

Configure rendering options for this session.

```json
{
  "type": "set_options",
  "id": "msg-002",
  "payload": {
    "width": 600,
    "height": 400,
    "format": "svg",
    "debounce_ms": 100
  }
}
```

### `request_gir`

Request the current GIR (e.g., for saving or external processing).

```json
{
  "type": "request_gir",
  "id": "msg-003",
  "payload": {}
}
```

---

## Server → Client Messages

### `render`

SVG rendering result in response to an `edit`.

```json
{
  "type": "render",
  "id": "msg-001",
  "payload": {
    "svg": "<svg ...>...</svg>",
    "gir_hash": "abc123...",
    "render_time_ms": 12
  }
}
```

The `id` matches the `edit` message that triggered this render. If the user types faster than the server renders, intermediate edits are dropped and only the latest result is sent.

### `error`

Parse or render error.

```json
{
  "type": "error",
  "id": "msg-001",
  "payload": {
    "code": "PARSE_ERROR",
    "message": "Unexpected token at position 15",
    "details": { "line": 1, "column": 16 }
  }
}
```

Errors do not close the connection. The client can send a corrected `edit`.

### `gir`

Response to `request_gir`.

```json
{
  "type": "gir",
  "id": "msg-003",
  "payload": {
    "gir": { "version": "1.0", "nodes": [...], "edges": [...] }
  }
}
```

### `validation`

Sent after each successful parse with real-time validation results.

```json
{
  "type": "validation",
  "id": "msg-001",
  "payload": {
    "valid": true,
    "warnings": []
  }
}
```

---

## Timing and Debounce

The server debounces rapid edits:

1. Client sends `edit` at time T
2. Server starts a debounce timer (default 100ms)
3. If another `edit` arrives before timer fires, reset timer and discard previous edit
4. When timer fires, parse → validate → render → send `render` response

This prevents wasted work on intermediate keystrokes. The debounce window is configurable via `set_options`.

---

## Connection Lifecycle

```
Client                          Server
  │                               │
  ├── WebSocket CONNECT ─────────→│
  │←── HTTP 101 Upgrade ──────────┤
  │                               │
  ├── set_options ───────────────→│   (optional)
  │                               │
  ├── edit ──────────────────────→│
  │←── render ─────────────────────┤
  │←── validation ────────────────┤
  │                               │
  ├── edit ──────────────────────→│
  │←── error ─────────────────────┤   (parse failed)
  │                               │
  ├── edit ──────────────────────→│   (user fixed it)
  │←── render ─────────────────────┤
  │                               │
  ├── CLOSE ─────────────────────→│
  │←── CLOSE ─────────────────────┤
```

---

## Performance Targets

- Parse-to-render latency: < 50ms for typical glyphs (< 50 nodes)
- WebSocket overhead: < 1ms
- Debounce default: 100ms
- Maximum concurrent connections: 100 (configurable)
