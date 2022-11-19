use lyn::Scanner;

use super::nonzero;

pub fn digit(scanner: &mut Scanner) -> Option<u8> {
    if scanner.take(&'0') {
        Some(0)
    } else {
        nonzero(scanner)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn not_found() {
        let mut scanner = Scanner::new("x");

        assert_eq!(digit(&mut scanner), None)
    }

    #[test]
    fn zero() {
        let mut scanner = Scanner::new("0");

        assert_eq!(digit(&mut scanner), Some(0))
    }

    #[test]
    fn non_zero() {
        let mut scanner = Scanner::new("1");

        assert_eq!(digit(&mut scanner), Some(1))
    }
}
