//! Game context: session state, proficiency tracking, rule management.
//!
//! Tracks player proficiency across the 5 geometric primitives and
//! manages active composition rules for the current session.

use std::collections::HashMap;

use crate::cosmic::RuleIndex;
use crate::types::GameConfig;

/// Per-session game state. Holds active rules and player proficiency.
#[derive(Debug, Clone)]
pub struct GameContext {
    /// Active composition rules for evaluation.
    pub rule_index: RuleIndex,
    /// Per-primitive proficiency scores (0.0–1.0), keyed by primitive name.
    pub proficiency: HashMap<String, f64>,
    /// Session identifier.
    pub session_id: String,
}

impl GameContext {
    /// Create a new game context from configuration.
    pub fn from_config(config: &GameConfig) -> Result<Self, String> {
        let rule_index = if config.rules_json.is_empty() {
            RuleIndex::default()
        } else {
            RuleIndex::from_json(&config.rules_json).map_err(|e| e.to_string())?
        };

        Ok(Self {
            rule_index,
            proficiency: default_proficiency(),
            session_id: config.session_id.clone(),
        })
    }

    /// Get proficiency for a primitive (defaults to 0.5 if unknown).
    pub fn proficiency_for(&self, primitive: &str) -> f64 {
        self.proficiency.get(primitive).copied().unwrap_or(0.5)
    }

    /// Update proficiency using exponential moving average.
    pub fn update_proficiency(&mut self, primitive: &str, score: f64) -> f64 {
        const ALPHA: f64 = 0.3;
        let current = self.proficiency_for(primitive);
        let updated = current * (1.0 - ALPHA) + score * ALPHA;
        let clamped = updated.clamp(0.0, 1.0);
        self.proficiency.insert(primitive.to_string(), clamped);
        clamped - current
    }
}

/// Default proficiency: each of the 5 geometric primitives starts at 0.5.
fn default_proficiency() -> HashMap<String, f64> {
    let mut m = HashMap::new();
    for name in &["point", "line", "angle", "curve", "enclosure"] {
        m.insert((*name).to_string(), 0.5);
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_config() -> GameConfig {
        GameConfig {
            rules_json: String::new(),
            session_id: "test".into(),
        }
    }

    #[test]
    fn create_from_empty_config() {
        let ctx = GameContext::from_config(&empty_config()).unwrap();
        assert_eq!(ctx.proficiency.len(), 5);
        assert_eq!(ctx.proficiency_for("point"), 0.5);
    }

    #[test]
    fn proficiency_update_ema() {
        let mut ctx = GameContext::from_config(&empty_config()).unwrap();
        let delta = ctx.update_proficiency("point", 1.0);
        assert!(delta > 0.0);
        assert!(ctx.proficiency_for("point") > 0.5);
    }

    #[test]
    fn proficiency_clamps() {
        let mut ctx = GameContext::from_config(&empty_config()).unwrap();
        for _ in 0..100 {
            ctx.update_proficiency("point", 1.0);
        }
        assert!(ctx.proficiency_for("point") <= 1.0);
    }

    #[test]
    fn create_with_rules_json() {
        let json = r#"[{
            "name": "test-rule",
            "required_pattern": { "nodes": [{"id": "p1", "node_type": "point"}], "edges": [] },
            "weight": 1.0,
            "difficulty": 1
        }]"#;
        let config = GameConfig {
            rules_json: json.into(),
            session_id: "test".into(),
        };
        let ctx = GameContext::from_config(&config).unwrap();
        assert_eq!(ctx.rule_index.rules().len(), 1);
    }
}
