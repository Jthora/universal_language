//! Modding support: runtime injection of custom composition rules.

use crate::context::GameContext;
use crate::types::{CompositionRule, LoadResult};

/// Load custom composition rules from a JSON string into the game context.
///
/// Rules are validated before being added. Invalid rules are reported
/// in the result but do not prevent valid rules from loading.
pub fn load_custom_definitions(ctx: &mut GameContext, rules_json: &str) -> LoadResult {
    let rules: Vec<serde_json::Value> = match serde_json::from_str(rules_json) {
        Ok(r) => r,
        Err(e) => {
            return LoadResult {
                loaded: 0,
                errors: vec![format!("Invalid JSON array: {e}")],
            };
        }
    };

    let mut loaded = 0;
    let mut errors = Vec::new();
    let mut valid_rules = Vec::new();

    for (i, value) in rules.into_iter().enumerate() {
        match serde_json::from_value::<CompositionRule>(value) {
            Ok(rule) => {
                if let Err(e) = validate_rule(&rule) {
                    errors.push(format!("Rule {i} '{}': {e}", rule.name));
                } else {
                    valid_rules.push(rule);
                    loaded += 1;
                }
            }
            Err(e) => {
                errors.push(format!("Rule {i}: parse error: {e}"));
            }
        }
    }

    ctx.rule_index.add_rules(valid_rules);

    LoadResult { loaded, errors }
}

/// Validate a composition rule.
fn validate_rule(rule: &CompositionRule) -> Result<(), String> {
    if rule.name.is_empty() {
        return Err("Rule name cannot be empty".into());
    }
    if rule.weight < 0.0 || rule.weight > 1.0 {
        return Err(format!("Weight must be 0.0–1.0, got {}", rule.weight));
    }
    if rule.difficulty == 0 || rule.difficulty > 5 {
        return Err(format!("Difficulty must be 1–5, got {}", rule.difficulty));
    }
    if rule.required_pattern.nodes.is_empty() {
        return Err("Required pattern must have at least one node".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::GameConfig;

    fn empty_ctx() -> GameContext {
        GameContext::from_config(&GameConfig {
            rules_json: String::new(),
            session_id: "test".into(),
        })
        .unwrap()
    }

    #[test]
    fn load_valid_rules() {
        let mut ctx = empty_ctx();
        let json = r#"[{
            "name": "has-enclosure",
            "required_pattern": { "nodes": [{"id": "e", "node_type": "enclosure"}], "edges": [] },
            "weight": 0.8,
            "difficulty": 2
        }]"#;
        let result = load_custom_definitions(&mut ctx, json);
        assert_eq!(result.loaded, 1);
        assert!(result.errors.is_empty());
        assert_eq!(ctx.rule_index.rules().len(), 1);
    }

    #[test]
    fn load_invalid_json() {
        let mut ctx = empty_ctx();
        let result = load_custom_definitions(&mut ctx, "not json");
        assert_eq!(result.loaded, 0);
        assert_eq!(result.errors.len(), 1);
    }

    #[test]
    fn load_mixed_valid_invalid() {
        let mut ctx = empty_ctx();
        let json = r#"[
            {
                "name": "good-rule",
                "required_pattern": { "nodes": [{"id": "c", "node_type": "curve"}], "edges": [] },
                "weight": 1.0,
                "difficulty": 1
            },
            {
                "name": "",
                "required_pattern": { "nodes": [{"id": "p", "node_type": "point"}], "edges": [] },
                "weight": 1.0,
                "difficulty": 1
            }
        ]"#;
        let result = load_custom_definitions(&mut ctx, json);
        assert_eq!(result.loaded, 1);
        assert_eq!(result.errors.len(), 1);
    }

    #[test]
    fn validate_rule_checks() {
        let good = CompositionRule {
            name: "test".into(),
            operation: None,
            tier: crate::types::Tier::Conventional,
            required_pattern: crate::types::GirPattern {
                nodes: vec![crate::types::PatternNode {
                    id: "p".into(),
                    node_type: Some("point".into()),
                    sort: None,
                    label: None,
                }],
                edges: vec![],
            },
            forbidden_pattern: None,
            weight: 0.5,
            difficulty: 3,
        };
        assert!(validate_rule(&good).is_ok());

        let mut bad = good.clone();
        bad.name = String::new();
        assert!(validate_rule(&bad).is_err());
    }
}
