use graphlib::VertexId;

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
impl PartialEq<VertexId> for DNode {
    fn eq(&self, other: &VertexId) -> bool {
        self.node.eq(other)
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
        if self.eq(other) {
            // TODO FILTHY HACK
            return Ordering::Equal;
        }
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

    pub fn from_id(id: VertexId) -> DNode {
        DNode {
            g: f64::INFINITY,
            node: id,
            pre: None,
        }
    }

    pub fn new(g: f64, node: VertexId, pre: VertexId) -> DNode {
        DNode {
            g,
            node,
            pre: Some(pre),
        }
    }
}

pub fn algo(graph: &Graph, start: &VertexId, end: &VertexId) -> Option<Vec<VertexId>> {
    // TODO use some better type than Vec
    let mut unvisited: BTreeSet<DNode> = BTreeSet::new();
    unvisited.insert(DNode::start(*start));
    let mut visited: HashMap<VertexId, DNode> = HashMap::new();

    while let Some(mut node) = unvisited.pop_first() {
        println!("Evaluating {:?}", node.node);
        if node.node == *end {
            let mut path = vec![node.node];
            let mut cur = &mut node;
            while let Some(pre) = cur.pre {
                // UNWRAP
                println!("PATH NODE: {:?}", cur);
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
            let nb_pos = graph.fetch(n).unwrap();
            let g = nb_pos.dist(pos) + node.g;
            if let Some(mut nb) = unvisited.take(&DNode::from_id(*n)) {
                // TODO somehow only take if g < nb.g
                if g < nb.g {
                    nb.g = g;
                    nb.pre = Some(node.node);
                }
                unvisited.insert(nb);
                println!("Insert OLD");
            } else {
                println!("Insert NEW");
                unvisited.insert(DNode::new(g, *n, node.node));
            }
        }
        visited.insert(node.node, node);
    }

    None
}
