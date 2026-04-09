/**
 * Status bar showing parse/validation status, force picker, and pragmatic inference.
 */
import { useState, useCallback } from "react";
import { useEditorStore } from "../store/editorStore";
import { setForce, inferPragmatics } from "../core/index";
import type { ForceName, PragmaticInference } from "../core/index";

const FORCES: ForceName[] = ["assert", "query", "direct", "commit", "express", "declare"];

/** Map error patterns to human-readable explanations and fix suggestions. */
const ERROR_EXPLANATIONS: Array<{ pattern: RegExp; explanation: string; fix: string }> = [
  { pattern: /dangling.*modifier/i, explanation: "A modifier node exists but isn't connected to any entity or relation.", fix: "Connect the modifier with a modified_by edge to the node it describes." },
  { pattern: /dangling.*edge/i, explanation: "An edge references a node that doesn't exist in the graph.", fix: "Add the missing node, or remove the orphaned edge." },
  { pattern: /sort.*mismatch/i, explanation: "An operation received an operand of the wrong algebraic sort.", fix: "Check that entities, relations, modifiers, and assertions are used in the right positions." },
  { pattern: /missing.*subject|missing.*object/i, explanation: "A predication is incomplete — it needs subject (entity), relation, and object (entity).", fix: "Add the missing entity node and connect it with a contains edge inside the assertion frame." },
  { pattern: /empty.*enclosure/i, explanation: "An enclosure node has no children.", fix: "Add at least one child node inside the enclosure, or remove the empty enclosure." },
  { pattern: /cycle/i, explanation: "The graph contains a cycle that violates structural constraints.", fix: "Ensure the containment hierarchy is a DAG — no node should contain itself." },
  { pattern: /invalid.*force/i, explanation: "Unrecognized illocutionary force annotation.", fix: "Use one of: assert, query, direct, commit, express, declare." },
  { pattern: /parse|syntax/i, explanation: "The UL-Script input has a syntax error.", fix: "Check for unmatched brackets, missing arrows (→), or invalid glyph marks." },
];

export function StatusBar() {
  const ready = useEditorStore((s) => s.ready);
  const gir = useEditorStore((s) => s.gir);
  const validation = useEditorStore((s) => s.validation);
  const error = useEditorStore((s) => s.error);
  const setResult = useEditorStore((s) => s.setResult);
  const [inferences, setInferences] = useState<PragmaticInference[]>([]);
  const [showInferences, setShowInferences] = useState(false);
  const [showErrors, setShowErrors] = useState(false);

  const nodeCount = gir?.nodes.length ?? 0;
  const edgeCount = gir?.edges.length ?? 0;

  const handleForce = useCallback(
    (force: ForceName) => {
      if (!gir) return;
      try {
        const result = setForce(gir, force);
        setResult({ gir: result, svg: null, validation: null, error: null });
      } catch (e) {
        setResult({ gir, svg: null, validation: null, error: String(e) });
      }
    },
    [gir, setResult],
  );

  const handlePragmatics = useCallback(() => {
    if (!gir) return;
    try {
      const results = inferPragmatics(gir);
      setInferences(results);
      setShowInferences(true);
    } catch {
      setInferences([]);
    }
  }, [gir]);

  return (
    <>
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
          <span
            style={{ color: validation.valid ? "#4ec9b0" : "#f44", cursor: validation.valid ? "default" : "pointer" }}
            onClick={() => !validation.valid && setShowErrors((v) => !v)}
            title={validation.valid ? "All 4 validation layers passed" : "Click to see error details"}
          >
            {validation.valid
              ? "✓ Valid"
              : `✗ ${validation.errors.length} error${validation.errors.length !== 1 ? "s" : ""} ⓘ`}
          </span>
        )}
        {error && <span style={{ color: "#f44" }}>Parse error</span>}

        {/* Force picker */}
        {gir && (
          <span style={{ display: "flex", alignItems: "center", gap: 4 }}>
            <span style={{ color: "#569cd6" }}>φ</span>
            <select
              onChange={(e) => handleForce(e.target.value as ForceName)}
              defaultValue=""
              style={{
                background: "#333",
                color: "#ccc",
                border: "1px solid #555",
                borderRadius: 3,
                fontSize: 11,
                padding: "1px 4px",
              }}
            >
              <option value="" disabled>force…</option>
              {FORCES.map((f) => (
                <option key={f} value={f}>{f}</option>
              ))}
            </select>
          </span>
        )}

        {/* Pragmatic inference button */}
        {gir && (
          <button
            onClick={handlePragmatics}
            style={{
              background: "#333",
              color: "#ccc",
              border: "1px solid #555",
              borderRadius: 3,
              fontSize: 11,
              padding: "1px 8px",
              cursor: "pointer",
            }}
          >
            ⟹ Infer
          </button>
        )}

        <span style={{ marginLeft: "auto" }}>UL Forge v0.1.0</span>
      </div>

      {/* Validation error details panel */}
      {showErrors && validation && !validation.valid && (
        <div
          style={{
            background: "#2d1515",
            borderTop: "1px solid #5a2020",
            padding: "8px 12px",
            fontSize: 12,
            fontFamily: "monospace",
            color: "#ccc",
            maxHeight: 160,
            overflowY: "auto",
          }}
        >
          <div style={{ display: "flex", justifyContent: "space-between", marginBottom: 4 }}>
            <span style={{ color: "#f44" }}>Validation Errors ({validation.errors.length})</span>
            <button
              onClick={() => setShowErrors(false)}
              style={{ background: "none", border: "none", color: "#888", cursor: "pointer" }}
            >✕</button>
          </div>
          {validation.errors.map((err, i) => {
            const match = ERROR_EXPLANATIONS.find((ex) => ex.pattern.test(err));
            return (
              <div key={i} style={{ marginBottom: 6, borderBottom: "1px solid #3a1010", paddingBottom: 4 }}>
                <div style={{ color: "#f88" }}>✗ {err}</div>
                {match && (
                  <>
                    <div style={{ color: "#aaa", marginLeft: 12 }}>↳ {match.explanation}</div>
                    <div style={{ color: "#4ec9b0", marginLeft: 12 }}>💡 {match.fix}</div>
                  </>
                )}
              </div>
            );
          })}
        </div>
      )}

      {/* Pragmatic inference results panel */}
      {showInferences && inferences.length > 0 && (
        <div
          style={{
            background: "#252526",
            borderTop: "1px solid #333",
            padding: "8px 12px",
            fontSize: 12,
            fontFamily: "monospace",
            color: "#ccc",
            maxHeight: 120,
            overflowY: "auto",
          }}
        >
          <div style={{ display: "flex", justifyContent: "space-between", marginBottom: 4 }}>
            <span style={{ color: "#569cd6" }}>Pragmatic Inferences ({inferences.length})</span>
            <button
              onClick={() => setShowInferences(false)}
              style={{ background: "none", border: "none", color: "#888", cursor: "pointer" }}
            >✕</button>
          </div>
          {inferences.map((inf, i) => (
            <div key={i} style={{ color: "#9cdcfe", marginBottom: 2 }}>
              <span style={{ color: "#dcdcaa" }}>{inf.rule}</span>: surface ⟹ intended
            </div>
          ))}
        </div>
      )}
    </>
  );
}
