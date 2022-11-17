use std::fmt;

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
        write!(
            f,
            "[{}{}{}{}{}]",
            option_to_string(&self.isotope),
            self.symbol.to_string(),
            option_to_string(&self.parity),
            option_to_string(&self.hydrogens),
            option_to_string(&self.charge),
        )
    }
}

fn option_to_string<T: fmt::Display>(option: &Option<T>) -> String {
    match option {
        Some(option) => option.to_string(),
        None => "".to_string(),
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
