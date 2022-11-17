use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum AtomParity {
    Counterclockwise,
    Clocwise,
}

impl fmt::Display for AtomParity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Counterclockwise => "@",
                Self::Clocwise => "@@",
            }
        )
    }
}
