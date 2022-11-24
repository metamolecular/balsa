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

    pub fn subvalence(&self, input: Option<&Edge>) -> u8 {
        todo!()
    }
}

#[cfg(test)]
mod subvalence {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]#[ignore]
    fn star_input_none() {
        let atom = Atom::star(vec![]);
        let input = None;

        assert_eq!(atom.subvalence(input), 0)
    }
}
