use lyn::{Action, Error, Scanner};

use crate::feature::Element;

pub fn element(scanner: &mut Scanner) -> Result<Option<Element>, Error> {
    Ok(scanner.scan(|symbol| match symbol {
        "A" => Some(Action::Require),
        "Ac" => Some(Action::Return(Element::Ac)),
        "Ag" => Some(Action::Return(Element::Ag)),
        "Al" => Some(Action::Return(Element::Al)),
        "Am" => Some(Action::Return(Element::Am)),
        "Ar" => Some(Action::Return(Element::Ar)),
        "As" => Some(Action::Return(Element::As)),
        "At" => Some(Action::Return(Element::At)),
        "Au" => Some(Action::Return(Element::Au)),
        "B" => Some(Action::Request(Element::B)),
        "Ba" => Some(Action::Return(Element::Ba)),
        "Be" => Some(Action::Return(Element::Be)),
        "Bi" => Some(Action::Return(Element::Bi)),
        "Bk" => Some(Action::Return(Element::Bk)),
        "Br" => Some(Action::Return(Element::Br)),
        "C" => Some(Action::Request(Element::C)),
        "Ca" => Some(Action::Return(Element::Ca)),
        "Cd" => Some(Action::Return(Element::Cd)),
        "Ce" => Some(Action::Return(Element::Ce)),
        "Cf" => Some(Action::Return(Element::Cf)),
        "Cl" => Some(Action::Return(Element::Cl)),
        "Cm" => Some(Action::Return(Element::Cm)),
        "Co" => Some(Action::Return(Element::Co)),
        "Cr" => Some(Action::Return(Element::Cr)),
        "Cs" => Some(Action::Return(Element::Cs)),
        "Cu" => Some(Action::Return(Element::Cu)),
        "D" => Some(Action::Require),
        "Dy" => Some(Action::Return(Element::Dy)),
        "E" => Some(Action::Require),
        "Er" => Some(Action::Return(Element::Er)),
        "Es" => Some(Action::Return(Element::Es)),
        "Eu" => Some(Action::Return(Element::Eu)),
        "F" => Some(Action::Request(Element::F)),
        "Fe" => Some(Action::Return(Element::Fe)),
        "Fm" => Some(Action::Return(Element::Fm)),
        "Fr" => Some(Action::Return(Element::Fr)),
        "G" => Some(Action::Require),
        "Ga" => Some(Action::Return(Element::Ga)),
        "Gd" => Some(Action::Return(Element::Gd)),
        "Ge" => Some(Action::Return(Element::Ge)),
        "H" => Some(Action::Request(Element::H)),
        "He" => Some(Action::Return(Element::He)),
        "Hf" => Some(Action::Return(Element::Hf)),
        "Hg" => Some(Action::Return(Element::Hg)),
        "Ho" => Some(Action::Return(Element::Ho)),
        "I" => Some(Action::Request(Element::I)),
        "In" => Some(Action::Return(Element::In)),
        "Ir" => Some(Action::Return(Element::Ir)),
        "K" => Some(Action::Request(Element::K)),
        "Kr" => Some(Action::Return(Element::Kr)),
        "L" => Some(Action::Require),
        "La" => Some(Action::Return(Element::La)),
        "Li" => Some(Action::Return(Element::Li)),
        "Lr" => Some(Action::Return(Element::Lr)),
        "Lu" => Some(Action::Return(Element::Lu)),
        "M" => Some(Action::Require),
        "Mg" => Some(Action::Return(Element::Mg)),
        "Mn" => Some(Action::Return(Element::Mn)),
        "Mo" => Some(Action::Return(Element::Mo)),
        "N" => Some(Action::Request(Element::N)),
        "Na" => Some(Action::Return(Element::Na)),
        "Nb" => Some(Action::Return(Element::Nb)),
        "Nd" => Some(Action::Return(Element::Nd)),
        "Ne" => Some(Action::Return(Element::Ne)),
        "Ni" => Some(Action::Return(Element::Ni)),
        "No" => Some(Action::Return(Element::No)),
        "Np" => Some(Action::Return(Element::Np)),
        "O" => Some(Action::Request(Element::O)),
        "Os" => Some(Action::Return(Element::Os)),
        "P" => Some(Action::Request(Element::P)),
        "Pa" => Some(Action::Return(Element::Pa)),
        "Pb" => Some(Action::Return(Element::Pb)),
        "Pd" => Some(Action::Return(Element::Pd)),
        "Pm" => Some(Action::Return(Element::Pm)),
        "Po" => Some(Action::Return(Element::Po)),
        "Pr" => Some(Action::Return(Element::Pr)),
        "Pt" => Some(Action::Return(Element::Pt)),
        "Pu" => Some(Action::Return(Element::Pu)),
        "R" => Some(Action::Require),
        "Ra" => Some(Action::Return(Element::Ra)),
        "Rb" => Some(Action::Return(Element::Rb)),
        "Re" => Some(Action::Return(Element::Re)),
        "Rf" => Some(Action::Return(Element::Rf)),
        "Rh" => Some(Action::Return(Element::Rh)),
        "Rn" => Some(Action::Return(Element::Rn)),
        "Ru" => Some(Action::Return(Element::Ru)),
        "S" => Some(Action::Request(Element::S)),
        "Sb" => Some(Action::Return(Element::Sb)),
        "Sc" => Some(Action::Return(Element::Sc)),
        "Se" => Some(Action::Return(Element::Se)),
        "Si" => Some(Action::Return(Element::Si)),
        "Sm" => Some(Action::Return(Element::Sm)),
        "Sn" => Some(Action::Return(Element::Sn)),
        "Sr" => Some(Action::Return(Element::Sr)),
        "T" => Some(Action::Require),
        "Ta" => Some(Action::Return(Element::Ta)),
        "Tb" => Some(Action::Return(Element::Tb)),
        "Tc" => Some(Action::Return(Element::Tc)),
        "Te" => Some(Action::Return(Element::Te)),
        "Th" => Some(Action::Return(Element::Th)),
        "Ti" => Some(Action::Return(Element::Ti)),
        "Tl" => Some(Action::Return(Element::Tl)),
        "Tm" => Some(Action::Return(Element::Tm)),
        "U" => Some(Action::Return(Element::U)),
        "V" => Some(Action::Return(Element::V)),
        "W" => Some(Action::Return(Element::W)),
        "X" => Some(Action::Require),
        "Xe" => Some(Action::Return(Element::Xe)),
        "Y" => Some(Action::Require),
        "Yb" => Some(Action::Return(Element::Yb)),
        "Z" => Some(Action::Require),
        "Zn" => Some(Action::Return(Element::Zn)),
        "Zr" => Some(Action::Return(Element::Zr)),
        _ => None,
    })?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn not_found() {
        let mut scanner = Scanner::new("x");

        assert_eq!(element(&mut scanner), Ok(None))
    }

    #[test]
    fn cap_found_lower_not_found_error() {
        let mut scanner = Scanner::new("Ax");

        assert_eq!(element(&mut scanner), Err(Error::Character(1)))
    }

    #[test]
    fn cap_found_lower_not_found_ok() {
        let mut scanner = Scanner::new("Bx");

        assert_eq!(element(&mut scanner), Ok(Some(Element::B)))
    }

    #[test]
    fn cap_found_lower_not_found() {
        let mut scanner = Scanner::new("Br");

        assert_eq!(element(&mut scanner), Ok(Some(Element::Br)))
    }
}
