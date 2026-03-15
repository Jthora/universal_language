/**
 * UL Forge VS Code Extension
 *
 * Provides UL-Script language support:
 * - Syntax highlighting (TextMate grammar)
 * - Live SVG preview (webview panel)
 * - Parse/validation diagnostics (Problems panel)
 * - Commands: preview, export SVG/TikZ, show GIR, validate, insert glyph
 */
import * as vscode from "vscode";
import * as path from "path";
import * as fs from "fs";

// ── WASM Interface ──

/** Runtime interface for the ul_wasm module (4 exported functions) */
interface UlWasm {
  parse(script: string): unknown;
  validate(girJson: string): string | { valid: boolean; errors?: string[]; warnings?: string[] };
  render(girJson: string): string;
  deparse(girJson: string): string;
}

// ── WASM Loading ──

let wasmModule: UlWasm | null = null;

async function loadWasm(context: vscode.ExtensionContext): Promise<UlWasm> {
  if (wasmModule) return wasmModule;

  // The WASM pkg is bundled alongside the extension
  const wasmDir = path.join(context.extensionPath, "wasm-pkg");
  const jsPath = path.join(wasmDir, "ul_wasm.js");

  if (!fs.existsSync(jsPath)) {
    throw new Error(
      "WASM module not found. Run `wasm-pack build` and copy wasm-pkg/ into the extension."
    );
  }

  // Dynamic import of the WASM JS glue
  const mod = await import(jsPath);
  const wasmPath = path.join(wasmDir, "ul_wasm_bg.wasm");
  const wasmBytes = fs.readFileSync(wasmPath);
  await mod.default(wasmBytes);
  wasmModule = mod as UlWasm;
  return wasmModule;
}

// ── Diagnostics ──

const diagnosticCollection =
  vscode.languages.createDiagnosticCollection("ul-forge");

function updateDiagnostics(document: vscode.TextDocument, wasm: UlWasm) {
  if (document.languageId !== "ul") return;

  const text = document.getText();
  const diagnostics: vscode.Diagnostic[] = [];

  try {
    const gir = wasm.parse(text);
    const girJson = JSON.stringify(gir);
    const validationResult = wasm.validate(girJson);
    const validation =
      typeof validationResult === "string"
        ? JSON.parse(validationResult)
        : validationResult;

    if (validation.errors) {
      for (const err of validation.errors) {
        const msg = typeof err === "string" ? err : err.toString();
        diagnostics.push(
          new vscode.Diagnostic(
            new vscode.Range(0, 0, 0, text.length),
            msg,
            vscode.DiagnosticSeverity.Error
          )
        );
      }
    }
    if (validation.warnings) {
      for (const warn of validation.warnings) {
        const msg = typeof warn === "string" ? warn : warn.toString();
        diagnostics.push(
          new vscode.Diagnostic(
            new vscode.Range(0, 0, 0, text.length),
            msg,
            vscode.DiagnosticSeverity.Warning
          )
        );
      }
    }
  } catch (e: unknown) {
    const msg = e instanceof Error ? e.message : String(e);
    diagnostics.push(
      new vscode.Diagnostic(
        new vscode.Range(0, 0, 0, text.length),
        `Parse error: ${msg}`,
        vscode.DiagnosticSeverity.Error
      )
    );
  }

  diagnosticCollection.set(document.uri, diagnostics);
}

// ── Preview Webview ──

let previewPanel: vscode.WebviewPanel | undefined;

function getPreviewHtml(svg: string): string {
  return `<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <style>
    body { margin: 0; background: #1e1e1e; display: flex; align-items: center; justify-content: center; height: 100vh; }
    svg { max-width: 95vw; max-height: 95vh; }
    .error { color: #f48771; font-family: monospace; padding: 16px; white-space: pre-wrap; }
  </style>
</head>
<body>${svg}</body>
</html>`;
}

function updatePreview(document: vscode.TextDocument, wasm: UlWasm) {
  if (!previewPanel) return;
  if (document.languageId !== "ul") return;

  const text = document.getText();
  try {
    const gir = wasm.parse(text);
    const girJson = JSON.stringify(gir);
    const svg = wasm.render(girJson);
    previewPanel.webview.html = getPreviewHtml(svg);
  } catch (e: unknown) {
    const msg = e instanceof Error ? e.message : String(e);
    previewPanel.webview.html = getPreviewHtml(
      `<div class="error">${escapeHtml(msg)}</div>`
    );
  }
}

function escapeHtml(text: string): string {
  return text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}

// ── Canonical Glyphs for Insert ──

const CANONICAL_GLYPHS = [
  { label: "● Existence (Point)", script: "●" },
  { label: "● → ● Relation (Connection)", script: "● → ●" },
  { label: "∠60 Quality (Angle)", script: "∠60" },
  { label: "~ Process (Curve)", script: "~" },
  { label: "○ Concept (Circle)", script: "○" },
  { label: "△ Structure (Triangle)", script: "△" },
  { label: "□ Foundation (Square)", script: "□" },
  { label: "○{●} Point in Circle", script: "○{●}" },
  { label: "△{●} Point in Triangle", script: "△{●}" },
  { label: "□{●} Point in Square", script: "□{●}" },
  { label: "○{○} Containment", script: "○{○}" },
  { label: "○ & ○ Conjunction (AND)", script: "○ & ○" },
  { label: "○ | ○ Disjunction (OR)", script: "○ | ○" },
  { label: "● ← ● Inversion", script: "● ← ●" },
  { label: "● ↔ ● Duality", script: "● ↔ ●" },
  { label: "● → ● → ● Composition", script: "● → ● → ●" },
  { label: "○{● → ●} Embedding", script: "○{● → ●}" },
  { label: "○{~} Cycle", script: "○{~}" },
  { label: "△{● | ● | ●} Trinity", script: "△{● | ● | ●}" },
  { label: "○{○{○}} Self-Nesting", script: "○{○{○}}" },
];

