# Composer Workflows

> The 5 primary workflows for composing UL glyphs in the web editor.

---

## Workflow 1: Text-First Composition

**For:** Users comfortable with UL-Script syntax.

```
1. Type UL-Script in Monaco editor
2. See live SVG preview update on each keystroke
3. Validation errors appear as squiggles in the editor
4. Iterate until the glyph looks correct
5. Export SVG/PNG/TikZ
```

**Example session:**
```
User types: ◯(•)
Preview shows: circle containing a point

User extends: ◯(•) → △(•)
Preview shows: circle-with-point connected by arrow to triangle-with-point

User adds modifier: ◯(•) /60° → △(•)
Preview shows: same, with 60° angle mark on the arrow
```

This is the keyboard-driven, programmer-friendly workflow.

---

## Workflow 2: Visual-First Composition

**For:** Users who prefer spatial/visual thinking.

```
1. Open the visual glyph builder (canvas mode)
2. Drag a circle from the palette → place on canvas
3. Drag a point → drop inside the circle
4. Drag a triangle → place to the right
5. Drag a point → drop inside the triangle
6. Draw an arrow from circle to triangle (click source, click target)
7. UL-Script text updates automatically in the editor pane
8. Export
```

The visual builder generates GIR operations directly. The deparser converts the GIR to UL-Script for the text pane.

---

## Workflow 3: Template-Based Composition

**For:** Users who want to work with canonical lexicon glyphs.

```
1. Open the template palette (42 canonical glyphs)
2. Browse by sort: Entity | Relation | Modifier | Assertion
3. Click "CONCEPT" template → inserts ◯(•) at cursor
4. Click "STRUCTURE" template → inserts △(•) at cursor
5. Connect them manually with → operator
6. Modify and refine
7. Export
```

Templates provide pre-validated starting points. Users compose complex glyphs by combining templates.

---

## Workflow 4: Iterative Refinement

**For:** Editing existing glyphs.

```
1. Open a .ul file or paste UL-Script
2. Preview renders the existing glyph
3. Click on a mark in the SVG preview → highlights in text editor
4. Edit the text → preview updates
5. Or: click and drag a mark in the preview → GIR updates → text updates
6. Use the property inspector to change sort, label, or edge types
7. Save
```

Bidirectional selection linking (click SVG → highlight text, click text → highlight SVG) is essential for this workflow.

---

## Workflow 5: AI-Assisted Composition

**For:** Users exploring meaning relationships with AI help.

```
1. Type a natural language concept: "justice as balanced force"
2. AI suggests UL-Script: ◯(△(• •) ☰) — concept containing balanced structure
3. Preview renders the suggestion
4. User accepts, modifies, or asks for alternatives
5. AI explains the geometric reasoning behind the suggestion
6. User refines and exports
```

This workflow requires the AI integration layer (see [ai-integration/llm-interface.md](../../ai-integration/llm-interface.md)). In v1, this is a stretch goal — the interface is designed but the AI model fine-tuning may not be complete.

---

## Workflow Transitions

Users can switch between workflows at any time:

```
Text → Visual:  Click "Canvas Mode" button
Visual → Text:  Click "Editor Mode" button
Any → Template: Open template palette sidebar
Any → AI:       Open AI assistant panel
```

The GIR store ensures state is preserved across mode switches. Whatever the user built in one mode is immediately available in another.

---

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Enter` | Re-render preview |
| `Ctrl+Shift+P` | Open template palette |
| `Ctrl+E` | Toggle editor/canvas mode |
| `Ctrl+S` | Save (download .ul file) |
| `Ctrl+Shift+S` | Export SVG |
| `Ctrl+Z` / `Ctrl+Y` | Undo / Redo (full GIR history) |

---

## Undo/Redo

Undo operates on the GIR store, not on text edits:
- Each GIR mutation is recorded in a history stack
- Undo reverts the last GIR mutation → both text and SVG update
- Redo replays a reverted mutation
- Maximum 100 undo steps (configurable)
