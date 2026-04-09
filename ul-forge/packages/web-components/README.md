# @ul-forge/components

**Universal Language web components — embed UL glyphs in any HTML page.**

Zero build system. No React. No npm required. Just a `<script>` tag.

## Usage

```html
<script src="https://cdn.jsdelivr.net/npm/@ul-forge/components/dist/ul-components.iife.js"></script>

<!-- Static glyph: ● = point (entity), → = directed relation -->
<ul-symbol script="● → ●" width="200" height="200"></ul-symbol>

<!-- Interactive editor -->
<ul-composer width="400" height="300"></ul-composer>

<!-- Searchable lexicon -->
<ul-dictionary columns="3"></ul-dictionary>
```

### UL-Script Syntax

UL-Script uses geometric symbols, not function names:

| Symbol | ASCII | Meaning |
|--------|-------|---------|
| `●` | `*` | Point (entity) |
| `→` | `->` | Directed relation |
| `∠60` | `@60` | Angle modifier |
| `~` | `~` | Curve (process) |
| `□` | `/4` | Square enclosure |

## Components

### `<ul-symbol>`

Displays a static UL glyph from UL-Script text.

| Attribute | Default | Description |
|-----------|---------|-------------|
| `script` | `""` | UL-Script to parse and render |
| `width` | `200` | SVG width in pixels |
| `height` | `200` | SVG height in pixels |

### `<ul-composer>`

Interactive editor with primitive palette, live preview, validation status, and SVG export.

| Attribute | Default | Description |
|-----------|---------|-------------|
| `width` | `400` | Preview width |
| `height` | `300` | Preview height |
| `value` | `""` | Initial UL-Script |

Fires `ul-change` event on input: `{ detail: { script, gir, valid } }`

### `<ul-dictionary>`

Searchable browser for the 42 canonical UL lexicon entries.

| Attribute | Default | Description |
|-----------|---------|-------------|
| `columns` | `3` | Grid columns |
| `query` | `""` | Initial search query |

## License

CC0-1.0 — Public Domain
