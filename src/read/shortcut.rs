use lyn::{Action, Scanner};

use crate::feature::Shortcut;

pub fn shortcut(scanner: &mut Scanner) -> Result<Option<Shortcut>, lyn::Error> {
    scanner.scan(|symbol| match symbol {
        "B" => Some(Action::Request(Shortcut::B)),
        "Br" => Some(Action::Return(Shortcut::Br)),
        "C" => Some(Action::Request(Shortcut::C)),
        "Cl" => Some(Action::Return(Shortcut::Cl)),
        "N" => Some(Action::Return(Shortcut::N)),
        "O" => Some(Action::Return(Shortcut::O)),
        "F" => Some(Action::Return(Shortcut::F)),
        "I" => Some(Action::Return(Shortcut::I)),
        "P" => Some(Action::Return(Shortcut::P)),
        "S" => Some(Action::Return(Shortcut::S)),
        _ => None,
    })
}
