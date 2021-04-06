#[derive(Debug)]
pub struct Node {
    x: f64,
    y: f64,
}

impl Node {
    pub fn new(x: f64, y: f64) -> Node {
        Node { x, y }
    }

    pub fn dist(&self, other: &Node) -> f64 {
        self.sqr_dist(other).sqrt()
    }

    pub fn sqr_dist(&self, other: &Node) -> f64 {
        ((self.x - other.x) * (self.x - other.x)) + ((self.y - other.y) * (self.y - other.y))
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}

pub type Graph = graphlib::Graph<Node>;
