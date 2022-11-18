use std::{fmt, fmt::Write};

#[derive(Debug, PartialEq, Clone)]
pub enum BondKind {
    Elided,
    Single,
    Double,
    Triple,
    Up,
    Down,
}

impl fmt::Display for BondKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Elided => Ok(()),
            Self::Single => f.write_char('-'),
            Self::Double => f.write_char('='),
            Self::Triple => f.write_char('#'),
            Self::Up => f.write_char('/'),
            Self::Down => f.write_char('\\'),
        }
    }
}

impl BondKind {
    pub fn reverse(&self) -> Self {
        match self {
            BondKind::Elided => Self::Elided,
            BondKind::Single => Self::Single,
            BondKind::Double => Self::Double,
            BondKind::Triple => Self::Triple,
            BondKind::Up => Self::Down,
            BondKind::Down => Self::Up,
        }
    }
}

#[cfg(test)]
mod reverse {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn single() {
        let kind = BondKind::Single;

        assert_eq!(kind.reverse(), BondKind::Single)
    }

    #[test]
    fn up() {
        let kind = BondKind::Up;

        assert_eq!(kind.reverse(), BondKind::Down)
    }

    #[test]
    fn down() {
        let kind = BondKind::Down;

        assert_eq!(kind.reverse(), BondKind::Up)
    }
}
