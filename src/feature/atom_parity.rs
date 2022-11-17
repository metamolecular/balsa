use std::{fmt, fmt::Write};

#[derive(Debug, PartialEq, Clone)]
pub enum AtomParity {
    Counterclockwise,
    Clocwise,
}

impl fmt::Display for AtomParity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Counterclockwise => f.write_char('@'),
            Self::Clocwise => f.write_str("@@"),
        }
    }
}
