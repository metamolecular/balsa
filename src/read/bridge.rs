use lyn::Scanner;

use crate::feature::Bridge;

use super::{digit, missing_character, nonzero, Error};

pub fn bridge(scanner: &mut Scanner) -> Result<Option<Bridge>, Error> {
    if scanner.take(&'%') {
        if let Some(first) = nonzero(scanner) {
            if let Some(second) = digit(scanner) {
                Ok(Some(
                    Bridge::new(first * 10 + second).expect("bridge index"),
                ))
            } else {
                Err(missing_character(scanner))
            }
        } else {
            Err(missing_character(scanner))
        }
    } else if let Some(digit) = nonzero(scanner) {
        Ok(Some(Bridge::new(digit).expect("bridge index")))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_invalid() {
        let mut scanner = Scanner::new("%x");

        assert_eq!(bridge(&mut scanner), Err(Error::Character(1)))
    }

    #[test]
    fn percent_digit_invalid() {
        let mut scanner = Scanner::new("%1x");

        assert_eq!(bridge(&mut scanner), Err(Error::Character(2)))
    }

    #[test]
    fn percent_zero() {
        let mut scanner = Scanner::new("%0");

        assert_eq!(bridge(&mut scanner), Err(Error::Character(1)))
    }

    #[test]
    fn none() {
        let mut scanner = Scanner::new("x");

        assert_eq!(bridge(&mut scanner), Ok(None))
    }

    #[test]
    fn percent_digit_digit() {
        let mut scanner = Scanner::new("%42");

        assert_eq!(bridge(&mut scanner), Ok(Some(Bridge::B42)))
    }

    #[test]
    fn digit_zero() {
        let mut scanner = Scanner::new("0");

        assert_eq!(bridge(&mut scanner), Ok(None))
    }

    #[test]
    fn digit() {
        let mut scanner = Scanner::new("7");

        assert_eq!(bridge(&mut scanner), Ok(Some(Bridge::B7)))
    }
}
