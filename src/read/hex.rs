use lyn::Scanner;

use super::digit;

pub fn hex(scanner: &mut Scanner) -> Option<u8> {
    match scanner.transform(|character| match character {
        'a' => Some(0x0a),
        'b' => Some(0x0b),
        'c' => Some(0x0c),
        'd' => Some(0x0d),
        'e' => Some(0x0e),
        'f' => Some(0x0f),
        _ => None,
    }) {
        Some(digit) => Some(digit),
        None => digit(scanner),
    }
}