// ── Extension Activation ──

export function activate(context: vscode.ExtensionContext) {
  let wasm: UlWasm | null = null;
  let debounceTimer: ReturnType<typeof setTimeout> | undefined;

  const config = vscode.workspace.getConfiguration("ul-forge");
  const debounceMs = config.get<number>("preview.debounceMs", 200);

  // Initialize WASM lazily
  async function ensureWasm(): Promise<UlWasm> {
    if (!wasm) {
      wasm = await loadWasm(context);
    }
    return wasm;
  }

  // Debounced update
  function debouncedUpdate(document: vscode.TextDocument) {
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(async () => {
      const w = await ensureWasm();
      updateDiagnostics(document, w);
      updatePreview(document, w);
    }, debounceMs);
  }

  // ── Commands ──

  context.subscriptions.push(
    vscode.commands.registerCommand("ul-forge.openPreview", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor || editor.document.languageId !== "ul") {
        vscode.window.showWarningMessage("Open a .ul file first.");
        return;
      }

      previewPanel = vscode.window.createWebviewPanel(
        "ulForgePreview",
        "UL Preview",
        vscode.ViewColumn.Beside,
        { enableScripts: false }
      );
      previewPanel.onDidDispose(() => {
        previewPanel = undefined;
      });

      const w = await ensureWasm();
      updatePreview(editor.document, w);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("ul-forge.exportSvg", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor || editor.document.languageId !== "ul") return;

      const w = await ensureWasm();
      try {
        const gir = w.parse(editor.document.getText());
        const svg = w.render(JSON.stringify(gir));
        const uri = await vscode.window.showSaveDialog({
          filters: { SVG: ["svg"] },
          defaultUri: vscode.Uri.file(
            editor.document.fileName.replace(/\.ul$/, ".svg")
          ),
        });
        if (uri) {
          await vscode.workspace.fs.writeFile(
            uri,
            Buffer.from(svg, "utf-8")
          );
          vscode.window.showInformationMessage(`SVG exported to ${uri.fsPath}`);
        }
      } catch (e: unknown) {
        const msg = e instanceof Error ? e.message : String(e);
        vscode.window.showErrorMessage(`Export failed: ${msg}`);
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("ul-forge.exportTikz", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor || editor.document.languageId !== "ul") return;

      const w = await ensureWasm();
      try {
        const gir = w.parse(editor.document.getText());
        // Render as TikZ — call render with tikz format option
        // Note: the WASM render function uses default SVG; for TikZ we'd need
        // to extend the WASM API or use the REST API. For now, show GIR.
        const girJson = JSON.stringify(gir, null, 2);
        const doc = await vscode.workspace.openTextDocument({
          content: girJson,
          language: "json",
        });
        await vscode.window.showTextDocument(doc, vscode.ViewColumn.Beside);
      } catch (e: unknown) {
        const msg = e instanceof Error ? e.message : String(e);
        vscode.window.showErrorMessage(`Export failed: ${msg}`);
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("ul-forge.showGir", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor || editor.document.languageId !== "ul") return;

      const w = await ensureWasm();
      try {
        const gir = w.parse(editor.document.getText());
        const girJson = JSON.stringify(gir, null, 2);
        const doc = await vscode.workspace.openTextDocument({
          content: girJson,
          language: "json",
        });
        await vscode.window.showTextDocument(doc, vscode.ViewColumn.Beside);
      } catch (e: unknown) {
        const msg = e instanceof Error ? e.message : String(e);
        vscode.window.showErrorMessage(`Parse failed: ${msg}`);
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("ul-forge.validate", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor || editor.document.languageId !== "ul") return;

      const w = await ensureWasm();
      updateDiagnostics(editor.document, w);
      vscode.window.showInformationMessage("Validation complete. Check Problems panel.");
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("ul-forge.insertGlyph", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) return;

      const items = CANONICAL_GLYPHS.map((g) => ({
        label: g.label,
        detail: g.script,
      }));

      const pick = await vscode.window.showQuickPick(items, {
        placeHolder: "Select a canonical glyph to insert",
      });

      if (pick) {
        const glyph = CANONICAL_GLYPHS.find((g) => g.label === pick.label);
        if (glyph) {
          await editor.edit((editBuilder) => {
            editBuilder.insert(editor.selection.active, glyph.script);
          });
        }
      }
    })
  );

  // ── Document change listeners ──

  context.subscriptions.push(
    vscode.workspace.onDidChangeTextDocument((e) => {
      if (e.document.languageId === "ul") {
        debouncedUpdate(e.document);
      }
    })
  );

  context.subscriptions.push(
    vscode.workspace.onDidOpenTextDocument((document) => {
      if (document.languageId === "ul") {
        ensureWasm().then((w) => updateDiagnostics(document, w));
      }
    })
  );

  context.subscriptions.push(diagnosticCollection);
}

export function deactivate() {
  diagnosticCollection.dispose();
}
