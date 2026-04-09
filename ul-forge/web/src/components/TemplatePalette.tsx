/**
 * Template palette sidebar — 42 canonical glyph templates (Tier 1–3).
 *
 * Click a template to insert its UL-Script at the editor cursor.
 * The 42 entries correspond to the Universal Language lexicon.
 */
import { useCallback, useState } from "react";
import { useEditorStore } from "../store/editorStore";

export interface Template {
  name: string;
  script: string;
  symbol: string;
  description: string;
}

export interface TemplateGroup {
  tier: string;
  label: string;
  templates: Template[];
}

// ── Tier 1: Geometrically Forced (24) ─────────────────────────────
const TIER1: Template[] = [
  // 5 Primitives
  { name: "Existence",      script: "●",             symbol: "●",  description: "Point — fundamental entity" },
  { name: "Relation",       script: "● → ●",         symbol: "→",  description: "Directed line between entities" },
  { name: "Quality",        script: "∠60",            symbol: "∠",  description: "Angle — relation modifier" },
  { name: "Process",        script: "~",              symbol: "~",  description: "Curve — continuous path" },
  { name: "Concept",        script: "○",              symbol: "○",  description: "Circle enclosure — bounded region" },
  // Enclosure shapes
  { name: "Triangle",       script: "△",              symbol: "△",  description: "Triangle enclosure (D₃)" },
  { name: "Square",         script: "□",              symbol: "□",  description: "Square enclosure (D₄)" },
  // Distinguished angles
  { name: "Coincidence",    script: "∠0",             symbol: "∠₀", description: "0° — identity / agreement" },
  { name: "Perpendicularity", script: "∠90",          symbol: "⊥",  description: "90° — orthogonality" },
  { name: "Opposition",     script: "∠180",           symbol: "⊣",  description: "180° — full reversal" },
  { name: "Full Rotation",  script: "∠360",           symbol: "⟳",  description: "360° — complete cycle" },
  // Entity / relation modification
  { name: "Modify Entity",  script: "∠60 ●",          symbol: "m·e", description: "Angle applied to entity" },
  { name: "Modify Relation", script: "● →∠45 ●",     symbol: "m·r", description: "Angle applied to connection" },
  { name: "Coincident Pts", script: "● | ●",          symbol: "●●", description: "Entities at same position" },
  // Logical operations
  { name: "Negation",       script: "○{∠180 ● → ●}", symbol: "¬",  description: "Negated assertion" },
  { name: "Conjunction",    script: "○ & ○",          symbol: "∧",  description: "AND — overlapping regions" },
  { name: "Disjunction",    script: "○ | ○",          symbol: "∨",  description: "OR — adjacent regions" },
  // Structural operations
  { name: "Embedding",      script: "○{● → ●}",      symbol: "⌈⌉", description: "Statement → entity" },
  { name: "Abstraction",    script: "△{∠60}",         symbol: "⌊⌋", description: "Entity → modifier" },
  { name: "Composition",    script: "● → ● → ●",     symbol: "⟹",  description: "Chained relations" },
  { name: "Inversion",      script: "● ← ●",         symbol: "←",  description: "Reversed relation" },
  // Extended
  { name: "Cycle",          script: "○{~}",           symbol: "↻",  description: "Closed curve process" },
  { name: "Void Enclosure", script: "○{□}",           symbol: "◌",  description: "Bounded emptiness" },
  { name: "Containment",    script: "○{○}",           symbol: "⊙",  description: "Enclosure in enclosure" },
];

// ── Tier 2: Structurally Distinguished (14) ───────────────────────
const TIER2: Template[] = [
  { name: "Ray",            script: "● →",            symbol: "⟶",  description: "Directed line, open-ended" },
  { name: "Harmony",        script: "∠60",            symbol: "♮",  description: "60° — equilateral harmony" },
  { name: "Efficiency",     script: "∠120",           symbol: "⚡", description: "120° — optimal packing" },
  { name: "Foundation",     script: "□",              symbol: "▣",  description: "Square — stable base (D₄)" },
  { name: "Spiral",         script: "○{~ → ●}",      symbol: "🌀", description: "Growing curve — expansion" },
  { name: "Wave",           script: "~ | ~ | ~",      symbol: "≋",  description: "Periodic curve — oscillation" },
  { name: "Point in Circle", script: "○{●}",          symbol: "◉",  description: "Entity within concept" },
  { name: "Point in Triangle", script: "△{●}",        symbol: "⏶",  description: "Entity within structure" },
  { name: "Point in Square", script: "□{●}",          symbol: "⊡",  description: "Entity within foundation" },
  { name: "Duality",        script: "● ↔ ●",         symbol: "⇔",  description: "Symmetric opposition" },
  { name: "Trinity",        script: "△{● | ● | ●}",  symbol: "⊛",  description: "Three in harmony" },
  { name: "Self-Nesting",   script: "○{○{○}}",        symbol: "◎",  description: "Recursive enclosure" },
  { name: "Quantification", script: "∠90 ●",          symbol: "∀",  description: "Quantifier on entity" },
  { name: "Ray from Point", script: "● →",            symbol: "↗",  description: "Unbounded relation" },
];

// ── Tier 3: Conventional (4) ──────────────────────────────────────
const TIER3: Template[] = [
  { name: "Entity → Concept", script: "● → ○",        symbol: "→○", description: "Entity relates to concept" },
  { name: "Concept ↔ Concept", script: "○ ↔ ○",      symbol: "⇿",  description: "Concepts in dialogue" },
  { name: "Nested Process",   script: "○{~ → ○{●}}", symbol: "⧫",  description: "Process targeting concept" },
  { name: "Full Glyph",       script: "○{● →∠60 △{● | ~}}", symbol: "✦", description: "Multi-primitive composition" },
];

