/**
 * OperationComposer — UI for applying Σ_UL algebraic operations.
 *
 * Select an operation from the dropdown, provide operands (as UL-Script text),
 * and apply. The result is inserted into the editor.
 */
import { useState, useCallback } from "react";
import { useEditorStore } from "../store/editorStore";
import { applyOperation, parse, deparse, setForce } from "../core/index";
import type { OperationName, ForceName } from "../core/index";

interface OperationDef {
  name: string;
  arity: number;
  signature: string;
  description: string;
  placeholders: string[];
}

const OPERATIONS: OperationDef[] = [
  { name: "predicate", arity: 3, signature: "e × r × e → a", description: "Create a statement from subject, relation, object", placeholders: ["Subject (entity)", "Relation", "Object (entity)"] },
  { name: "modify_entity", arity: 2, signature: "m × e → e", description: "Apply a modifier to an entity", placeholders: ["Modifier", "Entity"] },
  { name: "modify_relation", arity: 2, signature: "m × r → r", description: "Apply a modifier to a relation", placeholders: ["Modifier", "Relation"] },
  { name: "negate", arity: 1, signature: "a → a", description: "Negate a statement", placeholders: ["Assertion"] },
  { name: "conjoin", arity: 2, signature: "a × a → a", description: "AND — combine two statements", placeholders: ["Assertion 1", "Assertion 2"] },
  { name: "disjoin", arity: 2, signature: "a × a → a", description: "OR — combine two statements", placeholders: ["Assertion 1", "Assertion 2"] },
  { name: "embed", arity: 1, signature: "a → e", description: "Turn a statement into an entity (nominalization)", placeholders: ["Assertion"] },
  { name: "abstract", arity: 1, signature: "e → m", description: "Turn an entity into a modifier", placeholders: ["Entity"] },
  { name: "compose", arity: 2, signature: "r × r → r", description: "Chain two relations", placeholders: ["Relation 1", "Relation 2"] },
  { name: "invert", arity: 1, signature: "r → r", description: "Reverse a relation", placeholders: ["Relation"] },
  { name: "quantify", arity: 2, signature: "m × e → a", description: "Quantify over an entity", placeholders: ["Quantifier (modifier)", "Entity"] },
  { name: "bind", arity: 2, signature: "e × a → a", description: "Bind a variable in an assertion", placeholders: ["Variable (entity)", "Assertion"] },
  { name: "modify_assertion", arity: 2, signature: "m × a → a", description: "Modify assertion (evidentiality, emphasis)", placeholders: ["Modifier", "Assertion"] },
  // ── Extensions (modal, performative, pragmatic) ──
  { name: "necessity", arity: 1, signature: "a → a", description: "□ — assert something is necessarily true", placeholders: ["Assertion"] },
  { name: "possibility", arity: 1, signature: "a → a", description: "◇ — assert something is possibly true", placeholders: ["Assertion"] },
  { name: "counterfactual", arity: 2, signature: "a × a → a", description: "□→ — if A were true, then B", placeholders: ["Antecedent", "Consequent"] },
  { name: "set_force", arity: 1, signature: "a → a", description: "Set performative force (use force selector below)", placeholders: ["Assertion"] },
];

const inputStyle: React.CSSProperties = {
  width: "100%",
  background: "#2d2d2d",
  color: "#d4d4d4",
  border: "1px solid #555",
  borderRadius: 3,
  padding: "4px 8px",
  fontSize: 12,
  fontFamily: "monospace",
};

