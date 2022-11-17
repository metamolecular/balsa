use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Stereodescriptor {
    Counterclockwise,
    Clocwise,
}

impl fmt::Display for Stereodescriptor {
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
