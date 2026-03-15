import { describe, it, expect } from "vitest";
import { ULMessage } from "../index.js";

const testSender = { id: "agent-test", capabilities: ["parse"] as const };

// UUID v4 regex: 8-4-4-4-12 hex digits, version nibble = 4, variant = [89ab]
const UUID_V4_RE = /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/;

describe("ULMessage.create", () => {
  it("produces envelope with version 1.0", () => {
    const env = ULMessage.create({
      sender: testSender,
      intent: "assert",
      payload: { format: "gir", content: {} },
    });
    expect(env.version).toBe("1.0");
  });

  it("generates a valid UUID v4 for message id", () => {
    const env = ULMessage.create({
      sender: testSender,
      intent: "query",
      payload: { format: "ul-script", content: "●" },
    });
    expect(env.message.id).toMatch(UUID_V4_RE);
  });

  it("generates unique IDs across calls", () => {
    const ids = new Set(
      Array.from({ length: 50 }, () =>
        ULMessage.create({
          sender: testSender,
          intent: "assert",
          payload: { format: "gir" },
        }).message.id,
      ),
    );
    expect(ids.size).toBe(50);
  });

  it("generates valid ISO 8601 timestamp", () => {
    const env = ULMessage.create({
      sender: testSender,
      intent: "assert",
      payload: { format: "gir" },
    });
    const parsed = new Date(env.message.timestamp);
    expect(parsed.getTime()).not.toBeNaN();
  });

  it("includes context when provided", () => {
    const env = ULMessage.create({
      sender: testSender,
      intent: "refine",
      payload: { format: "gir" },
      context: { conversation_id: "conv-1", in_reply_to: "msg-0" },
    });
    expect(env.message.context?.conversation_id).toBe("conv-1");
    expect(env.message.context?.in_reply_to).toBe("msg-0");
  });

  it("omits context when not provided", () => {
    const env = ULMessage.create({
      sender: testSender,
      intent: "assert",
      payload: { format: "gir" },
    });
    expect(env.message.context).toBeUndefined();
  });
});

describe("ULMessage.ack", () => {
  it("sets intent to ack", () => {
    const env = ULMessage.ack(testSender, "original-msg-id");
    expect(env.message.intent).toBe("ack");
  });

  it("sets in_reply_to in context", () => {
    const env = ULMessage.ack(testSender, "original-msg-id", "conv-42");
    expect(env.message.context?.in_reply_to).toBe("original-msg-id");
    expect(env.message.context?.conversation_id).toBe("conv-42");
  });

  it("payload contains reference to original message", () => {
    const env = ULMessage.ack(testSender, "msg-xyz");
    expect(env.message.payload.format).toBe("reference");
    expect(env.message.payload.content).toBe("msg-xyz");
  });
});

describe("ULMessage.advertise", () => {
  it("sets intent to capability_advertisement", () => {
    const env = ULMessage.advertise(testSender, [
      { name: "parse", input: "ul-script", output: "gir" },
    ]);
    expect(env.message.intent).toBe("capability_advertisement");
  });

  it("includes operations in payload", () => {
    const ops = [
      { name: "parse", input: "ul-script", output: "gir" },
      { name: "validate", input: "gir", output: "validation-result" },
    ];
    const env = ULMessage.advertise(testSender, ops);
    const content = env.message.payload.content as Record<string, unknown>;
    expect(content.operations).toEqual(ops);
    expect(content.protocol_version).toBe("1.0");
  });
});
