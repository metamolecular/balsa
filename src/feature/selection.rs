use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Selection {
    B,
    C,
    N,
    O,
    P,
    S,
}

impl fmt::Display for Selection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Selection::B => "b",
                Selection::C => "c",
                Selection::N => "n",
                Selection::O => "o",
                Selection::P => "p",
                Selection::S => "s",
            }
        )
    }
}
