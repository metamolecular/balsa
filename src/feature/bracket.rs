use std::{fmt, fmt::Write};

use super::{AtomParity, Charge, Isotope, Symbol, VirtualHydrogen};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Bracket {
    pub symbol: Symbol,
    pub isotope: Option<Isotope>,
    pub parity: Option<AtomParity>,
    pub hydrogens: Option<VirtualHydrogen>,
    pub charge: Option<Charge>,
}

impl fmt::Display for Bracket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('[')?;

        if let Some(isotope) = &self.isotope {
            isotope.fmt(f)?
        }

        self.symbol.fmt(f)?;

        if let Some(parity) = &self.parity {
            parity.fmt(f)?
        }

        if let Some(hydrogens) = &self.hydrogens {
            hydrogens.fmt(f)?
        }

        if let Some(charge) = &self.charge {
            charge.fmt(f)?
        }

        f.write_char(']')
    }
}

#[cfg(test)]
mod to_string {
    use crate::feature::Element;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn kitchen_sink() {
        let bracket = Bracket {
            symbol: Symbol::Element(Element::C),
            isotope: Some(Isotope::new(13).unwrap()),
            parity: Some(AtomParity::Counterclockwise),
            hydrogens: Some(VirtualHydrogen::H),
            charge: Some(Charge::Plus),
        };

        assert_eq!(bracket.to_string(), "[13C@H+]")
    }
}
