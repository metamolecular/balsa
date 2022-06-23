use lyn::Scanner;

pub fn uint16(scanner: &mut Scanner, limit: usize) -> Option<u16> {
    if scanner.is_done() || limit == 0 {
        return None;
    }

    let mut result = String::new();

    loop {
        if result.len() == limit {
            break Some(result.parse().expect("digits"));
        }

        let next = scanner.transform(|character| match character {
            '0' => Some('0'),
            '1' => Some('1'),
            '2' => Some('2'),
            '3' => Some('3'),
            '4' => Some('4'),
            '5' => Some('5'),
            '6' => Some('6'),
            '7' => Some('7'),
            '8' => Some('8'),
            '9' => Some('9'),
            _ => None,
        });

        match next {
            Some(next) => result.push(next),
            None => {
                break if result.is_empty() {
                    None
                } else {
                    Some(result.parse().expect("digits"))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn empty_zero() {
        let mut scanner = Scanner::new("");

        assert_eq!(uint16(&mut scanner, 0), None)
    }

    #[test]
    fn empty_nonzero() {
        let mut scanner = Scanner::new("");

        assert_eq!(uint16(&mut scanner, 1), None)
    }

    #[test]
    fn waiting_zero() {
        let mut scanner = Scanner::new("abc");

        assert_eq!(uint16(&mut scanner, 0), None)
    }

    #[test]
    fn mismatch_start() {
        let mut scanner = Scanner::new("abc");

        assert_eq!(uint16(&mut scanner, 1), None)
    }

    #[test]
    fn match_before_limit_two() {
        let mut scanner = Scanner::new("12c");

        assert_eq!(uint16(&mut scanner, 1), Some(1))
    }

    #[test]
    fn match_at_limit_one() {
        let mut scanner = Scanner::new("1bc");

        assert_eq!(uint16(&mut scanner, 1), Some(1))
    }

    #[test]
    fn match_at_limit_two() {
        let mut scanner = Scanner::new("12c");

        assert_eq!(uint16(&mut scanner, 2), Some(12))
    }

    #[test]
    fn match_after_limit_two() {
        let mut scanner = Scanner::new("123");

        assert_eq!(uint16(&mut scanner, 2), Some(12))
    }

    #[test]
    fn match_end_before_limit() {
        let mut scanner = Scanner::new("12");

        assert_eq!(uint16(&mut scanner, 3), Some(12))
    }
}
