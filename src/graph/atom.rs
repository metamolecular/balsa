use crate::feature::{AtomKind, Bracket, Selection, Shortcut};

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

    pub fn shortcut(shortcut: Shortcut, bonds: Vec<Bond>) -> Self {
        Self {
            kind: AtomKind::Shortcut(shortcut),
            bonds,
        }
    }

    pub fn selection(selection: Selection, bonds: Vec<Bond>) -> Self {
        Self {
            kind: AtomKind::Selection(selection),
            bonds,
        }
    }

    pub fn bracket(bracket: Bracket, bonds: Vec<Bond>) -> Self {
        Self {
            kind: AtomKind::Bracket(bracket),
            bonds,
        }
    }

    pub fn valence(&self) -> u8 {
        let virtual_hydrogens = self.kind.virtual_hydrogens();
        let bond_order_sum =
            self.bonds.iter().fold(0, |s, b| s + b.kind.bond_order());

        virtual_hydrogens + bond_order_sum
    }

    pub fn subvalence(&self) -> u8 {
        self.kind.subvalence(self.valence())
    }

    pub fn implicit_hydrogens(&self) -> u8 {
        match &self.kind {
            AtomKind::Star => 0,
            AtomKind::Shortcut(_) => self.subvalence(),
            AtomKind::Selection(_) => match self.subvalence() {
                0 => 0,
                subvalence => subvalence - 1,
            },
            AtomKind::Bracket(_) => 0,
        }
    }

    pub fn hydrogens(&self) -> u8 {
        match &self.kind {
            AtomKind::Star => 0,
            AtomKind::Shortcut(_) => self.implicit_hydrogens(),
            AtomKind::Selection(_) => self.implicit_hydrogens(),
            AtomKind::Bracket(bracket) => bracket.hydrogens(),
        }
    }
}

#[cfg(test)]
mod valence {
    use crate::feature::{Element, Symbol, VirtualHydrogen};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn zerovalent() {
        let atom = Atom::star(vec![]);

        assert_eq!(atom.valence(), 0)
    }

    #[test]
    fn subvalent_hydrogens_some_bonds_none() {
        let atom = Atom::bracket(
            Bracket {
                symbol: Symbol::Element(Element::C),
                hydrogens: Some(VirtualHydrogen::H),
                ..Default::default()
            },
            vec![],
        );

        assert_eq!(atom.valence(), 1)
    }

    #[test]
    fn subvalent_hydrogens_some_bonds_one() {
        let atom = Atom::bracket(
            Bracket {
                symbol: Symbol::Element(Element::C),
                hydrogens: Some(VirtualHydrogen::H),
                ..Default::default()
            },
            vec![Bond::double(1)],
        );

        assert_eq!(atom.valence(), 3)
    }
}

#[cfg(test)]
mod subvalence {
    use crate::feature::{Element, Selection, Symbol, VirtualHydrogen};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn star_subvalent() {
        let atom = Atom::star(vec![]);

        assert_eq!(atom.subvalence(), 0)
    }

    #[test]
    fn shortcut_subvalent() {
        let atom = Atom::shortcut(Shortcut::C, vec![]);

        assert_eq!(atom.subvalence(), 4)
    }

    #[test]
    fn selection_subvalent() {
        let atom = Atom::selection(Selection::C, vec![]);

        assert_eq!(atom.subvalence(), 4)
    }

    #[test]
    fn bracket_subvalent() {
        let atom = Atom::bracket(
            Bracket {
                symbol: Symbol::Element(Element::C),
                hydrogens: Some(VirtualHydrogen::H1),
                ..Default::default()
            },
            vec![Bond::single(1), Bond::single(1)],
        );

        assert_eq!(atom.subvalence(), 1)
    }

    #[test]
    fn bracket_homovalent() {
        let atom = Atom::bracket(
            Bracket {
                hydrogens: Some(VirtualHydrogen::H1),
                ..Default::default()
            },
            vec![Bond::single(1), Bond::double(2)],
        );

        assert_eq!(atom.subvalence(), 0)
    }

    #[test]
    fn bracket_supervalent() {
        let atom = Atom::bracket(
            Bracket {
                symbol: Symbol::Element(Element::C),
                hydrogens: Some(VirtualHydrogen::H1),
                ..Default::default()
            },
            vec![Bond::double(1), Bond::double(2)],
        );

        assert_eq!(atom.subvalence(), 0)
    }
}

#[cfg(test)]
mod implicit_hydrogens {
    use crate::feature;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn star_zerovalent() {
        let atom = Atom::star(vec![]);

        assert_eq!(atom.implicit_hydrogens(), 0)
    }

    #[test]
    fn shortcut_carbon_neighbors_elided() {
        let atom = Atom::shortcut(feature::Shortcut::C, vec![Bond::elided(1)]);

        assert_eq!(atom.implicit_hydrogens(), 3)
    }

    #[test]
    fn selection_carbon_neighbors_elided() {
        let atom =
            Atom::selection(feature::Selection::C, vec![Bond::elided(1)]);

        assert_eq!(atom.implicit_hydrogens(), 2)
    }

    #[test]
    fn bracket_carbon_neighbors_elided() {
        let atom = Atom::bracket(
            feature::Bracket {
                symbol: feature::Symbol::Element(feature::Element::C),
                ..Default::default()
            },
            vec![Bond::elided(1)],
        );

        assert_eq!(atom.implicit_hydrogens(), 0)
    }
}

#[cfg(test)]
mod hydrogens {
    use crate::feature;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn star_shortcut() {
        let atom = Atom::star(vec![]);

        assert_eq!(atom.hydrogens(), 0)
    }

    #[test]
    fn shortcut_carbon() {
        let atom = Atom::shortcut(feature::Shortcut::C, vec![]);

        assert_eq!(atom.hydrogens(), 4)
    }

    #[test]
    fn selection_carbon() {
        let atom = Atom::selection(feature::Selection::C, vec![]);

        assert_eq!(atom.hydrogens(), 3)
    }

    #[test]
    fn bracket_carbon_hydrogens_some() {
        let atom = Atom::bracket(
            feature::Bracket {
                symbol: feature::Symbol::Element(feature::Element::C),
                hydrogens: Some(feature::VirtualHydrogen::H),
                ..Default::default()
            },
            vec![],
        );

        assert_eq!(atom.hydrogens(), 1)
    }
}
