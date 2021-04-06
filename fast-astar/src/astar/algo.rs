use crate::Graph;
use graphlib::VertexId;
use std::collections::{BTreeSet, HashMap};

use super::types::*;

/// ### A* Algorithm
/// Implementation based on https://www.redblobgames.com/pathfinding/a-star/implementation.html#cplusplus
pub fn solve(graph: &Graph, start: &VertexId, end: &VertexId) -> Option<Vec<VertexId>> {
    let mut unvisited: BTreeSet<ASNode> = BTreeSet::new();
    unvisited.insert(ASNode::start(*start));
    let mut visited: HashMap<VertexId, ASNode> = HashMap::new();

    let end_pos = graph.fetch(end).unwrap();

    while let Some(mut node) = unvisited.pop_first() {
        // println!("Evaluating {:?} ({:?})", graph.fetch(&node.node), node.node);
        if node.node == *end {
            let mut path = vec![node.node];
            let mut cur = &mut node;
            while let Some(pre) = cur.pre {
                // UNWRAP
                // println!("PATH NODE: {:?}", cur);
                cur = visited.get_mut(&pre).unwrap();
                path.push(pre);
            }
            return Some(path);
        }
        let pos = graph.fetch(&node.node).unwrap();

        for n in graph.neighbors(&node.node) {
            // TODO might have to revisit, in case the heuristic is not perfect
            if visited.contains_key(n) {
                continue;
            }
            let nb_pos = graph.fetch(n).unwrap();
            let g = nb_pos.dist(pos) + node.g;
            let h = nb_pos.dist(end_pos);
            // TODO somehow test if take works
            if let Some(mut nb) = unvisited.take(&ASNode::from_id(*n)) {
                // TODO somehow only take if g < nb.g
                if g + h < nb.f {
                    nb.g = g;
                    nb.f = g + h;
                    nb.pre = Some(node.node);
                }
                // println!("Insert OLD");
                unvisited.insert(nb);
            } else {
                // println!("Insert NEW: {:?}", n);
                unvisited.insert(ASNode::new(g, h, n.clone(), node.node));
            }
        }
        visited.insert(node.node, node);
    }

    None
}
