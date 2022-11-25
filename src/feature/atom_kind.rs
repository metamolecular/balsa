use std::{fmt, fmt::Write};

use super::{AtomParity, Bracket, Element, Selection, Shortcut, Symbol};

#[derive(Debug, PartialEq, Clone)]
pub enum AtomKind {
    Star,
    Shortcut(Shortcut),
    Selection(Selection),
    Bracket(Bracket),
}

impl Default for AtomKind {
    fn default() -> Self {
        Self::Star
    }
}

impl fmt::Display for AtomKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Star => f.write_char('*'),
            Self::Shortcut(shortcut) => shortcut.fmt(f),
            Self::Selection(selection) => selection.fmt(f),
            Self::Bracket(bracket) => bracket.fmt(f),
        }
    }
}

impl AtomKind {
    /// Inverts configuration for Bracket variant given one or more virtual
    /// hydrogens.
    pub fn invert_configuration(&mut self) {
        if let AtomKind::Bracket(bracket) = self {
            if bracket.hydrogens.is_some() {
                bracket.parity = match bracket.parity {
                    Some(AtomParity::Counterclockwise) => {
                        Some(AtomParity::Clockwise)
                    }
                    Some(AtomParity::Clockwise) => {
                        Some(AtomParity::Counterclockwise)
                    }
                    None => None,
                };
            }
        }
    }

    /// Returns subvalence, as defined in the working paper.
    pub fn subvalence(&self, valence: u8) -> u8 {
        let element: Element = match self {
            Self::Star => return 0,
            Self::Shortcut(shortcut) => shortcut.into(),
            Self::Selection(selection) => selection.into(),
            Self::Bracket(bracket) => match &bracket.symbol {
                Symbol::Star => return 0,
                Symbol::Element(element) => element.clone(),
                Symbol::Selection(selection) => selection.into(),
            },
        };

        for default_valence in element.default_valences() {
            if default_valence >= &valence {
                return default_valence - valence;
            }
        }

        0
    }

    pub fn virtual_hydrogens(&self) -> u8 {
        match self {
            AtomKind::Star | AtomKind::Shortcut(_) | AtomKind::Selection(_) => {
                0
            }
            AtomKind::Bracket(bracket) => match &bracket.hydrogens {
                Some(hydrogens) => hydrogens.into(),
                None => 0,
            },
        }
    }
}

#[cfg(test)]
mod invert_configuration {
    use crate::feature::{Symbol, VirtualHydrogen};

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn star() {
        let mut kind = AtomKind::Star;

        kind.invert_configuration();

        assert_eq!(kind, AtomKind::Star)
    }

    #[test]
    fn bracket_with_descriptor_without_hydrogen() {
        let mut kind = AtomKind::Bracket(Bracket {
            symbol: Symbol::Star,
            isotope: None,
            parity: Some(AtomParity::Counterclockwise),
            hydrogens: None,
            charge: None,
        });

        kind.invert_configuration();

        assert_eq!(
            kind,
            AtomKind::Bracket(Bracket {
                symbol: Symbol::Star,
                isotope: None,
                parity: Some(AtomParity::Counterclockwise),
                hydrogens: None,
                charge: None,
            })
        )
    }

    #[test]
    fn bracket_with_descriptor_and_hydrogen() {
        let mut kind = AtomKind::Bracket(Bracket {
            symbol: Symbol::Star,
            isotope: None,
            parity: Some(AtomParity::Counterclockwise),
            hydrogens: Some(VirtualHydrogen::H1),
            charge: None,
        });

        kind.invert_configuration();

        assert_eq!(
            kind,
            AtomKind::Bracket(Bracket {
                symbol: Symbol::Star,
                isotope: None,
                parity: Some(AtomParity::Clockwise),
                hydrogens: Some(VirtualHydrogen::H1),
                charge: None,
            })
        )
    }
}

#[cfg(test)]
mod self_subvalence {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn star() {
        let kind = AtomKind::Star;

        assert_eq!(kind.subvalence(1), 0)
    }

    #[test]
    fn shortcut_subvalent() {
        let kind = AtomKind::Shortcut(Shortcut::C);

        assert_eq!(kind.subvalence(0), 4)
    }

    #[test]
    fn selection_subvalent() {
        let kind = AtomKind::Selection(Selection::C);

        assert_eq!(kind.subvalence(0), 4)
    }

    #[test]
    fn bracket_star() {
        let kind = AtomKind::Bracket(Bracket {
            symbol: Symbol::Star,
            ..Default::default()
        });

        assert_eq!(kind.subvalence(0), 0)
    }

    #[test]
    fn bracket_element() {
        let kind = AtomKind::Bracket(Bracket {
            symbol: Symbol::Element(Element::C),
            ..Default::default()
        });

        assert_eq!(kind.subvalence(0), 4)
    }

    #[test]
    fn bracket_selection() {
        let kind = AtomKind::Bracket(Bracket {
            symbol: Symbol::Selection(Selection::C),
            ..Default::default()
        });

        assert_eq!(kind.subvalence(0), 4)
    }

    #[test]
    fn defaults_one_subvalent() {
        let kind = AtomKind::Selection(Selection::C);

        assert_eq!(kind.subvalence(0), 4)
    }

    #[test]
    fn defaults_one_valent() {
        let kind = AtomKind::Selection(Selection::C);

        assert_eq!(kind.subvalence(4), 0)
    }

    #[test]
    fn defaults_one_supervalent() {
        let kind = AtomKind::Selection(Selection::C);

        assert_eq!(kind.subvalence(5), 0)
    }

    #[test]
    fn defaults_two_subvalent_first() {
        let kind = AtomKind::Selection(Selection::N);

        assert_eq!(kind.subvalence(2), 1)
    }

    #[test]
    fn defaults_two_subvalent_second() {
        let kind = AtomKind::Selection(Selection::N);

        assert_eq!(kind.subvalence(4), 1)
    }
}
