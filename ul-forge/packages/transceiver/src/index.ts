/**
 * @ul-forge/transceiver — UL Transceiver Protocol for TypeScript
 *
 * Encode, decode, and build UL Transceiver messages for AI-to-AI
 * semantic communication using Universal Language GIR structures.
 *
 * @example
 * ```typescript
 * import { ULMessage, Codec } from '@ul-forge/transceiver';
 *
 * const msg = ULMessage.create({
 *   sender: { id: 'agent-alpha', capabilities: ['parse', 'validate'] },
 *   intent: 'assert',
 *   payload: { format: 'gir', content: myGir },
 * });
 *
 * const json = Codec.json.encode(msg);
 * const decoded = Codec.json.decode(json);
 * ```
 */

export type {
  ULEnvelope,
  ULMessage as ULMessageType,
  AgentIdentity,
  Capability,
  Intent,
  Payload,
  PayloadFormat,
  MessageContext,
  CapabilityAdvertisement,
} from "./message.js";

export { Codec } from "./codec.js";

import type {
  ULEnvelope,
  AgentIdentity,
  Intent,
  Payload,
  MessageContext,
} from "./message.js";

/** Generate UUID v4 — works in Node >=18 and all modern browsers. */
function generateUUID(): string {
  const bytes = new Uint8Array(16);
  crypto.getRandomValues(bytes);
  bytes[6] = (bytes[6] & 0x0f) | 0x40; // version 4
  bytes[8] = (bytes[8] & 0x3f) | 0x80; // variant 1
  const h = Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");
  return `${h.slice(0, 8)}-${h.slice(8, 12)}-${h.slice(12, 16)}-${h.slice(16, 20)}-${h.slice(20)}`;
}

/** Options for creating a UL Transceiver message. */
export interface CreateMessageOptions {
  sender: AgentIdentity;
  intent: Intent;
  payload: Payload;
  context?: MessageContext;
}

/** Helper for building UL Transceiver messages. */
export const ULMessage = {
  /** Create a new UL Transceiver envelope with auto-generated ID and timestamp. */
  create(options: CreateMessageOptions): ULEnvelope {
    return {
      version: "1.0",
      message: {
        id: generateUUID(),
        timestamp: new Date().toISOString(),
        sender: options.sender,
        intent: options.intent,
        payload: options.payload,
        ...(options.context ? { context: options.context } : {}),
      },
    };
  },

  /** Create an ACK message replying to a received message. */
  ack(sender: AgentIdentity, replyToId: string, conversationId?: string): ULEnvelope {
    return ULMessage.create({
      sender,
      intent: "ack",
      payload: { format: "reference", content: replyToId },
      context: {
        in_reply_to: replyToId,
        ...(conversationId ? { conversation_id: conversationId } : {}),
      },
    });
  },

  /** Create a capability advertisement message. */
  advertise(
    sender: AgentIdentity,
    operations: Array<{ name: string; input: string; output: string }>,
  ): ULEnvelope {
    return ULMessage.create({
      sender,
      intent: "capability_advertisement",
      payload: {
        format: "capability-list",
        content: {
          operations,
          supported_sorts: ["entity", "relation", "modifier", "assertion"],
          protocol_version: "1.0",
        },
      },
    });
  },
};
