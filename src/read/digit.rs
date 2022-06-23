use lyn::Scanner;

use super::nonzero;

pub fn digit(scanner: &mut Scanner) -> Option<u8> {
    if scanner.take(&'0') {
        Some(0)
    } else {
        nonzero(scanner)
    }
}
