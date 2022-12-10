use crate::feature::{AtomKind, BondKind, Bridge};

pub trait Follower {
    /// Signals the traversal of an unrooted atom. This method may be called
    /// more than once.
    fn root(&mut self, root: &AtomKind);

    /// Signlas the extension of the last atom.
    fn extend(&mut self, bond_kind: &BondKind, atom_kind: &AtomKind);

    /// Signals the attachment of a bridge to the last atom.
    fn bridge(&mut self, bond_kind: &BondKind, bridge: &Bridge);

    /// Signals the start of a branch traversal.
    fn push(&mut self);

    /// Signals the end of a branch traversal.
    fn pop(&mut self);
}
