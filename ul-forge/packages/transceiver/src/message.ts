/**
 * @ul-forge/transceiver — Message types
 *
 * TypeScript types for the UL Transceiver Protocol v1.0 wire format.
 */

/** UL operations an agent can perform. */
export type Capability =
  | "parse"
  | "validate"
  | "render"
  | "evaluate"
  | "compose"
  | "deparse"
  | "analyze";

/** Agent identity and capabilities. */
export interface AgentIdentity {
  id: string;
  capabilities?: Capability[];
}

/** Message intent type. */
export type Intent =
  | "assert"
  | "query"
  | "propose"
  | "refine"
  | "validate_request"
  | "validate_response"
  | "capability_advertisement"
  | "ack";

/** Payload format discriminator. */
export type PayloadFormat =
  | "gir"
  | "ul-script"
  | "validation-result"
  | "capability-list"
  | "reference";

/** Message payload. */
export interface Payload {
  format: PayloadFormat;
  content?: unknown;
  confidence?: number;
}

/** Conversational context for threading. */
export interface MessageContext {
  conversation_id?: string;
  in_reply_to?: string;
  domain?: string;
  sequence_number?: number;
}

/** A single UL Transceiver Protocol message. */
export interface ULMessage {
  id: string;
  timestamp: string;
  sender: AgentIdentity;
  intent: Intent;
  payload: Payload;
  context?: MessageContext;
}

/** Top-level protocol envelope. */
export interface ULEnvelope {
  version: "1.0";
  message: ULMessage;
}

/** Capability advertisement payload content. */
export interface CapabilityAdvertisement {
  operations: Array<{ name: string; input: string; output: string }>;
  supported_sorts?: string[];
  max_gir_nodes?: number;
  protocol_version: string;
}
