use crate::feature::AtomKind;

use super::Bond;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Atom {
    pub kind: AtomKind,
    pub bonds: Vec<Bond>,
}

impl Atom {
    pub fn star(bonds: Vec<Bond>) -> Self {
        Self {
            kind: AtomKind::Star,
            bonds,
        }
    }
}
