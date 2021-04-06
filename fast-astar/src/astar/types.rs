use graphlib::VertexId;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub(super) struct ASNode {
    pub(super) g: f64,
    pub(super) f: f64,
    pub(super) node: VertexId,
    pub(super) pre: Option<VertexId>,
}

impl PartialEq for ASNode {
    fn eq(&self, other: &Self) -> bool {
        // TODO rethink maybe
        // Two DNodes are only equal, if their node is the same
        self.node.eq(&other.node)
    }
}
impl PartialEq<VertexId> for ASNode {
    fn eq(&self, other: &VertexId) -> bool {
        self.node.eq(other)
    }
}
impl Eq for ASNode {}

impl PartialOrd for ASNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for ASNode {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.node.hash(h);
    }
}

impl Ord for ASNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // TODO this is a mess
        use std::cmp::Ordering;
        if self.eq(other) {
            // TODO FILTHY HACK
            return Ordering::Equal;
        }
        if self.f == f64::NAN {
            if other.f == f64::NAN {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        } else if self.f == f64::NEG_INFINITY {
            if other.f == f64::NAN {
                Ordering::Greater
            } else if other.f == f64::NEG_INFINITY {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        } else if self.f == f64::INFINITY {
            if other.f == f64::INFINITY {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            if other.f == f64::NAN || other.f == f64::NEG_INFINITY {
                Ordering::Greater
            } else if other.f == f64::INFINITY {
                Ordering::Less
            } else {
                // <= because otherwise the insertion stuff gets messed up
                if self.f <= other.f {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        }
    }
}

impl ASNode {
    pub(super) fn start(node: VertexId) -> ASNode {
        ASNode {
            f: 0.,
            g: 0.,
            node,
            pre: None,
        }
    }

    pub(super) fn from_id(id: VertexId) -> ASNode {
        ASNode {
            f: f64::INFINITY,
            g: f64::INFINITY,
            node: id,
            pre: None,
        }
    }

    pub(super) fn new(g: f64, h: f64, node: VertexId, pre: VertexId) -> ASNode {
        ASNode {
            f: g + h,
            g,
            node,
            pre: Some(pre),
        }
    }
}
