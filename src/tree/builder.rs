use crate::{
    feature::{AtomKind, BondKind, Bridge},
    follow::Follower,
};

use super::{Atom, Bond, Edge, Target};

#[derive(Debug, PartialEq)]
pub enum Link {
    Gap(Atom),
    Bond(BondKind, Atom),
}

#[derive(Debug, PartialEq)]
pub struct Builder {
    root: Option<Atom>,
    chain: Vec<Link>,
    stack: Vec<usize>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            root: None,
            chain: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn build(mut self) -> Atom {
        self.pop_back(0);

        match self.root.take() {
            Some(root) => root,
            None => todo!(),
        }
    }

    fn pop_back(&mut self, index: usize) {
        while let Some(link) = self.chain.pop() {
            let head = self.head();

            head.edges.push(match link {
                Link::Gap(atom) => Edge::Gap(atom),
                Link::Bond(bond_kind, atom) => Edge::Bond(Bond {
                    kind: bond_kind,
                    target: Target::Atom(atom),
                }),
            });

            if self.chain.len() == index {
                break;
            }
        }
    }

    fn head(&mut self) -> &mut Atom {
        match self.chain.last_mut() {
            Some(parent) => match parent {
                Link::Gap(atom) => atom,
                Link::Bond(_, atom) => atom,
            },
            None => self.root.as_mut().expect("root"),
        }
    }
}

impl Follower for Builder {
    fn root(&mut self, kind: &AtomKind) {
        if self.root.is_none() {
            self.root.replace(Atom::new(kind));
        } else {
            self.chain.push(Link::Gap(Atom::new(kind)))
        }
    }

    fn extend(&mut self, bond_kind: &BondKind, atom_kind: &AtomKind) {
        self.chain
            .push(Link::Bond(bond_kind.clone(), Atom::new(atom_kind)))
    }

    fn bridge(&mut self, bond_kind: &BondKind, bridge: &Bridge) {
        let head = self.head();

        head.edges.push(Edge::Bond(Bond {
            kind: bond_kind.clone(),
            target: Target::Bridge(bridge.clone()),
        }))
    }

    fn push(&mut self) {
        self.stack.push(self.chain.len())
    }

    fn pop(&mut self) {
        let index = self.stack.pop().expect("index");

        self.pop_back(index);
    }
}

#[cfg(test)]
mod build {
    use crate::tree::Edge;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn p1() {
        let mut builder = Builder::new();

        builder.root(&AtomKind::Star);

        assert_eq!(builder.build(), Atom::star(vec![]))
    }

    #[test]
    fn p1_p1() {
        let mut builder = Builder::new();

        builder.root(&AtomKind::Star);
        builder.root(&AtomKind::Star);

        assert_eq!(builder.build(), Atom::star(vec![Edge::gap_star(vec![])]))
    }

    #[test]
    fn p2() {
        let mut builder = Builder::new();

        builder.root(&AtomKind::Star);
        builder.extend(&BondKind::Elided, &AtomKind::Star);

        assert_eq!(builder.build(), Atom::star(vec![Edge::elided_star(vec![])]))
    }

    #[test]
    fn p1_p2() {
        let mut builder = Builder::new();

        builder.root(&AtomKind::Star);
        builder.root(&AtomKind::Star);
        builder.extend(&BondKind::Elided, &AtomKind::Star);

        assert_eq!(
            builder.build(),
            Atom::star(vec![Edge::gap_star(vec![Edge::elided_star(vec![])])])
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
            Atom::star(vec![Edge::elided_star(vec![Edge::elided_star(
                vec![]
            )])])
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
            Atom::star(vec![
                Edge::elided_star(vec![]),
                Edge::elided_star(vec![])
            ])
        )
    }

    #[test]
    fn c3() {
        let mut builder = Builder::new();

        builder.root(&AtomKind::Star);
        builder.bridge(&BondKind::Elided, &Bridge::B1);
        builder.extend(&BondKind::Elided, &AtomKind::Star);
        builder.extend(&BondKind::Elided, &AtomKind::Star);
        builder.bridge(&BondKind::Elided, &Bridge::B1);

        assert_eq!(
            builder.build(),
            Atom::star(vec![
                Edge::elided_bridge(Bridge::B1),
                Edge::elided_star(vec![Edge::elided_star(vec![
                    Edge::elided_bridge(Bridge::B1)
                ])])
            ])
        )
    }
}
