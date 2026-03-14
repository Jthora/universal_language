# API Endpoints

> REST endpoint reference for UL Forge API.

---

## POST `/parse`

Parse UL-Script text into GIR JSON.

**Request:**
```
Content-Type: text/plain
```
```
◯(•) → △(•)
```

**Response (200):**
```json
{
  "gir": {
    "version": "1.0",
    "nodes": [
      { "id": "n1", "type": "enclosure", "sort": "e", "shape": "circle", "label": "concept" },
      { "id": "n2", "type": "point", "sort": "e" },
      { "id": "n3", "type": "line", "sort": "r", "directed": true },
      { "id": "n4", "type": "enclosure", "sort": "e", "shape": "triangle", "label": "structure" },
      { "id": "n5", "type": "point", "sort": "e" }
    ],
    "edges": [
      { "type": "contains", "source": "n1", "target": "n2" },
      { "type": "connects", "source": "n3", "target": "n1" },
      { "type": "connects", "source": "n3", "target": "n4" },
      { "type": "contains", "source": "n4", "target": "n5" }
    ]
  },
  "warnings": []
}
```

**Error (400):**
```json
{
  "error": {
    "code": "PARSE_ERROR",
    "message": "Unexpected character '!' at line 1, column 5",
    "details": { "line": 1, "column": 5, "found": "!" }
  }
}
```

---

## POST `/render`

Render GIR JSON to SVG.

**Request:**
```
Content-Type: application/json
```
```json
{
  "gir": { "version": "1.0", "nodes": [...], "edges": [...] },
  "options": {
    "format": "svg",
    "width": 400,
    "height": 400,
    "embed_gir": true
  }
}
```

`options` is optional. Defaults: `format=svg`, `width=400`, `height=400`, `embed_gir=true`.

**Response (200):**
```
Content-Type: image/svg+xml
```
```xml
<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 400 400" ...>
  ...
</svg>
```

**For TikZ output:**
```json
{ "gir": {...}, "options": { "format": "tikz" } }
```

Response Content-Type: `text/plain`

---

## POST `/validate`

Validate a GIR document against schema, sort rules, and invariants.

**Request:**
```
Content-Type: application/json
```
```json
{
  "gir": { "version": "1.0", "nodes": [...], "edges": [...] },
  "options": {
    "check_geometry": false
  }
}
```

**Response (200 — valid):**
```json
{
  "valid": true,
  "errors": [],
  "warnings": []
}
```

**Response (200 — invalid):**
```json
{
  "valid": false,
  "errors": [
    {
      "code": "CONTAINMENT_CYCLE",
      "message": "Containment cycle detected: n1 → n2 → n1",
      "node_ids": ["n1", "n2"]
    }
  ],
  "warnings": []
}
```

Note: validation failures return 200 with `valid: false`, not 400. HTTP 400 is reserved for malformed requests (invalid JSON, missing `gir` field).

---

## POST `/convert`

Convert between any two supported formats.

**Request:**
```
Content-Type: application/json
```
```json
{
  "input": "◯(•) → △(•)",
  "input_format": "ul-script",
  "output_format": "svg",
  "options": {}
}
```

Supported formats: `ul-script`, `gir-json`, `svg`, `tikz`

**Response:** Content-Type matches target format.

---

## GET `/health`

Health check endpoint.

**Response (200):**
```json
{
  "status": "ok",
  "version": "1.0.0",
  "uptime_seconds": 3600
}
```

---

## Common Request Headers

| Header | Required | Description |
|--------|----------|-------------|
| `Content-Type` | Yes | `text/plain` for /parse, `application/json` for others |
| `Accept` | No | Preferred response format |

---

## Size Limits

- Maximum request body: 1 MB
- Maximum nodes per GIR: 10,000
- Maximum UL-Script length: 100,000 characters

Exceeding limits returns HTTP 413 (Payload Too Large).
