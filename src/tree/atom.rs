use crate::feature::{AtomKind, BondKind, Bracket, Selection, Shortcut};

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

    pub fn shortcut(shortcut: Shortcut, edges: Vec<Edge>) -> Self {
        Self {
            kind: AtomKind::Shortcut(shortcut),
            edges,
        }
    }

    pub fn selection(selection: Selection, edges: Vec<Edge>) -> Self {
        Self {
            kind: AtomKind::Selection(selection),
            edges,
        }
    }

    pub fn bracket(bracket: Bracket, edges: Vec<Edge>) -> Self {
        Self {
            kind: AtomKind::Bracket(bracket),
            edges,
        }
    }

    pub fn valence(&self, input: Option<&BondKind>) -> u8 {
        let input_bond_order = match input {
            Some(input) => input.bond_order(),
            None => 0,
        };
        let virtual_hydrogens = self.virtual_hydrogens();
        let child_bond_order_sum =
            self.edges.iter().fold(0, |r, e| r + e.bond_order());

        input_bond_order + virtual_hydrogens + child_bond_order_sum
    }

    pub fn virtual_hydrogens(&self) -> u8 {
        match &self.kind {
            AtomKind::Star | AtomKind::Shortcut(_) | AtomKind::Selection(_) => {
                0
            }
            AtomKind::Bracket(bracket) => match &bracket.hydrogens {
                Some(hydrogens) => hydrogens.into(),
                None => 0,
            },
        }
    }

    pub fn subvalence(&self, input: Option<&BondKind>) -> u8 {
        self.kind.subvalence(self.valence(input))
    }
}

#[cfg(test)]
mod valence {
    use super::*;
    use crate::feature::VirtualHydrogen;
    use pretty_assertions::assert_eq;

    #[test]
    fn star_input_none_children_none() {
        let atom = Atom::star(vec![]);
        let input = None;

        assert_eq!(atom.valence(input), 0)
    }

    #[test]
    fn star_input_double_children_none() {
        let atom = Atom::star(vec![]);
        let input = Some(BondKind::Double);

        assert_eq!(atom.valence(input.as_ref()), 2)
    }

    #[test]
    fn bracket_hydrogens_one_input_single_edges_none() {
        let atom = Atom::bracket(
            Bracket {
                hydrogens: Some(VirtualHydrogen::H),
                ..Default::default()
            },
            vec![],
        );
        let input = Some(BondKind::Single);

        assert_eq!(atom.valence(input.as_ref()), 2)
    }

    #[test]
    fn bracket_hydrogens_one_input_single_edges_gap() {
        let atom = Atom::bracket(
            Bracket {
                hydrogens: Some(VirtualHydrogen::H),
                ..Default::default()
            },
            vec![Edge::gap_star(vec![])],
        );
        let input = Some(BondKind::Single);

        assert_eq!(atom.valence(input.as_ref()), 2)
    }

    #[test]
    fn bracket_hydrogens_one_input_single_edges_bond_double() {
        let atom = Atom::bracket(
            Bracket {
                hydrogens: Some(VirtualHydrogen::H),
                ..Default::default()
            },
            vec![Edge::bond_star(BondKind::Double, vec![])],
        );
        let input = Some(BondKind::Single);

        assert_eq!(atom.valence(input.as_ref()), 4)
    }
}

#[cfg(test)]
mod subvalence {
    use crate::feature::{Element, Symbol, VirtualHydrogen};

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn star_input_single() {
        let atom = Atom::star(vec![]);
        let input = Some(&BondKind::Single);

        assert_eq!(atom.subvalence(input), 0)
    }

    #[test]
    fn shortcut_input_single() {
        let atom = Atom::shortcut(Shortcut::C, vec![]);
        let input = Some(&BondKind::Single);

        assert_eq!(atom.subvalence(input), 3)
    }

    #[test]
    fn selection_input_single() {
        let atom = Atom::selection(Selection::C, vec![]);
        let input = Some(&BondKind::Single);

        assert_eq!(atom.subvalence(input), 3)
    }

    #[test]
    fn bracket_input_single() {
        let atom = Atom::bracket(
            Bracket {
                symbol: Symbol::Element(Element::C),
                hydrogens: Some(VirtualHydrogen::H2),
                ..Default::default()
            },
            vec![],
        );
        let input = Some(&BondKind::Single);

        assert_eq!(atom.subvalence(input), 1)
    }
}
