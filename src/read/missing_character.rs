use lyn::Scanner;

use super::Error;

pub fn missing_character(scanner: &mut Scanner) -> Error {
    if scanner.is_done() {
        Error::EndOfLine
    } else {
        Error::Character(scanner.cursor())
    }
}
