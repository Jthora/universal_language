# Vision Interface

> Recognizing handwritten or photographed UL glyphs via vision models.

---

## Use Case

A user draws a UL glyph on paper, takes a photo, and uploads it to UL Forge. The vision interface converts the image to GIR.

```
Photo of handwritten glyph
  │
  ▼
Vision model (GPT-4V, Claude Vision, Gemini Vision)
  │
  ▼
Structured output: GIR JSON or UL-Script
  │
  ▼
Validator
  │
  ▼
Rendered SVG (cleaned-up digital version of the handwritten glyph)
```

---

## Pipeline

### Step 1: Image Preprocessing

- Crop to glyph region (if page contains multiple glyphs)
- Convert to grayscale
- Threshold to binary (black marks on white background)
- Normalize size to 400×400 pixels

### Step 2: Vision Model Recognition

Send the preprocessed image to a vision-capable LLM with a prompt:

```
System: "You are a Universal Language glyph recognizer. Given an image 
         of a handwritten UL glyph, identify the geometric primitives
         and their spatial relationships. Output valid UL-Script.
         
         Primitives: point (•), line segment (—), angle (∠), 
         curve (~), circle (◯), triangle (△), square (□).
         
         Relationships: containment (inside), connection (→), 
         adjacency (touching), intersection (crossing)."

User: [image]
```

### Step 3: Validation and Correction

The vision model output goes through the same validation loop as LLM text generation:
1. Parse the UL-Script output
2. Validate the GIR
3. If invalid, feed errors back with the image for retry

---

## Challenges

| Challenge | Mitigation |
|-----------|-----------|
| Ambiguous handwriting | Present top-3 interpretations to user |
| Scale ambiguity | Normalize all marks relative to the largest enclosure |
| Connection vs. adjacency | Use proximity heuristic (touching = adjacent, overlapping = intersects) |
| Novel glyphs | If no canonical template matches, output raw geometric description |

---

## Alternative: Classical CV Pipeline

For offline or privacy-sensitive deployments, a classical computer vision pipeline (no cloud API):

1. **Contour detection** (OpenCV) → identify closed shapes (enclosures)
2. **Hough line transform** → detect lines and line segments
3. **Corner detection** → identify points and angles
4. **Topological analysis** → determine containment, adjacency, intersection
5. **GIR construction** → assemble detected primitives into a GIR document

This pipeline is less accurate than vision LLMs but works offline and is faster.

---

## Scope in v1

Vision integration is a Phase 4 stretch goal. The interface design is documented now to ensure the GIR format supports the necessary metadata (approximate positions, confidence scores). Implementation depends on:
- Core pipeline stability (Phase 1)
- Web editor file upload UI (Phase 2)
- Availability of vision model APIs
