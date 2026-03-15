/**
 * Visual Canvas — interactive SVG canvas for drag-and-drop glyph composition.
 *
 * Maintains a local scene model (nodes + edges + positions) that syncs
 * bidirectionally with the text source via UL-Script generation.
 *
 * Primitives: click toolbar button then click canvas to place.
 * Connections: click "Connect" tool, then click source node, then target node.
 * Selection: click a node to select; press Delete to remove.
 * Dragging: in Select mode, drag nodes to reposition.
 */
import { useState, useRef, useCallback, useEffect } from "react";
import { useEditorStore } from "../store/editorStore";
import type { Node as GirNode, Edge as GirEdge, NodeType, EnclosureShape } from "../core/index";

// ── Local scene types ──

interface CanvasNode {
  id: string;
  type: NodeType;
  shape?: EnclosureShape;
  degrees?: number;
  x: number;
  y: number;
}

interface CanvasEdge {
  from: string;
  to: string;
  type: "connects" | "contains";
}

type Tool = "select" | "point" | "circle" | "triangle" | "square" | "curve" | "angle" | "connect";

// ── UL-Script generation from scene ──

function nodeToScript(node: CanvasNode): string {
  switch (node.type) {
    case "point":
      return "●";
    case "enclosure":
      switch (node.shape) {
        case "triangle": return "△";
        case "square":   return "□";
        default:         return "○";
      }
    case "curve":
      return "~";
    case "angle":
      return `∠${node.degrees ?? 60}`;
    case "line":
      return "→";
    default:
      return "●";
  }
}

function sceneToScript(nodes: CanvasNode[], edges: CanvasEdge[]): string {
  if (nodes.length === 0) return "";

  // Build adjacency: from → [{to, type}]
  const adj = new Map<string, { to: string; type: string }[]>();
  for (const edge of edges) {
    if (!adj.has(edge.from)) adj.set(edge.from, []);
    adj.get(edge.from)!.push({ to: edge.to, type: edge.type });
  }

  // Find root nodes (no incoming "connects" edges)
  const hasIncoming = new Set(
    edges.filter((e) => e.type === "connects").map((e) => e.to),
  );
  // Also mark contained nodes
  const isContained = new Set(
    edges.filter((e) => e.type === "contains").map((e) => e.to),
  );

  const roots = nodes.filter(
    (n) => !hasIncoming.has(n.id) && !isContained.has(n.id),
  );

  // If nothing is a root, just use all nodes (disconnected)
  const starts = roots.length > 0 ? roots : nodes;

  const visited = new Set<string>();

  function generate(nodeId: string): string {
    if (visited.has(nodeId)) return "";
    visited.add(nodeId);

    const node = nodes.find((n) => n.id === nodeId);
    if (!node) return "";

    let script = nodeToScript(node);
    const children = adj.get(nodeId) || [];

    // Handle containment: enclosure { contents }
    const contained = children.filter((c) => c.type === "contains");
    if (contained.length > 0 && node.type === "enclosure") {
      const inner = contained
        .map((c) => generate(c.to))
        .filter(Boolean)
        .join(" | ");
      if (inner) {
        script = `${script}{${inner}}`;
      }
    }

    // Handle connections: node → target
    const connected = children.filter((c) => c.type === "connects");
    if (connected.length > 0) {
      const chain = connected
        .map((c) => generate(c.to))
        .filter(Boolean)
        .join(" → ");
      if (chain) {
        script = `${script} → ${chain}`;
      }
    }

    return script;
  }

  const lines = starts
    .map((r) => generate(r.id))
    .filter(Boolean);

  // Any remaining unvisited nodes get their own line
  for (const node of nodes) {
    if (!visited.has(node.id)) {
      lines.push(nodeToScript(node));
    }
  }

  return lines.join("\n");
}

// ── Build scene from GIR ──

