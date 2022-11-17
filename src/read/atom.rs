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
