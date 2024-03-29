use lyn::Scanner;

use super::{digit, element, missing_character, nonzero, selection, Error};
use crate::feature::{
    AtomParity, Bracket, Charge, Isotope, Symbol, VirtualHydrogen,
};

pub fn bracket(scanner: &mut Scanner) -> Result<Option<Bracket>, Error> {
    if !scanner.take(&'[') {
        return Ok(None);
    }

    let result = Ok(Some(Bracket {
        isotope: isotope(scanner),
        symbol: match symbol(scanner)? {
            Some(symbol) => symbol,
            None => return Err(missing_character(scanner)),
        },
        parity: atom_parity(scanner),
        hydrogens: virtual_hydrogen(scanner),
        charge: charge(scanner),
    }));

    if scanner.take(&']') {
        result
    } else {
        Err(missing_character(scanner))
    }
}

fn isotope(scanner: &mut Scanner) -> Option<Isotope> {
    let mut sum = match nonzero(scanner) {
        Some(digit) => digit as u16,
        None => return None,
    };

    for _ in 0..2 {
        sum = match digit(scanner) {
            Some(digit) => sum * 10 + digit as u16,
            None => return Some(Isotope::new(sum).expect("isotope")),
        };
    }

    Some(Isotope::new(sum).expect("isotope"))
}

fn symbol(scanner: &mut Scanner) -> Result<Option<Symbol>, Error> {
    if let Some(element) = element(scanner)? {
        Ok(Some(Symbol::Element(element)))
    } else if let Some(selection) = selection(scanner) {
        Ok(Some(Symbol::Selection(selection)))
    } else if star(scanner) {
        Ok(Some(Symbol::Star))
    } else {
        Ok(None)
    }
}

fn star(scanner: &mut Scanner) -> bool {
    scanner.take(&'*')
}

fn atom_parity(scanner: &mut Scanner) -> Option<AtomParity> {
    if scanner.take(&'@') {
        if scanner.take(&'@') {
            Some(AtomParity::Clockwise)
        } else {
            Some(AtomParity::Counterclockwise)
        }
    } else {
        None
    }
}

fn virtual_hydrogen(scanner: &mut Scanner) -> Option<VirtualHydrogen> {
    if scanner.take(&'H') {
        match nonzero(scanner) {
            Some(digit) => Some(VirtualHydrogen::new(digit).expect("digit")),
            _ => Some(VirtualHydrogen::default()),
        }
    } else {
        None
    }
}

fn charge(scanner: &mut Scanner) -> Option<Charge> {
    if scanner.take(&'+') {
        match nonzero(scanner) {
            Some(digit) => Some(Charge::new(digit as i8).expect("charge")),
            None => Some(Charge::Plus),
        }
    } else if scanner.take(&'-') {
        match nonzero(scanner) {
            Some(digit) => Some(Charge::new(digit as i8 * -1).expect("charge")),
            None => Some(Charge::Minus),
        }
    } else {
        None
    }
}

#[cfg(test)]
mod isotope {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn leading_zero() {
        let mut scanner = Scanner::new("007");

        assert_eq!(isotope(&mut scanner), None)
    }

    #[test]
    fn single_digit() {
        let mut scanner = Scanner::new("7");

        assert_eq!(isotope(&mut scanner), Some(Isotope::new(7).unwrap()))
    }

    #[test]
    fn double_digit() {
        let mut scanner = Scanner::new("42");

        assert_eq!(isotope(&mut scanner), Some(Isotope::new(42).unwrap()))
    }

