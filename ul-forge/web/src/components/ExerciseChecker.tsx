/**
 * ExerciseChecker — interactive exercise verification panel.
 *
 * Select an exercise, type your UL-Script answer, and get instant feedback
 * on whether the expression parses, validates, and uses the expected operations.
 */
import { useState, useCallback } from "react";
import { parse, validate, detectOperations } from "../core/index";

interface Exercise {
  id: string;
  label: string;
  prompt: string;
  expectedOps: string[];
}

const EXERCISES: Exercise[] = [
  { id: "2.1", label: "2.1: Simple Predication", prompt: "Encode: \"Fire is hot.\"", expectedOps: ["predicate"] },
  { id: "2.2", label: "2.2: Modified Entity", prompt: "Encode: \"The ancient tree stands tall.\"", expectedOps: ["modify_entity", "predicate"] },
  { id: "2.3", label: "2.3: Compound Assertion", prompt: "Encode: \"The sun rises AND the birds sing.\"", expectedOps: ["conjoin"] },
  { id: "3.1", label: "3.1: Embedding", prompt: "\"The fact that rain falls makes farmers happy.\"", expectedOps: ["embed", "predicate", "modify_entity"] },
  { id: "3.2", label: "3.2: Binding", prompt: "Encode: \"Every dog has a bone.\"", expectedOps: ["quantify", "bind", "predicate"] },
];

interface CheckResult {
  ok: boolean;
  parsed: boolean;
  valid: boolean;
  detectedOps: string[];
  missing: string[];
  message: string;
}

function check(input: string, expectedOps: string[]): CheckResult {
  try {
    const gir = parse(input);
    const vr = validate(gir);
    if (!vr.valid) {
      return { ok: false, parsed: true, valid: false, detectedOps: [], missing: expectedOps, message: `Validation errors: ${vr.errors.join("; ")}` };
    }
    const detected = detectOperations(gir);
    const missing = expectedOps.filter((op) => !detected.includes(op));
    if (missing.length === 0) {
      return { ok: true, parsed: true, valid: true, detectedOps: detected, missing: [], message: "Correct! All expected operations detected." };
    }
    return { ok: false, parsed: true, valid: true, detectedOps: detected, missing, message: `Missing operations: ${missing.join(", ")}. Detected: ${detected.join(", ") || "(none)"}` };
  } catch (e) {
    return { ok: false, parsed: false, valid: false, detectedOps: [], missing: expectedOps, message: `Parse error: ${e instanceof Error ? e.message : String(e)}` };
  }
}

const panelStyle: React.CSSProperties = {
  padding: "12px 16px",
  background: "#252526",
  borderTop: "1px solid #333",
  fontFamily: "system-ui, sans-serif",
  fontSize: 13,
};

const inputStyle: React.CSSProperties = {
  width: "100%",
  background: "#2d2d2d",
  color: "#d4d4d4",
  border: "1px solid #555",
  borderRadius: 3,
  padding: "6px 8px",
  fontSize: 13,
  fontFamily: "monospace",
  boxSizing: "border-box",
};

export function ExerciseChecker() {
  const [selectedIdx, setSelectedIdx] = useState(0);
  const [answer, setAnswer] = useState("");
  const [result, setResult] = useState<CheckResult | null>(null);

  const exercise = EXERCISES[selectedIdx];

  const handleCheck = useCallback(() => {
    if (!answer.trim()) return;
    setResult(check(answer, exercise.expectedOps));
  }, [answer, exercise]);

  return (
    <div style={panelStyle}>
      <div style={{ display: "flex", alignItems: "center", gap: 8, marginBottom: 8 }}>
        <span style={{ fontWeight: 600 }}>Exercise:</span>
        <select
          value={selectedIdx}
          onChange={(e) => { setSelectedIdx(Number(e.target.value)); setResult(null); setAnswer(""); }}
          style={{ background: "#2d2d2d", color: "#d4d4d4", border: "1px solid #555", borderRadius: 3, padding: "2px 6px", fontSize: 12 }}
        >
          {EXERCISES.map((ex, i) => (
            <option key={ex.id} value={i}>{ex.label}</option>
          ))}
        </select>
      </div>

      <div style={{ marginBottom: 8, color: "#9cdcfe" }}>{exercise.prompt}</div>

      <div style={{ display: "flex", gap: 8 }}>
        <input
          type="text"
          value={answer}
          onChange={(e) => setAnswer(e.target.value)}
          onKeyDown={(e) => { if (e.key === "Enter") handleCheck(); }}
          placeholder="Type your UL-Script answer..."
          style={{ ...inputStyle, flex: 1 }}
        />
        <button
          onClick={handleCheck}
          style={{
            background: "#094771",
            color: "#fff",
            border: "1px solid #555",
            borderRadius: 3,
            padding: "4px 12px",
            cursor: "pointer",
            fontSize: 12,
            whiteSpace: "nowrap",
          }}
        >
          Check
        </button>
      </div>

      {result && (
        <div
          style={{
            marginTop: 8,
            padding: "6px 10px",
            borderRadius: 3,
            background: result.ok ? "#1a3a1a" : "#3a1a1a",
            border: `1px solid ${result.ok ? "#2d6b2d" : "#6b2d2d"}`,
            fontSize: 12,
          }}
        >
          {result.ok ? "\u2705 " : "\u274c "}
          {result.message}
        </div>
      )}
    </div>
  );
}
