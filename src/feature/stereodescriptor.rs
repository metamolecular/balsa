use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Stereodescriptor {
    Left,
    Right,
}

impl fmt::Display for Stereodescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Left => "@",
                Self::Right => "@@",
            }
        )
    }
}
