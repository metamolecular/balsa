use std::{fmt, fmt::Write};

#[derive(Debug, PartialEq, Clone)]
pub enum AtomParity {
    Counterclockwise,
    Clockwise,
}

impl fmt::Display for AtomParity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Counterclockwise => f.write_char('@'),
            Self::Clockwise => f.write_str("@@"),
        }
    }
}