function girToScene(
  girNodes: GirNode[],
  girEdges: GirEdge[],
  prevPositions: Map<string, { x: number; y: number }>,
): { nodes: CanvasNode[]; edges: CanvasEdge[] } {
  const nodes: CanvasNode[] = [];
  const cols = Math.max(3, Math.ceil(Math.sqrt(girNodes.length)));
  const spacing = 80;
  const offset = 60;

  girNodes.forEach((gn, i) => {
    const prev = prevPositions.get(gn.id);
    nodes.push({
      id: gn.id,
      type: gn.type,
      shape: gn.shape,
      degrees: gn.measure,
      x: prev ? prev.x : offset + (i % cols) * spacing,
      y: prev ? prev.y : offset + Math.floor(i / cols) * spacing,
    });
  });

  const edges: CanvasEdge[] = girEdges
    .filter((e) => e.type === "connects" || e.type === "contains")
    .map((e) => ({
      from: e.source,
      to: e.target,
      type: e.type as "connects" | "contains",
    }));

  return { nodes, edges };
}

// ── Node rendering ──

const NODE_SIZE: Record<NodeType, number> = {
  point: 10,
  enclosure: 30,
  line: 10,
  angle: 18,
  curve: 18,
};

const NODE_COLORS: Record<NodeType, { fill: string; stroke: string }> = {
  point:     { fill: "#d4d4d4", stroke: "none" },
  enclosure: { fill: "none",    stroke: "#569cd6" },
  line:      { fill: "none",    stroke: "#d4d4d4" },
  angle:     { fill: "none",    stroke: "#dcdcaa" },
  curve:     { fill: "none",    stroke: "#ce9178" },
};

function renderNode(node: CanvasNode, selected: boolean) {
  const r = NODE_SIZE[node.type] || 10;
  const color = NODE_COLORS[node.type] || { fill: "#888", stroke: "none" };
  const highlight = selected ? "#569cd6" : "transparent";

  switch (node.type) {
    case "point":
      return (
        <g key={node.id}>
          <circle cx={0} cy={0} r={r + 4} fill="transparent" stroke={highlight} strokeWidth={2} />
          <circle cx={0} cy={0} r={r} fill={color.fill} />
          <text x={0} y={r + 16} textAnchor="middle" fill="#888" fontSize={10}>●</text>
        </g>
      );
    case "enclosure": {
      const shape = node.shape || "circle";
      return (
        <g key={node.id}>
          <circle cx={0} cy={0} r={r + 6} fill="transparent" stroke={highlight} strokeWidth={2} />
          {shape === "circle" && (
            <circle cx={0} cy={0} r={r} fill={color.fill} stroke={color.stroke} strokeWidth={2} />
          )}
          {shape === "triangle" && (
            <polygon
              points={`0,${-r} ${r * 0.87},${r * 0.5} ${-r * 0.87},${r * 0.5}`}
              fill={color.fill}
              stroke={color.stroke}
              strokeWidth={2}
            />
          )}
          {shape === "square" && (
            <rect
              x={-r * 0.7}
              y={-r * 0.7}
              width={r * 1.4}
              height={r * 1.4}
              fill={color.fill}
              stroke={color.stroke}
              strokeWidth={2}
            />
          )}
          <text x={0} y={r + 16} textAnchor="middle" fill="#888" fontSize={10}>
            {shape === "triangle" ? "△" : shape === "square" ? "□" : "○"}
          </text>
        </g>
      );
    }
    case "curve":
      return (
        <g key={node.id}>
          <circle cx={0} cy={0} r={r + 4} fill="transparent" stroke={highlight} strokeWidth={2} />
          <path
            d={`M${-r},0 Q${-r / 2},${-r / 2} 0,0 Q${r / 2},${r / 2} ${r},0`}
            fill="none"
            stroke={color.stroke}
            strokeWidth={2}
          />
          <text x={0} y={r + 16} textAnchor="middle" fill="#888" fontSize={10}>~</text>
        </g>
      );
    case "angle":
      return (
        <g key={node.id}>
          <circle cx={0} cy={0} r={r + 4} fill="transparent" stroke={highlight} strokeWidth={2} />
          <path
            d={`M${r},0 L0,0 L${r * 0.5},${-r * 0.87}`}
            fill="none"
            stroke={color.stroke}
            strokeWidth={2}
          />
          <text x={0} y={r + 16} textAnchor="middle" fill="#888" fontSize={10}>
            ∠{node.degrees ?? 60}
          </text>
        </g>
      );
    case "line":
      return (
        <g key={node.id}>
          <circle cx={0} cy={0} r={r + 4} fill="transparent" stroke={highlight} strokeWidth={2} />
          <line x1={-r} y1={0} x2={r} y2={0} stroke={color.stroke} strokeWidth={2} markerEnd="url(#arrowhead)" />
          <text x={0} y={r + 16} textAnchor="middle" fill="#888" fontSize={10}>→</text>
        </g>
      );
    default:
      return (
        <g key={node.id}>
          <circle cx={0} cy={0} r={8} fill="#888" />
        </g>
      );
  }
}

