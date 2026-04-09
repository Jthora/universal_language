declare module "ul-wasm" {
  // ── Runtime initialization ──
  export function init(): void;
  export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;
  export function parseUlScript(input: string): any;
  export function deparseGir(gir_json: string): any;
  export function validateGir(gir_json: string, check_geometry: boolean): any;
  export function renderSvg(gir_json: string, width: number, height: number): any;
  export function renderGlyphPreview(gir_json: string): any;

  // ── Game context ──
  export function createContext(config_json: string): number;
  export function evaluate(ctx_id: number, gir_json: string): any;
  export function scoreComposition(ctx_id: number, gir_json: string, target_json: string): any;
  export function evaluateJaneAttempt(ctx_id: number, attempt_json: string, expected_json: string): any;
  export function validateSequence(ctx_id: number, glyphs_json: string): any;
  export function getAnimationSequence(gir_json: string, width: number, height: number): any;
  export function layout(gir_json: string, width: number, height: number): any;
  export function loadCustomDefinitions(ctx_id: number, rules_json: string): any;

  // ── Algebraic composer ──
  export function applyOperation(operation: string, operands_json: string): any;
  export function composeGir(gir_a_json: string, gir_b_json: string, operation: string): any;
  export function detectOperations(gir_json: string): any;
  export function analyzeStructure(gir_json: string): any;

  // ── Teaching system ──
  export function getHints(attempt_json: string, target_json: string): any;
  export function analyzeHints(gir_json: string): any;
  export function getNextPuzzle(proficiency_json: string): any;

  // ── Lexicon ──
  export function queryLexicon(query: string): any;
  export function lookupLexiconEntry(name: string): any;

  // ── Extensions (modal, performative, pragmatic) ──
  export function set_force(gir_json: string, force: string): any;
  export function infer_pragmatics(gir_json: string): any;

  // ── Equivalence ──
  export function compareGir(gir_a_json: string, gir_b_json: string, level: string): any;

  type SyncInitInput = BufferSource | WebAssembly.Module;
  type InitOutput = any;

  type InitInput =
    | RequestInfo
    | URL
    | Response
    | BufferSource
    | WebAssembly.Module;

  export default function __wbg_init(
    module_or_path?: InitInput | Promise<InitInput>
  ): Promise<any>;
}
