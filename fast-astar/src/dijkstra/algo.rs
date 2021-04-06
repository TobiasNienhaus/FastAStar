use crate::{Graph, PathfindingResult};
use graphlib::VertexId;
use std::collections::{BTreeSet, HashMap};

use super::types::*;

/// ### Dijkstra's Algorithm
/// Implementation based on https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Algorithm
pub fn solve(graph: &Graph, start: &VertexId, end: &VertexId) -> PathfindingResult {
    let mut unvisited: BTreeSet<DNode> = BTreeSet::new();
    unvisited.insert(DNode::start(*start));
    let mut visited: HashMap<VertexId, DNode> = HashMap::new();
    let mut count = 0;

    while let Some(mut node) = unvisited.pop_first() {
        count += 1;
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
            return PathfindingResult::solved(count, path);
        }
        let pos = graph.fetch(&node.node).unwrap();

        for n in graph.neighbors(&node.node) {
            if visited.contains_key(n) {
                continue;
            }
            let nb_pos = graph.fetch(n).unwrap();
            let g = nb_pos.dist(pos) + node.g;
            // TODO somehow test if take works
            if let Some(mut nb) = unvisited.take(&DNode::from_id(*n)) {
                // TODO somehow only take if g < nb.g
                if g < nb.g {
                    nb.g = g;
                    nb.pre = Some(node.node);
                }
                // println!("Insert OLD");
                unvisited.insert(nb);
            } else {
                // println!("Insert NEW: {:?}", n);
                unvisited.insert(DNode::new(g, n.clone(), node.node));
            }
        }
        visited.insert(node.node, node);
    }

    PathfindingResult::unsolved(count)
}
