# File Formats

> Specification of all file formats produced and consumed by UL Forge.

---

## Format Summary

| Extension | Content | MIME type | Primary consumer |
|-----------|---------|-----------|-----------------|
| `.ul` | UL-Script (Unicode text) | `text/x-ul-script` | Parser (`ul-parse`) |
| `.ul.json` | UL-GIR (JSON graph) | `application/vnd.ul.gir+json` | All tools |
| `.ul.svg` | Rendered SVG with embedded GIR | `image/svg+xml` | Browsers, image viewers |

---

## `.ul` — UL-Script

Plain text file containing UL-Script. UTF-8 encoding. Line endings: LF (Unix) preferred, CRLF tolerated.

```
# Comment: the concept of existence
○{●}

# Composed: knowledge containing learning
○{ △{●} |→ ~{●} }
```

**Conventions:**
- One glyph definition per line (or multi-line with continuation)
- Comments start with `#`
- Blank lines separate glyph definitions
- File may contain multiple glyph definitions (batch mode)

**Markdown embedding:** UL-Script blocks can be embedded in Markdown using fenced code blocks:

````markdown
The glyph for "existence" is:

```ul
○{●}
```
````

This is a Markdown convention, not a UL Forge format. The parser extracts `ul` blocks from Markdown when given a `.md` file.

---

## `.ul.json` — UL-GIR

JSON document following the graph-with-tree-spine schema. See [graph-with-tree-spine.md](graph-with-tree-spine.md) for the full data model.

**Top-level structure:**
```json
{
  "ul_gir": "1.0",
  "root": "n1",
  "nodes": [ ... ],
  "edges": [ ... ],
  "metadata": {
    "source": "example.ul",
    "generated_by": "ul-parse 1.0.0",
    "timestamp": "2026-03-14T12:00:00Z"
  }
}
```

**Requirements:**
- `ul_gir`: Version string. Tools must refuse unknown major versions.
- `root`: ID of the root node. Must reference a valid node.
- `nodes`: Array of node objects. Order is serialization order (tree-spine DFS).
- `edges`: Array of edge objects. Tree-spine edges first, cross-edges after.
- `metadata`: Optional. Provenance information.

**Multi-glyph documents:**
```json
{
  "ul_gir": "1.0",
  "glyphs": [
    { "root": "n1", "nodes": [...], "edges": [...] },
    { "root": "n10", "nodes": [...], "edges": [...] }
  ]
}
```

---

## `.ul.svg` — Rendered SVG with Embedded GIR

Standard SVG file with UL-GIR embedded in the `<metadata>` element.

```xml
<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg"
     xmlns:ul="urn:ul:gir:1.0"
     viewBox="0 0 200 200"
     width="200" height="200">
  
  <!-- Embedded GIR for structure recovery -->
  <metadata>
    <ul:gir version="1.0" hash="sha256:a1b2c3d4...">
      <![CDATA[
        {"ul_gir":"1.0","root":"n1","nodes":[...],"edges":[...]}
      ]]>
    </ul:gir>
  </metadata>
  
  <!-- Render hash for sync detection -->
  <desc class="ul-render-hash">sha256:a1b2c3d4...</desc>
  
  <!-- Visual rendering -->
  <g class="ul-glyph" data-ul-root="n1">
    <circle cx="100" cy="100" r="80" 
            stroke="black" stroke-width="2" fill="none"
            data-ul-node="n1"/>
    <circle cx="100" cy="100" r="3" fill="black"
            data-ul-node="n2"/>
  </g>
</svg>
```

**Sync detection:** When `metadata hash == desc hash`, the GIR and visuals are in sync. When they differ, the SVG has been edited outside UL Forge. See [decision-log.md](decision-log.md) for the full sync policy.

**Data attributes:** SVG elements carry `data-ul-node` and `data-ul-edge` attributes linking visual elements to GIR nodes/edges. This enables hover-to-inspect and click-to-select in the web editor.

---

## Version Negotiation

All formats carry a version field:
- `.ul` files: no explicit version (backwards compatible — new syntax is additive)
- `.ul.json` files: `"ul_gir": "1.0"` — major.minor
- `.ul.svg` files: `<ul:gir version="1.0">` in metadata

**Rules:**
1. Tools MUST refuse to process GIR with an unknown major version
2. Tools SHOULD ignore unknown fields (forward compatibility)
3. Tools MUST include all required fields from their supported version (backward compatibility)
4. Minor version bumps add optional fields only
5. Major version bumps may change required fields or semantics
