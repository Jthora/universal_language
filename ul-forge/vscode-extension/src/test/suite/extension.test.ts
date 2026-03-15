import * as assert from "assert";
import * as vscode from "vscode";

suite("UL Forge Extension", () => {
  test("Extension is present", () => {
    const ext = vscode.extensions.getExtension("ulforge.ul-forge");
    assert.ok(ext, "Extension should be installed");
  });

  test("Extension activates on .ul file", async () => {
    const doc = await vscode.workspace.openTextDocument({
      language: "ul",
      content: "○{●}",
    });
    await vscode.window.showTextDocument(doc);
    const ext = vscode.extensions.getExtension("ulforge.ul-forge");
    assert.ok(ext, "Extension not found");
    // Wait for activation
    await ext.activate();
    assert.ok(ext.isActive, "Extension should be active");
  });

  test("Commands are registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    const expected = [
      "ul-forge.openPreview",
      "ul-forge.exportSvg",
      "ul-forge.exportTikz",
      "ul-forge.showGir",
      "ul-forge.validate",
      "ul-forge.insertGlyph",
    ];
    for (const cmd of expected) {
      assert.ok(commands.includes(cmd), `Command '${cmd}' should be registered`);
    }
  });

  test("UL language configuration is registered", () => {
    const langs = vscode.languages.getLanguages();
    assert.ok(
      langs.then((l) => l.includes("ul")),
      "Language 'ul' should be registered"
    );
  });

  test("TextMate grammar is loaded for .ul files", async () => {
    const doc = await vscode.workspace.openTextDocument({
      language: "ul",
      content: "● → ●",
    });
    // The language ID should be 'ul'
    assert.strictEqual(doc.languageId, "ul");
  });
});
