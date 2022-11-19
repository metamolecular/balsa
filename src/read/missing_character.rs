use lyn::Scanner;

use super::Error;

pub fn missing_character(scanner: &mut Scanner) -> Error {
    if scanner.is_done() {
        Error::EndOfLine
    } else {
        Error::Character(scanner.cursor())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn not_done() {
        let mut scanner = Scanner::new("x");

        assert_eq!(missing_character(&mut scanner), Error::Character(0))
    }

    #[test]
    fn done() {
        let mut scanner = Scanner::new("");

        assert_eq!(missing_character(&mut scanner), Error::EndOfLine)
    }
}
