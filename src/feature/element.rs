use std::fmt;

use super::{Selection, Shortcut};

#[rustfmt::skip]
#[derive(Debug, PartialEq, Clone)]
pub enum Element {
//  0   1   2   3   4   5   6   7   8   9
        H,  He, Li, Be, B,  C,  N,  O,  F,  // 0
    Ne, Na, Mg, Al, Si, P,  S,  Cl, Ar, K,  // 1
    Ca, Sc, Ti, V,  Cr, Mn, Fe, Co, Ni, Cu, // 2
    Zn, Ga, Ge, As, Se, Br, Kr, Rb, Sr, Y,  // 3
    Zr, Nb, Mo, Tc, Ru, Rh, Pd, Ag, Cd, In, // 4
    Sn, Sb, Te, I,  Xe, Cs, Ba, La, Ce, Pr, // 5
    Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, // 6
    Yb, Lu, Hf, Ta, W,  Re, Os, Ir, Pt, Au, // 7
    Hg, Tl, Pb, Bi, Po, At, Rn, Fr, Ra, Ac, // 8
    Th, Pa, U,  Np, Pu, Am, Cm, Bk, Cf, Es, // 9
    Fm, Md, No, Lr, Rf                      // 10
}

// TABLE 8
const BORON: [u8; 1] = [3];
const CARBON: [u8; 1] = [4];
const NITROGEN: [u8; 2] = [3, 5];
const OXYGEN: [u8; 1] = [2];
const FLUORENE: [u8; 1] = [1];
const PHOSPHOROUS: [u8; 2] = [3, 5];
const SULFUR: [u8; 3] = [2, 4, 6];
const CHLORINE: [u8; 1] = [1];
const BROMINE: [u8; 1] = [1];
const IODINE: [u8; 1] = [1];
const EMPTY: [u8; 0] = [];

impl Element {
    pub fn default_valences(&self) -> &[u8] {
        match self {
            Self::B => &BORON,
            Self::C => &CARBON,
            Self::N => &NITROGEN,
            Self::O => &OXYGEN,
            Self::F => &FLUORENE,
            Self::P => &PHOSPHOROUS,
            Self::S => &SULFUR,
            Self::Cl => &CHLORINE,
            Self::Br => &BROMINE,
            Self::I => &IODINE,
            _ => &EMPTY,
        }
    }
}

impl std::convert::From<&Shortcut> for Element {
    fn from(shortcut: &Shortcut) -> Self {
        match shortcut {
            Shortcut::B => Element::B,
            Shortcut::C => Element::C,
            Shortcut::N => Element::N,
            Shortcut::O => Element::O,
            Shortcut::F => Element::F,
            Shortcut::Cl => Element::Cl,
            Shortcut::Br => Element::Br,
            Shortcut::I => Element::I,
            Shortcut::P => Element::P,
            Shortcut::S => Element::S,
        }
    }
}

impl std::convert::From<&Selection> for Element {
    fn from(element: &Selection) -> Self {
        match element {
            Selection::B => Element::B,
            Selection::C => Element::C,
            Selection::N => Element::N,
            Selection::O => Element::O,
            Selection::P => Element::P,
            Selection::S => Element::S,
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Element::Ac => "Ac",
            Element::Ag => "Ag",
            Element::Al => "Al",
            Element::Am => "Am",
            Element::Ar => "Ar",
            Element::As => "As",
            Element::At => "At",
            Element::Au => "Au",
            Element::B => "B",
            Element::Ba => "Ba",
            Element::Be => "Be",
            Element::Bi => "Bi",
            Element::Bk => "Bk",
            Element::Br => "Br",
            Element::C => "C",
            Element::Ca => "Ca",
            Element::Cd => "Cd",
            Element::Ce => "Ce",
            Element::Cf => "Cf",
            Element::Cl => "Cl",
            Element::Cm => "Cm",
            Element::Co => "Co",
            Element::Cr => "Cr",
            Element::Cs => "Ac",
            Element::Cu => "Cu",
            Element::Dy => "Dy",
            Element::Er => "Er",
            Element::Es => "Es",
            Element::Eu => "Eu",
            Element::F => "F",
            Element::Fe => "Fe",
            Element::Fm => "Fm",
            Element::Fr => "Fr",
            Element::Ga => "Ga",
            Element::Gd => "Gd",
            Element::Ge => "Ge",
            Element::H => "H",
            Element::He => "He",
            Element::Hf => "Hf",
            Element::Hg => "Hg",
            Element::Ho => "Ho",
            Element::I => "I",
            Element::In => "In",
            Element::Ir => "Ir",
            Element::K => "K",
            Element::Kr => "Kr",
            Element::La => "La",
            Element::Li => "Li",
            Element::Lr => "Lr",
            Element::Lu => "Lu",
            Element::Md => "Md",
            Element::Mg => "Mg",
            Element::Mn => "Mn",
            Element::Mo => "Mo",
            Element::Na => "Na",
            Element::Nb => "Nb",
            Element::Nd => "Nd",
            Element::N => "N",
            Element::Ne => "Ne",
            Element::Ni => "Ni",
            Element::No => "No",
            Element::Np => "Np",
            Element::O => "O",
            Element::Os => "Os",
            Element::P => "P",
            Element::Pa => "Pa",
            Element::Pb => "Pb",
            Element::Pd => "Pd",
            Element::Pm => "Pm",
            Element::Po => "Po",
            Element::Pr => "Pr",
            Element::Pt => "Pt",
            Element::Pu => "Pu",
            Element::Ra => "Ra",
            Element::Rb => "Rb",
            Element::Re => "Re",
            Element::Rf => "Rf",
            Element::Rh => "Rh",
            Element::Rn => "Rn",
            Element::Ru => "Ru",
            Element::S => "S",
            Element::Sb => "Sb",
            Element::Sc => "Sc",
            Element::Se => "Se",
            Element::Si => "Si",
            Element::Sm => "Sm",
            Element::Sn => "Sn",
            Element::Sr => "Sr",
            Element::Ta => "Ta",
            Element::Tb => "Tb",
            Element::Tc => "Tc",
            Element::Te => "Te",
            Element::Th => "Th",
            Element::Ti => "Ti",
            Element::Tl => "Tl",
            Element::Tm => "Tm",
            Element::U => "U",
            Element::V => "V",
            Element::W => "W",
            Element::Xe => "Xe",
            Element::Y => "Y",
            Element::Yb => "Yb",
            Element::Zn => "Zn",
            Element::Zr => "Zr",
        })
    }
}

#[cfg(test)]
mod default_valence {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn disallowed() {
        let element = Element::H;

        assert_eq!(element.default_valences(), [])
    }

    #[test]
    fn boron() {
        let element = Element::B;

        assert_eq!(element.default_valences(), [3].as_ref())
    }
}
