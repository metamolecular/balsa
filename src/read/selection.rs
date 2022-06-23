use lyn::Scanner;

use crate::feature::Selection;

pub fn selection(scanner: &mut Scanner) -> Option<Selection> {
    scanner.transform(|character| match character {
        'b' => Some(Selection::B),
        'c' => Some(Selection::C),
        'n' => Some(Selection::N),
        'o' => Some(Selection::O),
        'p' => Some(Selection::P),
        's' => Some(Selection::S),
        _ => None,
    })
}
