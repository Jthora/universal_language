import { describe, it, expect } from "vitest";
import { Codec } from "../codec.js";
import type { ULEnvelope } from "../message.js";

const sampleEnvelope: ULEnvelope = {
  version: "1.0",
  message: {
    id: "550e8400-e29b-41d4-a716-446655440000",
    timestamp: "2026-03-15T00:00:00.000Z",
    sender: { id: "agent-alpha", capabilities: ["parse", "validate"] },
    intent: "assert",
    payload: { format: "gir", content: { nodes: [], edges: [] } },
  },
};

describe("Codec.json", () => {
  it("encode produces valid JSON string", () => {
    const json = Codec.json.encode(sampleEnvelope);
    expect(typeof json).toBe("string");
    expect(() => JSON.parse(json)).not.toThrow();
  });

  it("encodePretty produces indented JSON", () => {
    const json = Codec.json.encodePretty(sampleEnvelope);
    expect(json).toContain("\n");
    expect(json).toContain("  ");
  });

  it("decode reconstructs envelope", () => {
    const json = Codec.json.encode(sampleEnvelope);
    const decoded = Codec.json.decode(json);
    expect(decoded).toEqual(sampleEnvelope);
  });

  it("roundtrip preserves all fields", () => {
    const withContext: ULEnvelope = {
      ...sampleEnvelope,
      message: {
        ...sampleEnvelope.message,
        context: {
          conversation_id: "conv-123",
          in_reply_to: "msg-456",
          domain: "mathematics",
          sequence_number: 3,
        },
      },
    };
    const decoded = Codec.json.decode(Codec.json.encode(withContext));
    expect(decoded.message.context?.conversation_id).toBe("conv-123");
    expect(decoded.message.context?.sequence_number).toBe(3);
  });

  it("decode rejects invalid JSON", () => {
    expect(() => Codec.json.decode("not json")).toThrow();
  });

  it("decode rejects wrong protocol version", () => {
    const badVersion = JSON.stringify({ version: "2.0", message: {} });
    expect(() => Codec.json.decode(badVersion)).toThrow("Unsupported protocol version");
  });
});
