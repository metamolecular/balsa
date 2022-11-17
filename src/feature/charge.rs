use core::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Charge {
    Minus9,
    Minus8,
    Minus7,
    Minus6,
    Minus5,
    Minus4,
    Minus3,
    Minus2,
    Minus1,
    Minus,
    Plus,
    Plus1,
    Plus2,
    Plus3,
    Plus4,
    Plus5,
    Plus6,
    Plus7,
    Plus8,
    Plus9,
}

impl Charge {
    pub fn new(value: i8) -> Option<Charge> {
        match value {
            -9 => Some(Charge::Minus9),
            -8 => Some(Charge::Minus8),
            -7 => Some(Charge::Minus7),
            -6 => Some(Charge::Minus6),
            -5 => Some(Charge::Minus5),
            -4 => Some(Charge::Minus4),
            -3 => Some(Charge::Minus3),
            -2 => Some(Charge::Minus2),
            -1 => Some(Charge::Minus1),
            1 => Some(Charge::Plus1),
            2 => Some(Charge::Plus2),
            3 => Some(Charge::Plus3),
            4 => Some(Charge::Plus4),
            5 => Some(Charge::Plus5),
            6 => Some(Charge::Plus6),
            7 => Some(Charge::Plus7),
            8 => Some(Charge::Plus8),
            9 => Some(Charge::Plus9),
            _ => None,
        }
    }
}

impl fmt::Display for Charge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Charge::Minus9 => "-9",
            Charge::Minus8 => "-8",
            Charge::Minus7 => "-7",
            Charge::Minus6 => "-6",
            Charge::Minus5 => "-5",
            Charge::Minus4 => "-4",
            Charge::Minus3 => "-3",
            Charge::Minus2 => "-2",
            Charge::Minus1 => "-1",
            Charge::Minus => "-",
            Charge::Plus => "+",
            Charge::Plus1 => "+1",
            Charge::Plus2 => "+2",
            Charge::Plus3 => "+3",
            Charge::Plus4 => "+4",
            Charge::Plus5 => "+5",
            Charge::Plus6 => "+6",
            Charge::Plus7 => "+7",
            Charge::Plus8 => "+8",
            Charge::Plus9 => "+9",
        })
    }
}
