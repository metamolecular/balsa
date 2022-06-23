use std::cmp::{Ord, Ordering};
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};
use std::hash::{Hash, Hasher};

use crate::feature::Bridge;

#[derive(Eq, PartialEq, PartialOrd)]
struct Index(u8);

impl Ord for Index {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

#[derive(Debug, Eq)]
struct Pair(usize, usize);

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        (self.0.eq(&other.0) && self.1.eq(&other.1))
            || (self.0.eq(&other.1) && self.1.eq(&other.0))
    }
}

impl Hash for Pair {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        (self.0 + self.1).hash(hasher)
    }
}

pub struct BridgePool {
    counter: u8,
    borrowed: HashMap<Pair, u8>,
    replaced: BinaryHeap<Index>,
}

impl BridgePool {
    pub fn new() -> Self {
        Self {
            counter: 1,
            borrowed: HashMap::new(),
            replaced: BinaryHeap::new(),
        }
    }

    pub fn hit(&mut self, sid: usize, tid: usize) -> Bridge {
        let next = match self.replaced.pop() {
            Some(next) => next.0,
            None => {
                let next = self.counter;
                self.counter += 1;

                next
            }
        };

        match self.borrowed.entry(Pair(sid, tid)) {
            Entry::Occupied(occupied) => {
                let result = occupied.remove();

                self.replaced.push(Index(result));

                Bridge::new(result).expect("rnum")
            }
            Entry::Vacant(vacant) => {
                vacant.insert(next);

                Bridge::new(next).expect("rnum")
            }
        }
    }
}

#[cfg(test)]
mod pair {
    use super::*;

    #[test]
    fn hashmap() {
        let mut map = HashMap::new();

        map.insert(Pair(0, 1), 0);
        map.insert(Pair(1, 0), 1);

        assert_eq!(map.len(), 1)
    }
}

#[cfg(test)]
mod hit {
    use super::*;

    #[test]
    fn unknown() {
        let mut pool = BridgePool::new();

        assert_eq!(pool.hit(1, 2), Bridge::B1);
        assert_eq!(pool.hit(1, 5), Bridge::B2);
        assert_eq!(pool.hit(13, 42), Bridge::B3)
    }

    #[test]
    fn known() {
        let mut pool = BridgePool::new();

        assert_eq!(pool.hit(0, 1), Bridge::B1);
        assert_eq!(pool.hit(1, 0), Bridge::B1)
    }

    #[test]
    fn unknown_with_one_returned() {
        let mut pool = BridgePool::new();

        assert_eq!(pool.hit(0, 1), Bridge::B1);
        assert_eq!(pool.hit(1, 0), Bridge::B1);
        assert_eq!(pool.hit(13, 42), Bridge::B1)
    }

    #[test]
    fn unknown_with_two_returned() {
        let mut pool = BridgePool::new();

        assert_eq!(pool.hit(0, 1), Bridge::B1);
        assert_eq!(pool.hit(1, 3), Bridge::B2);
        assert_eq!(pool.hit(2, 4), Bridge::B3);
        assert_eq!(pool.hit(3, 1), Bridge::B2);
        assert_eq!(pool.hit(1, 0), Bridge::B1);
        assert_eq!(pool.hit(3, 5), Bridge::B1)
    }
}
