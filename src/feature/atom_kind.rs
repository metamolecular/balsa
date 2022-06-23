use std::fmt;

use super::{Bracket, Selection, Shortcut, Stereodescriptor};

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
            Self::Star => write!(f, "*"),
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
            if bracket.virtual_hydrogen.is_some() {
                bracket.stereodescriptor = match bracket.stereodescriptor {
                    Some(Stereodescriptor::Left) => {
                        Some(Stereodescriptor::Right)
                    }
                    Some(Stereodescriptor::Right) => {
                        Some(Stereodescriptor::Left)
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
            stereodescriptor: Some(Stereodescriptor::Left),
            virtual_hydrogen: None,
            charge: None,
        });

        kind.invert_configuration();

        assert_eq!(
            kind,
            AtomKind::Bracket(Bracket {
                symbol: Symbol::Star,
                isotope: None,
                stereodescriptor: Some(Stereodescriptor::Left),
                virtual_hydrogen: None,
                charge: None,
            })
        )
    }

    #[test]
    fn bracket_with_descriptor_and_hydrogen() {
        let mut kind = AtomKind::Bracket(Bracket {
            symbol: Symbol::Star,
            isotope: None,
            stereodescriptor: Some(Stereodescriptor::Left),
            virtual_hydrogen: Some(VirtualHydrogen::H1),
            charge: None,
        });

        kind.invert_configuration();

        assert_eq!(
            kind,
            AtomKind::Bracket(Bracket {
                symbol: Symbol::Star,
                isotope: None,
                stereodescriptor: Some(Stereodescriptor::Right),
                virtual_hydrogen: Some(VirtualHydrogen::H1),
                charge: None,
            })
        )
    }
}
