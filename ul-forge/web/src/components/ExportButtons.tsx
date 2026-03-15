/**
 * Export controls — download glyph as SVG, PNG, or GIR JSON.
 */
import { useCallback } from "react";
import { useEditorStore } from "../store/editorStore";

function downloadBlob(blob: Blob, filename: string) {
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  a.click();
  URL.revokeObjectURL(url);
}

export function ExportButtons() {
  const svg = useEditorStore((s) => s.svg);
  const gir = useEditorStore((s) => s.gir);

  const exportSvg = useCallback(() => {
    if (!svg) return;
    const blob = new Blob([svg], { type: "image/svg+xml" });
    downloadBlob(blob, "glyph.svg");
  }, [svg]);

  const exportPng = useCallback(() => {
    if (!svg) return;

    const img = new Image();
    const svgBlob = new Blob([svg], { type: "image/svg+xml" });
    const url = URL.createObjectURL(svgBlob);

    img.onload = () => {
      const canvas = document.createElement("canvas");
      const scale = 2; // 2x for retina
      canvas.width = img.naturalWidth * scale;
      canvas.height = img.naturalHeight * scale;
      const ctx = canvas.getContext("2d");
      if (!ctx) return;
      ctx.scale(scale, scale);
      ctx.drawImage(img, 0, 0);
      URL.revokeObjectURL(url);

      canvas.toBlob((blob) => {
        if (blob) downloadBlob(blob, "glyph.png");
      }, "image/png");
    };

    img.src = url;
  }, [svg]);

  const exportGir = useCallback(() => {
    if (!gir) return;
    const json = JSON.stringify(gir, null, 2);
    const blob = new Blob([json], { type: "application/json" });
    downloadBlob(blob, "glyph.gir.json");
  }, [gir]);

  const hasOutput = !!svg;

  return (
    <div style={{ display: "flex", gap: 6 }}>
      <button
        onClick={exportSvg}
        disabled={!hasOutput}
        title="Download as SVG"
        style={buttonStyle(hasOutput)}
      >
        SVG
      </button>
      <button
        onClick={exportPng}
        disabled={!hasOutput}
        title="Download as PNG (2x)"
        style={buttonStyle(hasOutput)}
      >
        PNG
      </button>
      <button
        onClick={exportGir}
        disabled={!gir}
        title="Download GIR JSON"
        style={buttonStyle(!!gir)}
      >
        GIR
      </button>
    </div>
  );
}

function buttonStyle(enabled: boolean): React.CSSProperties {
  return {
    background: enabled ? "#0e639c" : "#333",
    border: "none",
    borderRadius: 3,
    color: enabled ? "#fff" : "#666",
    cursor: enabled ? "pointer" : "default",
    padding: "3px 10px",
    fontSize: 12,
    fontWeight: 500,
  };
}
