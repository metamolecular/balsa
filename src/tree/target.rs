use crate::feature::Bridge;

use super::Atom;

#[derive(Debug, PartialEq, Clone)]
pub enum Target {
    Atom(Atom),
    Bridge(Bridge),
}
