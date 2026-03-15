/**
 * Zustand store — single source of truth for the editor state.
 * Includes undo/redo history for source text changes.
 */
import { create } from "zustand";
import type { Gir, ValidationResult } from "../core/index";

const MAX_UNDO = 100;

export interface EditorState {
  /** Current UL-Script source text. */
  source: string;
  /** Parsed GIR (null if parse failed). */
  gir: Gir | null;
  /** Rendered SVG string (null if render failed). */
  svg: string | null;
  /** Validation result. */
  validation: ValidationResult | null;
  /** Parse/render error message (null if success). */
  error: string | null;
  /** Whether the WASM module is ready. */
  ready: boolean;
  /** Undo stack. */
  undoStack: string[];
  /** Redo stack. */
  redoStack: string[];

  // Actions
  setSource: (source: string) => void;
  setReady: (ready: boolean) => void;
  setResult: (result: {
    gir: Gir | null;
    svg: string | null;
    validation: ValidationResult | null;
    error: string | null;
  }) => void;
  undo: () => void;
  redo: () => void;
}

export const useEditorStore = create<EditorState>((set, get) => ({
  source: "○{●}",
  gir: null,
  svg: null,
  validation: null,
  error: null,
  ready: false,
  undoStack: [],
  redoStack: [],

  setSource: (source) => {
    const { source: prev, undoStack } = get();
    if (source === prev) return;
    const newUndo = [...undoStack, prev].slice(-MAX_UNDO);
    set({ source, undoStack: newUndo, redoStack: [] });
  },
  setReady: (ready) => set({ ready }),
  setResult: (result) =>
    set({
      gir: result.gir,
      svg: result.svg,
      validation: result.validation,
      error: result.error,
    }),
  undo: () => {
    const { undoStack, source, redoStack } = get();
    if (undoStack.length === 0) return;
    const prev = undoStack[undoStack.length - 1];
    set({
      source: prev,
      undoStack: undoStack.slice(0, -1),
      redoStack: [...redoStack, source],
    });
  },
  redo: () => {
    const { redoStack, source, undoStack } = get();
    if (redoStack.length === 0) return;
    const next = redoStack[redoStack.length - 1];
    set({
      source: next,
      redoStack: redoStack.slice(0, -1),
      undoStack: [...undoStack, source],
    });
  },
}));
