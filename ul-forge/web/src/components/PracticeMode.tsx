/**
 * PracticeMode — load game levels and evaluate compositions in real-time.
 *
 * Loads level JSON files from /levels/, presents exercises sequentially,
 * and uses the game evaluation engine for scoring.
 */
import { useState, useEffect, useCallback } from "react";
import { parse, validate, detectOperations, createContext, evaluate } from "../core/index";
import type { Gir } from "../core/index";

interface LevelExercise {
  prompt: string;
  hint: string;
  expectedOps: string[];
  sampleAnswer: string;
  force?: string;
}

interface Level {
  level: number;
  title: string;
  description: string;
  exercises: LevelExercise[];
}

const panelStyle: React.CSSProperties = {
  padding: 16,
  background: "#1e1e1e",
  color: "#d4d4d4",
  fontFamily: "system-ui, sans-serif",
  fontSize: 13,
  overflowY: "auto",
  height: "100%",
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

interface Feedback {
  score: number;
  grade: string;
  opsOk: boolean;
  message: string;
}

function evaluateAnswer(input: string, expectedOps: string[]): Feedback {
  let gir: Gir;
  try {
    gir = parse(input);
  } catch (e) {
    return { score: 0, grade: "F", opsOk: false, message: `Parse error: ${e instanceof Error ? e.message : String(e)}` };
  }
  const vr = validate(gir);
  if (!vr.valid) {
    return { score: 0.1, grade: "F", opsOk: false, message: `Validation: ${vr.errors.join("; ")}` };
  }
  const detected = detectOperations(gir);
  const missing = expectedOps.filter((op) => !detected.includes(op));
  const opsOk = missing.length === 0;

  // Use game evaluation for scoring
  let evalScore = 0.5;
  try {
    const ctxId = createContext({});
    const result = evaluate(ctxId, gir);
    evalScore = result.score;
  } catch {
    // fallback score based on ops match
  }

  const finalScore = opsOk ? Math.max(evalScore, 0.7) : evalScore * 0.5;
  const grade = finalScore >= 0.9 ? "A" : finalScore >= 0.7 ? "B" : finalScore >= 0.5 ? "C" : "D";
  const msg = opsOk
    ? `Operations match! Score: ${(finalScore * 100).toFixed(0)}%`
    : `Missing operations: ${missing.join(", ")}. Detected: ${detected.join(", ") || "(none)"}`;

  return { score: finalScore, grade, opsOk, message: msg };
}

export function PracticeMode() {
  const [levels, setLevels] = useState<Level[]>([]);
  const [currentLevel, setCurrentLevel] = useState(0);
  const [currentExercise, setCurrentExercise] = useState(0);
  const [answer, setAnswer] = useState("");
  const [feedback, setFeedback] = useState<Feedback | null>(null);
  const [showHint, setShowHint] = useState(false);
  const [completed, setCompleted] = useState<Set<string>>(new Set());

  useEffect(() => {
    Promise.all(
      [1, 2, 3, 4, 5].map((n) =>
        fetch(`/levels/level-${n}.json`).then((r) => r.json() as Promise<Level>)
      )
    ).then(setLevels).catch(() => {});
  }, []);

  const level = levels[currentLevel];
  const exercise = level?.exercises[currentExercise];

  const handleCheck = useCallback(() => {
    if (!exercise || !answer.trim()) return;
    const fb = evaluateAnswer(answer, exercise.expectedOps);
    setFeedback(fb);
    if (fb.opsOk) {
      setCompleted((s) => new Set(s).add(`${currentLevel}-${currentExercise}`));
    }
  }, [answer, exercise, currentLevel, currentExercise]);

  const handleNext = useCallback(() => {
    if (!level) return;
    if (currentExercise < level.exercises.length - 1) {
      setCurrentExercise((i) => i + 1);
    } else if (currentLevel < levels.length - 1) {
      setCurrentLevel((l) => l + 1);
      setCurrentExercise(0);
    }
    setAnswer("");
    setFeedback(null);
    setShowHint(false);
  }, [level, currentExercise, currentLevel, levels.length]);

  if (!level || !exercise) {
    return <div style={panelStyle}>Loading levels...</div>;
  }

  const total = levels.reduce((s, l) => s + l.exercises.length, 0);
  const done = completed.size;

  return (
    <div style={panelStyle}>
      <div style={{ display: "flex", justifyContent: "space-between", marginBottom: 12 }}>
        <div>
          <strong>Level {level.level}:</strong> {level.title}
          <span style={{ color: "#888", marginLeft: 8 }}>({currentExercise + 1}/{level.exercises.length})</span>
        </div>
        <div style={{ fontSize: 11, color: "#888" }}>
          Progress: {done}/{total}
        </div>
      </div>

      <div style={{ marginBottom: 8 }}>{exercise.prompt}</div>

      <div style={{ display: "flex", gap: 8, marginBottom: 8 }}>
        <input
          type="text"
          value={answer}
          onChange={(e) => setAnswer(e.target.value)}
          onKeyDown={(e) => { if (e.key === "Enter") handleCheck(); }}
          placeholder="Type your UL-Script answer..."
          style={{ ...inputStyle, flex: 1 }}
        />
        <button onClick={handleCheck} style={{ background: "#094771", color: "#fff", border: "1px solid #555", borderRadius: 3, padding: "4px 12px", cursor: "pointer", fontSize: 12 }}>
          Check
        </button>
        <button onClick={() => setShowHint((h) => !h)} style={{ background: "none", color: "#9cdcfe", border: "1px solid #555", borderRadius: 3, padding: "4px 8px", cursor: "pointer", fontSize: 12 }}>
          Hint
        </button>
        <button onClick={handleNext} style={{ background: "#333", color: "#d4d4d4", border: "1px solid #555", borderRadius: 3, padding: "4px 8px", cursor: "pointer", fontSize: 12 }}>
          Next →
        </button>
      </div>

      {showHint && <div style={{ color: "#ce9178", fontSize: 12, marginBottom: 6 }}>💡 {exercise.hint}</div>}

      {feedback && (
        <div style={{
          padding: "6px 10px",
          borderRadius: 3,
          background: feedback.opsOk ? "#1a3a1a" : "#3a1a1a",
          border: `1px solid ${feedback.opsOk ? "#2d6b2d" : "#6b2d2d"}`,
          fontSize: 12,
        }}>
          <strong>{feedback.grade}</strong> — {feedback.message}
        </div>
      )}

      {/* Level selector */}
      <div style={{ marginTop: 16, display: "flex", gap: 4 }}>
        {levels.map((l, i) => (
          <button
            key={l.level}
            onClick={() => { setCurrentLevel(i); setCurrentExercise(0); setAnswer(""); setFeedback(null); setShowHint(false); }}
            style={{
              background: i === currentLevel ? "#094771" : "#333",
              color: "#d4d4d4",
              border: "1px solid #555",
              borderRadius: 3,
              padding: "2px 8px",
              cursor: "pointer",
              fontSize: 11,
            }}
          >
            L{l.level}
          </button>
        ))}
      </div>
    </div>
  );
}
