/**
 * Status bar showing parse/validation status.
 */
import { useEditorStore } from "../store/editorStore";

export function StatusBar() {
  const ready = useEditorStore((s) => s.ready);
  const gir = useEditorStore((s) => s.gir);
  const validation = useEditorStore((s) => s.validation);
  const error = useEditorStore((s) => s.error);

  const nodeCount = gir?.nodes.length ?? 0;
  const edgeCount = gir?.edges.length ?? 0;

  return (
    <div
      style={{
        height: 28,
        display: "flex",
        alignItems: "center",
        gap: 16,
        padding: "0 12px",
        background: "#1e1e1e",
        borderTop: "1px solid #333",
        color: "#888",
        fontSize: 12,
        fontFamily: "monospace",
      }}
    >
      <span style={{ color: ready ? "#4ec9b0" : "#f44" }}>
        {ready ? "● WASM ready" : "○ Loading…"}
      </span>
      {gir && (
        <span>
          {nodeCount} node{nodeCount !== 1 ? "s" : ""}, {edgeCount} edge
          {edgeCount !== 1 ? "s" : ""}
        </span>
      )}
      {validation && (
        <span style={{ color: validation.valid ? "#4ec9b0" : "#f44" }}>
          {validation.valid
            ? "✓ Valid"
            : `✗ ${validation.errors.length} error${validation.errors.length !== 1 ? "s" : ""}`}
        </span>
      )}
      {error && <span style={{ color: "#f44" }}>Parse error</span>}
      <span style={{ marginLeft: "auto" }}>UL Forge v0.1.0</span>
    </div>
  );
}
