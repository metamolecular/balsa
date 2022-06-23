use crate::feature::AtomKind;

use super::Edge;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Atom {
    pub kind: AtomKind,
    pub edges: Vec<Edge>,
}

impl Atom {
    pub fn new(kind: &AtomKind) -> Self {
        Self {
            kind: kind.clone(),
            edges: Vec::new(),
        }
    }

    pub fn star(edges: Vec<Edge>) -> Self {
        Self {
            kind: AtomKind::Star,
            edges,
        }
    }
}
