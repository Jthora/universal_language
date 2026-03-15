# Type 1 — MCP Server: `ul-mcp`

> Model Context Protocol server exposing UL operations as discoverable AI tools.

---

## Purpose

MCP (Model Context Protocol) is the emerging standard for how AI assistants discover and invoke tools. An MCP server makes UL Forge available to any MCP-compatible client: Claude Desktop, VS Code Copilot (via MCP), custom LangChain agents, etc.

---

## Crate Structure

```
ul-forge/crates/ul-mcp/
  Cargo.toml
  src/
    main.rs              # Entry point, MCP server setup
    tools.rs             # Tool definitions and handlers
```

### Cargo.toml

```toml
[package]
name = "ul-mcp"
version.workspace = true
edition.workspace = true
license.workspace = true

[[bin]]
name = "ul-mcp"
path = "src/main.rs"

[dependencies]
ul-core = { path = "../ul-core" }
serde = { workspace = true }
serde_json = { workspace = true }
tokio = { version = "1", features = ["full"] }
# MCP SDK — use whichever Rust MCP library is mature at build time
# Options: mcp-rs, rmcp, or implement stdio JSON-RPC directly
```

---

## Tools Exposed

| Tool Name | Description | Input | Output |
|-----------|-------------|-------|--------|
| `ul_parse` | Parse UL-Script into GIR | `{ ul_script: string }` | `{ gir: GIR }` |
| `ul_validate` | Validate GIR against Σ_UL | `{ gir: GIR, check_geometry?: bool }` | `{ valid, errors, warnings }` |
| `ul_render` | Render GIR to visual output | `{ gir: GIR, width?: number, height?: number, format?: "svg"\|"tikz" }` | `{ output: string }` |
| `ul_deparse` | Convert GIR back to UL-Script | `{ gir: GIR }` | `{ ul_script: string }` |
| `ul_parse_and_render` | Full pipeline in one call | `{ ul_script: string, width?: number, height?: number }` | `{ svg, gir, validation }` |

---

## Tool Schema (JSON-RPC)

```json
{
  "name": "ul_parse",
  "description": "Parse Universal Language Script (UL-Script) text into a Graph Intermediate Representation (GIR). UL-Script uses 5 geometric primitives: point, line, angle, curve, enclosure.",
  "inputSchema": {
    "type": "object",
    "required": ["ul_script"],
    "properties": {
      "ul_script": {
        "type": "string",
        "description": "UL-Script text to parse, e.g. 'enclosure(circle, point(existence))'"
      }
    }
  }
}
```

```json
{
  "name": "ul_validate",
  "description": "Validate a GIR document against the Σ_UL formal specification. Checks schema integrity, sort constraints, graph invariants, and optionally geometric satisfiability.",
  "inputSchema": {
    "type": "object",
    "required": ["gir"],
    "properties": {
      "gir": {
        "type": "object",
        "description": "A GIR document with nodes, edges, and root"
      },
      "check_geometry": {
        "type": "boolean",
        "default": false,
        "description": "Whether to run geometric satisfiability checks (Layer 4)"
      }
    }
  }
}
```

```json
{
  "name": "ul_render",
  "description": "Render a validated GIR document to SVG or TikZ output.",
  "inputSchema": {
    "type": "object",
    "required": ["gir"],
    "properties": {
      "gir": { "type": "object" },
      "width": { "type": "number", "default": 400 },
      "height": { "type": "number", "default": 400 },
      "format": { "type": "string", "enum": ["svg", "tikz"], "default": "svg" }
    }
  }
}
```

---

## Transport

MCP servers communicate over **stdio** (JSON-RPC 2.0 over stdin/stdout). This is the standard MCP transport.

```
Client ──stdin──→ ul-mcp process ──stdout──→ Client
```

### Running

```bash
# Direct invocation
ul-mcp

# Via MCP client configuration (e.g., Claude Desktop)
# In claude_desktop_config.json:
{
  "mcpServers": {
    "ul-forge": {
      "command": "ul-mcp",
      "args": []
    }
  }
}
```

### Docker

```bash
# MCP over stdio, run interactively
docker run -i ghcr.io/jthora/ul-forge ul-mcp
```

---

## Implementation Sketch

```rust
// src/main.rs
use std::io::{self, BufRead, Write};
use serde_json::{json, Value};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    for line in stdin.lock().lines() {
        let line = line.expect("stdin read error");
        let request: Value = serde_json::from_str(&line).expect("invalid JSON-RPC");

        let response = match request["method"].as_str() {
            Some("initialize") => handle_initialize(&request),
            Some("tools/list") => handle_tools_list(&request),
            Some("tools/call") => handle_tools_call(&request),
            _ => json_rpc_error(request["id"].clone(), -32601, "Method not found"),
        };

        writeln!(out, "{}", serde_json::to_string(&response).unwrap()).unwrap();
        out.flush().unwrap();
    }
}
```

```rust
// src/tools.rs
use ul_core::{parser, validator, renderer};

pub fn call_tool(name: &str, arguments: &Value) -> Result<Value, String> {
    match name {
        "ul_parse" => {
            let script = arguments["ul_script"].as_str().ok_or("missing ul_script")?;
            let gir = parser::parse(script).map_err(|e| e.to_string())?;
            let json = gir.to_json().map_err(|e| e.to_string())?;
            Ok(serde_json::from_str(&json).unwrap())
        }
        "ul_validate" => {
            let gir_json = serde_json::to_string(&arguments["gir"]).map_err(|e| e.to_string())?;
            let gir = ul_core::Gir::from_json(&gir_json).map_err(|e| e.to_string())?;
            let check_geo = arguments["check_geometry"].as_bool().unwrap_or(false);
            let result = validator::validate(&gir, check_geo);
            Ok(json!({
                "valid": result.valid,
                "errors": result.errors.iter().map(|e| e.to_string()).collect::<Vec<_>>(),
                "warnings": result.warnings,
            }))
        }
        "ul_render" => {
            let gir_json = serde_json::to_string(&arguments["gir"]).map_err(|e| e.to_string())?;
            let gir = ul_core::Gir::from_json(&gir_json).map_err(|e| e.to_string())?;
            let options = renderer::RenderOptions {
                width: arguments["width"].as_f64().unwrap_or(400.0),
                height: arguments["height"].as_f64().unwrap_or(400.0),
                format: match arguments["format"].as_str() {
                    Some("tikz") => renderer::OutputFormat::TikZ,
                    _ => renderer::OutputFormat::Svg,
                },
                embed_gir: false,
            };
            let output = renderer::render(&gir, &options).map_err(|e| e.to_string())?;
            Ok(json!({ "output": output }))
        }
        "ul_deparse" => {
            let gir_json = serde_json::to_string(&arguments["gir"]).map_err(|e| e.to_string())?;
            let gir = ul_core::Gir::from_json(&gir_json).map_err(|e| e.to_string())?;
            let script = parser::deparse(&gir).map_err(|e| e.to_string())?;
            Ok(json!({ "ul_script": script }))
        }
        _ => Err(format!("unknown tool: {name}")),
    }
}
```

---

## Testing

```bash
# Smoke test: list tools
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | ul-mcp

# Smoke test: parse
echo '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"ul_parse","arguments":{"ul_script":"point(existence)"}}}' | ul-mcp
```

Integration tests should verify all 5 tools return well-formed responses for both valid and invalid inputs.
