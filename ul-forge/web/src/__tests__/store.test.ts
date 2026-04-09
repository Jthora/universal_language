import { describe, it, expect, beforeEach } from "vitest";
import { useEditorStore } from "../store/editorStore";

// Reset store state between tests
beforeEach(() => {
  useEditorStore.setState({
    source: "○{●}",
    gir: null,
    svg: null,
    validation: null,
    error: null,
    ready: false,
    undoStack: [],
    redoStack: [],
  });
});

describe("editorStore", () => {
  it("has correct initial state", () => {
    const state = useEditorStore.getState();
    expect(state.source).toBe("○{●}");
    expect(state.gir).toBeNull();
    expect(state.svg).toBeNull();
    expect(state.ready).toBe(false);
    expect(state.undoStack).toEqual([]);
    expect(state.redoStack).toEqual([]);
  });

  it("setSource updates source and pushes to undo stack", () => {
    const { setSource } = useEditorStore.getState();
    setSource("●");
    const state = useEditorStore.getState();
    expect(state.source).toBe("●");
    expect(state.undoStack).toEqual(["○{●}"]);
    expect(state.redoStack).toEqual([]);
  });

  it("setSource clears redo stack", () => {
    const store = useEditorStore.getState();
    store.setSource("●");
    store.setSource("△");
    store.undo();
    // After undo: source="●", redo=["△"]
    expect(useEditorStore.getState().redoStack.length).toBe(1);

    useEditorStore.getState().setSource("□");
    expect(useEditorStore.getState().redoStack).toEqual([]);
  });

  it("undo restores previous source", () => {
    const store = useEditorStore.getState();
    store.setSource("●");
    store.setSource("△");
    useEditorStore.getState().undo();
    const state = useEditorStore.getState();
    expect(state.source).toBe("●");
    expect(state.redoStack).toContain("△");
  });

  it("redo restores undone source", () => {
    const store = useEditorStore.getState();
    store.setSource("●");
    useEditorStore.getState().undo();
    useEditorStore.getState().redo();
    expect(useEditorStore.getState().source).toBe("●");
  });

  it("undo on empty stack is a no-op", () => {
    const before = useEditorStore.getState().source;
    useEditorStore.getState().undo();
    expect(useEditorStore.getState().source).toBe(before);
  });

  it("redo on empty stack is a no-op", () => {
    const before = useEditorStore.getState().source;
    useEditorStore.getState().redo();
    expect(useEditorStore.getState().source).toBe(before);
  });

  it("setReady marks WASM as initialized", () => {
    useEditorStore.getState().setReady(true);
    expect(useEditorStore.getState().ready).toBe(true);
  });

  it("setResult updates gir, svg, validation, error", () => {
    useEditorStore.getState().setResult({
      gir: { nodes: [], edges: [] } as any,
      svg: "<svg></svg>",
      validation: { valid: true, errors: [], warnings: [], layers: { schema: [], sort: [], invariant: [], geometry: [] } },
      error: null,
    });
    const state = useEditorStore.getState();
    expect(state.svg).toBe("<svg></svg>");
    expect(state.validation?.valid).toBe(true);
    expect(state.error).toBeNull();
  });

  it("undo stack is capped at 100 entries", () => {
    for (let i = 0; i < 110; i++) {
      useEditorStore.getState().setSource(`source-${i}`);
    }
    expect(useEditorStore.getState().undoStack.length).toBeLessThanOrEqual(100);
  });
});
