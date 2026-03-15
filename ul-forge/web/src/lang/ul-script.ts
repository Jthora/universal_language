/**
 * UL-Script language definition for Monaco Editor.
 *
 * Provides syntax highlighting for:
 * - Points (● *)
 * - Enclosures (○ △ □ and ASCII /0 /3 /4)
 * - Arrows (→ ← ↔ -> <- <->)
 * - Curves (~)
 * - Angle modifiers (∠ @) with numbers
 * - Operators (| & { })
 * - Comments (#)
 */
import type * as monaco from "monaco-editor";

export const UL_LANGUAGE_ID = "ul-script";

export const languageConfiguration: monaco.languages.LanguageConfiguration = {
  comments: { lineComment: "#" },
  brackets: [
    ["{", "}"],
    ["(", ")"],
  ],
  autoClosingPairs: [
    { open: "{", close: "}" },
    { open: "(", close: ")" },
  ],
  surroundingPairs: [
    { open: "{", close: "}" },
    { open: "(", close: ")" },
  ],
};

export const monarchTokensProvider: monaco.languages.IMonarchLanguage = {
  defaultToken: "",
  tokenPostfix: ".ul",

  tokenizer: {
    root: [
      // Comments
      [/#.*$/, "comment"],

      // Point marks
      [/[●*]/, "keyword.point"],

      // Enclosure marks
      [/[○△□]/, "keyword.enclosure"],
      [/\/[034]/, "keyword.enclosure"],

      // Arrows
      [/[→←↔]/, "keyword.arrow"],
      [/<->|->|<-/, "keyword.arrow"],

      // Curves
      [/~/, "keyword.curve"],

      // Angle with number
      [/[∠@]/, "keyword.angle"],

      // Numbers (after angle)
      [/\d+(\.\d+)?/, "number"],

      // Operators
      [/[|&]/, "operator"],

      // Brackets
      [/[{}()]/, "delimiter.bracket"],

      // Whitespace
      [/\s+/, "white"],
    ],
  },
};

/** Register UL-Script language in Monaco. Call once at startup. */
export function registerUlScript(m: typeof monaco) {
  m.languages.register({ id: UL_LANGUAGE_ID });
  m.languages.setLanguageConfiguration(UL_LANGUAGE_ID, languageConfiguration);
  m.languages.setMonarchTokensProvider(UL_LANGUAGE_ID, monarchTokensProvider);
}