    #[test]
    fn triple_digit() {
        let mut scanner = Scanner::new("999");

        assert_eq!(isotope(&mut scanner), Some(Isotope::new(999).unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use crate::feature::{Element, Selection};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn no_open() {
        let mut scanner = Scanner::new("X");

        assert_eq!(bracket(&mut scanner), Ok(None))
    }

    #[test]
    fn open_no_close() {
        let mut scanner = Scanner::new("[");

        assert_eq!(bracket(&mut scanner), Err(Error::EndOfLine))
    }

    #[test]
    fn open_invalid() {
        let mut scanner = Scanner::new("[0");

        assert_eq!(bracket(&mut scanner), Err(Error::Character(1)))
    }

    #[test]
    fn open_element_no_close() {
        let mut scanner = Scanner::new("[C");

        assert_eq!(bracket(&mut scanner), Err(Error::EndOfLine))
    }

    #[test]
    fn open_isotope_no_close() {
        let mut scanner = Scanner::new("[1");

        assert_eq!(bracket(&mut scanner), Err(Error::EndOfLine))
    }

    #[test]
    fn isotope_zero() {
        let mut scanner = Scanner::new("[0C]");

        assert_eq!(bracket(&mut scanner), Err(Error::Character(1)))
    }

    #[test]
    fn isotope_overflow() {
        let mut scanner = Scanner::new("[1234C]");

        assert_eq!(bracket(&mut scanner), Err(Error::Character(4)))
    }

    #[test]
    fn charge_overflow() {
        let mut scanner = Scanner::new("[C+10]");

        assert_eq!(bracket(&mut scanner), Err(Error::Character(4)))
    }

    #[test]
    fn element() {
        let mut scanner = Scanner::new("[C]");

        assert_eq!(
            bracket(&mut scanner),
            Ok(Some(Bracket {
                symbol: Symbol::Element(Element::C),
                ..Default::default()
            }))
        )
    }

    #[test]
    fn selected_element() {
        let mut scanner = Scanner::new("[c]");

        assert_eq!(
            bracket(&mut scanner),
            Ok(Some(Bracket {
                symbol: Symbol::Selection(Selection::C),
                ..Default::default()
            }))
        )
    }

    #[test]
    fn open_star_close() {
        let mut scanner = Scanner::new("[*]");

        assert_eq!(bracket(&mut scanner), Ok(Some(Bracket::default())))
    }

    #[test]
    fn element_with_isotope() {
        let mut scanner = Scanner::new("[1H]");

        assert_eq!(
            bracket(&mut scanner),
            Ok(Some(Bracket {
                symbol: Symbol::Element(Element::H),
                isotope: Some(Isotope::new(1).unwrap()),
                ..Default::default()
            }))
        )
    }

    #[test]
    fn element_with_stereodescriptor() {
        let mut scanner = Scanner::new("[C@]");

        assert_eq!(
            bracket(&mut scanner),
            Ok(Some(Bracket {
                symbol: Symbol::Element(Element::C),
                parity: Some(AtomParity::Counterclockwise),
                ..Default::default()
            }))
        )
    }

    #[test]
    fn element_with_virtual_hydrogen() {
        let mut scanner = Scanner::new("[CH1]");

        assert_eq!(
            bracket(&mut scanner),
            Ok(Some(Bracket {
                symbol: Symbol::Element(Element::C),
                hydrogens: Some(VirtualHydrogen::H1),
                ..Default::default()
            }))
        )
    }

    #[test]
    fn element_with_charge() {
        let mut scanner = Scanner::new("[C+1]");

        assert_eq!(
            bracket(&mut scanner),
            Ok(Some(Bracket {
                symbol: Symbol::Element(Element::C),
                charge: Some(Charge::Plus1),
                ..Default::default()
            }))
        )
    }

    #[test]
    fn kitchen_sink() {
        let mut scanner = Scanner::new("[12C@H1+2]");

        assert_eq!(
            bracket(&mut scanner),
            Ok(Some(Bracket {
                symbol: Symbol::Element(Element::C),
                isotope: Some(Isotope::new(12).unwrap()),
                parity: Some(AtomParity::Counterclockwise),
                hydrogens: Some(VirtualHydrogen::H1),
                charge: Some(Charge::Plus2),
            }))
        )
    }
}
