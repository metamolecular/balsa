use crate::feature::BondKind;

use super::Target;

#[derive(Debug, PartialEq, Clone)]
pub struct Bond {
    pub kind: BondKind,
    pub target: Target,
}