// ── Tool palette button ──

const TOOLS: { tool: Tool; label: string; symbol: string }[] = [
  { tool: "select",   label: "Select",    symbol: "⇱" },
  { tool: "point",    label: "Point",     symbol: "●" },
  { tool: "circle",   label: "Circle",    symbol: "○" },
  { tool: "triangle", label: "Triangle",  symbol: "△" },
  { tool: "square",   label: "Square",    symbol: "□" },
  { tool: "curve",    label: "Curve",     symbol: "~" },
  { tool: "angle",    label: "Angle",     symbol: "∠" },
  { tool: "connect",  label: "Connect",   symbol: "⟶" },
];

// ── Main component ──

export function VisualCanvas() {
  const gir = useEditorStore((s) => s.gir);
  const setSource = useEditorStore((s) => s.setSource);

  const [tool, setTool] = useState<Tool>("select");
  const [sceneNodes, setSceneNodes] = useState<CanvasNode[]>([]);
  const [sceneEdges, setSceneEdges] = useState<CanvasEdge[]>([]);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [connectFrom, setConnectFrom] = useState<string | null>(null);

  const svgRef = useRef<SVGSVGElement>(null);
  const dragging = useRef<{
    id: string;
    offsetX: number;
    offsetY: number;
  } | null>(null);
  // Track whether the canvas just pushed a text change
  const canvasPushed = useRef(false);

  // Sync GIR → scene (when text changes externally)
  useEffect(() => {
    if (canvasPushed.current) {
      canvasPushed.current = false;
      return;
    }
    if (!gir) return;
    const prevPositions = new Map<string, { x: number; y: number }>();
    for (const n of sceneNodes) {
      prevPositions.set(n.id, { x: n.x, y: n.y });
    }
    const { nodes, edges } = girToScene(gir.nodes, gir.edges, prevPositions);
    setSceneNodes(nodes);
    setSceneEdges(edges);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [gir]);

  // Push scene → text
  const pushToText = useCallback(
    (nodes: CanvasNode[], edges: CanvasEdge[]) => {
      canvasPushed.current = true;
      const script = sceneToScript(nodes, edges);
      setSource(script || "●");
    },
    [setSource],
  );

  // Generate a unique node id
  const nextId = useRef(1);
  const genId = () => `cv_${nextId.current++}`;

  // ── Canvas click: add primitive at position ──
  const handleCanvasClick = useCallback(
    (e: React.MouseEvent<SVGSVGElement>) => {
      if (tool === "select" || tool === "connect") {
        setSelectedId(null);
        setConnectFrom(null);
        return;
      }
      if (!svgRef.current) return;
      const rect = svgRef.current.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const y = e.clientY - rect.top;
      const id = genId();

      let newNode: CanvasNode;
      switch (tool) {
        case "point":
          newNode = { id, type: "point", x, y };
          break;
        case "circle":
          newNode = { id, type: "enclosure", shape: "circle", x, y };
          break;
        case "triangle":
          newNode = { id, type: "enclosure", shape: "triangle", x, y };
          break;
        case "square":
          newNode = { id, type: "enclosure", shape: "square", x, y };
          break;
        case "curve":
          newNode = { id, type: "curve", x, y };
          break;
        case "angle":
          newNode = { id, type: "angle", degrees: 60, x, y };
          break;
        default:
          return;
      }

      const updated = [...sceneNodes, newNode];
      setSceneNodes(updated);
      pushToText(updated, sceneEdges);
    },
    [tool, sceneNodes, sceneEdges, pushToText],
  );

  // ── Node click ──
  const handleNodeClick = useCallback(
    (e: React.MouseEvent, nodeId: string) => {
      e.stopPropagation();
      if (tool === "connect") {
        if (!connectFrom) {
          setConnectFrom(nodeId);
          setSelectedId(nodeId);
        } else if (connectFrom !== nodeId) {
          // Determine edge type: if source is an enclosure → contains, else → connects
          const sourceNode = sceneNodes.find((n) => n.id === connectFrom);
          const edgeType: "connects" | "contains" =
            sourceNode?.type === "enclosure" ? "contains" : "connects";

          const newEdge: CanvasEdge = { from: connectFrom, to: nodeId, type: edgeType };
          const updatedEdges = [...sceneEdges, newEdge];
          setSceneEdges(updatedEdges);
          pushToText(sceneNodes, updatedEdges);
          setConnectFrom(null);
          setSelectedId(null);
        }
      } else {
        setSelectedId(nodeId);
      }
    },
    [tool, connectFrom, sceneNodes, sceneEdges, pushToText],
  );

  // ── Drag ──
  const handleNodeMouseDown = useCallback(
    (e: React.MouseEvent, nodeId: string) => {
      if (tool !== "select") return;
      e.stopPropagation();
      const node = sceneNodes.find((n) => n.id === nodeId);
      if (!node || !svgRef.current) return;
      const rect = svgRef.current.getBoundingClientRect();
      dragging.current = {
        id: nodeId,
        offsetX: e.clientX - rect.left - node.x,
        offsetY: e.clientY - rect.top - node.y,
      };
      setSelectedId(nodeId);
    },
    [tool, sceneNodes],
  );

  const handleMouseMove = useCallback(
    (e: React.MouseEvent<SVGSVGElement>) => {
      if (!dragging.current || !svgRef.current) return;
      const rect = svgRef.current.getBoundingClientRect();
      const x = e.clientX - rect.left - dragging.current.offsetX;
      const y = e.clientY - rect.top - dragging.current.offsetY;
      setSceneNodes((prev) =>
        prev.map((n) =>
          n.id === dragging.current!.id ? { ...n, x, y } : n,
        ),
      );
    },
    [],
  );

  const handleMouseUp = useCallback(() => {
    dragging.current = null;
  }, []);

  // ── Delete ──
  const deleteSelected = useCallback(() => {
    if (!selectedId) return;
    const updatedNodes = sceneNodes.filter((n) => n.id !== selectedId);
    const updatedEdges = sceneEdges.filter(
      (e) => e.from !== selectedId && e.to !== selectedId,
    );
    setSceneNodes(updatedNodes);
    setSceneEdges(updatedEdges);
    setSelectedId(null);
    pushToText(updatedNodes, updatedEdges);
  }, [selectedId, sceneNodes, sceneEdges, pushToText]);

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (
        (e.key === "Delete" || e.key === "Backspace") &&
        document.activeElement?.tagName !== "INPUT" &&
        document.activeElement?.tagName !== "TEXTAREA"
      ) {
        deleteSelected();
      }
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [deleteSelected]);

  // ── Render ──
  return (
    <div style={{ height: "100%", display: "flex", flexDirection: "column" }}>
      {/* Toolbar */}
      <div
        style={{
          display: "flex",
          gap: 2,
          padding: "6px 8px",
          background: "#252526",
          borderBottom: "1px solid #333",
          flexWrap: "wrap",
        }}
      >
        {TOOLS.map((t) => (
          <button
            key={t.tool}
            onClick={() => {
              setTool(t.tool);
              if (t.tool !== "connect") setConnectFrom(null);
            }}
            title={t.label}
            style={{
              background: tool === t.tool ? "#094771" : "#333",
              border: tool === t.tool ? "1px solid #569cd6" : "1px solid #555",
              borderRadius: 3,
              color: "#d4d4d4",
              cursor: "pointer",
              padding: "4px 8px",
              fontSize: 14,
              minWidth: 32,
            }}
          >
            {t.symbol}
          </button>
        ))}
        <div style={{ flex: 1 }} />
        {connectFrom && (
          <span style={{ color: "#569cd6", fontSize: 12, alignSelf: "center" }}>
            Click target node...
          </span>
        )}
        {selectedId && (
          <button
            onClick={deleteSelected}
            title="Delete selected (Del)"
            style={{
              background: "#5a1d1d",
              border: "1px solid #a33",
              borderRadius: 3,
              color: "#f48",
              cursor: "pointer",
              padding: "4px 8px",
              fontSize: 12,
            }}
          >
            ✕ Delete
          </button>
        )}
      </div>

      {/* Canvas */}
      <svg
        ref={svgRef}
        style={{
          flex: 1,
          background: "#1e1e1e",
          cursor:
            tool === "select"
              ? "default"
              : tool === "connect"
                ? "crosshair"
                : "cell",
        }}
        onClick={handleCanvasClick}
        onMouseMove={handleMouseMove}
        onMouseUp={handleMouseUp}
        onMouseLeave={handleMouseUp}
      >
        {/* Arrowhead marker */}
        <defs>
          <marker
            id="arrowhead"
            markerWidth="10"
            markerHeight="7"
            refX="10"
            refY="3.5"
            orient="auto"
          >
            <polygon points="0 0, 10 3.5, 0 7" fill="#d4d4d4" />
          </marker>
          <marker
            id="arrowhead-blue"
            markerWidth="10"
            markerHeight="7"
            refX="10"
            refY="3.5"
            orient="auto"
          >
            <polygon points="0 0, 10 3.5, 0 7" fill="#569cd6" />
          </marker>
        </defs>

        {/* Grid dots */}
        <pattern id="grid" width="40" height="40" patternUnits="userSpaceOnUse">
          <circle cx="20" cy="20" r="1" fill="#333" />
        </pattern>
        <rect width="100%" height="100%" fill="url(#grid)" />

        {/* Edges */}
        {sceneEdges.map((edge, i) => {
          const from = sceneNodes.find((n) => n.id === edge.from);
          const to = sceneNodes.find((n) => n.id === edge.to);
          if (!from || !to) return null;

          if (edge.type === "contains") {
            // Dashed line for containment
            return (
              <line
                key={`e-${i}`}
                x1={from.x}
                y1={from.y}
                x2={to.x}
                y2={to.y}
                stroke="#569cd6"
                strokeWidth={1.5}
                strokeDasharray="5,3"
                markerEnd="url(#arrowhead-blue)"
                opacity={0.6}
              />
            );
          }

          return (
            <line
              key={`e-${i}`}
              x1={from.x}
              y1={from.y}
              x2={to.x}
              y2={to.y}
              stroke="#d4d4d4"
              strokeWidth={1.5}
              markerEnd="url(#arrowhead)"
            />
          );
        })}

        {/* Nodes */}
        {sceneNodes.map((node) => (
          <g
            key={node.id}
            transform={`translate(${node.x}, ${node.y})`}
            style={{ cursor: tool === "select" ? "grab" : "pointer" }}
            onClick={(e) => handleNodeClick(e, node.id)}
            onMouseDown={(e) => handleNodeMouseDown(e, node.id)}
          >
            {renderNode(node, node.id === selectedId || node.id === connectFrom)}
          </g>
        ))}

        {/* Empty state */}
        {sceneNodes.length === 0 && (
          <text
            x="50%"
            y="50%"
            textAnchor="middle"
            dominantBaseline="middle"
            fill="#555"
            fontSize={14}
          >
            Select a tool and click to place primitives
          </text>
        )}
      </svg>
    </div>
  );
}
