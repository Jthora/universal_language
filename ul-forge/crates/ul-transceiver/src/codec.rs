//! # UL Transceiver — Codec
//!
//! Encode/decode UL Transceiver Protocol messages to/from JSON.

use crate::message::ULEnvelope;
use thiserror::Error;

/// Codec errors.
#[derive(Debug, Error)]
pub enum CodecError {
    #[error("JSON encode/decode error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Supported wire encodings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Codec {
    /// JSON encoding (UTF-8).
    Json,
}

impl Codec {
    /// Encode an envelope to bytes.
    pub fn encode(&self, envelope: &ULEnvelope) -> Result<Vec<u8>, CodecError> {
        match self {
            Codec::Json => {
                let json = serde_json::to_vec(envelope)?;
                Ok(json)
            }
        }
    }

    /// Encode an envelope to a pretty-printed string.
    pub fn encode_pretty(&self, envelope: &ULEnvelope) -> Result<String, CodecError> {
        match self {
            Codec::Json => {
                let json = serde_json::to_string_pretty(envelope)?;
                Ok(json)
            }
        }
    }

    /// Decode bytes into an envelope.
    pub fn decode(&self, data: &[u8]) -> Result<ULEnvelope, CodecError> {
        match self {
            Codec::Json => {
                let envelope: ULEnvelope = serde_json::from_slice(data)?;
                Ok(envelope)
            }
        }
    }

    /// Decode a string into an envelope.
    pub fn decode_str(&self, data: &str) -> Result<ULEnvelope, CodecError> {
        match self {
            Codec::Json => {
                let envelope: ULEnvelope = serde_json::from_str(data)?;
                Ok(envelope)
            }
        }
    }
}
