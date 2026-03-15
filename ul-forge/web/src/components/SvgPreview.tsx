/**
 * SVG preview pane — renders the current glyph as SVG.
 */
import { useRef, useEffect } from "react";
import { useEditorStore } from "../store/editorStore";

export function SvgPreview() {
  const svg = useEditorStore((s) => s.svg);
  const error = useEditorStore((s) => s.error);
  const validation = useEditorStore((s) => s.validation);
  const containerRef = useRef<HTMLDivElement>(null);

  // Zoom state
  const scaleRef = useRef(1);

  // Render SVG into container
  useEffect(() => {
    const el = containerRef.current;
    if (!el) return;

    if (svg) {
      el.innerHTML = svg;
      const svgEl = el.querySelector("svg");
      if (svgEl) {
        svgEl.style.width = "100%";
        svgEl.style.height = "100%";
        svgEl.style.transform = `scale(${scaleRef.current})`;
        svgEl.style.transformOrigin = "center center";
      }
    } else {
      el.innerHTML = "";
    }
  }, [svg]);

  // Wheel zoom via non-passive native listener
  useEffect(() => {
    const el = containerRef.current;
    if (!el) return;

    function onWheel(e: WheelEvent) {
      e.preventDefault();
      const delta = e.deltaY > 0 ? 0.9 : 1.1;
      scaleRef.current = Math.max(0.1, Math.min(10, scaleRef.current * delta));
      const svgEl = el!.querySelector("svg");
      if (svgEl) {
        svgEl.style.transform = `scale(${scaleRef.current})`;
      }
    }

    el.addEventListener("wheel", onWheel, { passive: false });
    return () => el.removeEventListener("wheel", onWheel);
  }, []);

  const warnings = validation?.warnings ?? [];

  return (
    <div style={{ height: "100%", display: "flex", flexDirection: "column" }}>
      <div
        ref={containerRef}
        style={{
          flex: 1,
          overflow: "hidden",
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          background: "#1a1a1a",
        }}
      />
      {(error || warnings.length > 0) && (
        <div
          style={{
            padding: "8px 12px",
            background: error ? "#3a1a1a" : "#3a3a1a",
            color: error ? "#ff6b6b" : "#f0e070",
            fontSize: 13,
            fontFamily: "monospace",
            maxHeight: 80,
            overflow: "auto",
          }}
        >
          {error && <div>{error}</div>}
          {warnings.map((w, i) => (
            <div key={i}>⚠ {w}</div>
          ))}
        </div>
      )}
    </div>
  );
}
