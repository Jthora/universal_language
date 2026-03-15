/**
 * UL Forge - Web Editor
 *
 * Three-pane layout: Template palette (opt) + editor/canvas + SVG preview.
 * Toggle between text editor and visual canvas with Ctrl+E.
 */
import { useState, useCallback } from "react";
import { ScriptEditor } from "./components/ScriptEditor";
import { SvgPreview } from "./components/SvgPreview";
import { ErrorBoundary } from "./components/ErrorBoundary";
import { StatusBar } from "./components/StatusBar";
import { TemplatePalette } from "./components/TemplatePalette";
import { ExportButtons } from "./components/ExportButtons";
import { VisualCanvas } from "./components/VisualCanvas";
import { useLivePreview } from "./hooks/useLivePreview";
import { useKeyboardShortcuts } from "./hooks/useKeyboardShortcuts";

type EditMode = "text" | "visual";

export default function App() {
  const [paletteOpen, setPaletteOpen] = useState(false);
  const [mode, setMode] = useState<EditMode>("text");
  const togglePalette = useCallback(() => setPaletteOpen((v) => !v), []);
  const toggleMode = useCallback(() => setMode((m) => (m === "text" ? "visual" : "text")), []);

  useLivePreview();
  useKeyboardShortcuts({ togglePalette, toggleMode });

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
          title="Toggle template palette"
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
          {"\u2630"}
        </button>
        <span style={{ flex: 1 }}>UL Forge</span>
        <button
          onClick={toggleMode}
          title={`Switch to ${mode === "text" ? "visual" : "text"} mode (Ctrl+E)`}
          style={{
            background: mode === "visual" ? "#094771" : "none",
            border: "1px solid #555",
            borderRadius: 3,
            color: "#d4d4d4",
            cursor: "pointer",
            padding: "2px 10px",
            fontSize: 12,
          }}
        >
          {mode === "text" ? "Visual" : "Text"}
        </button>
        <ExportButtons />
      </header>

      <main style={{ flex: 1, display: "flex", overflow: "hidden" }}>
        <TemplatePalette
          visible={paletteOpen}
          onClose={() => setPaletteOpen(false)}
        />

        <div
          style={{
            flex: 1,
            borderRight: "1px solid #333",
            minWidth: 0,
          }}
        >
          {mode === "text" ? <ScriptEditor /> : <VisualCanvas />}
        </div>

        <div style={{ flex: 1, minWidth: 0 }}>
          <ErrorBoundary>
            <SvgPreview />
          </ErrorBoundary>
        </div>
      </main>

      <StatusBar />
    </div>
  );
}
