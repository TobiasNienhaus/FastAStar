use std::f64::INFINITY;

use graphlib::{VertexId};
use super::graph_types::{self, Node};

pub type Graph = graph_types::Graph;

#[derive(Debug)]
struct DNode {
    g: f64,
    node: VertexId,
    pre: Option<VertexId>
}

impl PartialEq for DNode {
    fn eq(&self, other: &Self) -> bool {
        // TODO rethink maybe
        // Two DNodes are only equal, if their node is the same
        self.node.eq(&other.node)
    }
}

impl PartialOrd for DNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.g.partial_cmp(&other.g)
    }
}

impl DNode {
    pub fn start(node: VertexId) -> DNode {
        DNode {
            g: 0., node, pre: None
        }
    }

    pub fn other(node: VertexId) -> DNode {
        DNode {
            g: f64::INFINITY, node, pre: None
        }
    }
}

pub fn algo(graph: &Graph, start: &VertexId, end: &VertexId) {
    // TODO use some better type than Vec
    let mut nodes: Vec<DNode> = Vec::with_capacity(graph.vertex_count());
    nodes.extend(graph.vertices().map(|v| {
        if v == start { DNode::start(*v) } else { DNode::other(*v) }
    }));

    let print_node = |n: &DNode| {
        println!("Node: {:?}\n- Data: {:?}", n, graph.fetch(&n.node));
    };
    nodes.iter().for_each(print_node);
    println!("-------------------------\nMin node:\n");
    print_node(nodes.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap());

    let min_node = || {
        // UNWRAP
        nodes.iter().min_by(|a, b| a.partial_cmp(b).unwrap())
    };

    while let Some(node) = min_node() {
        let pos = graph.fetch(&node.node).unwrap();
    }
}