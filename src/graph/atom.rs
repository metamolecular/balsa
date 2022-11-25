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
