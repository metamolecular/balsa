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
