//! Animation keyframe generation from GIR layout.
//!
//! Produces construction-order keyframes (BFS from root) for Phaser tween integration.

use ul_core::Gir;

use crate::types::{AnimationKeyframe, AnimationSequence, Easing};

/// Base delay between successive node appearances (ms).
const NODE_DELAY_MS: u32 = 150;
/// Duration of each node's appear tween (ms).
const APPEAR_DURATION_MS: u32 = 300;

/// Generate a construction-order animation sequence for a GIR.
///
/// Nodes appear in BFS order from the root. Each node fades in and scales up
/// at its final layout position.
pub fn get_animation_sequence(gir: &Gir, width: f64, height: f64) -> AnimationSequence {
    let positioned = ul_core::compute_layout(gir, width, height);

    // BFS order from root
    let bfs_order = bfs_order(gir);

    let mut keyframes = Vec::new();
    let mut node_index = 0u32;

    for node_id in &bfs_order {
        // Find this node's position in the layout
        let elem = positioned
            .elements
            .iter()
            .find(|e| e.node_id == *node_id);

        let (x, y) = match elem {
            Some(e) => (e.x, e.y),
            None => continue, // invisible nodes (e.g., implicit root)
        };

        let start_time = node_index * NODE_DELAY_MS;

        // Keyframe 1: invisible (start state)
        keyframes.push(AnimationKeyframe {
            node_id: node_id.clone(),
            timestamp_ms: start_time,
            x,
            y,
            scale: 0.0,
            rotation: 0.0,
            opacity: 0.0,
            easing: Easing::EaseOut,
        });

        // Keyframe 2: visible (end state)
        keyframes.push(AnimationKeyframe {
            node_id: node_id.clone(),
            timestamp_ms: start_time + APPEAR_DURATION_MS,
            x,
            y,
            scale: 1.0,
            rotation: 0.0,
            opacity: 1.0,
            easing: Easing::EaseOut,
        });

        node_index += 1;
    }

    let total = if node_index == 0 {
        0
    } else {
        (node_index - 1) * NODE_DELAY_MS + APPEAR_DURATION_MS
    };

    AnimationSequence {
        keyframes,
        total_duration_ms: total,
    }
}

/// BFS traversal order from the GIR root.
fn bfs_order(gir: &Gir) -> Vec<String> {
    let mut order = Vec::new();
    let mut queue = std::collections::VecDeque::new();
    let mut visited = std::collections::HashSet::new();

    queue.push_back(gir.root.clone());
    visited.insert(gir.root.clone());

    while let Some(node_id) = queue.pop_front() {
        // Skip implicit root from animation
        let is_implicit = gir
            .node(&node_id)
            .and_then(|n| n.label.as_deref())
            .is_some_and(|l| l == "_implicit_root");

        if !is_implicit {
            order.push(node_id.clone());
        }

        for child_id in gir.children_of(&node_id) {
            if visited.insert(child_id.to_string()) {
                queue.push_back(child_id.to_string());
            }
        }
    }

    order
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_point_animation() {
        let gir = ul_core::parser::parse("*").unwrap();
        let seq = get_animation_sequence(&gir, 200.0, 200.0);

        // Should have at least 2 keyframes (start + end) for visible nodes
        assert!(
            seq.keyframes.len() >= 2,
            "expected >=2 keyframes, got {}",
            seq.keyframes.len()
        );
        assert!(seq.total_duration_ms > 0);

        // First keyframe should be invisible
        let first = &seq.keyframes[0];
        assert_eq!(first.opacity, 0.0);
        assert_eq!(first.scale, 0.0);

        // Second keyframe should be visible
        let second = &seq.keyframes[1];
        assert_eq!(second.opacity, 1.0);
        assert_eq!(second.scale, 1.0);
    }

    #[test]
    fn enclosure_animation_has_multiple_nodes() {
        let gir = ul_core::parser::parse("/0{*}").unwrap();
        let seq = get_animation_sequence(&gir, 200.0, 200.0);

        // Should have keyframes for both enclosure and point
        let unique_nodes: std::collections::HashSet<&str> =
            seq.keyframes.iter().map(|k| k.node_id.as_str()).collect();
        assert!(
            unique_nodes.len() >= 2,
            "expected >=2 unique nodes, got {:?}",
            unique_nodes
        );
    }

    #[test]
    fn bfs_order_starts_from_root() {
        let gir = ul_core::parser::parse("/0{*}").unwrap();
        let order = bfs_order(&gir);
        assert!(!order.is_empty());
    }
}
