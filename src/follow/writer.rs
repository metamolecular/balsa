use crate::{
    feature::{AtomKind, BondKind, Bridge},
    follow::Follower,
};

#[derive(Debug, PartialEq)]
pub struct Writer {
    base: String,
    stack: Vec<String>,
}

impl Writer {
    pub fn new() -> Self {
        Self {
            base: String::new(),
            stack: Vec::new(),
        }
    }

    pub fn write(self) -> String {
        self.base + &self.stack.join("")
    }
}

impl Follower for Writer {
    fn root(&mut self, root: &AtomKind) {
        let top = match self.stack.last_mut() {
            Some(string) => string,
            None => &mut self.base,
        };

        if !top.is_empty() {
            top.push('.');
        }

        top.push_str(&root.to_string());
    }

    fn extend(&mut self, bond_kind: &BondKind, atom_kind: &AtomKind) {
        let top = match self.stack.last_mut() {
            Some(string) => string,
            None => &mut self.base,
        };

        top.push_str(&bond_kind.to_string());
        top.push_str(&atom_kind.to_string());
    }

    fn bridge(&mut self, bond_kind: &BondKind, cut: &Bridge) {
        let top = match self.stack.last_mut() {
            Some(string) => string,
            None => &mut self.base,
        };

        top.push_str(&bond_kind.to_string());
        top.push_str(&cut.to_string());
    }

    fn push(&mut self) {
        self.stack.push("(".into());
    }

    fn pop(&mut self) {
        let top = self.stack.pop().expect("top");
        let last = match self.stack.last_mut() {
            Some(string) => string,
            None => &mut self.base,
        };

        last.push_str(&top);
        last.push(')');
    }
}

#[cfg(test)]
mod write {
    use crate::feature::Shortcut;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn p1() {
        let mut writer = Writer::new();

        writer.root(&AtomKind::Star);

        assert_eq!(writer.write(), "*")
    }

    #[test]
    fn p2() {
        let mut writer = Writer::new();

        writer.root(&AtomKind::Star);
        writer.extend(&BondKind::Single, &AtomKind::Star);

        assert_eq!(writer.write(), "*-*")
    }

    #[test]
    fn p1_p1() {
        let mut writer = Writer::new();

        writer.root(&AtomKind::Star);
        writer.root(&AtomKind::Star);

        assert_eq!(writer.write(), "*.*")
    }

    #[test]
    fn p1_p2() {
        let mut writer = Writer::new();

        writer.root(&AtomKind::Star);
        writer.root(&AtomKind::Star);
        writer.extend(&BondKind::Elided, &AtomKind::Star);

        assert_eq!(writer.write(), "*.**")
    }

    #[test]
    fn p3() {
        let mut writer = Writer::new();

        writer.root(&AtomKind::Star);
        writer.extend(&BondKind::Single, &AtomKind::Star);
        writer.extend(&BondKind::Single, &AtomKind::Star);

        assert_eq!(writer.write(), "*-*-*")
    }

    #[test]
    fn p3_branched() {
        let mut writer = Writer::new();

        writer.root(&AtomKind::Star);
        writer.push();
        writer.extend(&BondKind::Elided, &AtomKind::Shortcut(Shortcut::C));
        writer.pop();
        writer.extend(&BondKind::Elided, &AtomKind::Shortcut(Shortcut::N));

        assert_eq!(writer.write(), "*(C)N")
    }

    #[test]
    fn c3() {
        let mut writer = Writer::new();

        writer.root(&AtomKind::Star);
        writer.bridge(&BondKind::Single, &Bridge::B1);
        writer.extend(&BondKind::Single, &AtomKind::Star);
        writer.extend(&BondKind::Double, &AtomKind::Star);
        writer.bridge(&BondKind::Single, &Bridge::B1);

        assert_eq!(writer.write(), "*-1-*=*-1")
    }

    #[test]
    fn c3_branched() {
        let mut writer = Writer::new();

        writer.root(&AtomKind::Star);
        writer.push();
        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.bridge(&BondKind::Elided, &Bridge::B1);
        writer.pop();
        writer.bridge(&BondKind::Elided, &Bridge::B1);

        assert_eq!(writer.write(), "*(**1)1")
    }

    #[test]
    fn nested_branch() {
        let mut writer = Writer::new();

        writer.root(&AtomKind::Star);
        writer.push();
        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.push();
        writer.extend(&BondKind::Single, &AtomKind::Star);
        writer.pop();
        writer.extend(&BondKind::Elided, &AtomKind::Star);
        writer.pop();
        writer.extend(&BondKind::Double, &AtomKind::Star);

        assert_eq!(writer.write(), "*(*(-*)*)=*")
    }
}
