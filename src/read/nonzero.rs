use lyn::Scanner;

pub fn nonzero(scanner: &mut Scanner) -> Option<u8> {
    scanner.transform(|character| match character {
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        _ => None,
    })
}
