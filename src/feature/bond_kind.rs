use std::fmt;

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
            Self::Elided => write!(f, ""),
            Self::Single => write!(f, "-"),
            Self::Double => write!(f, "="),
            Self::Triple => write!(f, "#"),
            Self::Up => write!(f, "/"),
            Self::Down => write!(f, "\\"),
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
mod tests {
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
