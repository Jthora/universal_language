# Type 1 — API Discovery Endpoints

> Self-describing REST endpoints that let AI agents discover UL Forge capabilities automatically.

---

## Endpoints

| Endpoint | Method | Returns | Purpose |
|----------|--------|---------|---------|
| `/capabilities` | GET | OpenAPI 3.1 JSON | Full API specification |
| `/schema/gir` | GET | GIR JSON Schema | Machine-readable document schema |
| `/schema/validation-result` | GET | ValidationResult JSON Schema | Machine-readable result schema |
| `/schema/render-options` | GET | RenderOptions JSON Schema | Machine-readable options schema |
| `/.well-known/ai-plugin.json` | GET | Plugin manifest | OpenAI-compatible plugin descriptor |

---

## `/capabilities` — OpenAPI Spec

Auto-generated from the Axum routes using `utoipa` crate (or manually maintained).

### Implementation Approach

Option A: **Generated** — Use `utoipa` derive macros on handler functions:

```rust
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(parse, validate, render, convert, health),
    components(schemas(Gir, Node, Edge, Sort, NodeType, EdgeType, ValidationResult, RenderOptions))
)]
struct ApiDoc;

// Route:
.route("/capabilities", get(|| async { Json(ApiDoc::openapi()) }))
```

Option B: **Static** — Maintain `openapi.json` in the repo, serve as static file. Risk of drift.

**Recommended: Option A** — generated from code guarantees consistency.

### Dependencies

```toml
[dependencies]
utoipa = { version = "5", features = ["axum_extras"] }
utoipa-swagger-ui = { version = "8", features = ["axum"] } # optional: Swagger UI at /docs
```

---

## `/schema/*` — JSON Schema Endpoints

Serve the pre-generated schema files from P3:

```rust
async fn schema_gir() -> impl IntoResponse {
    let schema = include_str!("../../schemas/gir.schema.json");
    (
        [(header::CONTENT_TYPE, "application/schema+json")],
        schema,
    )
}

// Routes:
.route("/schema/gir", get(schema_gir))
.route("/schema/validation-result", get(schema_validation_result))
.route("/schema/render-options", get(schema_render_options))
```

---

## `/.well-known/ai-plugin.json` — Plugin Manifest

Standard location for AI assistants to discover plugin capabilities. Based on OpenAI's plugin specification.

```json
{
  "schema_version": "v1",
  "name_for_human": "UL Forge",
  "name_for_model": "ul_forge",
  "description_for_human": "Parse, validate, and render Universal Language glyphs — a formal symbolic writing system based on 5 geometric primitives.",
  "description_for_model": "Universal Language Forge processes UL-Script (a formal text notation) into GIR (Graph Intermediate Representation) and renders to SVG. It uses 5 geometric primitives: Point (existence), Line (relation), Angle (modification), Curve (process), Enclosure (concept). Use this to create, validate, and visualize formal symbolic compositions.",
  "auth": {
    "type": "none"
  },
  "api": {
    "type": "openapi",
    "url": "/capabilities"
  },
  "logo_url": null,
  "contact_email": "jono@archangel.agency",
  "legal_info_url": "https://github.com/Jthora/universal_language/blob/main/LICENSE"
}
```

---

## Response Headers

All schema endpoints should include:

```
Content-Type: application/schema+json
Cache-Control: public, max-age=86400
Access-Control-Allow-Origin: *
```

The CORS header allows in-browser agent tools to fetch schemas directly.

---

## Changes to `ul-api/src/routes.rs`

Add the 5 new routes to the existing Axum router:

```rust
pub fn create_router() -> Router {
    Router::new()
        // Existing routes
        .route("/health", get(health))
        .route("/parse", post(parse_handler))
        .route("/validate", post(validate_handler))
        .route("/render", post(render_handler))
        .route("/convert", post(convert_handler))
        // Discovery routes (new)
        .route("/capabilities", get(capabilities_handler))
        .route("/schema/gir", get(schema_gir_handler))
        .route("/schema/validation-result", get(schema_validation_handler))
        .route("/schema/render-options", get(schema_render_handler))
        .route("/.well-known/ai-plugin.json", get(plugin_manifest_handler))
}
```

---

## Testing

```bash
# OpenAPI spec is valid JSON
curl -s http://localhost:8080/capabilities | python -m json.tool

# Schema validates a known-good GIR
curl -s http://localhost:8080/schema/gir | python -c "
import json, sys
schema = json.load(sys.stdin)
assert schema['type'] == 'object'
assert 'nodes' in schema['properties']
print('Schema valid')
"

# Plugin manifest has required fields
curl -s http://localhost:8080/.well-known/ai-plugin.json | python -c "
import json, sys
manifest = json.load(sys.stdin)
assert manifest['name_for_model'] == 'ul_forge'
assert manifest['api']['url'] == '/capabilities'
print('Manifest valid')
"
```
