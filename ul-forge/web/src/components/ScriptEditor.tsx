/**
 * UL-Script text editor pane using Monaco Editor.
 */
import { useRef, useCallback, useEffect } from "react";
import Editor, { type OnMount } from "@monaco-editor/react";
import type * as monaco from "monaco-editor";
import {
  UL_LANGUAGE_ID,
  registerUlScript,
} from "../lang/ul-script";
import { useEditorStore } from "../store/editorStore";

let registered = false;

export function ScriptEditor() {
  const source = useEditorStore((s) => s.source);
  const setSource = useEditorStore((s) => s.setSource);
  const error = useEditorStore((s) => s.error);
  const editorRef = useRef<monaco.editor.IStandaloneCodeEditor | null>(null);
  const monacoRef = useRef<typeof monaco | null>(null);

  const handleMount: OnMount = useCallback(
    (editor, monacoInstance) => {
      editorRef.current = editor;
      monacoRef.current = monacoInstance;
      if (!registered) {
        registerUlScript(monacoInstance);
        registered = true;
      }
      // Set initial model language
      const model = editor.getModel();
      if (model) {
        monacoInstance.editor.setModelLanguage(model, UL_LANGUAGE_ID);
      }
    },
    [],
  );

  const handleChange = useCallback(
    (value: string | undefined) => {
      if (value !== undefined) {
        setSource(value);
      }
    },
    [setSource],
  );

  // Show parse errors as Monaco markers
  useEffect(() => {
    const editor = editorRef.current;
    const m = monacoRef.current;
    if (!editor || !m) return;
    const model = editor.getModel();
    if (!model) return;

    if (error) {
      // Try to extract line/column from error message
      const match = error.match(/line (\d+), column (\d+)/);
      const line = match ? parseInt(match[1], 10) : 1;
      const col = match ? parseInt(match[2], 10) : 1;
      m.editor.setModelMarkers(model, "ul-forge", [
        {
          severity: m.MarkerSeverity.Error,
          message: error,
          startLineNumber: line,
          startColumn: col,
          endLineNumber: line,
          endColumn: col + 1,
        },
      ]);
    } else {
      m.editor.setModelMarkers(model, "ul-forge", []);
    }
  }, [error]);

  return (
    <Editor
      height="100%"
      defaultLanguage="plaintext"
      defaultValue={source}
      onChange={handleChange}
      onMount={handleMount}
      theme="vs-dark"
      options={{
        fontSize: 18,
        minimap: { enabled: false },
        lineNumbers: "on",
        wordWrap: "on",
        scrollBeyondLastLine: false,
        automaticLayout: true,
        padding: { top: 12 },
      }}
    />
  );
}
