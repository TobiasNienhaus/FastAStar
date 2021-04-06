use graphlib::VertexId;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub(super) struct DNode {
    pub(super) g: f64,
    pub(super) node: VertexId,
    pub(super) pre: Option<VertexId>,
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
        Some(self.cmp(other))
    }
}

impl Hash for DNode {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.node.hash(h);
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
                Ordering::Greater
            } else if other.g == f64::NEG_INFINITY {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        } else if self.g == f64::INFINITY {
            if other.g == f64::INFINITY {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            if other.g == f64::NAN || other.g == f64::NEG_INFINITY {
                Ordering::Greater
            } else if other.g == f64::INFINITY {
                Ordering::Less
            } else {
                // <= because otherwise the insertion stuff gets messed up
                if self.g <= other.g {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        }
    }
}

impl DNode {
    pub(super) fn start(node: VertexId) -> DNode {
        DNode {
            g: 0.,
            node,
            pre: None,
        }
    }

    pub(super) fn from_id(id: VertexId) -> DNode {
        DNode {
            g: f64::INFINITY,
            node: id,
            pre: None,
        }
    }

    pub(super) fn new(g: f64, node: VertexId, pre: VertexId) -> DNode {
        DNode {
            g,
            node,
            pre: Some(pre),
        }
    }
}
