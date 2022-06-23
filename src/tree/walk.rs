use super::{Edge, Target};
use crate::{follow::Follower, tree::Atom};

pub fn walk(root: &Atom, follower: &mut impl Follower) {
    follower.root(&root.kind);

    let mut stack = edges(0, root).collect::<Vec<_>>();
    let mut depth = 0;

    while let Some((root, push, edge)) = stack.pop() {
        if root < depth {
            follower.pop();

            depth = root;
        }

        match edge {
            Edge::Bond(bond) => match &bond.target {
                Target::Atom(atom) => {
                    depth += 1;

                    if push {
                        follower.push();
                    }

                    stack.extend(edges(depth, atom));
                    follower.extend(&bond.kind, &atom.kind)
                }
                Target::Bridge(bridge) => {
                    follower.bridge(&bond.kind, bridge);
                }
            },
            Edge::Gap(atom) => {
                depth += 1;

                if push {
                    follower.push();
                }

                stack.extend(edges(depth, atom));
                follower.root(&atom.kind);
            }
        }
    }
}

fn edges(
    root: usize,
    atom: &Atom,
) -> impl Iterator<Item = (usize, bool, &Edge)> {
    atom.edges
        .iter()
        .rev()
        .enumerate()
        .map(move |(i, edge)| (root, atom.edges.len() > 1 && i != 0, edge))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feature::{
        AtomKind, BondKind, Bracket, Bridge, Element, Symbol,
    };
    use crate::follow::Writer;
    use pretty_assertions::assert_eq;

    #[test]
    fn p1() {
        let root = Atom::star(vec![]);
        let mut writer = Writer::new();

        walk(&root, &mut writer);

        assert_eq!(writer.write(), "*")
    }

    #[test]
    fn bracket() {
        let root = Atom {
            kind: AtomKind::Bracket(Bracket {
                symbol: Symbol::Element(Element::Tc),
                ..Default::default()
            }),
            edges: vec![],
        };
        let mut writer = Writer::new();

        walk(&root, &mut writer);

        assert_eq!(writer.write(), "[Tc]");
    }

    #[test]
    fn p2() {
        let root = Atom::star(vec![Edge::elided_star(vec![])]);
        let mut writer = Writer::new();

        walk(&root, &mut writer);

        assert_eq!(writer.write(), "**")
    }

    #[test]
    fn p2_triple() {
        let root = Atom::star(vec![Edge::bond_star(BondKind::Triple, vec![])]);
        let mut writer = Writer::new();

        walk(&root, &mut writer);

        assert_eq!(writer.write(), "*#*")
    }

    #[test]
    fn p3_branched() {
        let root = Atom::star(vec![
            Edge::elided_star(vec![]),
            Edge::elided_star(vec![]),
        ]);
        let mut writer = Writer::new();

        walk(&root, &mut writer);

        assert_eq!(writer.write(), "*(*)*")
    }

    #[test]
    fn p2_p1_branch_center() {
        let root =
            Atom::star(vec![Edge::gap_star(vec![]), Edge::elided_star(vec![])]);
        let mut writer = Writer::new();

        walk(&root, &mut writer);

        assert_eq!(writer.write(), "*(.*)*")
    }

    #[test]
    fn p3_terminal() {
        let root =
            Atom::star(vec![Edge::elided_star(vec![Edge::elided_star(
                vec![],
            )])]);
        let mut writer = Writer::new();

        walk(&root, &mut writer);

        assert_eq!(writer.write(), "***")
    }

    #[test]
    fn c3() {
        let root = Atom::star(vec![
            Edge::elided_bridge(Bridge::B1),
            Edge::elided_star(vec![Edge::elided_star(vec![
                Edge::elided_bridge(Bridge::B1),
            ])]),
        ]);
        let mut writer = Writer::new();

        walk(&root, &mut writer);

        assert_eq!(writer.write(), "*1**1")
    }

    #[test]
    fn c4() {
        let root = Atom::star(vec![
            Edge::elided_star(vec![Edge::elided_star(vec![
                Edge::elided_star(vec![Edge::elided_bridge(Bridge::B1)]),
            ])]),
            Edge::elided_bridge(Bridge::B1),
        ]);
        let mut writer = Writer::new();

        walk(&root, &mut writer);

        assert_eq!(writer.write(), "*(***1)1")
    }

    #[test]
    fn s3_terminal() {
        let root = Atom::star(vec![Edge::elided_star(vec![
            Edge::elided_star(vec![]),
            Edge::elided_star(vec![]),
        ])]);
        let mut writer = Writer::new();

        walk(&root, &mut writer);

        assert_eq!(writer.write(), "**(*)*")
    }

    #[test]
    fn s4_terminal() {
        let root = Atom::star(vec![Edge::elided_star(vec![
            Edge::elided_star(vec![]),
            Edge::elided_star(vec![]),
            Edge::elided_star(vec![]),
        ])]);
        let mut writer = Writer::new();

        walk(&root, &mut writer);

        assert_eq!(writer.write(), "**(*)(*)*")
    }

    #[test]
    fn diamond() {
        let root = Atom::star(vec![
            Edge::elided_star(vec![
                Edge::elided_star(vec![Edge::elided_star(vec![
                    Edge::elided_bridge(Bridge::B1),
                    Edge::elided_bridge(Bridge::B2),
                ])]),
                Edge::elided_bridge(Bridge::B2),
            ]),
            Edge::elided_bridge(Bridge::B1),
        ]);
        let mut writer = Writer::new();

        walk(&root, &mut writer);

        assert_eq!(writer.write(), "*(*(**12)2)1")
    }
}
