use crate::feature::{BondKind, Bridge};

use super::{Atom, Bond, Target};

#[derive(Debug, PartialEq, Clone)]
pub enum Edge {
    Bond(Bond),
    Gap(Atom),
}

impl Edge {
    pub fn bond_star(bond_kind: BondKind, children: Vec<Edge>) -> Self {
        Self::Bond(Bond {
            kind: bond_kind,
            target: Target::Atom(Atom::star(children)),
        })
    }

    pub fn elided_star(children: Vec<Edge>) -> Self {
        Self::Bond(Bond {
            kind: BondKind::Elided,
            target: Target::Atom(Atom::star(children)),
        })
    }

    pub fn gap_star(children: Vec<Edge>) -> Self {
        Self::Gap(Atom::star(children))
    }

    pub fn elided_bridge(bridge: Bridge) -> Self {
        Self::Bond(Bond {
            kind: BondKind::Elided,
            target: Target::Bridge(bridge),
        })
    }
}
