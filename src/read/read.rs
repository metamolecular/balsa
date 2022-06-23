use lyn::Scanner;

use super::{
    bond, bracket, bridge, missing_character, selection, shortcut, Error,
};
use crate::{
    feature::{AtomKind, BondKind},
    follow::Follower,
};

pub fn read(string: &str, follower: &mut impl Follower) -> Result<(), Error> {
    let mut scanner = Scanner::new(string);

    sequence(None, &mut scanner, follower)?;

    if scanner.is_done() {
        Ok(())
    } else {
        Err(Error::Character(scanner.cursor()))
    }
}

fn sequence<F: Follower>(
    input: Option<&BondKind>,
    scanner: &mut Scanner,
    reporter: &mut F,
) -> Result<bool, Error> {
    let atom_kind = match atom(scanner)? {
        Some(kind) => kind,
        None => return Ok(false),
    };

    match &input {
        Some(bond_kind) => reporter.extend(bond_kind, &atom_kind),
        None => reporter.root(&atom_kind),
    }

    loop {
        if union(scanner, reporter)?
            || branch(scanner, reporter)?
            || split(scanner, reporter)?
        {
            continue;
        }

        break Ok(true);
    }
}

fn union<F: Follower>(
    scanner: &mut Scanner,
    reporter: &mut F,
) -> Result<bool, Error> {
    match bond(scanner) {
        Some(bond_kind) => {
            if cut_or_sequence(&bond_kind, scanner, reporter)? {
                Ok(true)
            } else {
                Err(missing_character(scanner))
            }
        }
        None => cut_or_sequence(&BondKind::Elided, scanner, reporter),
    }
}

fn cut_or_sequence<F: Follower>(
    bond_kind: &BondKind,
    scanner: &mut Scanner,
    reporter: &mut F,
) -> Result<bool, Error> {
    if let Some(bridge) = bridge(scanner)? {
        reporter.bridge(bond_kind, &bridge);

        Ok(true)
    } else {
        sequence(Some(bond_kind), scanner, reporter)
    }
}

fn branch<F: Follower>(
    scanner: &mut Scanner,
    reporter: &mut F,
) -> Result<bool, Error> {
    if !scanner.take(&'(') {
        return Ok(false);
    }

    reporter.push();

    if scanner.take(&'.') {
        if !sequence(None, scanner, reporter)? {
            return Err(missing_character(scanner));
        }
    } else {
        let bond_kind = match bond(scanner) {
            Some(bond_kind) => bond_kind,
            None => BondKind::Elided,
        };

        if !sequence(Some(&bond_kind), scanner, reporter)? {
            return Err(missing_character(scanner));
        }
    }

    if scanner.take(&')') {
        reporter.pop();

        Ok(true)
    } else {
        Err(missing_character(scanner))
    }
}

fn split<F: Follower>(
    scanner: &mut Scanner,
    reporter: &mut F,
) -> Result<bool, Error> {
    if !scanner.take(&'.') {
        return Ok(false);
    }

    if sequence(None, scanner, reporter)? {
        Ok(true)
    } else {
        Err(missing_character(scanner))
    }
}

