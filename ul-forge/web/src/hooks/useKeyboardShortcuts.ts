/**
 * Global keyboard shortcut handler.
 */
import { useEffect } from "react";
import { useEditorStore } from "../store/editorStore";

interface UseKeyboardShortcutsOptions {
  togglePalette: () => void;
  toggleMode?: () => void;
}

export function useKeyboardShortcuts({ togglePalette, toggleMode }: UseKeyboardShortcutsOptions) {
  const undo = useEditorStore((s) => s.undo);
  const redo = useEditorStore((s) => s.redo);

  useEffect(() => {
    function onKeyDown(e: KeyboardEvent) {
      const mod = e.metaKey || e.ctrlKey;

      // Ctrl+Shift+P — toggle palette
      if (mod && e.shiftKey && e.key === "p") {
        e.preventDefault();
        togglePalette();
        return;
      }

      // Ctrl+Shift+Z — redo
      if (mod && e.shiftKey && e.key === "z") {
        e.preventDefault();
        redo();
        return;
      }

      // Ctrl+E — toggle text/visual mode
      if (mod && !e.shiftKey && e.key === "e") {
        e.preventDefault();
        toggleMode?.();
        return;
      }

      // Ctrl+Z — undo (only when not in Monaco editor which has its own undo)
      // We skip this since Monaco handles its own undo; store-level undo is
      // for programmatic source changes (template inserts, etc.)
    }

    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [undo, redo, togglePalette, toggleMode]);
}
