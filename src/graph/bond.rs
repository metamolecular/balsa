use crate::feature::BondKind;

#[derive(Debug, PartialEq, Clone)]
pub struct Bond {
    pub kind: BondKind,
    pub tid: usize,
}

impl Bond {
    pub fn elided(tid: usize) -> Self {
        Self {
            kind: BondKind::Elided,
            tid,
        }
    }

    pub fn triple(tid: usize) -> Self {
        Self {
            kind: BondKind::Triple,
            tid
        }
    }
}
