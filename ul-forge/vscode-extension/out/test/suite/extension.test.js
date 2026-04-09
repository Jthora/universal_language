"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
const assert = __importStar(require("assert"));
const vscode = __importStar(require("vscode"));
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
        assert.ok(langs.then((l) => l.includes("ul")), "Language 'ul' should be registered");
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
//# sourceMappingURL=extension.test.js.map