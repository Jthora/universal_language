/**
 * @ul-forge/transceiver — Codec
 *
 * Encode/decode UL Transceiver Protocol messages to/from JSON.
 */

import type { ULEnvelope } from "./message.js";

/** Codec for serializing/deserializing UL Transceiver messages. */
export const Codec = {
  json: {
    /** Encode an envelope to a JSON string. */
    encode(envelope: ULEnvelope): string {
      return JSON.stringify(envelope);
    },

    /** Encode an envelope to a pretty-printed JSON string. */
    encodePretty(envelope: ULEnvelope): string {
      return JSON.stringify(envelope, null, 2);
    },

    /** Decode a JSON string into an envelope. */
    decode(data: string): ULEnvelope {
      const parsed = JSON.parse(data) as ULEnvelope;
      if (parsed.version !== "1.0") {
        throw new Error(`Unsupported protocol version: ${parsed.version}`);
      }
      return parsed;
    },
  },
} as const;
