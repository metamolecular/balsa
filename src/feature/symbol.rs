use std::fmt;

use super::{Element, Selection};

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    Star,
    Element(Element),
    Selection(Selection),
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Star => write!(f, "*"),
            Self::Element(element) => element.fmt(f),
            Self::Selection(selection) => selection.fmt(f),
        }
    }
}

impl std::default::Default for Symbol {
    fn default() -> Self {
        Symbol::Star
    }
}
