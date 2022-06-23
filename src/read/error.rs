use std::convert;

#[derive(Debug, PartialEq)]
pub enum Error {
    EndOfLine,
    Character(usize),
}

impl convert::From<lyn::Error> for Error {
    fn from(value: lyn::Error) -> Self {
        match value {
            lyn::Error::EndOfLine => Self::EndOfLine,
            lyn::Error::Character(pos) => Self::Character(pos),
        }
    }
}
