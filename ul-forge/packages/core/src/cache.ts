// SYNC: Canonical source is packages/sdk/src/cache.ts
// To sync: cp ../sdk/src/cache.ts src/cache.ts
/**
 * @ul-forge/sdk — Result cache for game-loop performance.
 *
 * Caches WASM call results by GIR hash to avoid redundant computation.
 * Call clearCaches() when the GIR changes.
 */

import type { Gir } from "./types.js";

const MAX_CACHE_ENTRIES = 256;

/** Deterministic hash of a GIR for cache keys. */
export function girHash(gir: Gir): string {
  return JSON.stringify({ r: gir.root, n: gir.nodes.length, e: gir.edges.length });
}

export class ResultCache<T> {
  private cache = new Map<string, T>();

  get(key: string): T | undefined {
    return this.cache.get(key);
  }

  set(key: string, value: T): void {
    if (this.cache.size >= MAX_CACHE_ENTRIES) {
      const first = this.cache.keys().next().value;
      if (first !== undefined) this.cache.delete(first);
    }
    this.cache.set(key, value);
  }

  clear(): void {
    this.cache.clear();
  }

  get size(): number {
    return this.cache.size;
  }
}
