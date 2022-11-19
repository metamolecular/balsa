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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn not_found() {
        let mut scanner = Scanner::new("x");

        assert_eq!(selection(&mut scanner), None)
    }

    #[test]
    fn found() {
        let mut scanner = Scanner::new("b");

        assert_eq!(selection(&mut scanner), Some(Selection::B))
    }
}