// ── Extensions: Modal, Performative, Pragmatic ───────────────────
const EXTENSIONS: Template[] = [
  { name: "Necessity",        script: "[]{○{● → ●}}",             symbol: "□",  description: "Necessarily: solid bold border" },
  { name: "Possibility",      script: "<>{○{● → ●}}",             symbol: "◇",  description: "Possibly: dashed border" },
  { name: "Counterfactual",   script: "[]->{○{● → ●}}{○{● → ●}}", symbol: "□→", description: "If P were true, Q" },
  { name: "Query",            script: "query{○{● → ●}}",          symbol: "?",  description: "Question force" },
  { name: "Direct",           script: "direct{○{● → ●}}",         symbol: "!",  description: "Command force" },
  { name: "Commit",           script: "commit{○{● → ●}}",         symbol: "⊢",  description: "Promise force" },
  { name: "Express",          script: "express{○{● → ●}}",        symbol: "♡",  description: "Expressive force" },
  { name: "Declare",          script: "declare{○{● → ●}}",        symbol: "⊨",  description: "Declarative force" },
  { name: "Evidential",       script: "?{○{● → ●}}",              symbol: "?…", description: "Evidential modification" },
  { name: "Emphatic",         script: "!{○{● → ●}}",              symbol: "!…", description: "Emphatic modification" },
  { name: "Hedged",           script: "~?{○{● → ●}}",             symbol: "~?", description: "Hedged modification" },
];

export const TEMPLATE_GROUPS: TemplateGroup[] = [
  { tier: "T1", label: "Geometrically Forced", templates: TIER1 },
  { tier: "T2", label: "Structurally Distinguished", templates: TIER2 },
  { tier: "T3", label: "Conventional", templates: TIER3 },
  { tier: "EXT", label: "Modal / Force / Pragmatic", templates: EXTENSIONS },
];

/** Flat list of all 42 templates */
export const ALL_TEMPLATES: Template[] = [
  ...TIER1, ...TIER2, ...TIER3,
];

interface TemplatePaletteProps {
  visible: boolean;
  onClose: () => void;
}

export function TemplatePalette({ visible, onClose }: TemplatePaletteProps) {
  const setSource = useEditorStore((s) => s.setSource);
  const source = useEditorStore((s) => s.source);
  const [collapsed, setCollapsed] = useState<Record<string, boolean>>({});

  const insertTemplate = useCallback(
    (script: string) => {
      const newSource = source.trim()
        ? `${source}\n${script}`
        : script;
      setSource(newSource);
    },
    [source, setSource],
  );

  const toggleTier = useCallback((tier: string) => {
    setCollapsed((prev) => ({ ...prev, [tier]: !prev[tier] }));
  }, []);

  if (!visible) return null;

  return (
    <div
      style={{
        width: 240,
        height: "100%",
        background: "#252526",
        borderRight: "1px solid #333",
        display: "flex",
        flexDirection: "column",
        overflow: "hidden",
      }}
    >
      {/* Header */}
      <div
        style={{
          padding: "10px 12px",
          borderBottom: "1px solid #333",
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
          fontSize: 13,
          fontWeight: 600,
        }}
      >
        <span>Templates (42)</span>
        <button
          onClick={onClose}
          style={{
            background: "none",
            border: "none",
            color: "#888",
            cursor: "pointer",
            fontSize: 16,
            padding: "0 4px",
          }}
        >
          ✕
        </button>
      </div>

      {/* Template list grouped by tier */}
      <div style={{ flex: 1, overflow: "auto", padding: "4px 0" }}>
        {TEMPLATE_GROUPS.map((group) => (
          <div key={group.tier}>
            {/* Tier header */}
            <button
              onClick={() => toggleTier(group.tier)}
              style={{
                display: "flex",
                alignItems: "center",
                gap: 6,
                width: "100%",
                padding: "6px 12px",
                background: "#1e1e1e",
                border: "none",
                borderBottom: "1px solid #333",
                color: "#569cd6",
                cursor: "pointer",
                fontSize: 11,
                fontWeight: 600,
                textTransform: "uppercase",
                letterSpacing: "0.5px",
              }}
            >
              <span style={{ fontSize: 9 }}>{collapsed[group.tier] ? "▶" : "▼"}</span>
              {group.label} ({group.templates.length})
            </button>

            {/* Templates in this tier */}
            {!collapsed[group.tier] &&
              group.templates.map((t) => (
                <button
                  key={`${group.tier}-${t.name}`}
                  onClick={() => insertTemplate(t.script)}
                  title={`${t.name}: ${t.description}\nInsert: ${t.script}`}
                  style={{
                    display: "flex",
                    alignItems: "center",
                    gap: 10,
                    width: "100%",
                    padding: "8px 12px",
                    background: "none",
                    border: "none",
                    borderBottom: "1px solid #2a2a2a",
                    color: "#d4d4d4",
                    cursor: "pointer",
                    textAlign: "left",
                    fontSize: 13,
                  }}
                  onMouseEnter={(e) => {
                    e.currentTarget.style.background = "#2a2d2e";
                  }}
                  onMouseLeave={(e) => {
                    e.currentTarget.style.background = "none";
                  }}
                >
                  <span
                    style={{
                      fontSize: 22,
                      width: 32,
                      textAlign: "center",
                      flexShrink: 0,
                    }}
                  >
                    {t.symbol}
                  </span>
                  <div>
                    <div style={{ fontWeight: 500 }}>{t.name}</div>
                    <div
                      style={{
                        fontSize: 11,
                        color: "#888",
                        fontFamily: "monospace",
                      }}
                    >
                      {t.script}
                    </div>
                  </div>
                </button>
              ))}
          </div>
        ))}
      </div>
    </div>
  );
}
