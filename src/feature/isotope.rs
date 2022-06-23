use std;

#[derive(Debug, PartialEq, Clone)]
pub struct Isotope(u16);

impl Isotope {
    pub fn new(value: u16) -> Option<Self> {
        if value > 0 && value < 1000 {
            Some(Isotope(value))
        } else {
            None
        }
    }
}

impl std::fmt::Display for Isotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
