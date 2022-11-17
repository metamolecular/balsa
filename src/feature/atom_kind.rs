use std::{fmt, fmt::Write};

use super::{AtomParity, Bracket, Selection, Shortcut};

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
                        Some(AtomParity::Clocwise)
                    }
                    Some(AtomParity::Clocwise) => {
                        Some(AtomParity::Counterclockwise)
                    }
                    None => None,
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
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
                parity: Some(AtomParity::Clocwise),
                hydrogens: Some(VirtualHydrogen::H1),
                charge: None,
            })
        )
    }
}
