use graphlib::VertexId;

#[feature(map_first_last)]
use std::collections::BTreeSet;

use std::collections::HashMap;

pub type Graph = super::graph_types::Graph;

#[derive(Debug, Clone)]
struct DNode {
    g: f64,
    node: VertexId,
    pre: Option<VertexId>,
}

impl PartialEq for DNode {
    fn eq(&self, other: &Self) -> bool {
        // TODO rethink maybe
        // Two DNodes are only equal, if their node is the same
        self.node.eq(&other.node)
    }
}
impl Eq for DNode {}

impl PartialOrd for DNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.g.partial_cmp(&other.g)
    }
}

impl Ord for DNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // TODO this is a mess
        use std::cmp::Ordering;
        if self.g == f64::NAN {
            if other.g == f64::NAN {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        } else if self.g == f64::NEG_INFINITY {
            if other.g == f64::NAN {
                Ordering::Less
            } else if other.g == f64::NEG_INFINITY {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        } else if self.g == f64::INFINITY {
            if other.g == f64::INFINITY {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        } else {
            if other.g == f64::NAN || other.g == f64::NEG_INFINITY {
                Ordering::Greater
            } else if other.g == f64::INFINITY {
                Ordering::Less
            } else {
                if self.g < other.g {
                    Ordering::Less
                } else if self.g == other.g {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            }
        }
    }
}

impl DNode {
    pub fn start(node: VertexId) -> DNode {
        DNode {
            g: 0.,
            node,
            pre: None,
        }
    }

    pub fn other(node: VertexId) -> DNode {
        DNode {
            g: f64::INFINITY,
            node,
            pre: None,
        }
    }
}

pub fn algo(graph: &Graph, start: &VertexId, end: &VertexId) -> Option<Vec<VertexId>> {
    // TODO use some better type than Vec
    let mut nodes: Vec<DNode> = Vec::with_capacity(graph.vertex_count());
    let mut visited: HashMap<VertexId, DNode> = HashMap::new();
    nodes.extend(graph.vertices().map(|v| {
        if v == start {
            DNode::start(*v)
        } else {
            DNode::other(*v)
        }
    }));

    let print_node = |n: &DNode| {
        println!("Node: {:?}\n- Data: {:?}", n, graph.fetch(&n.node));
    };
    nodes.iter().for_each(print_node);
    println!("-------------------------\nMin node:\n");
    print_node(
        nodes
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap(),
    );

    // Closures just wouldn't cut it anymore
    fn min_node<'a>(nvec: &'a Vec<DNode>) -> Option<usize> {
        nvec.iter()
            .enumerate()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|val| {
                if val.1.g == f64::INFINITY {
                    None
                } else {
                    Some(val)
                }
            })
            .flatten() // Big Brain Inc.
            .map(|a| a.0)
    }

    fn find_by_id<'a>(nvec: &'a mut Vec<DNode>, id: VertexId) -> Option<&'a mut DNode> {
        nvec.iter_mut().find(|a| a.node == id)
    }

    while let Some(index) = min_node(&nodes) {
        let mut node = nodes.remove(index);
        println!("Evaluating {:?}", node.node);
        if node.node == *end {
            let mut path = vec![node.node];
            let mut cur = &mut node;
            while let Some(pre) = cur.pre {
                // UNWRAP
                cur = visited.get_mut(&pre).unwrap();
                path.push(pre);
            }
            return Some(path);
        }
        let pos = graph.fetch(&node.node).unwrap();

        for n in graph.neighbors(&node.node) {
            if visited.contains_key(n) {
                continue;
            }
            // TODO find_by_id is O(n) at least which is bad
            if let Some(nb) = find_by_id(&mut nodes, *n) {
                let nb_pos = graph.fetch(n).unwrap();
                let g = nb_pos.dist(pos) + node.g;
                match nb.pre {
                    None => {
                        nb.g = g;
                        nb.pre = Some(node.node);
                    }
                    Some(_) => {
                        if g < nb.g {
                            nb.g = g;
                            nb.pre = Some(node.node);
                        }
                    }
                }
            }
        }
        visited.insert(node.node, node);
    }

    None
}