export function OperationComposer() {
  const [selectedOp, setSelectedOp] = useState<string>("predicate");
  const [operands, setOperands] = useState<string[]>(["", "", ""]);
  const [result, setResult] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [collapsed, setCollapsed] = useState(true);
  const [force, setForceType] = useState<ForceName>("query");
  const setSource = useEditorStore((s) => s.setSource);

  const op = OPERATIONS.find((o) => o.name === selectedOp) ?? OPERATIONS[0];

  const handleOpChange = useCallback((name: string) => {
    setSelectedOp(name);
    const def = OPERATIONS.find((o) => o.name === name);
    setOperands(new Array(def?.arity ?? 1).fill(""));
    setResult(null);
    setError(null);
  }, []);

  const handleApply = useCallback(() => {
    try {
      const args = operands.slice(0, op.arity);
      const girs = args.map((text) => parse(text));
      let res;
      if (op.name === "set_force") {
        res = setForce(girs[0], force);
      } else {
        res = applyOperation(op.name as OperationName, girs);
      }
      const deparsed = deparse(res);
      setResult(deparsed);
      setError(null);
    } catch (e) {
      setError(String(e));
      setResult(null);
    }
  }, [op, operands, force]);

  const handleInsert = useCallback(() => {
    if (!result) return;
    setSource(result);
  }, [result, setSource]);

  if (collapsed) {
    return (
      <div
        style={{
          padding: "4px 12px",
          background: "#1e1e1e",
          borderBottom: "1px solid #333",
          fontSize: 12,
          fontFamily: "monospace",
          color: "#569cd6",
          cursor: "pointer",
        }}
        onClick={() => setCollapsed(false)}
      >
        Σ Operation Composer ▸
      </div>
    );
  }

  return (
    <div
      style={{
        padding: "8px 12px",
        background: "#1e1e1e",
        borderBottom: "1px solid #333",
        fontSize: 12,
        fontFamily: "monospace",
        color: "#d4d4d4",
      }}
    >
      <div style={{ display: "flex", justifyContent: "space-between", marginBottom: 8 }}>
        <span style={{ color: "#569cd6", fontWeight: "bold" }}>Σ Operation Composer</span>
        <button
          onClick={() => setCollapsed(true)}
          style={{ background: "none", border: "none", color: "#888", cursor: "pointer" }}
        >▾</button>
      </div>

      {/* Operation selector */}
      <div style={{ marginBottom: 6 }}>
        <select
          value={selectedOp}
          onChange={(e) => handleOpChange(e.target.value)}
          style={{
            background: "#333",
            color: "#d4d4d4",
            border: "1px solid #555",
            borderRadius: 3,
            fontSize: 12,
            padding: "2px 6px",
            width: "100%",
          }}
        >
          {OPERATIONS.map((o) => (
            <option key={o.name} value={o.name}>
              {o.name} : {o.signature}
            </option>
          ))}
        </select>
        <div style={{ color: "#888", marginTop: 2 }}>{op.description}</div>
      </div>

      {/* Operand inputs */}
      {op.placeholders.map((ph, i) => (
        <div key={i} style={{ marginBottom: 4 }}>
          <label style={{ color: "#888", display: "block", marginBottom: 2 }}>{ph}:</label>
          <input
            type="text"
            style={inputStyle}
            placeholder={`UL-Script for ${ph.toLowerCase()}`}
            value={operands[i] || ""}
            onChange={(e) => {
              const next = [...operands];
              next[i] = e.target.value;
              setOperands(next);
            }}
          />
        </div>
      ))}

      {/* Force selector (shown only for set_force) */}
      {selectedOp === "set_force" && (
        <div style={{ marginBottom: 4 }}>
          <label style={{ color: "#888", display: "block", marginBottom: 2 }}>Force:</label>
          <select
            value={force}
            onChange={(e) => setForceType(e.target.value as ForceName)}
            style={{
              background: "#333",
              color: "#d4d4d4",
              border: "1px solid #555",
              borderRadius: 3,
              fontSize: 12,
              padding: "2px 6px",
              width: "100%",
            }}
          >
            {(["assert", "query", "direct", "commit", "express", "declare"] as ForceName[]).map((f) => (
              <option key={f} value={f}>{f}</option>
            ))}
          </select>
        </div>
      )}

      {/* Apply button */}
      <div style={{ display: "flex", gap: 8, marginTop: 8 }}>
        <button
          onClick={handleApply}
          style={{
            background: "#264f78",
            color: "#d4d4d4",
            border: "1px solid #569cd6",
            borderRadius: 3,
            padding: "4px 12px",
            cursor: "pointer",
            fontSize: 12,
          }}
        >
          Apply {op.name}
        </button>
        {result && (
          <button
            onClick={handleInsert}
            style={{
              background: "#1e3a1e",
              color: "#4ec9b0",
              border: "1px solid #4ec9b0",
              borderRadius: 3,
              padding: "4px 12px",
              cursor: "pointer",
              fontSize: 12,
            }}
          >
            Insert into Editor
          </button>
        )}
      </div>

      {/* Result / Error */}
      {result && (
        <pre
          style={{
            marginTop: 8,
            padding: 8,
            background: "#1a2a1a",
            border: "1px solid #333",
            borderRadius: 3,
            color: "#4ec9b0",
            maxHeight: 120,
            overflowY: "auto",
            whiteSpace: "pre-wrap",
            fontSize: 11,
          }}
        >
          {result}
        </pre>
      )}
      {error && (
        <div style={{ marginTop: 8, color: "#f44" }}>✗ {error}</div>
      )}
    </div>
  );
}
