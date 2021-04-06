use graphlib::VertexId;

pub enum PathfindingResult {
    Solved { visited: usize, path: Vec<VertexId> },
    Unsolved { visited: usize },
}

impl PathfindingResult {
    pub fn solved(visited: usize, path: Vec<VertexId>) -> PathfindingResult {
        PathfindingResult::Solved { visited, path }
    }

    pub fn unsolved(visited: usize) -> PathfindingResult {
        PathfindingResult::Unsolved { visited }
    }

    pub fn is_solved(&self) -> bool {
        match self {
            PathfindingResult::Solved {
                visited: _,
                path: _,
            } => true,
            _ => false,
        }
    }

    pub fn visited(&self) -> usize {
        match self {
            PathfindingResult::Solved { visited, path: _ } => *visited,
            PathfindingResult::Unsolved { visited } => *visited,
        }
    }

    pub fn path(&self) -> Option<&Vec<VertexId>> {
        match self {
            PathfindingResult::Solved { visited: _, path } => Some(path),
            PathfindingResult::Unsolved { visited: _ } => None,
        }
    }
}
