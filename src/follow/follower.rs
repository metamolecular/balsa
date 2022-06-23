use crate::feature::{AtomKind, BondKind, Bridge};

pub trait Follower {
    fn root(&mut self, root: &AtomKind);

    fn extend(&mut self, bond_kind: &BondKind, atom_kind: &AtomKind);

    fn bridge(&mut self, bond_kind: &BondKind, cut: &Bridge);

    fn push(&mut self);

    fn pop(&mut self);
}
