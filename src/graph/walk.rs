use std::collections::HashMap;

use crate::follow::Follower;

use super::{Atom, Bond, BridgePool};

pub fn walk(atoms: &Vec<Atom>, follower: &mut impl Follower) {
    let mut atoms = atoms.into_iter().enumerate().collect::<HashMap<_, _>>();

    for i in 0..atoms.len() {
        walk_root(i, &mut atoms, follower)
    }
}

fn walk_root(
    mut hid: usize,
    atoms: &mut HashMap<usize, &Atom>,
    follower: &mut impl Follower,
) {
    let root = match atoms.remove(&hid) {
        Some(atom) => atom,
        None => return,
    };
    let mut bridge_pool = BridgePool::new();
    let mut stack = bonds(hid, root, None).collect::<Vec<_>>();

    follower.root(&root.kind);

    while let Some((push, sid, bond)) = stack.pop() {
        if sid != hid {
            follower.pop();
        }

        let target = match atoms.remove(&bond.tid) {
            Some(target) => target,
            None => {
                follower.bridge(&bond.kind, &bridge_pool.hit(sid, bond.tid));

                continue
            }
        };

        if push {
            follower.push();
        }

        hid = bond.tid;

        stack.extend(bonds(bond.tid, &target, Some(sid)));
        follower.extend(&bond.kind, &target.kind);
    }
}

fn bonds(
    id: usize,
    atom: &Atom,
    back: Option<usize>,
) -> impl Iterator<Item = (bool, usize, &Bond)> {
    atom.bonds
        .iter()
        .filter(move |bond| match back {
            Some(back) => bond.tid != back,
            None => true,
        })
        .rev()
        .enumerate()
        .map(move |(i, bond)| {
            (
                i != 0 && atom.bonds.len() > if back.is_some() { 2 } else { 1 },
                id,
                bond,
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{follow::Writer, feature::{Bracket, Symbol, AtomKind, Element}};
    use pretty_assertions::assert_eq;

    #[test]
    fn p1() {
        let graph = vec![Atom::star(vec![])];
        let mut writer = Writer::new();

        walk(&graph, &mut writer);

        assert_eq!(writer.write(), "*")
    }

    #[test]
    fn bracket() {
        let graph = vec![Atom {
            kind: AtomKind::Bracket(Bracket {
                symbol: Symbol::Element(Element::Tc),
                ..Default::default()
            }),
            bonds: vec![]
        }];
        let mut writer = Writer::new();

        walk(&graph, &mut writer);

        assert_eq!(writer.write(), "[Tc]");
    }

    #[test]
    fn p1_p2() {
        let graph = vec![
            Atom::star(vec![]),
            Atom::star(vec![Bond::elided(2)]),
            Atom::star(vec![Bond::elided(1)]),
        ];
        let mut writer = Writer::new();

        walk(&graph, &mut writer);

        assert_eq!(writer.write(), "*.**");
    }

    #[test]
    fn p2() {
        let graph = vec![
            Atom::star(vec![Bond::elided(1)]),
            Atom::star(vec![Bond::elided(0)]),
        ];
        let mut writer = Writer::new();

        walk(&graph, &mut writer);

        assert_eq!(writer.write(), "**")
    }

    #[test]
    fn p2_triple() {
        let graph = vec![
            Atom::star(vec![Bond::triple(1)]),
            Atom::star(vec![Bond::triple(0)])
        ];
        let mut writer = Writer::new();

        walk(&graph, &mut writer);

        assert_eq!(writer.write(), "*#*")
    }

    #[test]
    fn p3_terminal() {
        let root = vec![
            Atom::star(vec![Bond::elided(1)]),
            Atom::star(vec![Bond::elided(0), Bond::elided(2)]),
            Atom::star(vec![Bond::elided(1)]),
        ];
        let mut writer = Writer::new();

        walk(&root, &mut writer);

        assert_eq!(writer.write(), "***")
    }

    #[test]
    fn p3_branched() {
        let root = vec![
            Atom::star(vec![Bond::elided(1), Bond::elided(2)]),
            Atom::star(vec![Bond::elided(0)]),
            Atom::star(vec![Bond::elided(0)]),
        ];
        let mut writer = Writer::new();

        walk(&root, &mut writer);

        assert_eq!(writer.write(), "*(*)*")
    }

    #[test]
    fn c3() {
        let root = vec![
            Atom::star(vec![Bond::elided(1), Bond::elided(2)]),
            Atom::star(vec![Bond::elided(0), Bond::elided(2)]),
            Atom::star(vec![Bond::elided(1), Bond::elided(0)]),
        ];
        let mut writer = Writer::new();

        walk(&root, &mut writer);

        assert_eq!(writer.write(), "*(**1)1")
    }

    #[test]
    fn c4() {
        let root = vec![
            Atom::star(vec![Bond::elided(1), Bond::elided(3)]),
            Atom::star(vec![Bond::elided(0), Bond::elided(2)]),
            Atom::star(vec![Bond::elided(1), Bond::elided(3)]),
            Atom::star(vec![Bond::elided(2), Bond::elided(0)]),
        ];
        let mut writer = Writer::new();

        walk(&root, &mut writer);

        assert_eq!(writer.write(), "*(***1)1")
    }

    #[test]
    fn s3_terminal() {
        let graph = vec![
            Atom::star(vec![Bond::elided(1)]),
            Atom::star(vec![
                Bond::elided(0),
                Bond::elided(2),
                Bond::elided(3),
            ]),
            Atom::star(vec![Bond::elided(1)]),
            Atom::star(vec![Bond::elided(1)])
        ];
        let mut writer = Writer::new();

        walk(&graph, &mut writer);

        assert_eq!(writer.write(), "**(*)*")
    }

    #[test]
    fn s4_terminal() {
        let graph = vec![
            Atom::star(vec![Bond::elided(1)]),
            Atom::star(vec![
                Bond::elided(0),
                Bond::elided(2),
                Bond::elided(3),
                Bond::elided(4),
            ]),
            Atom::star(vec![Bond::elided(1)]),
            Atom::star(vec![Bond::elided(1)]),
            Atom::star(vec![Bond::elided(1)])
        ];
        let mut writer = Writer::new();

        walk(&graph, &mut writer);

        assert_eq!(writer.write(), "**(*)(*)*")
    }

    #[test]
    fn diamond() {
        let graph = vec![
            Atom::star(vec![Bond::elided(1), Bond::elided(3)]),
            Atom::star(vec![
                Bond::elided(0),
                Bond::elided(2),
                Bond::elided(3),
            ]),
            Atom::star(vec![Bond::elided(1), Bond::elided(3)]),
            Atom::star(vec![
                Bond::elided(2),
                Bond::elided(0),
                Bond::elided(1),
            ]),
        ];
        let mut writer = Writer::new();

        walk(&graph, &mut writer);

        assert_eq!(writer.write(), "*(*(**12)2)1")
    }
}