fn atom(scanner: &mut Scanner) -> Result<Option<AtomKind>, Error> {
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
    use crate::follow::Writer;

    use super::*;

    #[test]
    fn blank() {
        let mut writer = Writer::new();

        read("", &mut writer).unwrap();

        assert_eq!(writer.write(), "")
    }

    #[test]
    fn leading_paren() {
        let mut writer = Writer::new();

        assert_eq!(read("(", &mut writer), Err(Error::Character(0)))
    }

    #[test]
    fn invalid_tail() {
        let mut writer = Writer::new();

        assert_eq!(read("*?", &mut writer), Err(Error::Character(1)))
    }

    #[test]
    fn trailing_bond() {
        let mut writer = Writer::new();

        assert_eq!(read("*-", &mut writer), Err(Error::EndOfLine))
    }

    #[test]
    fn trailing_dot() {
        let mut writer = Writer::new();

        assert_eq!(read("*.", &mut writer), Err(Error::EndOfLine))
    }

    #[test]
    fn open_paran_eol() {
        let mut writer = Writer::new();

        assert_eq!(read("*(", &mut writer), Err(Error::EndOfLine))
    }

    #[test]
    fn missing_close_paren() {
        let mut writer = Writer::new();

        assert_eq!(read("*(*", &mut writer), Err(Error::EndOfLine))
    }

    #[test]
    fn bond_to_invalid() {
        let mut writer = Writer::new();

        assert_eq!(read("*-X", &mut writer), Err(Error::Character(2)))
    }

    #[test]
    fn split_to_invalid() {
        let mut writer = Writer::new();

        assert_eq!(read("*.X", &mut writer), Err(Error::Character(2)))
    }

    #[test]
    fn bond_dot() {
        let mut writer = Writer::new();

        assert_eq!(read("*-.", &mut writer), Err(Error::Character(2)))
    }

    #[test]
    fn branch_invalid() {
        let mut writer = Writer::new();

        assert_eq!(read("*(X", &mut writer), Err(Error::Character(2)))
    }

    #[test]
    fn branch_rnum() {
        let mut writer = Writer::new();

        assert_eq!(read("*(1)*", &mut writer), Err(Error::Character(2)))
    }

    #[test]
    fn branch_bond_rnum() {
        let mut writer = Writer::new();

        assert_eq!(read("*(-1", &mut writer), Err(Error::Character(3)))
    }

    #[test]
    fn dot_rnum() {
        let mut writer = Writer::new();

        assert_eq!(read("*.1", &mut writer), Err(Error::Character(2)))
    }

    #[test]
    fn branch_split_eol() {
        let mut writer = Writer::new();

        assert_eq!(read("*(.", &mut writer), Err(Error::EndOfLine))
    }

    #[test]
    fn branch_split_invalid() {
        let mut writer = Writer::new();

        assert_eq!(read("*(.x", &mut writer), Err(Error::Character(3)))
    }

    #[test]
    fn p1() {
        let mut writer = Writer::new();

        read("*", &mut writer).unwrap();

        assert_eq!(writer.write(), "*")
    }

    #[test]
    fn shortcut_c() {
        let mut writer = Writer::new();

        read("C", &mut writer).unwrap();

        assert_eq!(writer.write(), "C")
    }

    #[test]
    fn shortcut_c_selected() {
        let mut writer = Writer::new();

        read("cc", &mut writer).unwrap();

        assert_eq!(writer.write(), "cc")
    }

    #[test]
    fn shortcut_cl() {
        let mut writer = Writer::new();

        read("Cl", &mut writer).unwrap();

        assert_eq!(writer.write(), "Cl")
    }

    #[test]
    fn bracket() {
        let mut writer = Writer::new();

        read("[CH4]", &mut writer).unwrap();

        assert_eq!(writer.write(), "[CH4]")
    }

    #[test]
    fn elided_cut() {
        let mut writer = Writer::new();

        read("*1", &mut writer).unwrap();

        assert_eq!(writer.write(), "*1")
    }

    #[test]
    fn single_cut() {
        let mut writer = Writer::new();

        read("*-1", &mut writer).unwrap();

        assert_eq!(writer.write(), "*-1")
    }

    #[test]
    fn p1_p1() {
        let mut writer = Writer::new();

        read("*.*", &mut writer).unwrap();

        assert_eq!(writer.write(), "*.*")
    }

    #[test]
    fn p1_p2_branched_inner() {
        let mut writer = Writer::new();

        read("*(.*)*", &mut writer).unwrap();

        assert_eq!(writer.write(), "*(.*)*")
    }

    #[test]
    fn p2_elided() {
        let mut writer = Writer::new();

        read("**", &mut writer).unwrap();

        assert_eq!(writer.write(), "**")
    }

    #[test]
    fn p2_single() {
        let mut writer = Writer::new();

        read("*-*", &mut writer).unwrap();

        assert_eq!(writer.write(), "*-*")
    }

    #[test]
    fn p3_elided() {
        let mut writer = Writer::new();

        read("***", &mut writer).unwrap();

        assert_eq!(writer.write(), "***")
    }

    #[test]
    fn p3_branched_elided() {
        let mut writer = Writer::new();

        read("*(F)Cl", &mut writer).unwrap();

        assert_eq!(writer.write(), "*(F)Cl")
    }

    #[test]
    fn p4_branched_elided() {
        let mut writer = Writer::new();

        read("*(**)*", &mut writer).unwrap();

        assert_eq!(writer.write(), "*(**)*")
    }

    #[test]
    fn p4_branched_outside() {
        let mut writer = Writer::new();

        read("*(-*)=**", &mut writer).unwrap();

        assert_eq!(writer.write(), "*(-*)=**")
    }

    #[test]
    fn s3_internal() {
        let mut writer = Writer::new();

        read("*(*)(*)*", &mut writer).unwrap();

        assert_eq!(writer.write(), "*(*)(*)*")
    }

    #[test]
    fn double_nested() {
        let mut writer = Writer::new();

        read("*(*(*-*)*)*", &mut writer).unwrap();

        assert_eq!(writer.write(), "*(*(*-*)*)*")
    }

    #[test]
    fn triple_nested() {
        let mut writer = Writer::new();

        read("*(-*(=*)*)*", &mut writer).unwrap();

        assert_eq!(writer.write(), "*(-*(=*)*)*")
    }

    #[test]
    fn s4_internal() {
        let mut writer = Writer::new();

        read("*(-*)(=*)(#*)*", &mut writer).unwrap();

        assert_eq!(writer.write(), "*(-*)(=*)(#*)*")
    }

    #[test]
    fn s4_external() {
        let mut writer = Writer::new();

        read("**(-*)(=*)*", &mut writer).unwrap();

        assert_eq!(writer.write(), "**(-*)(=*)*")
    }

    #[test]
    fn branch_ordering() {
        let mut writer = Writer::new();

        read("C(F)Cl", &mut writer).unwrap();

        assert_eq!(writer.write(), "C(F)Cl")
    }
}
