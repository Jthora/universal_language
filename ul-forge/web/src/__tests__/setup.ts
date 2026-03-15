import "@testing-library/jest-dom";

// Stub Web Worker for jsdom environment
class WorkerStub {
  onmessage: ((e: MessageEvent) => void) | null = null;
  postMessage() {}
  terminate() {}
  addEventListener() {}
  removeEventListener() {}
  dispatchEvent() { return false; }
  onerror = null;
  onmessageerror = null;
}
globalThis.Worker = WorkerStub as any;
