#!/usr/bin/env python3
"""Write App.tsx with updated content."""
import os

CONTENT = r'''/**
 * UL Forge — Web Editor
 *
 * Three-pane layout: Template palette (opt) + editor + SVG preview.
 */
import { useState, useCallback } from "react";
import { ScriptEditor } from "./components/ScriptEditor";
import { SvgPreview } from "./components/SvgPreview";
import { StatusBar } from "./components/StatusBar";
import { TemplatePalette } from "./components/TemplatePalette";
import { ExportButtons } from "./components/ExportButtons";
import { useLivePreview } from "./hooks/useLivePreview";
import { useKeyboardShortcuts } from "./hooks/useKeyboardShortcuts";

export default function App() {
  const [paletteOpen, setPaletteOpen] = useState(false);
  const togglePalette = useCallback(() => setPaletteOpen((v) => !v), []);

  useLivePreview();
  useKeyboardShortcuts({ togglePalette });

  return (
    <div
      style={{
        height: "100vh",
        display: "flex",
        flexDirection: "column",
        background: "#1e1e1e",
        color: "#d4d4d4",
      }}
    >
      {/* Header */}
      <header
        style={{
          height: 40,
          display: "flex",
          alignItems: "center",
          padding: "0 16px",
          gap: 12,
          background: "#252526",
          borderBottom: "1px solid #333",
          fontFamily: "system-ui, sans-serif",
          fontSize: 14,
          fontWeight: 600,
        }}
      >
        <button
          onClick={togglePalette}
          title="Toggle template palette (Ctrl+Shift+P)"
          style={{
            background: paletteOpen ? "#3a3a3a" : "none",
            border: "1px solid #555",
            borderRadius: 3,
            color: "#d4d4d4",
            cursor: "pointer",
            padding: "2px 8px",
            fontSize: 13,
          }}
        >
          &#9776;
        </button>
        <span style={{ flex: 1 }}>UL Forge</span>
        <ExportButtons />
      </header>

      {/* Main area */}
      <main style={{ flex: 1, display: "flex", overflow: "hidden" }}>
        <TemplatePalette visible={paletteOpen} onClose={() => setPaletteOpen(false)} />

        {/* Editor pane */}
        <div
          style={{
            flex: 1,
            borderRight: "1px solid #333",
            minWidth: 0,
          }}
        >
          <ScriptEditor />
        </div>

        {/* Preview pane */}
        <div style={{ flex: 1, minWidth: 0 }}>
          <SvgPreview />
        </div>
      </main>

      {/* Status bar */}
      <StatusBar />
    </div>
  );
}
'''

target = os.path.join(
    os.path.dirname(__file__), 
    "web", "src", "App.tsx"
)
with open(target, "w") as f:
    f.write(CONTENT.lstrip())
print(f"Wrote {len(CONTENT.lstrip())} bytes to {target}")
