use std::{default, fmt};

#[derive(Debug, PartialEq, Clone)]
pub enum VirtualHydrogen {
    H,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    H7,
    H8,
    H9,
}

impl VirtualHydrogen {
    pub fn new(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::H1),
            2 => Some(Self::H2),
            3 => Some(Self::H3),
            4 => Some(Self::H4),
            5 => Some(Self::H5),
            6 => Some(Self::H6),
            7 => Some(Self::H7),
            8 => Some(Self::H8),
            9 => Some(Self::H9),
            _ => None,
        }
    }
}

impl default::Default for VirtualHydrogen {
    fn default() -> Self {
        VirtualHydrogen::H1
    }
}

impl From<&VirtualHydrogen> for u8 {
    fn from(value: &VirtualHydrogen) -> Self {
        match value {
            VirtualHydrogen::H | VirtualHydrogen::H1 => 1,
            VirtualHydrogen::H2 => 2,
            VirtualHydrogen::H3 => 3,
            VirtualHydrogen::H4 => 4,
            VirtualHydrogen::H5 => 5,
            VirtualHydrogen::H6 => 6,
            VirtualHydrogen::H7 => 7,
            VirtualHydrogen::H8 => 8,
            VirtualHydrogen::H9 => 9,
        }
    }
}

impl fmt::Display for VirtualHydrogen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::H => "H",
            Self::H1 => "H1",
            Self::H2 => "H2",
            Self::H3 => "H3",
            Self::H4 => "H4",
            Self::H5 => "H5",
            Self::H6 => "H6",
            Self::H7 => "H7",
            Self::H8 => "H8",
            Self::H9 => "H9",
        })
    }
}
