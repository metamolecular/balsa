use lyn::Scanner;

use crate::feature::BondKind;

pub fn bond(scanner: &mut Scanner) -> Option<BondKind> {
    scanner.transform(|target| match target {
        '-' => Some(BondKind::Single),
        '=' => Some(BondKind::Double),
        '#' => Some(BondKind::Triple),
        '/' => Some(BondKind::Up),
        '\\' => Some(BondKind::Down),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn invalid() {
        let mut scanner = Scanner::new("x");

        assert_eq!(bond(&mut scanner), None)
    }

    #[test]
    fn valid() {
        let mut scanner = Scanner::new("-");

        assert_eq!(bond(&mut scanner), Some(BondKind::Single))
    }
}
