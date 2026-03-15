/**
 * Hook: debounced parse → validate → render pipeline via Web Worker.
 *
 * Watches the source text in the store, debounces changes,
 * then sends to a WASM worker and updates the store with results.
 */
import { useEffect, useRef } from "react";
import { useEditorStore } from "../store/editorStore";

const DEBOUNCE_MS = 100;

let workerInstance: Worker | null = null;
let nextId = 0;

function getWorker(): Worker {
  if (!workerInstance) {
    workerInstance = new Worker(
      new URL("../workers/wasm.worker.ts", import.meta.url),
      { type: "module" },
    );
  }
  return workerInstance;
}

export function useLivePreview() {
  const source = useEditorStore((s) => s.source);
  const ready = useEditorStore((s) => s.ready);
  const setResult = useEditorStore((s) => s.setResult);
  const setReady = useEditorStore((s) => s.setReady);
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const currentIdRef = useRef(0);

  // Initialize worker on mount
  useEffect(() => {
    const worker = getWorker();
    const handler = (e: MessageEvent) => {
      const msg = e.data;
      if (msg.type === "ready") {
        setReady(true);
      } else if (msg.type === "result") {
        // Only apply result if it matches the latest request
        if (msg.id === currentIdRef.current) {
          setResult({
            gir: msg.gir,
            validation: msg.validation,
            svg: msg.svg,
            error: msg.error,
          });
        }
      }
    };
    worker.addEventListener("message", handler);
    worker.postMessage({ type: "init" });
    return () => worker.removeEventListener("message", handler);
  }, [setReady, setResult]);

  // Debounced pipeline dispatch
  useEffect(() => {
    if (!ready) return;

    if (timerRef.current) clearTimeout(timerRef.current);

    timerRef.current = setTimeout(() => {
      const id = ++nextId;
      currentIdRef.current = id;
      getWorker().postMessage({ type: "pipeline", id, source });
    }, DEBOUNCE_MS);

    return () => {
      if (timerRef.current) clearTimeout(timerRef.current);
    };
  }, [source, ready]);
}
