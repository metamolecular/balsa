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
        let virtual_hydrogens = self.kind.virtual_hydrogens();
        let child_bond_order_sum =
            self.edges.iter().fold(0, |r, e| r + e.bond_order());

        input_bond_order + virtual_hydrogens + child_bond_order_sum
    }

    pub fn subvalence(&self, input: Option<&BondKind>) -> u8 {
        self.kind.subvalence(self.valence(input))
    }

    pub fn implicit_hydrogens(&self, input: Option<&BondKind>) -> u8 {
        match &self.kind {
            AtomKind::Star => 0,
            AtomKind::Shortcut(_) => self.subvalence(input),
            AtomKind::Selection(_) => match self.subvalence(input) {
                0 => 0,
                subvalence => subvalence - 1,
            },
            AtomKind::Bracket(_) => 0,
        }
    }

    pub fn hydrogens(&self, input: Option<&BondKind>) -> u8 {
        match &self.kind {
            AtomKind::Star => 0,
            AtomKind::Shortcut(_) => self.implicit_hydrogens(input),
            AtomKind::Selection(_) => self.implicit_hydrogens(input),
            AtomKind::Bracket(bracket) => bracket.hydrogens(),
        }
    }
}

#[cfg(test)]
mod valence {
    use super::*;
    use crate::feature::VirtualHydrogen;
    use pretty_assertions::assert_eq;

    #[test]
    fn zerovalent() {
        let atom = Atom::star(vec![]);
        let input = None;

        assert_eq!(atom.valence(input), 0)
    }

    #[test]
    fn hydrogens_none_input_double() {
        let atom = Atom::star(vec![]);
        let input = Some(BondKind::Double);

        assert_eq!(atom.valence(input.as_ref()), 2)
    }

    #[test]
    fn hydrogens_one_input_single() {
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
    fn hydrogens_one_input_single_children_gap() {
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
    fn hydrogens_one_input_signle_children_single() {
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
    fn star_zerovalent() {
        let atom = Atom::star(vec![]);
        let input = Some(&BondKind::Single);

        assert_eq!(atom.subvalence(input), 0)
    }

    #[test]
    fn shortcut_subvalent() {
        let atom = Atom::shortcut(Shortcut::C, vec![]);
        let input = Some(&BondKind::Single);

        assert_eq!(atom.subvalence(input), 3)
    }

    #[test]
    fn selection_subvalent() {
        let atom = Atom::selection(Selection::C, vec![]);
        let input = Some(&BondKind::Single);

        assert_eq!(atom.subvalence(input), 3)
    }

    #[test]
    fn bracket_subvalent() {
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

#[cfg(test)]
mod implicit_hydrogens {
    use crate::feature;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn star_zerovalent() {
        let atom = Atom::star(vec![]);
        let input = None;

        assert_eq!(atom.implicit_hydrogens(input), 0)
    }

    #[test]
    fn shortcut_carbon_input_elided_children_none() {
        let atom = Atom::shortcut(feature::Shortcut::C, vec![]);
        let input = Some(&feature::BondKind::Elided);

        assert_eq!(atom.implicit_hydrogens(input), 3)
    }

    #[test]
    fn selection_carbon_input_elided_children_none() {
        let atom = Atom::selection(feature::Selection::C, vec![]);
        let input = Some(&BondKind::Elided);

        assert_eq!(atom.implicit_hydrogens(input), 2)
    }

    #[test]
    fn bracket_carbon_input_elided_children_none() {
        let atom = Atom::bracket(
            feature::Bracket {
                symbol: feature::Symbol::Element(feature::Element::C),
                ..Default::default()
            },
            vec![],
        );
        let input = Some(&BondKind::Elided);

        assert_eq!(atom.implicit_hydrogens(input), 0)
    }
}

#[cfg(test)]
mod hydrogens {
    use pretty_assertions::assert_eq;
    use crate::feature;
    use super::*;

    #[test]
    fn star() {
        let atom = Atom::star(vec![]);
        let input = None;

        assert_eq!(atom.hydrogens(input), 0)
    }

    #[test]
    fn shortcut() {
        let atom = Atom::shortcut(feature::Shortcut::C, vec![]);
        let input = None;

        assert_eq!(atom.hydrogens(input), 4)
    }

    #[test]
    fn selection() {
        let atom = Atom::selection(feature::Selection::C, vec![]);
        let input = None;

        assert_eq!(atom.hydrogens(input), 3)
    }

    #[test]
    fn bracket_hydrogens_some() {
        let atom = Atom::bracket(feature::Bracket {
            hydrogens: Some(feature::VirtualHydrogen::H2),
            ..Default::default()
        }, vec![]);
        let input = None;

        assert_eq!(atom.hydrogens(input), 2)
    }
}
