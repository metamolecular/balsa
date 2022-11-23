use lyn::Scanner;

use crate::feature::AtomKind;

use super::{bracket, selection, shortcut, Error};

pub fn atom(scanner: &mut Scanner) -> Result<Option<AtomKind>, Error> {
    if scanner.take(&'*') {
        Ok(Some(AtomKind::Star))
    } else if let Some(shortcut) = shortcut(scanner)? {
        Ok(Some(AtomKind::Shortcut(shortcut)))
    } else if let Some(selection) = selection(scanner) {
        Ok(Some(AtomKind::Selection(selection)))
    } else if let Some(bracket) = bracket(scanner)? {
        Ok(Some(AtomKind::Bracket(bracket)))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feature::{Bracket, Element, Selection, Shortcut, Symbol};
    use pretty_assertions::assert_eq;

    #[test]
    fn unrecognized() {
        let mut scanner = Scanner::new("X");

        assert_eq!(atom(&mut scanner), Ok(None))
    }

    #[test]
    fn star() {
        let mut scanner = Scanner::new("*");

        assert_eq!(atom(&mut scanner), Ok(Some(AtomKind::Star)))
    }

    #[test]
    fn selection() {
        let mut scanner = Scanner::new("c");

        assert_eq!(
            atom(&mut scanner),
            Ok(Some(AtomKind::Selection(Selection::C)))
        )
    }

    #[test]
    fn shortcut() {
        let mut scanner = Scanner::new("C");

        assert_eq!(
            atom(&mut scanner),
            Ok(Some(AtomKind::Shortcut(Shortcut::C)))
        )
    }

    #[test]
    fn bracket() {
        let mut scanner = Scanner::new("[C]");

        assert_eq!(
            atom(&mut scanner),
            Ok(Some(AtomKind::Bracket(Bracket {
                symbol: Symbol::Element(Element::C),
                ..Default::default()
            })))
        )
    }
}
