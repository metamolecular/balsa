use std::collections::HashMap;

use crate::{
    feature::{AtomKind, BondKind, Bridge},
    follow::Follower,
};

use super::{Atom, Bond};

#[derive(Debug, PartialEq)]
pub struct Builder {
    atoms: Vec<Atom>,
    head: Option<usize>,
    stack: Vec<usize>,
    bridges: HashMap<Bridge, (usize, usize, BondKind)>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            atoms: Vec::new(),
            stack: Vec::new(),
            head: None,
            bridges: HashMap::new(),
        }
    }

    pub fn build(self) -> Vec<Atom> {
        self.atoms
    }
}

impl Follower for Builder {
    fn root(&mut self, kind: &AtomKind) {
        let id = self.atoms.len();

        self.atoms.push(Atom {
            kind: kind.clone(),
            bonds: Vec::new(),
        });
        self.head.replace(id);
    }

    fn extend(&mut self, bond_kind: &BondKind, atom_kind: &AtomKind) {
        let id = self.atoms.len();
        let sid = self.head.replace(id).expect("head");

        self.atoms.get_mut(sid).expect("head").bonds.push(Bond {
            kind: bond_kind.clone(),
            tid: id,
        });
        self.atoms.push(Atom {
            kind: atom_kind.clone(),
            bonds: vec![Bond {
                kind: bond_kind.clone(),
                tid: sid,
            }],
        });
    }

    fn bridge(&mut self, source_kind: &BondKind, bridge: &Bridge) {
        match self.bridges.entry(bridge.clone()) {
            std::collections::hash_map::Entry::Occupied(occupied) => {
                let (tid, iid, target_kind) = occupied.remove();
                let sid = self.head.expect("head");
                let source = self.atoms.get_mut(sid).expect("source");

                source.bonds.push(Bond {
                    kind: source_kind.clone(),
                    tid,
                });

                let target = self.atoms.get_mut(tid).expect("target");

                target.bonds.insert(
                    iid,
                    Bond {
                        kind: target_kind,
                        tid: sid,
                    },
                )
            }
            std::collections::hash_map::Entry::Vacant(vacant) => {
                let head = self.head.expect("head");
                let insertion = self.atoms.get(head).expect("head").bonds.len();

                vacant.insert((head, insertion, source_kind.clone()));
            }
        }
    }

    fn push(&mut self) {
        self.stack.push(self.head.expect("head"))
    }

    fn pop(&mut self) {
        self.head.replace(self.stack.pop().expect("head"));
    }
}

#[cfg(test)]
mod build {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn p1() {
        let mut builder = Builder::new();

        builder.root(&AtomKind::Star);

        assert_eq!(builder.build(), vec![Atom::star(vec![])])
    }

    #[test]
    fn p1_p1() {
        let mut builder = Builder::new();

        builder.root(&AtomKind::Star);
        builder.root(&AtomKind::Star);

        assert_eq!(
            builder.build(),
            vec![Atom::star(vec![]), Atom::star(vec![])]
        )
    }

    #[test]
    fn p2() {
        let mut builder = Builder::new();

        builder.root(&AtomKind::Star);
        builder.extend(&BondKind::Elided, &AtomKind::Star);

        assert_eq!(
            builder.build(),
            vec![
                Atom::star(vec![Bond::elided(1)]),
                Atom::star(vec![Bond::elided(0)])
            ]
        )
    }

    #[test]
    fn p1_p2() {
        let mut builder = Builder::new();

        builder.root(&AtomKind::Star);
        builder.root(&AtomKind::Star);
        builder.extend(&BondKind::Elided, &AtomKind::Star);

        assert_eq!(
            builder.build(),
            vec![
                Atom::star(vec![]),
                Atom::star(vec![Bond::elided(2)]),
                Atom::star(vec![Bond::elided(1)])
            ]
        )
    }

    #[test]
    fn p3() {
        let mut builder = Builder::new();

        builder.root(&AtomKind::Star);
        builder.extend(&BondKind::Elided, &AtomKind::Star);
        builder.extend(&BondKind::Elided, &AtomKind::Star);

        assert_eq!(
            builder.build(),
            vec![
                Atom::star(vec![Bond::elided(1)]),
                Atom::star(vec![Bond::elided(0), Bond::elided(2)]),
                Atom::star(vec!(Bond::elided(1)))
            ]
        )
    }

    #[test]
    fn p3_branched() {
        let mut builder = Builder::new();

        builder.root(&AtomKind::Star);
        builder.push();
        builder.extend(&BondKind::Elided, &AtomKind::Star);
        builder.pop();
        builder.extend(&BondKind::Elided, &AtomKind::Star);

        assert_eq!(
            builder.build(),
            vec![
                Atom::star(vec![Bond::elided(1), Bond::elided(2)]),
                Atom::star(vec![Bond::elided(0)]),
                Atom::star(vec![Bond::elided(0)])
            ]
        )
    }

    #[test]
    fn c3_bridge_first() {
        let mut builder = Builder::new();

        builder.root(&AtomKind::Star);
        builder.bridge(&BondKind::Elided, &Bridge::B1);
        builder.extend(&BondKind::Elided, &AtomKind::Star);
        builder.extend(&BondKind::Elided, &AtomKind::Star);
        builder.bridge(&BondKind::Elided, &Bridge::B1);

        assert_eq!(
            builder.build(),
            vec![
                Atom::star(vec![Bond::elided(2), Bond::elided(1)]),
                Atom::star(vec![Bond::elided(0), Bond::elided(2)]),
                Atom::star(vec![Bond::elided(1), Bond::elided(0)])
            ]
        )
    }

    #[test]
    fn c3_bridge_last() {
        let mut builder = Builder::new();

        builder.root(&AtomKind::Star);
        builder.push();
        builder.extend(&BondKind::Elided, &AtomKind::Star);
        builder.extend(&BondKind::Elided, &AtomKind::Star);
        builder.bridge(&BondKind::Elided, &Bridge::B1);
        builder.pop();
        builder.bridge(&BondKind::Elided, &Bridge::B1);

        assert_eq!(
            builder.build(),
            vec![
                Atom::star(vec![Bond::elided(1), Bond::elided(2)]),
                Atom::star(vec![Bond::elided(0), Bond::elided(2)]),
                Atom::star(vec![Bond::elided(1), Bond::elided(0)])
            ]
        )
    }
}
